"""Task management endpoint tests.

This module contains tests for task creation, retrieval, updates, and most
importantly, user scoping to ensure tasks are properly isolated between users.
Tests verify that authentication is required and users can only access their own tasks.
"""

from uuid import UUID

import pytest
from fastapi.testclient import TestClient
from sqlalchemy.orm import Session

from altair.models.task import Task, TaskState
from altair.models.user import User
from altair.services.auth import create_access_token


@pytest.fixture
def second_user(db_session: Session) -> User:
    """Create a second test user for multi-user isolation tests."""
    from altair.services.auth import hash_password

    user = User(
        email="user2@example.com",
        hashed_password=hash_password("password456"),
        is_active=True,
    )
    db_session.add(user)
    db_session.commit()
    db_session.refresh(user)
    return user


@pytest.fixture
def second_user_token(second_user: User) -> str:
    """Generate auth token for the second user."""
    return create_access_token(data={"sub": second_user.email})


@pytest.fixture
def second_user_headers(second_user_token: str) -> dict[str, str]:
    """Generate auth headers for the second user."""
    return {"Authorization": f"Bearer {second_user_token}"}


@pytest.fixture
def sample_task(db_session: Session, test_user: User) -> Task:
    """Create a sample task for the test user."""
    task = Task(
        title="Sample task for testing",
        description="This is a test task",
        state=TaskState.INBOX,
        user_id=test_user.id,
    )
    db_session.add(task)
    db_session.commit()
    db_session.refresh(task)
    return task


class TestTaskAuthentication:
    """Test that task endpoints require authentication."""

    def test_quick_capture_requires_auth(self, client: TestClient):
        """Test quick-capture endpoint requires authentication."""
        response = client.post("/api/tasks/quick-capture?text=Test task")
        assert response.status_code == 401

    def test_list_tasks_requires_auth(self, client: TestClient):
        """Test list tasks endpoint requires authentication."""
        response = client.get("/api/tasks")
        assert response.status_code == 401

    def test_create_task_requires_auth(self, client: TestClient):
        """Test create task endpoint requires authentication."""
        response = client.post(
            "/api/tasks",
            json={"title": "Test task"},
        )
        assert response.status_code == 401

    def test_get_task_requires_auth(self, client: TestClient, sample_task: Task):
        """Test get single task endpoint requires authentication."""
        response = client.get(f"/api/tasks/{sample_task.id}")
        assert response.status_code == 401

    def test_update_task_requires_auth(self, client: TestClient, sample_task: Task):
        """Test update task endpoint requires authentication."""
        response = client.put(
            f"/api/tasks/{sample_task.id}",
            json={"title": "Updated title"},
        )
        assert response.status_code == 401


class TestQuickCapture:
    """Test quick capture endpoint for friction-free task creation."""

    def test_quick_capture_success(
        self,
        client: TestClient,
        auth_headers: dict,
        test_user: User,
        db_session: Session,
    ):
        """Test quick capture creates task successfully."""
        response = client.post(
            "/api/tasks/quick-capture?text=Buy groceries",
            headers=auth_headers,
        )

        assert response.status_code == 200
        data = response.json()
        assert data["title"] == "Buy groceries"
        assert data["state"] == "inbox"
        assert "id" in data
        assert "created_at" in data

        # Verify task in database
        task = db_session.query(Task).filter_by(id=UUID(data["id"])).first()
        assert task is not None
        assert task.user_id == test_user.id

    def test_quick_capture_assigns_to_current_user(
        self,
        client: TestClient,
        auth_headers: dict,
        test_user: User,
        db_session: Session,
    ):
        """Test quick capture assigns task to authenticated user."""
        response = client.post(
            "/api/tasks/quick-capture?text=Test task",
            headers=auth_headers,
        )

        assert response.status_code == 200
        task_id = response.json()["id"]

        task = db_session.query(Task).filter_by(id=UUID(task_id)).first()
        assert task.user_id == test_user.id


class TestTaskCreation:
    """Test full task creation endpoint."""

    def test_create_task_with_all_fields(
        self,
        client: TestClient,
        auth_headers: dict,
        test_user: User,
        db_session: Session,
    ):
        """Test creating task with all fields populated."""
        task_data = {
            "title": "Implement JWT auth",
            "description": "Add JWT authentication to the API",
            "state": "active",
            "cognitive_load": 8,
            "estimated_minutes": 120,
        }

        response = client.post(
            "/api/tasks",
            json=task_data,
            headers=auth_headers,
        )

        assert response.status_code == 200
        data = response.json()
        assert data["title"] == task_data["title"]
        assert data["description"] == task_data["description"]
        assert data["state"] == task_data["state"]
        assert data["cognitive_load"] == task_data["cognitive_load"]
        assert data["estimated_minutes"] == task_data["estimated_minutes"]

        # Verify in database
        task = db_session.query(Task).filter_by(id=UUID(data["id"])).first()
        assert task.user_id == test_user.id

    def test_create_task_minimal_fields(self, client: TestClient, auth_headers: dict):
        """Test creating task with only required fields."""
        response = client.post(
            "/api/tasks",
            json={"title": "Minimal task"},
            headers=auth_headers,
        )

        assert response.status_code == 200
        data = response.json()
        assert data["title"] == "Minimal task"
        assert data["state"] == "inbox"  # Default state


class TestTaskRetrieval:
    """Test task retrieval endpoints."""

    def test_list_tasks_returns_only_user_tasks(
        self,
        client: TestClient,
        auth_headers: dict,
        second_user_headers: dict,
        test_user: User,
        second_user: User,
        db_session: Session,
    ):
        """Test that users only see their own tasks, not other users' tasks."""
        # Create tasks for first user
        task1 = Task(title="User 1 Task 1", user_id=test_user.id)
        task2 = Task(title="User 1 Task 2", user_id=test_user.id)

        # Create tasks for second user
        task3 = Task(title="User 2 Task 1", user_id=second_user.id)
        task4 = Task(title="User 2 Task 2", user_id=second_user.id)

        db_session.add_all([task1, task2, task3, task4])
        db_session.commit()

        # First user should only see their tasks
        response1 = client.get("/api/tasks", headers=auth_headers)
        assert response1.status_code == 200
        user1_tasks = response1.json()
        assert len(user1_tasks) == 2
        assert all(task["title"].startswith("User 1") for task in user1_tasks)

        # Second user should only see their tasks
        response2 = client.get("/api/tasks", headers=second_user_headers)
        assert response2.status_code == 200
        user2_tasks = response2.json()
        assert len(user2_tasks) == 2
        assert all(task["title"].startswith("User 2") for task in user2_tasks)

    def test_get_single_task_success(
        self, client: TestClient, auth_headers: dict, sample_task: Task
    ):
        """Test retrieving a single task by ID."""
        response = client.get(
            f"/api/tasks/{sample_task.id}",
            headers=auth_headers,
        )

        assert response.status_code == 200
        data = response.json()
        assert data["id"] == str(sample_task.id)
        assert data["title"] == sample_task.title

    def test_get_task_from_another_user_fails(
        self,
        client: TestClient,
        second_user_headers: dict,
        sample_task: Task,
    ):
        """Test that users cannot access other users' tasks."""
        # sample_task belongs to test_user
        # Try to access it with second_user credentials
        response = client.get(
            f"/api/tasks/{sample_task.id}",
            headers=second_user_headers,
        )

        assert response.status_code == 404

    def test_get_nonexistent_task(self, client: TestClient, auth_headers: dict):
        """Test retrieving a task that doesn't exist."""
        fake_uuid = "550e8400-e29b-41d4-a716-446655440000"
        response = client.get(
            f"/api/tasks/{fake_uuid}",
            headers=auth_headers,
        )

        assert response.status_code == 404


class TestTaskUpdates:
    """Test task update functionality."""

    def test_update_task_success(
        self,
        client: TestClient,
        auth_headers: dict,
        sample_task: Task,
        db_session: Session,
    ):
        """Test updating a task successfully."""
        update_data = {
            "title": "Updated task title",
            "state": "active",
        }

        response = client.put(
            f"/api/tasks/{sample_task.id}",
            json=update_data,
            headers=auth_headers,
        )

        assert response.status_code == 200
        data = response.json()
        assert data["title"] == update_data["title"]
        assert data["state"] == update_data["state"]

        # Verify in database
        db_session.refresh(sample_task)
        assert sample_task.title == update_data["title"]
        assert sample_task.state == TaskState.ACTIVE

    def test_update_task_partial(
        self,
        client: TestClient,
        auth_headers: dict,
        sample_task: Task,
        db_session: Session,
    ):
        """Test partial task update only changes specified fields."""
        original_title = sample_task.title

        response = client.put(
            f"/api/tasks/{sample_task.id}",
            json={"cognitive_load": 9},
            headers=auth_headers,
        )

        assert response.status_code == 200
        data = response.json()
        assert data["cognitive_load"] == 9
        assert data["title"] == original_title  # Title unchanged

    def test_update_another_users_task_fails(
        self,
        client: TestClient,
        second_user_headers: dict,
        sample_task: Task,
    ):
        """Test that users cannot update other users' tasks."""
        response = client.put(
            f"/api/tasks/{sample_task.id}",
            json={"title": "Hacked!"},
            headers=second_user_headers,
        )

        assert response.status_code == 404

    def test_update_nonexistent_task(self, client: TestClient, auth_headers: dict):
        """Test updating a task that doesn't exist."""
        fake_uuid = "550e8400-e29b-41d4-a716-446655440000"
        response = client.put(
            f"/api/tasks/{fake_uuid}",
            json={"title": "Updated"},
            headers=auth_headers,
        )

        assert response.status_code == 404


class TestTaskPagination:
    """Test task list pagination."""

    def test_list_tasks_with_limit(
        self,
        client: TestClient,
        auth_headers: dict,
        test_user: User,
        db_session: Session,
    ):
        """Test limiting the number of tasks returned."""
        # Create 10 tasks
        for i in range(10):
            task = Task(title=f"Task {i}", user_id=test_user.id)
            db_session.add(task)
        db_session.commit()

        # Request only 5 tasks
        response = client.get("/api/tasks?limit=5", headers=auth_headers)

        assert response.status_code == 200
        tasks = response.json()
        assert len(tasks) == 5

    def test_list_tasks_with_skip(
        self,
        client: TestClient,
        auth_headers: dict,
        test_user: User,
        db_session: Session,
    ):
        """Test skipping tasks for pagination."""
        # Create 5 tasks
        created_tasks = []
        for i in range(5):
            task = Task(title=f"Task {i}", user_id=test_user.id)
            db_session.add(task)
            created_tasks.append(task)
        db_session.commit()

        # Get all tasks
        response1 = client.get("/api/tasks", headers=auth_headers)
        all_tasks = response1.json()

        # Skip first 2 tasks
        response2 = client.get("/api/tasks?skip=2", headers=auth_headers)
        skipped_tasks = response2.json()

        assert len(skipped_tasks) == len(all_tasks) - 2
