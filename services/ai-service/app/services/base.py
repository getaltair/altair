"""Base AI provider interface."""

from abc import ABC, abstractmethod

from app.models.requests import (
    ContextSuggestionRequest,
    TaskBreakdownRequest,
    TaskPrioritizationRequest,
    TimeEstimateRequest,
)
from app.models.responses import (
    ContextSuggestionResponse,
    TaskBreakdownResponse,
    TaskPrioritizationResponse,
    TimeEstimateResponse,
)


class AIProvider(ABC):
    """Abstract base class for AI providers."""

    @abstractmethod
    async def breakdown_task(self, request: TaskBreakdownRequest) -> TaskBreakdownResponse:
        """Break down a task into subtasks.

        Args:
            request: Task breakdown request

        Returns:
            TaskBreakdownResponse with suggested subtasks

        Raises:
            Exception: If AI provider fails
        """
        pass

    @abstractmethod
    async def prioritize_tasks(
        self, request: TaskPrioritizationRequest
    ) -> TaskPrioritizationResponse:
        """Suggest prioritization for tasks.

        Args:
            request: Task prioritization request

        Returns:
            TaskPrioritizationResponse with priority suggestions

        Raises:
            Exception: If AI provider fails
        """
        pass

    @abstractmethod
    async def estimate_time(self, request: TimeEstimateRequest) -> TimeEstimateResponse:
        """Estimate time required for a task.

        Args:
            request: Time estimate request

        Returns:
            TimeEstimateResponse with time estimates

        Raises:
            Exception: If AI provider fails
        """
        pass

    @abstractmethod
    async def suggest_context(self, request: ContextSuggestionRequest) -> ContextSuggestionResponse:
        """Provide contextual suggestions for a task.

        Args:
            request: Context suggestion request

        Returns:
            ContextSuggestionResponse with suggestions

        Raises:
            Exception: If AI provider fails
        """
        pass
