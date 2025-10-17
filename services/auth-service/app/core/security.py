"""Security utilities for password hashing and JWT tokens."""

from datetime import UTC, datetime, timedelta
from typing import Any

from jose import jwt
from passlib.context import CryptContext

from app.core.config import settings

pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")


def verify_password(plain_password: str, hashed_password: str) -> bool:
    """Verify a password against its hash."""
    result: bool = pwd_context.verify(plain_password, hashed_password)
    return result


def get_password_hash(password: str) -> str:
    """Generate password hash."""
    hashed: str = pwd_context.hash(password)
    return hashed


def create_access_token(subject: str | Any, expires_delta: timedelta | None = None) -> str:
    """Create JWT access token."""
    if expires_delta:
        expire = datetime.now(UTC) + expires_delta
    else:
        expire = datetime.now(UTC) + timedelta(minutes=settings.ACCESS_TOKEN_EXPIRE_MINUTES)

    to_encode = {"exp": expire, "sub": str(subject)}
    encoded_jwt: str = jwt.encode(to_encode, settings.SECRET_KEY, algorithm=settings.ALGORITHM)
    return encoded_jwt


def verify_token(token: str) -> dict[str, Any]:
    """Verify and decode JWT token."""
    payload: dict[str, Any] = jwt.decode(
        token, settings.SECRET_KEY, algorithms=[settings.ALGORITHM]
    )
    return payload
