"""FastAPI dependencies for authentication and database access.

This module provides dependency injection functions for FastAPI endpoints,
particularly for extracting and validating the current authenticated user
from JWT tokens.
"""

from typing import Annotated

from fastapi import Depends, HTTPException, status
from fastapi.security import OAuth2PasswordBearer
from sqlalchemy.orm import Session

from altair.database import get_db
from altair.models.user import User
from altair.services.auth import verify_token

# OAuth2 scheme - tells FastAPI where to look for token
# tokenUrl points to the login endpoint that will issue tokens
oauth2_scheme = OAuth2PasswordBearer(tokenUrl="/api/auth/login")


async def get_current_user(
    token: Annotated[str, Depends(oauth2_scheme)],
    db: Annotated[Session, Depends(get_db)],
) -> User:
    """Dependency that extracts and validates current user from JWT.

    This dependency is used to protect routes that require authentication.
    It extracts the JWT from the Authorization header, validates it, and
    returns the corresponding User object from the database.

    Args:
        token: JWT access token from Authorization header
        db: Database session

    Returns:
        User object for the authenticated user

    Raises:
        HTTPException: If token is invalid or user not found/inactive

    Example:
        @router.get("/api/tasks")
        async def list_tasks(current_user: User = Depends(get_current_user)):
            return {"tasks": get_user_tasks(current_user.id)}
    """
    payload = verify_token(token, expected_type="access")
    email = payload.get("sub")

    user = db.query(User).filter(User.email == email).first()
    if user is None:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED, detail="User not found"
        )

    if not user.is_active:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN, detail="Inactive user"
        )

    return user
