"""Central import point for all Altair database models.

This module serves as the single source of truth for model imports, making it easy
for Alembic to discover all models through a single import statement.

Alembic's autogenerate feature requires all models to be imported so they can be
registered with the Base metadata. By centralizing imports here, we only need to
import this module in alembic/env.py to ensure all models are discovered.
"""

from altair.models.base import Base, BaseModel
from altair.models.user import User

__all__ = [
    "Base",
    "BaseModel",
    "User",
]
