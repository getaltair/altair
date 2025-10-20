"""Tests for data models."""

import pytest
from pydantic import ValidationError

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
    TimeEstimate,
)


class TestTaskBreakdownRequest:
    """Tests for TaskBreakdownRequest model."""

    def test_valid_request(self) -> None:
        """Test creating valid task breakdown request."""
        request = TaskBreakdownRequest(
            task_title="Test task",
            task_description="Test description",
            max_subtasks=5,
        )
        assert request.task_title == "Test task"
        assert request.task_description == "Test description"
        assert request.max_subtasks == 5

    def test_empty_title_fails(self) -> None:
        """Test that empty title raises validation error."""
        with pytest.raises(ValidationError):
            TaskBreakdownRequest(task_title="", max_subtasks=5)

    def test_max_subtasks_bounds(self) -> None:
        """Test max_subtasks validation."""
        # Valid values
        TaskBreakdownRequest(task_title="Test", max_subtasks=1)
        TaskBreakdownRequest(task_title="Test", max_subtasks=20)

        # Invalid values
        with pytest.raises(ValidationError):
            TaskBreakdownRequest(task_title="Test", max_subtasks=0)
        with pytest.raises(ValidationError):
            TaskBreakdownRequest(task_title="Test", max_subtasks=21)


class TestTaskBreakdownResponse:
    """Tests for TaskBreakdownResponse model."""

    def test_valid_response(self) -> None:
        """Test creating valid task breakdown response."""
        subtasks = [
            SubtaskSuggestion(
                title="Subtask 1",
                description="Description 1",
                estimated_minutes=30,
                order=1,
            ),
            SubtaskSuggestion(
                title="Subtask 2",
                estimated_minutes=45,
                order=2,
            ),
        ]

        response = TaskBreakdownResponse(
            original_task="Test task",
            subtasks=subtasks,
            total_estimated_minutes=75,
            reasoning="Because reasons",
        )

        assert response.original_task == "Test task"
        assert len(response.subtasks) == 2
        assert response.total_estimated_minutes == 75
        assert response.reasoning == "Because reasons"


class TestPriorityLevel:
    """Tests for PriorityLevel enum."""

    def test_priority_levels(self) -> None:
        """Test all priority level values."""
        assert PriorityLevel.CRITICAL == "critical"
        assert PriorityLevel.HIGH == "high"
        assert PriorityLevel.MEDIUM == "medium"
        assert PriorityLevel.LOW == "low"


class TestPrioritySuggestion:
    """Tests for PrioritySuggestion model."""

    def test_valid_suggestion(self) -> None:
        """Test creating valid priority suggestion."""
        suggestion = PrioritySuggestion(
            task_title="Test task",
            priority=PriorityLevel.HIGH,
            reasoning="Important work",
            urgency_score=0.8,
            impact_score=0.9,
        )

        assert suggestion.task_title == "Test task"
        assert suggestion.priority == PriorityLevel.HIGH
        assert suggestion.urgency_score == 0.8
        assert suggestion.impact_score == 0.9

    def test_score_bounds(self) -> None:
        """Test score validation."""
        # Valid scores
        PrioritySuggestion(
            task_title="Test",
            priority=PriorityLevel.HIGH,
            reasoning="Test",
            urgency_score=0.0,
            impact_score=1.0,
        )

        # Invalid scores
        with pytest.raises(ValidationError):
            PrioritySuggestion(
                task_title="Test",
                priority=PriorityLevel.HIGH,
                reasoning="Test",
                urgency_score=-0.1,
                impact_score=0.5,
            )

        with pytest.raises(ValidationError):
            PrioritySuggestion(
                task_title="Test",
                priority=PriorityLevel.HIGH,
                reasoning="Test",
                urgency_score=0.5,
                impact_score=1.1,
            )


class TestTimeEstimate:
    """Tests for TimeEstimate model."""

    def test_valid_estimate(self) -> None:
        """Test creating valid time estimate."""
        estimate = TimeEstimate(
            optimistic_minutes=30,
            realistic_minutes=60,
            pessimistic_minutes=90,
            confidence=0.75,
        )

        assert estimate.optimistic_minutes == 30
        assert estimate.realistic_minutes == 60
        assert estimate.pessimistic_minutes == 90
        assert estimate.confidence == 0.75

    def test_minutes_must_be_positive(self) -> None:
        """Test that minutes must be >= 1."""
        with pytest.raises(ValidationError):
            TimeEstimate(
                optimistic_minutes=0,
                realistic_minutes=60,
                pessimistic_minutes=90,
                confidence=0.75,
            )
