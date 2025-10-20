"""Data models for AI service."""

from app.models.requests import (
    TaskBreakdownRequest,
    TaskPrioritizationRequest,
    TimeEstimateRequest,
    ContextSuggestionRequest,
)
from app.models.responses import (
    SubtaskSuggestion,
    TaskBreakdownResponse,
    PriorityLevel,
    PrioritySuggestion,
    TaskPrioritizationResponse,
    TimeEstimate,
    TimeEstimateResponse,
    ContextSuggestion,
    ContextSuggestionResponse,
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
