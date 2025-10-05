"""Database configuration and session management for Altair.

This module sets up the SQLAlchemy engine, session factory, and declarative base
for database operations. It provides dependency injection support for FastAPI
endpoints through the get_db() generator function.

Key components:
    - engine: SQLAlchemy database engine instance
    - SessionLocal: Session factory for creating database sessions
    - Base: Declarative base class for ORM models
    - get_db(): FastAPI dependency for database session injection

Usage:
    Import Base for model definitions:
        from altair.database import Base

        class User(Base):
            __tablename__ = "users"
            ...

    Use get_db() as a FastAPI dependency:
        @app.get("/users")
        def get_users(db: Session = Depends(get_db)):
            return db.query(User).all()
"""

from collections.abc import Iterator

from sqlalchemy import Engine, create_engine
from sqlalchemy.orm import DeclarativeBase, Session, sessionmaker

from altair.config import settings


# Declarative base for ORM model definitions
class Base(DeclarativeBase):
    pass


# Lazy initialization of database engine and session
_engine: Engine | None = None
_SessionLocal: sessionmaker[Session] | None = None


def get_engine() -> Engine:
    """Get or create the SQLAlchemy engine instance.

    Returns:
        Engine: SQLAlchemy database engine
    """
    global _engine
    if _engine is None:
        _engine = create_engine(settings.DATABASE_URL)
    return _engine


def get_session_local() -> sessionmaker[Session]:
    """Get or create the session factory.

    Returns:
        sessionmaker: SQLAlchemy session factory
    """
    global _SessionLocal
    if _SessionLocal is None:
        _SessionLocal = sessionmaker(
            autocommit=False, autoflush=False, bind=get_engine()
        )
    return _SessionLocal


def get_db() -> Iterator[Session]:
    """Provide a database session for FastAPI dependency injection.

    Creates a new SQLAlchemy session for each request and ensures proper
    cleanup after the request completes. This function is designed to be
    used with FastAPI's Depends() for automatic session management.

    Yields:
        Session: SQLAlchemy database session

    Example:
        @app.get("/items")
        def read_items(db: Session = Depends(get_db)):
            return db.query(Item).all()

    Note:
        The session is automatically closed in the finally block, ensuring
        connections are properly released even if an error occurs.
    """
    db = get_session_local()()
    try:
        yield db
    finally:
        db.close()
