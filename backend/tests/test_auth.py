"""Authentication endpoint tests.

This module contains tests for user registration, login, and authentication
flows. Tests cover both success and failure scenarios, including duplicate
email handling, invalid credentials, and token validation.
"""

from datetime import timedelta

from fastapi.testclient import TestClient
from sqlalchemy.orm import Session

from altair.models.user import User
from altair.services.auth import create_access_token, verify_password


class TestRegistration:
    """Test user registration endpoint."""

    def test_register_new_user(self, client: TestClient, db_session: Session):
        """Test successful user registration with valid email and password."""
        response = client.post(
            "/api/auth/register",
            json={
                "email": "newuser@example.com",
                "password": "securepassword123",
            },
        )

        assert response.status_code == 201
        data = response.json()
        assert data["email"] == "newuser@example.com"
        assert "id" in data
        assert "password" not in data
        assert "hashed_password" not in data

        # Verify user exists in database
        user = db_session.query(User).filter_by(email="newuser@example.com").first()
        assert user is not None
        assert user.email == "newuser@example.com"
        assert user.is_active is True

    def test_register_duplicate_email(self, client: TestClient, test_user: User):
        """Test registration fails with duplicate email."""
        response = client.post(
            "/api/auth/register",
            json={
                "email": test_user.email,
                "password": "anotherpassword123",
            },
        )

        assert response.status_code == 400
        assert "already registered" in response.json()["detail"].lower()

    def test_register_invalid_email(self, client: TestClient):
        """Test registration fails with invalid email format."""
        response = client.post(
            "/api/auth/register",
            json={
                "email": "not-an-email",
                "password": "securepassword123",
            },
        )

        assert response.status_code == 422

    def test_register_short_password(self, client: TestClient):
        """Test registration fails with password shorter than 8 characters."""
        response = client.post(
            "/api/auth/register",
            json={
                "email": "newuser@example.com",
                "password": "short",
            },
        )

        assert response.status_code == 422

    def test_password_is_hashed(self, client: TestClient, db_session: Session):
        """Test that password is hashed in database, not stored as plaintext."""
        password = "supersecretpassword"
        response = client.post(
            "/api/auth/register",
            json={
                "email": "hashtest@example.com",
                "password": password,
            },
        )

        assert response.status_code == 201

        # Verify password is hashed, not plaintext
        user = db_session.query(User).filter_by(email="hashtest@example.com").first()
        assert user is not None
        assert user.hashed_password != password
        assert user.hashed_password.startswith("$argon2")  # argon2 hash prefix


class TestLogin:
    """Test user login endpoint."""

    def test_login_success(self, client: TestClient, test_user: User):
        """Test successful login with correct credentials."""
        response = client.post(
            "/api/auth/login",
            data={
                "username": "test@example.com",  # OAuth2 uses 'username' field
                "password": "testpassword123",
            },
        )

        assert response.status_code == 200
        data = response.json()
        assert "access_token" in data
        assert data["token_type"] == "bearer"

    def test_login_wrong_password(self, client: TestClient, test_user: User):
        """Test login fails with incorrect password."""
        response = client.post(
            "/api/auth/login",
            data={
                "username": test_user.email,
                "password": "wrongpassword",
            },
        )

        assert response.status_code == 401
        assert "incorrect" in response.json()["detail"].lower()

    def test_login_nonexistent_user(self, client: TestClient):
        """Test login fails with non-existent email."""
        response = client.post(
            "/api/auth/login",
            data={
                "username": "nobody@example.com",
                "password": "anypassword",
            },
        )

        assert response.status_code == 401
        assert "incorrect" in response.json()["detail"].lower()

    def test_login_inactive_user(self, client: TestClient, inactive_user: User):
        """Test login fails for inactive user."""
        response = client.post(
            "/api/auth/login",
            data={
                "username": inactive_user.email,
                "password": "testpassword123",
            },
        )

        assert response.status_code == 403
        assert "inactive" in response.json()["detail"].lower()

    def test_token_contains_user_email(
        self, client: TestClient, test_user: User, auth_token: str
    ):
        """Test that generated token can be decoded and contains user email."""
        from jose import jwt

        from altair.config import settings

        payload = jwt.decode(
            auth_token, settings.SECRET_KEY, algorithms=[settings.ALGORITHM]
        )
        assert payload["sub"] == test_user.email


class TestProtectedEndpoints:
    """Test authentication requirements on protected endpoints."""

    def test_get_current_user_with_valid_token(
        self, client: TestClient, auth_headers: dict, test_user: User
    ):
        """Test accessing /api/auth/me with valid token returns user info."""
        response = client.get("/api/auth/me", headers=auth_headers)

        assert response.status_code == 200
        data = response.json()
        assert data["email"] == test_user.email
        assert data["id"] == str(test_user.id)
        assert "password" not in data
        assert "hashed_password" not in data

    def test_get_current_user_without_token(self, client: TestClient):
        """Test accessing /api/auth/me without token returns 401."""
        response = client.get("/api/auth/me")

        assert response.status_code == 401
        assert "not authenticated" in response.json()["detail"].lower()

    def test_get_current_user_with_invalid_token(self, client: TestClient):
        """Test accessing /api/auth/me with invalid token returns 401."""
        response = client.get(
            "/api/auth/me",
            headers={"Authorization": "Bearer invalid-token-here"},
        )

        assert response.status_code == 401

    def test_get_current_user_with_expired_token(
        self, client: TestClient, test_user: User
    ):
        """Test accessing /api/auth/me with expired token returns 401."""
        # Create an expired token (expired 1 minute ago)
        expired_token = create_access_token(
            data={"sub": test_user.email},
            expires_delta=timedelta(minutes=-1),
        )

        response = client.get(
            "/api/auth/me",
            headers={"Authorization": f"Bearer {expired_token}"},
        )

        assert response.status_code == 401

    def test_malformed_authorization_header(self, client: TestClient):
        """Test that malformed Authorization header is rejected."""
        # Missing "Bearer" prefix
        response = client.get(
            "/api/auth/me",
            headers={"Authorization": "some-token"},
        )

        assert response.status_code == 401


class TestPasswordSecurity:
    """Test password security measures."""

    def test_password_verification(self, test_user: User):
        """Test password verification utility works correctly."""
        # Correct password should verify
        assert verify_password("testpassword123", test_user.hashed_password)

        # Wrong password should not verify
        assert not verify_password("wrongpassword", test_user.hashed_password)

    def test_different_passwords_different_hashes(self, db_session: Session):
        """Test that same password on different users produces different hashes."""
        from altair.services.auth import hash_password

        hash1 = hash_password("samepassword")
        hash2 = hash_password("samepassword")

        # Argon2 includes a random salt, so same password produces different hashes
        assert hash1 != hash2

        # But both hashes should verify the same password
        assert verify_password("samepassword", hash1)
        assert verify_password("samepassword", hash2)
