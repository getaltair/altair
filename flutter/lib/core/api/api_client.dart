import 'package:dio/dio.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

import '../../features/auth/data/repositories/token_repository.dart';
import 'api_config.dart';
import 'interceptors/auth_interceptor.dart';

part 'api_client.g.dart';

/// Provides a singleton Dio instance configured for the Altair API
@riverpod
Dio dio(Ref ref) {
  final dio = Dio(
    BaseOptions(
      baseUrl: ApiConfig.fullBaseUrl,
      connectTimeout: ApiConfig.connectTimeout,
      receiveTimeout: ApiConfig.receiveTimeout,
      sendTimeout: ApiConfig.sendTimeout,
      headers: ApiConfig.defaultHeaders,
    ),
  );

  // Add auth interceptor for automatic token attachment and refresh
  dio.interceptors.add(
    AuthInterceptor(
      tokenRepository: ref.watch(tokenRepositoryProvider),
      baseUrl: ApiConfig.fullBaseUrl,
    ),
  );

  // Add logging interceptor in debug mode (after auth interceptor)
  if (ApiConfig.enableLogging) {
    dio.interceptors.add(
      LogInterceptor(
        requestHeader: true,
        requestBody: true,
        responseHeader: true,
        responseBody: true,
        error: true,
      ),
    );
  }

  return dio;
}

/// Provides the API client instance
@riverpod
ApiClient apiClient(Ref ref) {
  return ApiClient(
    dio: ref.watch(dioProvider),
    tokenRepository: ref.watch(tokenRepositoryProvider),
  );
}

/// API client for making authenticated and unauthenticated requests.
///
/// Wraps Dio with convenience methods for common HTTP operations.
/// Token attachment and refresh are handled automatically by the
/// AuthInterceptor, so these methods simply delegate to Dio.
class ApiClient {
  final Dio _dio;
  final TokenRepository _tokenRepository;

  ApiClient({
    required Dio dio,
    required TokenRepository tokenRepository,
  })  : _dio = dio,
        _tokenRepository = tokenRepository;

  /// Make an authenticated GET request
  ///
  /// Token attachment and refresh are handled automatically by AuthInterceptor.
  /// Throws [DioException] if the request fails.
  Future<Response<T>> get<T>(
    String path, {
    Map<String, dynamic>? queryParameters,
    Options? options,
  }) async {
    return _dio.get<T>(
      path,
      queryParameters: queryParameters,
      options: options,
    );
  }

  /// Make an authenticated POST request
  ///
  /// Token attachment and refresh are handled automatically by AuthInterceptor.
  /// Throws [DioException] if the request fails.
  Future<Response<T>> post<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
  }) async {
    return _dio.post<T>(
      path,
      data: data,
      queryParameters: queryParameters,
      options: options,
    );
  }

  /// Make an authenticated PUT request
  ///
  /// Token attachment and refresh are handled automatically by AuthInterceptor.
  /// Throws [DioException] if the request fails.
  Future<Response<T>> put<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
  }) async {
    return _dio.put<T>(
      path,
      data: data,
      queryParameters: queryParameters,
      options: options,
    );
  }

  /// Make an authenticated DELETE request
  ///
  /// Token attachment and refresh are handled automatically by AuthInterceptor.
  /// Throws [DioException] if the request fails.
  Future<Response<T>> delete<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
  }) async {
    return _dio.delete<T>(
      path,
      data: data,
      queryParameters: queryParameters,
      options: options,
    );
  }

  /// Make an unauthenticated request (for login, register, etc.)
  ///
  /// Use the raw Dio instance for endpoints that don't require authentication.
  Dio get rawDio => _dio;
}
