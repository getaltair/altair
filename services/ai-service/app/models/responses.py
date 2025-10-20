"""Response models for AI service API."""

from enum import Enum

from pydantic import BaseModel, Field


class SubtaskSuggestion(BaseModel):
    """A suggested subtask."""

    title: str = Field(..., description="Subtask title")
    description: str | None = Field(None, description="Detailed description")
    estimated_minutes: int | None = Field(None, ge=1, description="Estimated time in minutes")
    order: int = Field(..., ge=1, description="Suggested order of execution")


class TaskBreakdownResponse(BaseModel):
    """Response containing task breakdown suggestions."""

    original_task: str = Field(..., description="Original task title")
    subtasks: list[SubtaskSuggestion] = Field(..., description="List of suggested subtasks")
    total_estimated_minutes: int | None = Field(
        None, description="Total estimated time for all subtasks"
    )
    reasoning: str | None = Field(None, description="AI reasoning for the breakdown")


class PriorityLevel(str, Enum):
    """Priority levels for tasks."""

    CRITICAL = "critical"
    HIGH = "high"
    MEDIUM = "medium"
    LOW = "low"


class PrioritySuggestion(BaseModel):
    """Priority suggestion for a task."""

    task_id: str | None = Field(None, description="Task identifier from request")
    task_title: str = Field(..., description="Task title")
    priority: PriorityLevel = Field(..., description="Suggested priority level")
    reasoning: str = Field(..., description="Why this priority level")
    urgency_score: float = Field(..., ge=0.0, le=1.0, description="Urgency score (0-1)")
    impact_score: float = Field(..., ge=0.0, le=1.0, description="Impact score (0-1)")


class TaskPrioritizationResponse(BaseModel):
    """Response containing task prioritization suggestions."""

    suggestions: list[PrioritySuggestion] = Field(..., description="Priority suggestions per task")
    recommended_order: list[str] = Field(
        ..., description="Recommended execution order (task titles)"
    )


class TimeEstimate(BaseModel):
    """Time estimate for a task."""

    optimistic_minutes: int = Field(..., ge=1, description="Best-case estimate")
    realistic_minutes: int = Field(..., ge=1, description="Most likely estimate")
    pessimistic_minutes: int = Field(..., ge=1, description="Worst-case estimate")
    confidence: float = Field(..., ge=0.0, le=1.0, description="Confidence in estimate (0-1)")


class TimeEstimateResponse(BaseModel):
    """Response containing time estimates."""

    task_title: str = Field(..., description="Task being estimated")
    estimate: TimeEstimate = Field(..., description="Time estimates")
    factors: list[str] = Field(..., description="Factors considered in estimation")
    assumptions: list[str] = Field(..., description="Assumptions made")


class ContextSuggestion(BaseModel):
    """A contextual suggestion."""

    title: str = Field(..., description="Suggestion title")
    description: str = Field(..., description="Detailed suggestion")
    category: str = Field(..., description="Suggestion category: resource, tip, blocker, etc.")
    priority: PriorityLevel | None = Field(None, description="Suggestion priority")


class ContextSuggestionResponse(BaseModel):
    """Response containing contextual suggestions."""

    task_title: str = Field(..., description="Task being analyzed")
    suggestions: list[ContextSuggestion] = Field(..., description="List of suggestions")
    summary: str | None = Field(None, description="Overall summary of suggestions")
