"""Request models for AI service API."""

from typing import Optional

from pydantic import BaseModel, Field


class TaskBreakdownRequest(BaseModel):
    """Request to break down a task into subtasks."""

    task_title: str = Field(..., min_length=1, max_length=500)
    task_description: Optional[str] = Field(None, max_length=5000)
    context: Optional[str] = Field(
        None, max_length=2000, description="Additional context about the project or goal"
    )
    max_subtasks: int = Field(5, ge=1, le=20, description="Maximum number of subtasks to generate")


class TaskPrioritizationRequest(BaseModel):
    """Request to get prioritization suggestions for tasks."""

    tasks: list[dict[str, str]] = Field(
        ..., min_length=1, max_length=50, description="List of tasks with title and description"
    )
    context: Optional[str] = Field(
        None, max_length=2000, description="Project context or goals"
    )


class TimeEstimateRequest(BaseModel):
    """Request to estimate time for a task."""

    task_title: str = Field(..., min_length=1, max_length=500)
    task_description: Optional[str] = Field(None, max_length=5000)
    subtasks: Optional[list[str]] = Field(None, max_length=20, description="List of subtasks")
    skill_level: Optional[str] = Field(
        "intermediate", description="User skill level: beginner, intermediate, advanced"
    )


class ContextSuggestionRequest(BaseModel):
    """Request for contextual suggestions."""

    task_title: str = Field(..., min_length=1, max_length=500)
    task_description: Optional[str] = Field(None, max_length=5000)
    project_context: Optional[str] = Field(None, max_length=2000)
    suggestion_type: str = Field(
        "general", description="Type of suggestions: general, resources, tips, blockers"
    )
