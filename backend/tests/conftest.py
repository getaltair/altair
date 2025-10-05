"""Pytest configuration and fixtures for Altair testing.

This module provides reusable test fixtures for database setup, test clients,
and authentication utilities. All fixtures use an in-memory SQLite database
for fast, isolated test execution.

Key fixtures:
    - engine: Test database engine
    - db_session: Test database session with automatic rollback
    - client: FastAPI TestClient with database dependency override
    - test_user: Creates a test user in the database
    - auth_token: Provides valid JWT token for authenticated requests
"""

from collections.abc import Iterator

import pytest
from fastapi.testclient import TestClient
from sqlalchemy import StaticPool, create_engine
from sqlalchemy.orm import Session, sessionmaker

from altair.database import Base, get_db
from altair.main import app
from altair.models.user import User
from altair.services.auth import create_access_token, hash_password

# Use in-memory SQLite for fast, isolated tests
TEST_DATABASE_URL = "sqlite:///:memory:"


@pytest.fixture(scope="function")
def engine():
    """Create a fresh SQLAlchemy engine for each test.

    Uses SQLite in-memory database with StaticPool to ensure
    the same connection is reused across the test.

    Yields:
        Engine: SQLAlchemy engine instance
    """
    engine = create_engine(
        TEST_DATABASE_URL,
        connect_args={"check_same_thread": False},
        poolclass=StaticPool,
    )
    # Create all tables
    Base.metadata.create_all(bind=engine)
    yield engine
    # Drop all tables after test
    Base.metadata.drop_all(bind=engine)
    engine.dispose()


@pytest.fixture(scope="function")
def db_session(engine) -> Iterator[Session]:
    """Provide a test database session with automatic rollback.

    Creates a new session for each test and rolls back all changes
    after the test completes, ensuring test isolation.

    Args:
        engine: Test database engine from engine fixture

    Yields:
        Session: SQLAlchemy database session
    """
    test_session_local = sessionmaker(autocommit=False, autoflush=False, bind=engine)
    session = test_session_local()
    try:
        yield session
    finally:
        session.rollback()
        session.close()


@pytest.fixture(scope="function")
def client(db_session: Session) -> TestClient:
    """Provide a FastAPI TestClient with database dependency override.

    Overrides the get_db dependency to use the test database session
    instead of the production database.

    Args:
        db_session: Test database session from db_session fixture

    Returns:
        TestClient: FastAPI test client
    """

    def override_get_db():
        try:
            yield db_session
        finally:
            pass  # Session cleanup handled by db_session fixture

    app.dependency_overrides[get_db] = override_get_db

    with TestClient(app) as test_client:
        yield test_client

    # Clean up overrides after test
    app.dependency_overrides.clear()


@pytest.fixture
def test_user(db_session: Session) -> User:
    """Create a test user in the database.

    Creates a user with known credentials for use in authentication tests.

    Args:
        db_session: Test database session

    Returns:
        User: Created user object with test credentials
    """
    user = User(
        email="test@example.com",
        hashed_password=hash_password("testpassword123"),
        is_active=True,
    )
    db_session.add(user)
    db_session.commit()
    db_session.refresh(user)
    return user


@pytest.fixture
def inactive_user(db_session: Session) -> User:
    """Create an inactive test user in the database.

    Creates a user that is marked as inactive for testing
    access control.

    Args:
        db_session: Test database session

    Returns:
        User: Created inactive user object
    """
    user = User(
        email="inactive@example.com",
        hashed_password=hash_password("testpassword123"),
        is_active=False,
    )
    db_session.add(user)
    db_session.commit()
    db_session.refresh(user)
    return user


@pytest.fixture
def auth_token(test_user: User) -> str:
    """Generate a valid JWT token for the test user.

    Creates an access token that can be used for authenticated
    requests in tests.

    Args:
        test_user: Test user from test_user fixture

    Returns:
        str: JWT access token
    """
    token_data = {"sub": test_user.email}
    return create_access_token(data=token_data)


@pytest.fixture
def auth_headers(auth_token: str) -> dict[str, str]:
    """Generate authentication headers with Bearer token.

    Creates headers dictionary with Authorization header
    for use in authenticated requests.

    Args:
        auth_token: JWT token from auth_token fixture

    Returns:
        dict: Headers dictionary with Authorization header
    """
    return {"Authorization": f"Bearer {auth_token}"}
