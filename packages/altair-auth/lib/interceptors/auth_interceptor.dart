/// HTTP interceptor for adding authentication headers.
library;

import 'package:dio/dio.dart';
import 'package:logger/logger.dart';

import '../services/secure_storage_service.dart';

/// Interceptor that adds JWT tokens to HTTP requests.
class AuthInterceptor extends Interceptor {
  /// Creates an authentication interceptor.
  AuthInterceptor({
    required SecureStorageService storage,
    Logger? logger,
  })  : _storage = storage,
        _logger = logger ?? Logger();

  final SecureStorageService _storage;
  final Logger _logger;

  @override
  Future<void> onRequest(
    RequestOptions options,
    RequestInterceptorHandler handler,
  ) async {
    try {
      // Skip auth for login/register endpoints
      if (_shouldSkipAuth(options.path)) {
        return handler.next(options);
      }

      final accessToken = await _storage.getAccessToken();
      final tokenType = await _storage.getTokenType();

      if (accessToken != null && tokenType != null) {
        options.headers['Authorization'] = '$tokenType $accessToken';
        _logger.d('Added authorization header to request: ${options.path}');
      } else {
        _logger.w('No access token available for request: ${options.path}');
      }

      return handler.next(options);
    } catch (e, stackTrace) {
      _logger.e('Failed to add auth header', error: e, stackTrace: stackTrace);
      return handler.next(options);
    }
  }

  @override
  void onError(DioException err, ErrorInterceptorHandler handler) {
    // Handle 401 Unauthorized responses
    if (err.response?.statusCode == 401) {
      _logger.w('Received 401 Unauthorized response');
      // Token might be expired or invalid
      // The app should handle this by triggering a refresh or logout
    }
    handler.next(err);
  }

  /// Checks if authentication should be skipped for the given path.
  bool _shouldSkipAuth(String path) {
    const skipPaths = [
      '/auth/login',
      '/auth/register',
      '/auth/refresh',
      '/health',
    ];

    return skipPaths.any((skipPath) => path.contains(skipPath));
  }
}
