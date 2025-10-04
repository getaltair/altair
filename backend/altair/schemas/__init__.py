"""Schema exports for the Altair API."""

from altair.schemas.auth import (
    Token,
    TokenData,
    UserBase,
    UserCreate,
    UserLogin,
    UserResponse,
    UserUpdate,
)
from altair.schemas.task import TaskBase, TaskCreate, TaskResponse, TaskUpdate

__all__ = [
    # Auth schemas
    "Token",
    "TokenData",
    "UserBase",
    "UserCreate",
    "UserLogin",
    "UserResponse",
    "UserUpdate",
    # Task schemas
    "TaskBase",
    "TaskCreate",
    "TaskResponse",
    "TaskUpdate",
]
