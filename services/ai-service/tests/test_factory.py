"""Tests for AI provider factory."""

import pytest
from unittest.mock import patch

from app.services.factory import get_ai_provider
from app.services.openai_service import OpenAIService
from app.services.anthropic_service import AnthropicService
from app.services.ollama_service import OllamaService


class TestGetAIProvider:
    """Tests for get_ai_provider factory function."""

    @patch("app.services.factory.settings")
    def test_get_openai_provider(self, mock_settings: object) -> None:
        """Test getting OpenAI provider.

        Args:
            mock_settings: Mocked settings
        """
        mock_settings.AI_PROVIDER = "openai"
        mock_settings.OPENAI_API_KEY = "test-key"
        mock_settings.OPENAI_MODEL = "gpt-4"

        provider = get_ai_provider()
        assert isinstance(provider, OpenAIService)

    @patch("app.services.factory.settings")
    def test_get_openai_without_key_fails(self, mock_settings: object) -> None:
        """Test OpenAI provider fails without API key.

        Args:
            mock_settings: Mocked settings
        """
        mock_settings.AI_PROVIDER = "openai"
        mock_settings.OPENAI_API_KEY = ""

        with pytest.raises(ValueError, match="OpenAI API key not configured"):
            get_ai_provider()

    @patch("app.services.factory.settings")
    def test_get_anthropic_provider(self, mock_settings: object) -> None:
        """Test getting Anthropic provider.

        Args:
            mock_settings: Mocked settings
        """
        mock_settings.AI_PROVIDER = "anthropic"
        mock_settings.ANTHROPIC_API_KEY = "test-key"
        mock_settings.ANTHROPIC_MODEL = "claude-3-5-sonnet-20241022"

        provider = get_ai_provider()
        assert isinstance(provider, AnthropicService)

    @patch("app.services.factory.settings")
    def test_get_anthropic_without_key_fails(self, mock_settings: object) -> None:
        """Test Anthropic provider fails without API key.

        Args:
            mock_settings: Mocked settings
        """
        mock_settings.AI_PROVIDER = "anthropic"
        mock_settings.ANTHROPIC_API_KEY = ""

        with pytest.raises(ValueError, match="Anthropic API key not configured"):
            get_ai_provider()

    @patch("app.services.factory.settings")
    def test_get_ollama_provider(self, mock_settings: object) -> None:
        """Test getting Ollama provider.

        Args:
            mock_settings: Mocked settings
        """
        mock_settings.AI_PROVIDER = "ollama"
        mock_settings.OLLAMA_BASE_URL = "http://localhost:11434"
        mock_settings.OLLAMA_MODEL = "llama3"

        provider = get_ai_provider()
        assert isinstance(provider, OllamaService)

    @patch("app.services.factory.settings")
    def test_invalid_provider_fails(self, mock_settings: object) -> None:
        """Test invalid provider raises error.

        Args:
            mock_settings: Mocked settings
        """
        mock_settings.AI_PROVIDER = "invalid"

        with pytest.raises(ValueError, match="Invalid AI provider"):
            get_ai_provider()
