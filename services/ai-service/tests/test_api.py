"""Tests for AI API endpoints."""

from unittest.mock import AsyncMock, patch

import pytest
from fastapi.testclient import TestClient

from app.models.responses import (
    TaskBreakdownResponse,
    SubtaskSuggestion,
    TaskPrioritizationResponse,
    PrioritySuggestion,
    PriorityLevel,
    TimeEstimateResponse,
    TimeEstimate,
    ContextSuggestionResponse,
    ContextSuggestion,
)


class TestBreakdownEndpoint:
    """Tests for /api/ai/breakdown endpoint."""

    @patch("app.api.ai.get_ai_provider")
    def test_breakdown_success(
        self,
        mock_get_provider: AsyncMock,
        client: TestClient,
        sample_task_breakdown_request: dict[str, str | int],
    ) -> None:
        """Test successful task breakdown.

        Args:
            mock_get_provider: Mocked provider factory
            client: FastAPI test client
            sample_task_breakdown_request: Sample request data
        """
        # Mock AI provider
        mock_provider = AsyncMock()
        mock_provider.breakdown_task.return_value = TaskBreakdownResponse(
            original_task="Implement user authentication",
            subtasks=[
                SubtaskSuggestion(
                    title="Create auth models",
                    description="Define User and Token models",
                    estimated_minutes=30,
                    order=1,
                ),
                SubtaskSuggestion(
                    title="Implement JWT tokens",
                    estimated_minutes=60,
                    order=2,
                ),
            ],
            total_estimated_minutes=90,
            reasoning="Breaking down into clear steps",
        )
        mock_get_provider.return_value = mock_provider

        response = client.post("/api/ai/breakdown", json=sample_task_breakdown_request)

        assert response.status_code == 200
        data = response.json()
        assert data["original_task"] == "Implement user authentication"
        assert len(data["subtasks"]) == 2
        assert data["total_estimated_minutes"] == 90

    @patch("app.api.ai.get_ai_provider")
    def test_breakdown_validation_error(
        self,
        mock_get_provider: AsyncMock,
        client: TestClient,
    ) -> None:
        """Test breakdown with invalid request.

        Args:
            mock_get_provider: Mocked provider factory
            client: FastAPI test client
        """
        # Mock provider (won't be called due to validation error)
        mock_provider = AsyncMock()
        mock_get_provider.return_value = mock_provider

        response = client.post(
            "/api/ai/breakdown",
            json={"task_title": "", "max_subtasks": 5},  # Empty title
        )

        assert response.status_code == 422  # Validation error


class TestPrioritizeEndpoint:
    """Tests for /api/ai/prioritize endpoint."""

    @patch("app.api.ai.get_ai_provider")
    def test_prioritize_success(
        self,
        mock_get_provider: AsyncMock,
        client: TestClient,
        sample_prioritization_request: dict[str, list[dict[str, str]] | str],
    ) -> None:
        """Test successful task prioritization.

        Args:
            mock_get_provider: Mocked provider factory
            client: FastAPI test client
            sample_prioritization_request: Sample request data
        """
        mock_provider = AsyncMock()
        mock_provider.prioritize_tasks.return_value = TaskPrioritizationResponse(
            suggestions=[
                PrioritySuggestion(
                    task_title="Fix login bug",
                    priority=PriorityLevel.CRITICAL,
                    reasoning="Blocks users",
                    urgency_score=1.0,
                    impact_score=0.9,
                ),
                PrioritySuggestion(
                    task_title="Database migration",
                    priority=PriorityLevel.HIGH,
                    reasoning="Required for features",
                    urgency_score=0.7,
                    impact_score=0.8,
                ),
                PrioritySuggestion(
                    task_title="Add dark mode",
                    priority=PriorityLevel.LOW,
                    reasoning="Nice to have",
                    urgency_score=0.3,
                    impact_score=0.4,
                ),
            ],
            recommended_order=["Fix login bug", "Database migration", "Add dark mode"],
        )
        mock_get_provider.return_value = mock_provider

        response = client.post("/api/ai/prioritize", json=sample_prioritization_request)

        assert response.status_code == 200
        data = response.json()
        assert len(data["suggestions"]) == 3
        assert data["suggestions"][0]["priority"] == "critical"
        assert len(data["recommended_order"]) == 3


class TestEstimateEndpoint:
    """Tests for /api/ai/estimate endpoint."""

    @patch("app.api.ai.get_ai_provider")
    def test_estimate_success(
        self,
        mock_get_provider: AsyncMock,
        client: TestClient,
        sample_time_estimate_request: dict[str, str | list[str]],
    ) -> None:
        """Test successful time estimation.

        Args:
            mock_get_provider: Mocked provider factory
            client: FastAPI test client
            sample_time_estimate_request: Sample request data
        """
        mock_provider = AsyncMock()
        mock_provider.estimate_time.return_value = TimeEstimateResponse(
            task_title="Implement user authentication",
            estimate=TimeEstimate(
                optimistic_minutes=60,
                realistic_minutes=120,
                pessimistic_minutes=180,
                confidence=0.8,
            ),
            factors=["JWT complexity", "Testing requirements"],
            assumptions=["Intermediate skill level", "No major blockers"],
        )
        mock_get_provider.return_value = mock_provider

        response = client.post("/api/ai/estimate", json=sample_time_estimate_request)

        assert response.status_code == 200
        data = response.json()
        assert data["task_title"] == "Implement user authentication"
        assert data["estimate"]["realistic_minutes"] == 120
        assert len(data["factors"]) == 2
        assert len(data["assumptions"]) == 2


class TestSuggestEndpoint:
    """Tests for /api/ai/suggest endpoint."""

    @patch("app.api.ai.get_ai_provider")
    def test_suggest_success(
        self,
        mock_get_provider: AsyncMock,
        client: TestClient,
        sample_context_suggestion_request: dict[str, str],
    ) -> None:
        """Test successful context suggestions.

        Args:
            mock_get_provider: Mocked provider factory
            client: FastAPI test client
            sample_context_suggestion_request: Sample request data
        """
        mock_provider = AsyncMock()
        mock_provider.suggest_context.return_value = ContextSuggestionResponse(
            task_title="Learn React hooks",
            suggestions=[
                ContextSuggestion(
                    title="Official React docs",
                    description="Start with the official hooks documentation",
                    category="resource",
                    priority=PriorityLevel.HIGH,
                ),
                ContextSuggestion(
                    title="Common pitfall: stale closures",
                    description="Be aware of closure issues with state",
                    category="tip",
                    priority=PriorityLevel.MEDIUM,
                ),
            ],
            summary="Focus on useState and useEffect first",
        )
        mock_get_provider.return_value = mock_provider

        response = client.post("/api/ai/suggest", json=sample_context_suggestion_request)

        assert response.status_code == 200
        data = response.json()
        assert data["task_title"] == "Learn React hooks"
        assert len(data["suggestions"]) == 2
        assert data["suggestions"][0]["category"] == "resource"
        assert data["summary"] == "Focus on useState and useEffect first"
