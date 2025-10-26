"""Factory for creating AI provider instances."""

import logging
import os
from typing import Literal

from app.core.config import settings
from app.services.anthropic_service import AnthropicService
from app.services.base import AIProvider
from app.services.ollama_service import OllamaService
from app.services.openai_service import OpenAIService

logger = logging.getLogger(__name__)


def get_ai_provider(
    provider_name: Literal["openai", "anthropic", "ollama"] | None = None,
    api_key: str | None = None,
) -> AIProvider:
    """Get an AI provider instance with dynamic configuration.

    Args:
        provider_name: Provider to use (defaults to settings.AI_PROVIDER)
        api_key: API key for the provider (defaults to environment/settings)

    Returns:
        AIProvider instance based on configuration

    Raises:
        ValueError: If provider is not configured or invalid
    """
    # Use client-provided provider or fall back to server default
    provider = (provider_name or settings.AI_PROVIDER).lower()

    if provider == "openai":
        # Use client-provided API key or fall back to server config
        key = api_key or settings.OPENAI_API_KEY
        if not key:
            raise ValueError(
                "OpenAI API key not provided. "
                "Provide it in the request or configure OPENAI_API_KEY in server .env"
            )

        # Temporarily set environment variable for the provider
        # (OpenAIService reads from environment)
        if api_key:
            os.environ["OPENAI_API_KEY"] = api_key

        logger.info(f"Using OpenAI provider with model: {settings.OPENAI_MODEL}")
        logger.info(f"API key source: {'client-provided' if api_key else 'server config'}")
        return OpenAIService()

    elif provider == "anthropic":
        # Use client-provided API key or fall back to server config
        key = api_key or settings.ANTHROPIC_API_KEY
        if not key:
            raise ValueError(
                "Anthropic API key not provided. "
                "Provide it in the request or configure ANTHROPIC_API_KEY in server .env"
            )

        # Temporarily set environment variable for the provider
        if api_key:
            os.environ["ANTHROPIC_API_KEY"] = api_key

        logger.info(f"Using Anthropic provider with model: {settings.ANTHROPIC_MODEL}")
        logger.info(f"API key source: {'client-provided' if api_key else 'server config'}")
        return AnthropicService()

    elif provider == "ollama":
        logger.info(
            f"Using Ollama provider at {settings.OLLAMA_BASE_URL} "
            f"with model: {settings.OLLAMA_MODEL}"
        )
        return OllamaService()

    else:
        raise ValueError(
            f"Invalid AI provider: {provider}. " "Must be one of: openai, anthropic, ollama"
        )
