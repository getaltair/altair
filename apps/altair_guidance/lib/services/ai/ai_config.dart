/// Configuration for AI service.
library;

/// Configuration for AI service endpoints and settings.
class AIConfig {
  /// Creates an AI configuration.
  const AIConfig({
    required this.baseUrl,
    this.apiKey,
    this.enableSSL = true,
    this.breakdownTimeout = const Duration(seconds: 60),
    this.prioritizationTimeout = const Duration(seconds: 45),
    this.estimateTimeout = const Duration(seconds: 30),
    this.suggestionsTimeout = const Duration(seconds: 30),
    this.healthCheckTimeout = const Duration(seconds: 5),
  }) : assert(baseUrl.length > 0, 'Base URL cannot be empty');

  /// Base URL for the AI service.
  /// Must include protocol (http:// or https://).
  final String baseUrl;

  /// Optional API key for authentication.
  /// If provided, will be sent as Bearer token.
  final String? apiKey;

  /// Whether to enforce SSL/TLS connections.
  /// Should be true in production.
  final bool enableSSL;

  /// Timeout for task breakdown operations.
  final Duration breakdownTimeout;

  /// Timeout for prioritization operations.
  final Duration prioritizationTimeout;

  /// Timeout for time estimation operations.
  final Duration estimateTimeout;

  /// Timeout for context suggestions operations.
  final Duration suggestionsTimeout;

  /// Timeout for health check operations.
  final Duration healthCheckTimeout;

  /// Creates a development configuration.
  /// Uses localhost and relaxed settings.
  factory AIConfig.development() {
    return const AIConfig(
      baseUrl: 'http://localhost:8001/api',
      enableSSL: false,
    );
  }

  /// Creates a production configuration.
  /// Recommends HTTPS and API key for security.
  factory AIConfig.production({
    required String baseUrl,
    required String apiKey,
  }) {
    return AIConfig(
      baseUrl: baseUrl,
      apiKey: apiKey.isEmpty ? null : apiKey,
      enableSSL: baseUrl.startsWith('https://'),
    );
  }

  /// Creates configuration from environment variables.
  factory AIConfig.fromEnvironment() {
    const baseUrl = String.fromEnvironment(
      'AI_SERVICE_URL',
      defaultValue: 'http://localhost:8001/api',
    );
    const apiKey = String.fromEnvironment('AI_SERVICE_API_KEY');

    // Enable SSL based on URL scheme
    final usesHttps = baseUrl.startsWith('https://');

    return AIConfig(
      baseUrl: baseUrl,
      apiKey: apiKey.isEmpty ? null : apiKey,
      enableSSL: usesHttps,
    );
  }

  /// Creates configuration from user settings.
  ///
  /// Connects to the Altair AI backend service which proxies requests to
  /// the configured AI provider (OpenAI, Anthropic, or Ollama).
  ///
  /// Returns null if AI features are disabled or configuration is invalid.
  static AIConfig? fromSettings(dynamic settings) {
    // Import will be: import '../../models/ai_settings.dart';
    // For now, using dynamic to avoid circular dependency

    // Check if AI is enabled
    if (settings.enabled == false) {
      return null;
    }

    // Get provider name (assumes AIProvider enum has name property)
    final providerName = settings.provider.toString().split('.').last;

    // Default backend service URL (can be overridden via customBaseUrl)
    const defaultBackendUrl = 'http://localhost:8001/api';

    switch (providerName) {
      case 'ollama':
        // For Ollama, use either custom URL or local Ollama server
        // If customBaseUrl is set, use it (backend service)
        // Otherwise, use ollama_baseUrl or default Ollama server
        final baseUrl = settings.customBaseUrl ??
            settings.ollamaBaseUrl ??
            settings.provider.defaultBaseUrl;

        return AIConfig(
          baseUrl: baseUrl,
          apiKey: null, // Ollama doesn't use API keys
          enableSSL: baseUrl.startsWith('https://'),
        );

      case 'openai':
        // OpenAI requires API key
        final apiKey = settings.openaiApiKey;
        if (apiKey == null || apiKey.isEmpty) {
          return null; // Invalid configuration
        }

        final baseUrl = settings.customBaseUrl ?? defaultBackendUrl;
        return AIConfig(
          baseUrl: baseUrl,
          apiKey: apiKey,
          enableSSL: baseUrl.startsWith('https://'),
        );

      case 'anthropic':
        // Anthropic requires API key
        final apiKey = settings.anthropicApiKey;
        if (apiKey == null || apiKey.isEmpty) {
          return null; // Invalid configuration
        }

        final baseUrl = settings.customBaseUrl ?? defaultBackendUrl;
        return AIConfig(
          baseUrl: baseUrl,
          apiKey: apiKey,
          enableSSL: baseUrl.startsWith('https://'),
        );

      default:
        return null;
    }
  }

  /// Validates the configuration.
  /// Throws [StateError] if configuration is invalid.
  void validate() {
    // Check URL format
    final uri = Uri.tryParse(baseUrl);
    if (uri == null) {
      throw StateError('Invalid base URL: $baseUrl');
    }

    if (!uri.hasScheme || (uri.scheme != 'http' && uri.scheme != 'https')) {
      throw StateError('Base URL must use http:// or https://: $baseUrl');
    }
  }

  /// Gets authentication headers if API key is configured.
  Map<String, String> get authHeaders {
    if (apiKey == null) return {};
    return {'Authorization': 'Bearer $apiKey'};
  }

  /// Gets standard headers including authentication.
  Map<String, String> get headers {
    return {
      'Content-Type': 'application/json',
      ...authHeaders,
    };
  }

  @override
  String toString() =>
      'AIConfig(baseUrl: $baseUrl, hasApiKey: ${apiKey != null}, '
      'enableSSL: $enableSSL)';
}
