"""User management endpoints."""

from fastapi import APIRouter, HTTPException, status
from pydantic import BaseModel, EmailStr

router = APIRouter()


class UserResponse(BaseModel):
    """User response model."""

    id: str
    email: EmailStr
    created_at: str


@router.get("/me", response_model=UserResponse)
async def get_current_user() -> UserResponse:
    """Get current authenticated user."""
    # TODO: Implement user retrieval from JWT token
    raise HTTPException(
        status_code=status.HTTP_501_NOT_IMPLEMENTED,
        detail="User retrieval not yet implemented",
    )


@router.get("/{user_id}", response_model=UserResponse)
async def get_user(user_id: str) -> UserResponse:
    """Get user by ID."""
    # TODO: Implement user retrieval by ID
    raise HTTPException(
        status_code=status.HTTP_501_NOT_IMPLEMENTED,
        detail="User retrieval not yet implemented",
    )
