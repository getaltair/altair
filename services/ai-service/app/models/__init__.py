"""Data models for AI service."""

from app.models.requests import (
    ContextSuggestionRequest,
    TaskBreakdownRequest,
    TaskPrioritizationRequest,
    TimeEstimateRequest,
)
from app.models.responses import (
    ContextSuggestion,
    ContextSuggestionResponse,
    PriorityLevel,
    PrioritySuggestion,
    SubtaskSuggestion,
    TaskBreakdownResponse,
    TaskPrioritizationResponse,
    TimeEstimate,
    TimeEstimateResponse,
)

__all__ = [
    # Requests
    "TaskBreakdownRequest",
    "TaskPrioritizationRequest",
    "TimeEstimateRequest",
    "ContextSuggestionRequest",
    # Responses
    "SubtaskSuggestion",
    "TaskBreakdownResponse",
    "PriorityLevel",
    "PrioritySuggestion",
    "TaskPrioritizationResponse",
    "TimeEstimate",
    "TimeEstimateResponse",
    "ContextSuggestion",
    "ContextSuggestionResponse",
]
