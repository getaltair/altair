/// AI settings model.
library;

import 'package:equatable/equatable.dart';

/// Supported AI providers.
enum AIProvider {
  /// OpenAI (GPT models).
  openai('OpenAI', 'https://api.openai.com/v1'),

  /// Anthropic (Claude models).
  anthropic('Anthropic', 'https://api.anthropic.com/v1'),

  /// Ollama (local models).
  ollama('Ollama (Local)', 'http://localhost:11434');

  const AIProvider(this.displayName, this.defaultBaseUrl);

  /// User-friendly display name.
  final String displayName;

  /// Default base URL for this provider.
  final String defaultBaseUrl;
}

/// AI settings configuration.
class AISettings extends Equatable {
  /// Creates AI settings.
  const AISettings({
    this.enabled = false,
    this.provider = AIProvider.ollama,
    this.openaiApiKey,
    this.anthropicApiKey,
    this.ollamaBaseUrl,
    this.openaiModel = 'gpt-4-turbo-preview',
    this.anthropicModel = 'claude-3-5-sonnet-20241022',
    this.ollamaModel = 'llama3',
    this.customBaseUrl,
  });

  /// Whether AI features are enabled.
  final bool enabled;

  /// Selected AI provider.
  final AIProvider provider;

  /// OpenAI API key (stored securely).
  final String? openaiApiKey;

  /// Anthropic API key (stored securely).
  final String? anthropicApiKey;

  /// Ollama base URL (default: http://localhost:11434).
  final String? ollamaBaseUrl;

  /// Selected OpenAI model.
  final String openaiModel;

  /// Selected Anthropic model.
  final String anthropicModel;

  /// Selected Ollama model.
  final String ollamaModel;

  /// Custom base URL (for self-hosted services).
  final String? customBaseUrl;

  /// Gets the current API key based on selected provider.
  String? get currentApiKey {
    switch (provider) {
      case AIProvider.openai:
        return openaiApiKey;
      case AIProvider.anthropic:
        return anthropicApiKey;
      case AIProvider.ollama:
        return null; // Ollama doesn't need API key
    }
  }

  /// Gets the current model based on selected provider.
  String get currentModel {
    switch (provider) {
      case AIProvider.openai:
        return openaiModel;
      case AIProvider.anthropic:
        return anthropicModel;
      case AIProvider.ollama:
        return ollamaModel;
    }
  }

  /// Gets the base URL for the current provider.
  String get baseUrl {
    if (customBaseUrl != null && customBaseUrl!.isNotEmpty) {
      return customBaseUrl!;
    }

    switch (provider) {
      case AIProvider.openai:
        return AIProvider.openai.defaultBaseUrl;
      case AIProvider.anthropic:
        return AIProvider.anthropic.defaultBaseUrl;
      case AIProvider.ollama:
        return ollamaBaseUrl ?? AIProvider.ollama.defaultBaseUrl;
    }
  }

  /// Whether the current configuration is valid.
  bool get isValid {
    if (!enabled) return true;

    // Check if API key is provided for cloud providers
    if (provider == AIProvider.openai && (openaiApiKey == null || openaiApiKey!.isEmpty)) {
      return false;
    }
    if (provider == AIProvider.anthropic && (anthropicApiKey == null || anthropicApiKey!.isEmpty)) {
      return false;
    }

    // Ollama doesn't need API key, just ensure base URL is set
    if (provider == AIProvider.ollama) {
      final url = ollamaBaseUrl ?? AIProvider.ollama.defaultBaseUrl;
      if (url.isEmpty) return false;
    }

    return true;
  }

  /// Creates a copy with updated fields.
  AISettings copyWith({
    bool? enabled,
    AIProvider? provider,
    String? openaiApiKey,
    String? anthropicApiKey,
    String? ollamaBaseUrl,
    String? openaiModel,
    String? anthropicModel,
    String? ollamaModel,
    String? customBaseUrl,
  }) {
    return AISettings(
      enabled: enabled ?? this.enabled,
      provider: provider ?? this.provider,
      openaiApiKey: openaiApiKey ?? this.openaiApiKey,
      anthropicApiKey: anthropicApiKey ?? this.anthropicApiKey,
      ollamaBaseUrl: ollamaBaseUrl ?? this.ollamaBaseUrl,
      openaiModel: openaiModel ?? this.openaiModel,
      anthropicModel: anthropicModel ?? this.anthropicModel,
      ollamaModel: ollamaModel ?? this.ollamaModel,
      customBaseUrl: customBaseUrl ?? this.customBaseUrl,
    );
  }

  /// Converts to JSON for persistence.
  Map<String, dynamic> toJson() {
    return {
      'enabled': enabled,
      'provider': provider.name,
      'ollamaBaseUrl': ollamaBaseUrl,
      'openaiModel': openaiModel,
      'anthropicModel': anthropicModel,
      'ollamaModel': ollamaModel,
      'customBaseUrl': customBaseUrl,
      // Note: API keys are NOT stored in JSON (stored securely separately)
    };
  }

  /// Creates from JSON.
  factory AISettings.fromJson(Map<String, dynamic> json) {
    return AISettings(
      enabled: json['enabled'] as bool? ?? false,
      provider: AIProvider.values.firstWhere(
        (p) => p.name == json['provider'],
        orElse: () => AIProvider.ollama,
      ),
      ollamaBaseUrl: json['ollamaBaseUrl'] as String?,
      openaiModel: json['openaiModel'] as String? ?? 'gpt-4-turbo-preview',
      anthropicModel: json['anthropicModel'] as String? ?? 'claude-3-5-sonnet-20241022',
      ollamaModel: json['ollamaModel'] as String? ?? 'llama3',
      customBaseUrl: json['customBaseUrl'] as String?,
    );
  }

  @override
  List<Object?> get props => [
        enabled,
        provider,
        openaiApiKey,
        anthropicApiKey,
        ollamaBaseUrl,
        openaiModel,
        anthropicModel,
        ollamaModel,
        customBaseUrl,
      ];

  @override
  String toString() => 'AISettings('
      'enabled: $enabled, '
      'provider: ${provider.displayName}, '
      'model: $currentModel, '
      'valid: $isValid)';
}
