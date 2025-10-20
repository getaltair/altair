"""Application configuration using pydantic-settings."""

from typing import Literal

from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    """Application settings."""

    model_config = SettingsConfigDict(
        env_file=".env",
        env_file_encoding="utf-8",
        case_sensitive=True,
    )

    # Application
    APP_NAME: str = "Altair AI Service"
    ENVIRONMENT: Literal["development", "staging", "production"] = "development"
    DEBUG: bool = False

    # API
    API_V1_PREFIX: str = "/api"

    # AI Provider Configuration
    AI_PROVIDER: Literal["openai", "anthropic", "ollama"] = "openai"

    # OpenAI Configuration
    OPENAI_API_KEY: str = ""
    OPENAI_MODEL: str = "gpt-4-turbo-preview"
    OPENAI_MAX_TOKENS: int = 2000
    OPENAI_TEMPERATURE: float = 0.7

    # Anthropic Configuration
    ANTHROPIC_API_KEY: str = ""
    ANTHROPIC_MODEL: str = "claude-3-5-sonnet-20241022"
    ANTHROPIC_MAX_TOKENS: int = 2000
    ANTHROPIC_TEMPERATURE: float = 0.7

    # Ollama Configuration
    OLLAMA_BASE_URL: str = "http://localhost:11434"
    OLLAMA_MODEL: str = "llama3"
    OLLAMA_MAX_TOKENS: int = 2000
    OLLAMA_TEMPERATURE: float = 0.7

    # Request Configuration
    AI_REQUEST_TIMEOUT: int = 30
    AI_MAX_RETRIES: int = 3
    AI_RETRY_DELAY: float = 1.0

    # CORS
    CORS_ORIGINS: list[str] = ["http://localhost:3000", "http://localhost:8000"]

    # Rate Limiting
    RATE_LIMIT_PER_MINUTE: int = 60


settings = Settings()
