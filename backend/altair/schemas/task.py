"""Task schema definitions for the Altair ADHD-friendly task manager.

This module contains Pydantic models for task data validation and serialization,
specifically designed to support ADHD users with cognitive load tracking and
minimal-friction task capture.

Key components:
    - TaskBase: Base schema with common task fields
    - TaskCreate: Schema for creating new tasks
    - TaskUpdate: Schema for updating existing tasks
    - TaskResponse: Schema for task API responses
"""

from datetime import datetime
from typing import Optional
from uuid import UUID

from pydantic import BaseModel, ConfigDict, field_validator

from altair.models.task import TaskState


class TaskBase(BaseModel):
    """Base task schema with common fields for task creation and responses.

    This schema captures the essential information for ADHD-friendly task management,
    focusing on cognitive load awareness and flexible time estimation.

    Attributes:
        title: The task title or brief description
        description: Optional detailed description or notes about the task
        cognitive_load: Mental effort scale from 1-10, validated to ensure
            reasonable values
        estimated_minutes: User's estimate of time needed in minutes
    """

    title: str
    description: Optional[str] = None
    cognitive_load: Optional[int] = 5
    estimated_minutes: Optional[int] = None

    @field_validator("cognitive_load")
    @classmethod
    def validate_cognitive_load(cls, v: int | None) -> int | None:
        """Validate cognitive load is within the 1-10 scale."""
        if v is not None and (v < 1 or v > 10):
            raise ValueError("Cognitive load must be between 1 and 10")
        return v


class TaskCreate(TaskBase):
    """Schema for creating new tasks via the API.

    Inherits all fields from TaskBase and allows optional state specification.
    Tasks default to INBOX state if not specified, requiring no immediate
    decisions from the user to reduce decision fatigue.

    Attributes:
        state: Optional task state (defaults to INBOX in model)

    Example:
        ```python
        task = TaskCreate(
            title="Review pull request #42",
            cognitive_load=7,
            estimated_minutes=30
        )
        ```
    """

    state: Optional[str] = None

    @field_validator("state")
    @classmethod
    def validate_state(cls, v: str | None) -> str | None:
        """Validate state is a valid TaskState enum value."""
        if v and v not in [s.value for s in TaskState]:
            raise ValueError(f"Invalid state: {v}")
        return v


class TaskUpdate(BaseModel):
    """Schema for updating existing tasks.

    All fields are optional to support partial updates. Includes the state field
    to allow task progression through the ADHD-friendly state machine
    (INBOX → TRIAGE → ACTIVE/BLOCKED/SOMEDAY → DONE → ARCHIVED).

    Attributes:
        title: Updated task title
        description: Updated task description
        state: Task state transition (validated against TaskState enum)
        cognitive_load: Updated mental effort estimate (1-10, validated)
        estimated_minutes: Updated time estimate in minutes

    Example:
        ```python
        update = TaskUpdate(
            state="active",
            cognitive_load=6
        )
        ```
    """

    title: Optional[str] = None
    description: Optional[str] = None
    state: Optional[str] = None
    cognitive_load: Optional[int] = None
    estimated_minutes: Optional[int] = None

    @field_validator("state")
    @classmethod
    def validate_state(cls, v: str | None) -> str | None:
        """Validate state is a valid TaskState enum value."""
        if v and v not in [s.value for s in TaskState]:
            raise ValueError(f"Invalid state: {v}")
        return v

    @field_validator("cognitive_load")
    @classmethod
    def validate_cognitive_load(cls, v: int | None) -> int | None:
        """Validate cognitive load is within the 1-10 scale."""
        if v is not None and (v < 1 or v > 10):
            raise ValueError("Cognitive load must be between 1 and 10")
        return v


class TaskResponse(TaskBase):
    """Schema for task API responses.

    Extends TaskBase with read-only fields populated by the database. Used for
    serializing task data when returning from API endpoints.

    Attributes:
        id: Unique UUID identifier for the task
        state: Current task state in the state machine
        created_at: Timestamp when the task was created
        updated_at: Timestamp of the last update, None if never updated

    Example:
        ```python
        # Returned from GET /api/tasks/{id}
        {
            "id": "123e4567-e89b-12d3-a456-426614174000",
            "title": "Review pull request #42",
            "description": null,
            "cognitive_load": 7,
            "estimated_minutes": 30,
            "state": "inbox",
            "created_at": "2025-10-03T14:30:00Z",
            "updated_at": "2025-10-03T15:45:00Z"
        }
        ```
    """

    id: UUID
    state: str
    created_at: datetime
    updated_at: Optional[datetime]

    model_config = ConfigDict(from_attributes=True)
