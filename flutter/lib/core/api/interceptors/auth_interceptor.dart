import 'package:dio/dio.dart';

import '../../../features/auth/data/models/auth_tokens.dart';
import '../../../features/auth/data/repositories/token_repository.dart';

/// Dio interceptor that handles automatic token attachment and refresh.
///
/// This interceptor:
/// 1. Attaches the access token to all outgoing requests
/// 2. Intercepts 401 (Unauthorized) responses
/// 3. Attempts to refresh the token using the refresh token
/// 4. Retries the original request with the new access token
/// 5. Handles race conditions when multiple 401s occur simultaneously
///
/// If the refresh token is invalid or expired, the interceptor will
/// throw an exception, which should trigger logout in the app.
class AuthInterceptor extends Interceptor {
  final TokenRepository _tokenRepository;
  final Dio _refreshDio;

  /// Whether a token refresh is currently in progress
  bool _isRefreshing = false;

  /// Queue of pending request options waiting for token refresh
  final List<({RequestOptions options, ErrorInterceptorHandler handler})>
  _pendingRequests = [];

  AuthInterceptor({
    required TokenRepository tokenRepository,
    required String baseUrl,
  }) : _tokenRepository = tokenRepository,
       _refreshDio = Dio(
         BaseOptions(
           baseUrl: baseUrl,
           headers: {
             'Content-Type': 'application/json',
             'Accept': 'application/json',
           },
         ),
       );

  @override
  Future<void> onRequest(
    RequestOptions options,
    RequestInterceptorHandler handler,
  ) async {
    // Skip token attachment for auth endpoints (login, register, refresh)
    if (_isAuthEndpoint(options.path)) {
      return handler.next(options);
    }

    // Get access token from repository
    final accessToken = await _tokenRepository.getAccessToken();

    // Attach token to request if available
    if (accessToken != null) {
      options.headers['Authorization'] = 'Bearer $accessToken';
    }

    return handler.next(options);
  }

  @override
  Future<void> onError(
    DioException err,
    ErrorInterceptorHandler handler,
  ) async {
    // Only handle 401 Unauthorized errors
    if (err.response?.statusCode != 401) {
      return handler.next(err);
    }

    // Don't retry auth endpoints
    if (_isAuthEndpoint(err.requestOptions.path)) {
      return handler.next(err);
    }

    // If refresh is already in progress, queue this request
    if (_isRefreshing) {
      _pendingRequests.add((options: err.requestOptions, handler: handler));
      return;
    }

    // Start refresh process
    _isRefreshing = true;

    try {
      // Attempt to refresh the token
      final newTokens = await _refreshToken();

      // Update request with new token
      err.requestOptions.headers['Authorization'] =
          'Bearer ${newTokens.accessToken}';

      // Retry the original request
      final response = await Dio().fetch(err.requestOptions);

      // Process all pending requests with new token
      for (final pending in _pendingRequests) {
        pending.options.headers['Authorization'] =
            'Bearer ${newTokens.accessToken}';
        try {
          final pendingResponse = await Dio().fetch(pending.options);
          pending.handler.resolve(pendingResponse);
        } catch (e) {
          pending.handler.reject(
            e is DioException
                ? e
                : DioException(requestOptions: pending.options, error: e),
          );
        }
      }

      // Clear pending requests
      _pendingRequests.clear();

      // Resolve the original request
      return handler.resolve(response);
    } catch (e) {
      // Refresh failed - reject all pending requests
      for (final pending in _pendingRequests) {
        pending.handler.reject(
          DioException(
            requestOptions: pending.options,
            error: 'Token refresh failed',
            type: DioExceptionType.badResponse,
          ),
        );
      }

      _pendingRequests.clear();

      // Reject the original request
      return handler.reject(
        DioException(
          requestOptions: err.requestOptions,
          error: 'Token refresh failed',
          type: DioExceptionType.badResponse,
          response: err.response,
        ),
      );
    } finally {
      _isRefreshing = false;
    }
  }

  /// Check if the endpoint is an auth endpoint (login, register, refresh)
  ///
  /// These endpoints should not have token attachment or retry logic.
  bool _isAuthEndpoint(String path) {
    return path.contains('/auth/login') ||
        path.contains('/auth/register') ||
        path.contains('/auth/refresh');
  }

  /// Refresh the access token using the refresh token
  ///
  /// Makes a direct HTTP call to the /auth/refresh endpoint using a
  /// separate Dio instance to avoid interceptor loops.
  ///
  /// Throws [DioException] if refresh fails or refresh token is invalid.
  Future<AuthTokens> _refreshToken() async {
    final refreshToken = await _tokenRepository.getRefreshToken();

    if (refreshToken == null) {
      throw DioException(
        requestOptions: RequestOptions(path: '/auth/refresh'),
        error: 'No refresh token available',
        type: DioExceptionType.badResponse,
      );
    }

    final response = await _refreshDio.post<Map<String, dynamic>>(
      '/auth/refresh',
      data: {'refresh_token': refreshToken},
    );

    if (response.data == null) {
      throw DioException(
        requestOptions: RequestOptions(path: '/auth/refresh'),
        error: 'Empty response from refresh endpoint',
        type: DioExceptionType.badResponse,
      );
    }

    final tokens = AuthTokens.fromJson(response.data!);

    // Save new tokens to storage
    await _tokenRepository.saveTokens(tokens);

    return tokens;
  }
}
