"""Authentication API endpoints for user registration, login, and token refresh."""

from typing import Annotated

from fastapi import APIRouter, Depends, HTTPException, Request, status
from fastapi.security import OAuth2PasswordRequestForm
from pydantic import BaseModel
from slowapi import Limiter
from slowapi.util import get_remote_address
from sqlalchemy.orm import Session

from altair.database import get_db
from altair.dependencies import get_current_user, oauth2_scheme
from altair.models.user import User
from altair.schemas.auth import Token, UserCreate, UserResponse
from altair.services.auth import (
    blacklist_token,
    create_token_pair,
    hash_password,
    verify_password,
    verify_token,
)

router = APIRouter(prefix="/api/auth", tags=["auth"])

# Initialize rate limiter for auth endpoints
limiter = Limiter(key_func=get_remote_address)


@router.post(
    "/register", response_model=UserResponse, status_code=status.HTTP_201_CREATED
)
@limiter.limit("3/minute")
async def register(
    request: Request, user_data: UserCreate, db: Annotated[Session, Depends(get_db)]
) -> User:
    """Register a new user account.

    Creates a new user with the provided email and password. Email and username
    (if provided) must be unique. Password is automatically hashed before storage.

    Args:
        user_data: User registration data (email, password, optional username)
        db: Database session

    Returns:
        Created user object (without password)

    Raises:
        HTTPException 400: If email or username already exists
    """
    # Check if user already exists
    existing_user = db.query(User).filter(User.email == user_data.email).first()
    if existing_user:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST, detail="Email already registered"
        )

    # Check username uniqueness if provided
    if user_data.username:
        existing_username = (
            db.query(User).filter(User.username == user_data.username).first()
        )
        if existing_username:
            raise HTTPException(
                status_code=status.HTTP_400_BAD_REQUEST, detail="Username already taken"
            )

    # Create new user
    new_user = User(
        email=user_data.email,
        username=user_data.username,
        hashed_password=hash_password(user_data.password),
        is_active=True,
    )

    db.add(new_user)
    db.commit()
    db.refresh(new_user)

    return new_user


@router.post("/login", response_model=Token)
@limiter.limit("5/minute")
async def login(
    request: Request,
    form_data: Annotated[OAuth2PasswordRequestForm, Depends()],
    db: Annotated[Session, Depends(get_db)],
) -> dict:
    """Login and receive access + refresh tokens.

    Authenticates a user with email and password, returning both access and
    refresh tokens for subsequent API requests.

    Args:
        form_data: OAuth2 form with username (email) and password
        db: Database session

    Returns:
        Token pair with access_token, refresh_token, token_type, and expires_in

    Raises:
        HTTPException 401: If credentials are invalid
        HTTPException 403: If user account is inactive
    """
    # OAuth2PasswordRequestForm uses 'username' field, but we use email
    user = db.query(User).filter(User.email == form_data.username).first()

    if not user or not verify_password(form_data.password, user.hashed_password):
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Incorrect email or password",
            headers={"WWW-Authenticate": "Bearer"},
        )

    if not user.is_active:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN, detail="Inactive user"
        )

    # Generate token pair
    tokens = create_token_pair(user.email)
    return tokens


class RefreshRequest(BaseModel):
    """Request schema for token refresh endpoint."""

    refresh_token: str


@router.post("/refresh", response_model=Token)
@limiter.limit("10/minute")
async def refresh_tokens(
    request: Request,
    refresh_request: RefreshRequest,
    db: Annotated[Session, Depends(get_db)],
) -> dict:
    """Exchange refresh token for new access token.

    Takes a valid refresh token and returns a new token pair. This implements
    refresh token rotation for better security.

    Args:
        request: FastAPI Request object (for rate limiting)
        refresh_request: Request containing the refresh token
        db: Database session

    Returns:
        New token pair with access_token, refresh_token, token_type, and expires_in

    Raises:
        HTTPException 401: If refresh token is invalid or user not found/inactive
    """
    # Verify refresh token
    payload = verify_token(refresh_request.refresh_token, expected_type="refresh")
    email = payload.get("sub")

    # Verify user still exists and is active
    user = db.query(User).filter(User.email == email).first()
    if not user or not user.is_active:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED, detail="Invalid refresh token"
        )

    # Generate new token pair (implements token rotation)
    tokens = create_token_pair(user.email)
    return tokens


@router.get("/me", response_model=UserResponse)
async def get_current_user_info(
    current_user: Annotated[User, Depends(get_current_user)],
) -> User:
    """Get current authenticated user information.

    Returns the profile information for the currently authenticated user
    based on the JWT token provided in the Authorization header.

    Args:
        current_user: Current authenticated user from JWT token

    Returns:
        Current user object (without password)

    Raises:
        HTTPException 401: If token is invalid or user not found
    """
    return current_user


@router.post("/logout")
@limiter.limit("10/minute")
async def logout(
    request: Request,
    token: Annotated[str, Depends(oauth2_scheme)],
    current_user: Annotated[User, Depends(get_current_user)],
) -> dict:
    """Logout and revoke the current access token.

    Adds the current access token to the blacklist in Redis, preventing it
    from being used for further API calls. The token remains blacklisted
    until it naturally expires.

    Args:
        request: FastAPI request object (for rate limiting)
        token: Current access token from Authorization header
        current_user: Current authenticated user (validates token is active)

    Returns:
        Success message confirming logout

    Note:
        This implements server-side token revocation. Clients should also
        delete tokens from local storage for security.
    """
    # Blacklist the current access token
    blacklist_token(token, token_type="access")

    return {"message": "Successfully logged out"}
