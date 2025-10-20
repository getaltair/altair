"""Pytest configuration and fixtures."""

import pytest
from app.main import app
from fastapi.testclient import TestClient


@pytest.fixture
def client() -> TestClient:
    """Create a test client.

    Returns:
        FastAPI test client
    """
    return TestClient(app)


@pytest.fixture
def sample_task_breakdown_request() -> dict[str, str | int]:
    """Sample task breakdown request.

    Returns:
        Dict with task breakdown request data
    """
    return {
        "task_title": "Implement user authentication",
        "task_description": "Add JWT-based authentication to the API",
        "context": "Building a FastAPI backend",
        "max_subtasks": 5,
    }


@pytest.fixture
def sample_prioritization_request() -> dict[str, list[dict[str, str]] | str]:
    """Sample task prioritization request.

    Returns:
        Dict with prioritization request data
    """
    return {
        "tasks": [
            {"title": "Fix login bug", "description": "Users can't login"},
            {"title": "Add dark mode", "description": "UI enhancement"},
            {"title": "Database migration", "description": "Update schema"},
        ],
        "context": "Sprint planning for Q1",
    }


@pytest.fixture
def sample_time_estimate_request() -> dict[str, str | list[str]]:
    """Sample time estimate request.

    Returns:
        Dict with time estimate request data
    """
    return {
        "task_title": "Implement user authentication",
        "task_description": "Add JWT-based auth",
        "subtasks": [
            "Create auth models",
            "Implement JWT tokens",
            "Add auth middleware",
        ],
        "skill_level": "intermediate",
    }


@pytest.fixture
def sample_context_suggestion_request() -> dict[str, str]:
    """Sample context suggestion request.

    Returns:
        Dict with context suggestion request data
    """
    return {
        "task_title": "Learn React hooks",
        "task_description": "Understanding useState and useEffect",
        "project_context": "Building a todo app",
        "suggestion_type": "resources",
    }
