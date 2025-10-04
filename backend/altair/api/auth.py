"""Authentication API endpoints for user registration, login, and token refresh."""

from typing import Annotated

from fastapi import APIRouter, Depends, HTTPException, status
from fastapi.security import OAuth2PasswordRequestForm
from pydantic import BaseModel
from sqlalchemy.orm import Session

from altair.database import get_db
from altair.models.user import User
from altair.schemas.auth import Token, UserCreate, UserResponse
from altair.services.auth import (
    create_token_pair,
    hash_password,
    verify_password,
    verify_token,
)

router = APIRouter(prefix="/api/auth", tags=["auth"])


@router.post("/register", response_model=UserResponse, status_code=status.HTTP_201_CREATED)
async def register(
    user_data: UserCreate,
    db: Annotated[Session, Depends(get_db)]
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
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Email already registered"
        )

    # Check username uniqueness if provided
    if user_data.username:
        existing_username = db.query(User).filter(User.username == user_data.username).first()
        if existing_username:
            raise HTTPException(
                status_code=status.HTTP_400_BAD_REQUEST,
                detail="Username already taken"
            )

    # Create new user
    new_user = User(
        email=user_data.email,
        username=user_data.username,
        hashed_password=hash_password(user_data.password),
        is_active=True
    )

    db.add(new_user)
    db.commit()
    db.refresh(new_user)

    return new_user


@router.post("/login", response_model=Token)
async def login(
    form_data: Annotated[OAuth2PasswordRequestForm, Depends()],
    db: Annotated[Session, Depends(get_db)]
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
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Inactive user"
        )

    # Generate token pair
    tokens = create_token_pair(user.email)
    return tokens


class RefreshRequest(BaseModel):
    """Request schema for token refresh endpoint."""
    refresh_token: str


@router.post("/refresh", response_model=Token)
async def refresh_tokens(
    request: RefreshRequest,
    db: Annotated[Session, Depends(get_db)]
) -> dict:
    """Exchange refresh token for new access token.

    Takes a valid refresh token and returns a new token pair. This implements
    refresh token rotation for better security.

    Args:
        request: Request containing the refresh token
        db: Database session

    Returns:
        New token pair with access_token, refresh_token, token_type, and expires_in

    Raises:
        HTTPException 401: If refresh token is invalid or user not found/inactive
    """
    # Verify refresh token
    payload = verify_token(request.refresh_token, expected_type="refresh")
    email = payload.get("sub")

    # Verify user still exists and is active
    user = db.query(User).filter(User.email == email).first()
    if not user or not user.is_active:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Invalid refresh token"
        )

    # Generate new token pair (implements token rotation)
    tokens = create_token_pair(user.email)
    return tokens
