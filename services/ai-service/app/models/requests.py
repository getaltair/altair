"""Request models for AI service API."""

from typing import Literal

from pydantic import BaseModel, Field


class TaskBreakdownRequest(BaseModel):
    """Request to break down a task into subtasks."""

    task_title: str = Field(..., min_length=1, max_length=500)
    task_description: str | None = Field(None, max_length=5000)
    context: str | None = Field(
        None, max_length=2000, description="Additional context about the project or goal"
    )
    max_subtasks: int = Field(5, ge=1, le=20, description="Maximum number of subtasks to generate")

    # Client-provided provider configuration (optional, overrides server .env)
    provider: Literal["openai", "anthropic", "ollama"] | None = Field(
        None, description="AI provider to use (overrides server default)"
    )
    api_key: str | None = Field(
        None,
        description="API key for the provider (required for openai/anthropic if not in server .env)",
    )


class TaskPrioritizationRequest(BaseModel):
    """Request to get prioritization suggestions for tasks."""

    tasks: list[dict[str, str]] = Field(
        ..., min_length=1, max_length=50, description="List of tasks with title and description"
    )
    context: str | None = Field(None, max_length=2000, description="Project context or goals")

    # Client-provided provider configuration (optional, overrides server .env)
    provider: Literal["openai", "anthropic", "ollama"] | None = Field(
        None, description="AI provider to use (overrides server default)"
    )
    api_key: str | None = Field(
        None,
        description="API key for the provider (required for openai/anthropic if not in server .env)",
    )


class TimeEstimateRequest(BaseModel):
    """Request to estimate time for a task."""

    task_title: str = Field(..., min_length=1, max_length=500)
    task_description: str | None = Field(None, max_length=5000)
    subtasks: list[str] | None = Field(None, max_length=20, description="List of subtasks")
    skill_level: str | None = Field(
        "intermediate", description="User skill level: beginner, intermediate, advanced"
    )

    # Client-provided provider configuration (optional, overrides server .env)
    provider: Literal["openai", "anthropic", "ollama"] | None = Field(
        None, description="AI provider to use (overrides server default)"
    )
    api_key: str | None = Field(
        None,
        description="API key for the provider (required for openai/anthropic if not in server .env)",
    )


class ContextSuggestionRequest(BaseModel):
    """Request for contextual suggestions."""

    task_title: str = Field(..., min_length=1, max_length=500)
    task_description: str | None = Field(None, max_length=5000)
    project_context: str | None = Field(None, max_length=2000)
    suggestion_type: str = Field(
        "general", description="Type of suggestions: general, resources, tips, blockers"
    )

    # Client-provided provider configuration (optional, overrides server .env)
    provider: Literal["openai", "anthropic", "ollama"] | None = Field(
        None, description="AI provider to use (overrides server default)"
    )
    api_key: str | None = Field(
        None,
        description="API key for the provider (required for openai/anthropic if not in server .env)",
    )
