from sqlalchemy import Boolean, Column, String
from altair.models.base import BaseModel


class User(BaseModel):
    __tablename__ = "users"

    email = Column(String(255), nullable=False, unique=True, index=True)
    hashed_password = Column(String(1024))
    is_active = Column(Boolean, default=True)
