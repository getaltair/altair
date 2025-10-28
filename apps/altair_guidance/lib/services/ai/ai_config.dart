/// Configuration for AI service.
library;

import '../../models/ai_settings.dart';
import 'providers/ai_provider.dart';
import 'providers/anthropic_provider.dart';
import 'providers/ollama_provider.dart';
import 'providers/openai_provider.dart';

/// Factory for creating AI providers based on configuration.
class AIConfig {
  /// Private constructor to prevent instantiation.
  AIConfig._();

  /// Creates an AI provider from user settings.
  ///
  /// Directly connects to AI provider APIs (OpenAI or Anthropic) instead of
  /// going through a backend proxy.
  ///
  /// Returns null if AI features are disabled or configuration is invalid.
  static AIProvider? createProvider(AISettings settings) {
    // Check if AI is enabled
    if (!settings.enabled) {
      return null;
    }

    switch (settings.provider) {
      case AIProviderType.openai:
        // OpenAI requires API key
        final apiKey = settings.openaiApiKey;
        if (apiKey == null || apiKey.isEmpty) {
          return null; // Invalid configuration
        }

        // Use configured model or default
        final model = settings.openaiModel;
        return OpenAIProvider(apiKey: apiKey, model: model);

      case AIProviderType.anthropic:
        // Anthropic requires API key
        final apiKey = settings.anthropicApiKey;
        if (apiKey == null || apiKey.isEmpty) {
          return null; // Invalid configuration
        }

        // Use configured model or default
        final model = settings.anthropicModel;
        return AnthropicProvider(apiKey: apiKey, model: model);

      case AIProviderType.ollama:
        // Ollama doesn't require API key, just base URL
        final baseUrl =
            settings.ollamaBaseUrl ?? AIProviderType.ollama.defaultBaseUrl;
        final model = settings.ollamaModel;
        return OllamaProvider(baseUrl: baseUrl, model: model);
    }
  }
}
