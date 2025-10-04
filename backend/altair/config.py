"""Application configuration management using Pydantic Settings.

This module defines the application settings that can be configured via
environment variables or a .env file. Settings include database connections,
security parameters, and application-level configuration.
"""

from pydantic import model_validator
from pydantic_settings import BaseSettings, SettingsConfigDict

FAKE_KEY: str = "your-secret-key-change-this"


class Settings(BaseSettings):
    """Application settings loaded from environment variables or .env file.

    All settings have default values for local development but should be
    overridden in production via environment variables.

    Attributes:
        DATABASE_URL: PostgreSQL connection string
        REDIS_URL: Redis connection string for caching and sessions
        SECRET_KEY: JWT secret key for token signing (MUST be changed in production)
        ALGORITHM: JWT signing algorithm
        ACCESS_TOKEN_EXPIRE_MINUTES: JWT token expiration time in minutes
        APP_NAME: Application name used in API documentation
        DEBUG: Enable debug mode (disable in production)
    """

    # Database connections
    DATABASE_URL: str = "postgresql://altair:altair@localhost/altair"
    REDIS_URL: str = "redis://localhost:6379"

    # Security settings
    SECRET_KEY: str = FAKE_KEY
    ALGORITHM: str = "HS256"
    ACCESS_TOKEN_EXPIRE_MINUTES: int = 30

    # Application settings
    APP_NAME: str = "Altair"
    DEBUG: bool = True

    model_config = SettingsConfigDict(env_file=".env")

    @model_validator(mode="after")
    def validate_production_secrets(self) -> "Settings":
        if not self.DEBUG and self.SECRET_KEY == FAKE_KEY:
            raise ValueError(
                "SECRET_KEY must be changed in production."
                "Set the SECRET_KEY environment variable."
            )
        return self


settings = Settings()
