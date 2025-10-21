/// Configuration for AI service.
library;

import 'package:flutter/foundation.dart';

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
  /// Requires HTTPS and API key.
  factory AIConfig.production({
    required String baseUrl,
    required String apiKey,
  }) {
    assert(baseUrl.startsWith('https://'), 'Production must use HTTPS');
    assert(apiKey.isNotEmpty, 'Production requires API key');

    return AIConfig(
      baseUrl: baseUrl,
      apiKey: apiKey,
      enableSSL: true,
    );
  }

  /// Creates configuration from environment variables.
  factory AIConfig.fromEnvironment() {
    const baseUrl = String.fromEnvironment(
      'AI_SERVICE_URL',
      defaultValue: 'http://localhost:8001/api',
    );
    const apiKey = String.fromEnvironment('AI_SERVICE_API_KEY');

    // In production/release mode, enforce HTTPS
    if (kReleaseMode && !baseUrl.startsWith('https://')) {
      throw StateError(
        'AI_SERVICE_URL must use HTTPS in release mode. Got: $baseUrl',
      );
    }

    return AIConfig(
      baseUrl: baseUrl,
      apiKey: apiKey.isEmpty ? null : apiKey,
      enableSSL: kReleaseMode,
    );
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

    // Enforce HTTPS in production
    if (enableSSL && uri.scheme != 'https') {
      throw StateError('SSL is enabled but URL is not HTTPS: $baseUrl');
    }

    // Warn if no API key in production
    if (kReleaseMode && apiKey == null) {
      debugPrint(
        'WARNING: No API key configured for AI service in release mode',
      );
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
  String toString() => 'AIConfig(baseUrl: $baseUrl, hasApiKey: ${apiKey != null}, '
      'enableSSL: $enableSSL)';
}
