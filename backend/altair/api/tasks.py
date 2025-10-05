"""Task management API endpoints for Altair.

This module provides RESTful API routes for ADHD-friendly task management,
including quick capture functionality, CRUD operations, and task state transitions.
Designed to minimize cognitive load and decision fatigue through intuitive endpoints.

Example:
    Quick capture a task with minimal friction:
        POST /api/tasks/quick-capture?text="Buy groceries"

    List tasks with pagination:
        GET /api/tasks?skip=0&limit=20
"""

from typing import List
from uuid import UUID

from fastapi import APIRouter, Depends, HTTPException, Query
from sqlalchemy.orm import Session

from altair.database import get_db
from altair.dependencies import get_current_user
from altair.models import Task, TaskState, User
from altair.schemas.task import TaskCreate, TaskResponse, TaskUpdate

router = APIRouter()


@router.post("/quick-capture", response_model=TaskResponse)
def quick_capture(
    text: str, user: User = Depends(get_current_user), db: Session = Depends(get_db)
):
    """Quickly capture a task with minimal friction - ADHD-friendly endpoint.

    This endpoint prioritizes speed and simplicity over structure. Tasks are
    immediately saved to the INBOX state without requiring any decisions about
    priority, project, or categorization. This reduces cognitive load and prevents
    task loss due to delayed capture.

    Args:
        text: Raw text description of the task (becomes the task title)
        db: Database session dependency injection

    Returns:
        TaskResponse: The newly created task with generated ID and metadata

    Example:
        POST /quick-capture?text="Call dentist about appointment"

        Response:
        {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "title": "Call dentist about appointment",
            "state": "inbox",
            "created_at": "2025-10-03T10:30:00Z"
        }
    """
    task = Task(title=text, state=TaskState.INBOX, user_id=user.id)
    db.add(task)
    db.commit()
    db.refresh(task)
    return task


@router.get("", response_model=List[TaskResponse])
def list_tasks(
    skip: int = 0,
    limit: int = Query(default=100, le=500),
    user: User = Depends(get_current_user),
    db: Session = Depends(get_db),
):
    """Retrieve a paginated list of tasks.

    Returns tasks ordered by creation date (newest first). Supports pagination
    to prevent overwhelming the user with large task lists.

    Args:
        skip: Number of tasks to skip (for pagination). Defaults to 0.
        limit: Maximum number of tasks to return. Defaults to 100, max 500.
        db: Database session dependency injection

    Returns:
        List[TaskResponse]: List of task objects with all metadata

    Example:
        GET /tasks?skip=0&limit=20

        Returns the first 20 tasks created
    """
    tasks = (
        db.query(Task)
        .where(Task.user_id == user.id)
        .order_by(Task.created_at.desc())
        .offset(skip)
        .limit(limit)
        .all()
    )
    return tasks


@router.post("", response_model=TaskResponse)
def create_task(
    task: TaskCreate,
    user: User = Depends(get_current_user),
    db: Session = Depends(get_db),
):
    """Create a new task with full metadata (structured creation).

    Unlike quick-capture, this endpoint accepts a complete TaskCreate schema
    allowing specification of state, priority, project, and other fields.
    Use this when you have executive function available for detailed planning.

    Args:
        task: TaskCreate schema with all desired task fields
        db: Database session dependency injection

    Returns:
        TaskResponse: The newly created task with generated ID and timestamps

    Raises:
        HTTPException: 422 if validation fails on the TaskCreate schema

    Example:
        POST /tasks
        {
            "title": "Review Q4 budget",
            "state": "active",
            "priority": 2,
            "estimated_minutes": 45
        }
    """
    task_data = task.model_dump(exclude_unset=True)

    # Convert state string to TaskState enum if provided
    if "state" in task_data and task_data["state"]:
        task_data["state"] = TaskState(task_data["state"])

    db_task = Task(**task_data, user_id=user.id)
    db.add(db_task)
    db.commit()
    db.refresh(db_task)
    return db_task


@router.get("/{task_id}", response_model=TaskResponse)
def get_task(
    task_id: UUID, user: User = Depends(get_current_user), db: Session = Depends(get_db)
):
    """Retrieve a single task by its unique ID.

    Args:
        task_id: UUID of the task to retrieve
        db: Database session dependency injection

    Returns:
        TaskResponse: Complete task object with all fields and metadata

    Raises:
        HTTPException: 404 if task with given ID does not exist

    Example:
        GET /tasks/550e8400-e29b-41d4-a716-446655440000
    """
    task = db.query(Task).where(Task.user_id == user.id, Task.id == task_id).first()
    if not task:
        raise HTTPException(status_code=404, detail="Task not found")
    return task


@router.put("/{task_id}", response_model=TaskResponse)
def update_task(
    task_id: UUID,
    task_update: TaskUpdate,
    user: User = Depends(get_current_user),
    db: Session = Depends(get_db),
):
    """Update an existing task with partial or complete changes.

    Uses the TaskUpdate schema which allows partial updates - only fields
    provided in the request will be modified. This supports ADHD-friendly
    incremental task refinement without forcing complete re-entry of data.

    Args:
        task_id: UUID of the task to update
        task_update: TaskUpdate schema with fields to modify (partial allowed)
        db: Database session dependency injection

    Returns:
        TaskResponse: The updated task with new values and refreshed metadata

    Raises:
        HTTPException: 404 if task with given ID does not exist
        HTTPException: 422 if validation fails on provided update fields

    Example:
        PUT /tasks/550e8400-e29b-41d4-a716-446655440000
        {
            "state": "done"
        }

        Only updates the state field, leaving other fields unchanged
    """
    task = db.query(Task).where(Task.user_id == user.id, Task.id == task_id).first()
    if not task:
        raise HTTPException(status_code=404, detail="Task not found")

    # Only update fields that were explicitly provided
    update_data = task_update.model_dump(exclude_unset=True)

    # Convert state string to TaskState enum if provided
    if "state" in update_data and update_data["state"]:
        update_data["state"] = TaskState(update_data["state"])

    for field, value in update_data.items():
        setattr(task, field, value)

    db.commit()
    db.refresh(task)
    return task
