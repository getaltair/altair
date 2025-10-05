"""Task model for ADHD-friendly task management.

This module defines the Task model and TaskState enum, which form the core of
Altair's task management system. The design prioritizes ADHD-friendly workflows
by minimizing decision fatigue through a structured state machine and cognitive
load tracking.

Key components:
    - TaskState: Enum defining the task lifecycle states
    - Task: ORM model representing a task with ADHD-specific attributes

The state machine flows: INBOX → TRIAGED → ACTIVE/BLOCKED → DONE → ARCHIVED
This progression allows users to capture tasks quickly without immediate decisions,
then triage when executive function is available.
"""

import enum
from typing import TYPE_CHECKING, Optional
from uuid import UUID

from sqlalchemy import Enum, ForeignKey, Integer, String, Text
from sqlalchemy.orm import Mapped, mapped_column, relationship

from altair.models.base import BaseModel

if TYPE_CHECKING:
    from altair.models.user import User


class TaskState(enum.Enum):
    """Enumeration of possible task states in the ADHD-friendly workflow.

    The state progression is designed to minimize cognitive load and decision
    fatigue. Tasks start in INBOX (no decisions required) and progress through
    states as the user's executive function allows.

    State Flow:
        INBOX: Newly captured tasks, no categorization required
        TRIAGED: Task has been reviewed and categorized
        ACTIVE: Currently being worked on
        BLOCKED: Cannot proceed, waiting on external dependency
        DONE: Completed (celebration time!)
        ARCHIVED: Completed tasks after 30 days (for historical reference)

    Attributes:
        INBOX (str): Initial state for quick capture, value "inbox"
        TRIAGED (str): Task has been reviewed, value "triaged"
        ACTIVE (str): Currently in progress, value "active"
        BLOCKED (str): Waiting on external factor, value "blocked"
        DONE (str): Successfully completed, value "done"
        ARCHIVED (str): Completed and archived, value "archived"
    """

    INBOX = "inbox"
    TRIAGED = "triaged"
    ACTIVE = "active"
    BLOCKED = "blocked"
    DONE = "done"
    ARCHIVED = "archived"


class Task(BaseModel):
    """Task model with ADHD-specific attributes for cognitive load management.

    This model extends BaseModel to provide task tracking optimized for ADHD
    users. It includes fields to track cognitive load (mental effort required)
    and time estimates to help users make informed decisions about task selection.

    The state field uses TaskState enum to enforce valid state transitions and
    defaults to INBOX for friction-free task capture.

    Attributes:
        title (str): Brief task description, max 255 characters, required
        description (str): Detailed task information, optional long text
        state (TaskState): Current task state, defaults to INBOX for quick
            capture, indexed
        cognitive_load (int): Subjective mental effort rating 1-10, defaults to 5
            (used to help users choose tasks matching current energy levels)
        estimated_minutes (int): User's time estimate in minutes, optional
        user_id (UUID): Foreign key to users table, required, indexed
        user (User): Relationship to the User who owns this task

    Inherited Attributes:
        id (UUID): Unique identifier from BaseModel
        created_at (DateTime): Creation timestamp from BaseModel
        updated_at (DateTime): Last update timestamp from BaseModel

    Example:
        # Quick capture - minimal fields required
        task = Task(title="Review PR #123", user_id=user.id)
        # task.state automatically set to INBOX
        # task.cognitive_load defaults to 5

        # Detailed task with cognitive load assessment
        task = Task(
            title="Implement user authentication",
            description="Add JWT-based auth with refresh tokens",
            cognitive_load=8,  # High mental effort
            estimated_minutes=120,  # 2 hours estimate
            user_id=user.id
        )

        # Access task owner
        print(f"Task owned by: {task.user.email}")

    Note:
        All tasks must belong to a user. Future migrations will add project_id
        for task organization within projects.
    """

    __tablename__ = "tasks"

    # Brief task title, required for all tasks
    title: Mapped[str] = mapped_column(String(255), nullable=False)

    # Optional detailed description for context
    description: Mapped[Optional[str]] = mapped_column(Text)

    # State machine for ADHD-friendly workflow, starts at INBOX
    state: Mapped[TaskState] = mapped_column(
        Enum(TaskState), default=TaskState.INBOX, index=True
    )

    # Mental effort rating (1-10) to match tasks with current energy
    cognitive_load: Mapped[int] = mapped_column(Integer, default=5)

    # User's time estimate in minutes
    estimated_minutes: Mapped[Optional[int]] = mapped_column(Integer)

    user_id: Mapped[UUID] = mapped_column(
        ForeignKey("users.id"), nullable=False, index=True
    )
    user: Mapped["User"] = relationship("User", back_populates="tasks")

    # TODO: Add project IDs
