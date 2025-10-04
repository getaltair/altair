"""Base model class for all Altair database models.

This module provides the BaseModel abstract class that serves as the foundation
for all SQLAlchemy ORM models in Altair. It includes common fields that every
entity needs: a UUID primary key and timestamp tracking for creation and updates.

Key features:
    - UUID primary keys for distributed system compatibility
    - Automatic timestamp tracking (created_at, updated_at)
    - PostgreSQL-optimized UUID storage
    - Abstract base to prevent direct instantiation

Usage:
    Inherit from BaseModel for all database models:

        from altair.models.base import BaseModel

        class Task(BaseModel):
            __tablename__ = "tasks"

            title = Column(String, nullable=False)
            # ... other fields

    The id, created_at, and updated_at fields are automatically included.
"""

import uuid
from sqlalchemy import Column, DateTime
from sqlalchemy.dialects.postgresql import UUID
from sqlalchemy.sql import func
from altair.database import Base


class BaseModel(Base):
    """Abstract base model providing common fields for all database entities.

    This class serves as the parent for all ORM models in Altair, providing
    standard fields that every entity requires. It uses UUIDs for primary keys
    to support distributed systems and avoid collision issues with auto-incrementing
    integers.

    The class is marked as abstract to prevent SQLAlchemy from creating a table
    for it directly. All models should inherit from this class rather than
    directly from SQLAlchemy's Base.

    Attributes:
        id (UUID): Primary key using UUID4 for global uniqueness. Generated
            automatically on creation.
        created_at (DateTime): Timestamp of when the record was created. Set
            automatically by the database server on insert.
        updated_at (DateTime): Timestamp of when the record was last modified.
            Updated automatically by the database on each update.

    Example:
        class User(BaseModel):
            __tablename__ = "users"

            email = Column(String, unique=True, nullable=False)
            name = Column(String, nullable=False)

        # When creating a new user, id and timestamps are handled automatically
        user = User(email="user@example.com", name="John Doe")
        # user.id is automatically generated
        # user.created_at will be set by database on commit
    """

    __abstract__ = True

    # UUID primary key for distributed system compatibility and global uniqueness
    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)

    # Timestamp set by database server when record is created
    created_at = Column(DateTime(timezone=True), server_default=func.now(), index=True)

    # Timestamp automatically updated by database on each modification
    updated_at = Column(
        DateTime(timezone=True), server_default=func.now(), onupdate=func.now()
    )
