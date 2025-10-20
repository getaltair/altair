"""Factory for creating AI provider instances."""

import logging

from app.core.config import settings
from app.services.anthropic_service import AnthropicService
from app.services.base import AIProvider
from app.services.ollama_service import OllamaService
from app.services.openai_service import OpenAIService

logger = logging.getLogger(__name__)


def get_ai_provider() -> AIProvider:
    """Get the configured AI provider instance.

    Returns:
        AIProvider instance based on configuration

    Raises:
        ValueError: If provider is not configured or invalid
    """
    provider = settings.AI_PROVIDER.lower()

    if provider == "openai":
        if not settings.OPENAI_API_KEY:
            raise ValueError("OpenAI API key not configured")
        logger.info(f"Using OpenAI provider with model: {settings.OPENAI_MODEL}")
        return OpenAIService()

    elif provider == "anthropic":
        if not settings.ANTHROPIC_API_KEY:
            raise ValueError("Anthropic API key not configured")
        logger.info(f"Using Anthropic provider with model: {settings.ANTHROPIC_MODEL}")
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
