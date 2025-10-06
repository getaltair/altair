/// API configuration for the Altair backend.
///
/// Defines base URLs, timeouts, and other HTTP client settings.
class ApiConfig {
  ApiConfig._();

  /// Base URL for the FastAPI backend
  /// TODO: Make this configurable for different environments (dev/staging/prod)
  static const String baseUrl = 'http://localhost:8000';

  /// API version prefix
  static const String apiPrefix = '/api';

  /// Full base URL with API prefix
  static String get fullBaseUrl => '$baseUrl$apiPrefix';

  /// Connection timeout in milliseconds
  static const Duration connectTimeout = Duration(seconds: 10);

  /// Receive timeout in milliseconds (for reading response)
  static const Duration receiveTimeout = Duration(seconds: 30);

  /// Send timeout in milliseconds (for sending request)
  static const Duration sendTimeout = Duration(seconds: 30);

  /// Default headers for all requests
  static const Map<String, String> defaultHeaders = {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
  };

  /// Enable logging in debug mode
  static const bool enableLogging = true;
}
