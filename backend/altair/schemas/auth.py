"""Pydantic schemas for authentication and user management.

This module defines the data validation schemas for user registration, login,
and JWT token handling using Pydantic models. These schemas provide request
validation, response serialization, and automatic API documentation.
"""

from datetime import datetime
from typing import Optional
from uuid import UUID

from pydantic import BaseModel, ConfigDict, EmailStr, Field


class UserBase(BaseModel):
    """Base user schema with common fields.

    Contains the minimal user information shared across multiple schemas.
    Other user schemas inherit from this to ensure consistency.

    Attributes:
        email: User's email address (validated format)
    """

    email: EmailStr


class UserCreate(UserBase):
    """Schema for user registration requests.

    Used when creating a new user account. Validates password strength
    and optional username format.

    Attributes:
        email: User's email address (inherited from UserBase)
        password: Plain text password, min 8 characters (hashed before storage)
        username: Optional username, 3-50 characters if provided

    Note:
        Password complexity rules are currently basic (min length only).
        TODO: Make password complexity rules configurable.
    """

    # TODO: Make password complexity rules a configurable setting
    password: str = Field(min_length=8)
    username: Optional[str] = Field(default=None, min_length=3, max_length=50)


class UserLogin(UserBase):
    """Schema for user login requests.

    Used for authentication endpoints. No validation on password since
    it's being checked against stored hash, not creating new password.

    Attributes:
        email: User's email address (inherited from UserBase)
        password: Plain text password for verification
    """

    password: str  # No validation since already exists


class UserUpdate(BaseModel):
    """Schema for updating user profile information.

    All fields are optional to allow partial updates. Used for profile
    modification endpoints.

    Attributes:
        username: New username (3-50 chars) or None
        password: New password (min 8 chars) or None
        adhd_profile: User's ADHD preferences and settings (JSONB dict)

    Example adhd_profile:
        {
            "preferred_focus_duration": 25,
            "break_duration": 5,
            "notification_preferences": {"sound": true, "visual": true},
            "best_focus_times": ["morning", "late_night"]
        }
    """

    username: Optional[str] = Field(default=None, min_length=3, max_length=50)
    password: Optional[str] = Field(default=None, min_length=8)
    adhd_profile: Optional[dict] = None


class UserResponse(UserBase):
    """Schema for user data in API responses.

    Returns user information without sensitive data (password excluded).
    Configured to work with SQLAlchemy ORM models via from_attributes.

    Attributes:
        id: User's unique UUID identifier
        email: User's email address (inherited from UserBase)
        username: User's username (may be None)
        created_at: Account creation timestamp

    Configuration:
        from_attributes=True enables automatic conversion from ORM models
    """

    id: UUID
    username: Optional[str]
    created_at: datetime

    model_config = ConfigDict(from_attributes=True)


class Token(BaseModel):
    """Schema for JWT token pairs returned after authentication.

    Contains both access token (short-lived) and refresh token (long-lived)
    following OAuth2 bearer token standards.

    Attributes:
        access_token: JWT access token for API authentication
        refresh_token: JWT refresh token for obtaining new access tokens
        token_type: Always "bearer" for OAuth2 compliance
        expires_in: Access token lifetime in seconds (e.g., 1800 for 30 min)

    Example:
        {
            "access_token": "eyJhbGc...",
            "refresh_token": "eyJhbGc...",
            "token_type": "bearer",
            "expires_in": 1800
        }
    """

    access_token: str
    refresh_token: str
    token_type: str
    expires_in: int  # Seconds until expiration (e.g., 900 for 15 min)


class TokenData(BaseModel):
    """Schema for decoded JWT token payload data.

    Used internally to validate and type-check decoded JWT claims.
    The email field corresponds to the JWT "sub" (subject) claim.

    Attributes:
        email: User's email extracted from token, None if invalid
    """

    email: Optional[EmailStr] = None
