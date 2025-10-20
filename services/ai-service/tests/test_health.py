"""Tests for health check endpoint."""

from fastapi.testclient import TestClient


def test_health_check(client: TestClient) -> None:
    """Test health check endpoint returns ok status.

    Args:
        client: FastAPI test client
    """
    response = client.get("/api/health")
    assert response.status_code == 200
    data = response.json()
    assert data["status"] == "ok"
    assert data["service"] == "ai-service"
