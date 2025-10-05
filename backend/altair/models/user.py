"""User model for authentication and profile management.

This module defines the User model which stores user account information,
authentication credentials, and ADHD-specific preferences. The model supports
the authentication system and personalized ADHD-friendly features.
"""

from typing import TYPE_CHECKING, Optional
from sqlalchemy import Boolean, String
from sqlalchemy.dialects.postgresql import JSON
from sqlalchemy.orm import Mapped, mapped_column, relationship
from altair.models.base import BaseModel

if TYPE_CHECKING:
    from altair.models.task import Task


class User(BaseModel):
    """User model with authentication and ADHD profile support.

    Stores user account information including authentication credentials and
    personalized ADHD preferences. The adhd_profile field uses PostgreSQL's
    JSONB type for flexible storage of user-specific settings.

    Attributes:
        email (str): User's email address, unique identifier for login, indexed
        username (str): Optional display name, unique if provided, indexed
        hashed_password (str): Argon2 hashed password, never returned in API responses
        is_active (bool): Account status flag, defaults to True (inactive users can't login)
        adhd_profile (dict): JSONB field storing user preferences like:
            - preferred_focus_duration: Minutes for Pomodoro sessions
            - break_duration: Minutes for break periods
            - notification_preferences: Sound/visual notification settings
            - sensory_preferences: UI customizations for sensory needs
            - best_focus_times: Times of day when user focuses best
            - common_distractions: Known distraction triggers
        tasks (list[Task]): Relationship to all tasks owned by this user

    Inherited Attributes:
        id (UUID): Unique identifier from BaseModel
        created_at (DateTime): Account creation timestamp from BaseModel
        updated_at (DateTime): Last profile update timestamp from BaseModel

    Example:
        # Create new user with minimal information
        user = User(
            email="user@example.com",
            hashed_password=hash_password("secure_password"),
            is_active=True
        )

        # User with ADHD preferences
        user = User(
            email="adhd@example.com",
            username="focused_dev",
            hashed_password=hash_password("password123"),
            adhd_profile={
                "preferred_focus_duration": 25,
                "break_duration": 5,
                "notification_preferences": {"sound": False, "visual": True},
                "best_focus_times": ["morning", "late_night"]
            }
        )

        # Access user's tasks
        task_count = len(user.tasks)
        active_tasks = [t for t in user.tasks if t.state == TaskState.ACTIVE]

    Note:
        The hashed_password field should never be included in API responses.
        Use the UserResponse schema to exclude sensitive fields.
    """

    __tablename__ = "users"

    # Email address serves as the primary login identifier
    email: Mapped[str] = mapped_column(
        String(255), nullable=False, unique=True, index=True
    )

    # Optional username for display purposes
    username: Mapped[Optional[str]] = mapped_column(
        String(50), unique=True, nullable=True, index=True
    )

    # Argon2 hashed password (up to 1024 chars for future-proofing)
    hashed_password: Mapped[str] = mapped_column(String(1024))

    # Account status - inactive users cannot login
    is_active: Mapped[bool] = mapped_column(Boolean, default=True)

    # ADHD-specific preferences and settings stored as JSONB
    adhd_profile: Mapped[dict] = mapped_column(JSON, default=dict)

    tasks: Mapped[list["Task"]] = relationship("Task", back_populates="user")
