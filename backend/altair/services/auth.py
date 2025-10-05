"""Authentication service for password hashing and JWT token management."""

from datetime import datetime, timedelta, timezone
from typing import Any

from fastapi import HTTPException, status
from jose import JWTError, jwt
from passlib.context import CryptContext

from altair.config import settings
from altair.redis import get_redis_client

# Password hashing context using Argon2 (modern, recommended algorithm)
pwd_context = CryptContext(schemes=["argon2"], deprecated="auto")


def verify_password(plain_password: str, hashed_password: str) -> bool:
    """Verify a plain password against a hashed password."""
    return pwd_context.verify(plain_password, hashed_password)


def hash_password(plain_password: str) -> str:
    """Hash a plain password using Argon2."""
    return pwd_context.hash(plain_password)


def create_access_token(
    data: dict[str, Any], expires_delta: timedelta | None = None
) -> str:
    """Create a JWT access token.

    Args:
        data: Payload data to encode in the token (typically {"sub": email})
        expires_delta: Optional custom expiration time, defaults to
            ACCESS_TOKEN_EXPIRE_MINUTES

    Returns:
        Encoded JWT access token string
    """
    to_encode = data.copy()

    if expires_delta:
        expire = datetime.now(timezone.utc) + expires_delta
    else:
        expire = datetime.now(timezone.utc) + timedelta(
            minutes=settings.ACCESS_TOKEN_EXPIRE_MINUTES
        )

    to_encode.update(
        {"exp": expire, "type": "access"}  # Distinguish from refresh tokens
    )

    return jwt.encode(to_encode, settings.SECRET_KEY, algorithm=settings.ALGORITHM)


def create_refresh_token(data: dict[str, Any]) -> str:
    """Create a JWT refresh token (long-lived).

    Args:
        data: Payload data to encode in the token (typically {"sub": email})

    Returns:
        Encoded JWT refresh token string
    """
    to_encode = data.copy()
    expire = datetime.now(timezone.utc) + timedelta(
        days=settings.REFRESH_TOKEN_EXPIRE_DAYS
    )

    to_encode.update({"exp": expire, "type": "refresh"})

    return jwt.encode(to_encode, settings.SECRET_KEY, algorithm=settings.ALGORITHM)


def create_token_pair(email: str) -> dict[str, Any]:
    """Create both access and refresh tokens for a user.

    Args:
        email: User's email address to embed in token

    Returns:
        Dictionary containing access_token, refresh_token, token_type, and expires_in
    """
    token_data = {
        "sub": email
    }  # "sub" is JWT standard claim for subject (user identifier)

    access_token = create_access_token(token_data)
    refresh_token = create_refresh_token(token_data)

    return {
        "access_token": access_token,
        "refresh_token": refresh_token,
        "token_type": "bearer",
        "expires_in": settings.ACCESS_TOKEN_EXPIRE_MINUTES * 60,  # Convert to seconds
    }


def verify_token(token: str, expected_type: str = "access") -> dict[str, Any]:
    """Verify and decode a JWT token.

    Args:
        token: JWT token string to verify
        expected_type: Expected token type ("access" or "refresh")

    Returns:
        Decoded token payload

    Raises:
        HTTPException: If token is invalid, expired, or wrong type
    """
    credentials_exception = HTTPException(
        status_code=status.HTTP_401_UNAUTHORIZED,
        detail="Could not validate credentials",
        headers={"WWW-Authenticate": "Bearer"},
    )

    try:
        payload = jwt.decode(
            token, settings.SECRET_KEY, algorithms=[settings.ALGORITHM]
        )
        email: str | None = payload.get("sub")
        token_type: str | None = payload.get("type")

        if email is None or token_type != expected_type:
            raise credentials_exception

        return payload

    except JWTError:
        raise credentials_exception


def blacklist_token(token: str, token_type: str = "access") -> None:
    """Add a token to the blacklist (revoke it).

    Stores the token in Redis with an expiration matching the token's lifetime.
    This prevents revoked tokens from being used until they naturally expire.

    Args:
        token: JWT token string to blacklist
        token_type: Type of token ("access" or "refresh")

    Note:
        Uses Redis SETEX to automatically remove blacklisted tokens after expiration.
    """
    try:
        redis_client = get_redis_client()
        payload = jwt.decode(
            token, settings.SECRET_KEY, algorithms=[settings.ALGORITHM]
        )

        # Calculate time until token expires
        exp_timestamp = payload.get("exp")
        if exp_timestamp:
            now = datetime.now(timezone.utc).timestamp()
            ttl = int(exp_timestamp - now)

            if ttl > 0:
                # Store token in blacklist with expiration
                redis_client.setex(f"blacklist:{token}", ttl, token_type)
    except (JWTError, Exception):
        # If token is invalid or Redis fails, it's not critical
        # The token will fail validation anyway
        pass


def is_token_blacklisted(token: str) -> bool:
    """Check if a token has been blacklisted (revoked).

    Args:
        token: JWT token string to check

    Returns:
        bool: True if token is blacklisted, False otherwise
    """
    try:
        redis_client = get_redis_client()
        result: int = redis_client.exists(f"blacklist:{token}")  # type: ignore[assignment]
        return result > 0
    except Exception:
        # If Redis is unavailable, fail open (allow the token)
        # This prevents Redis outages from breaking all authentication
        return False
