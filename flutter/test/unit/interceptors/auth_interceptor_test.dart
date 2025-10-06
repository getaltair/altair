import 'package:altair/core/api/api_config.dart';
import 'package:altair/core/api/interceptors/auth_interceptor.dart';
import 'package:altair/features/auth/data/models/auth_tokens.dart';
import 'package:altair/features/auth/data/repositories/token_repository.dart';
import 'package:dio/dio.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mockito/annotations.dart';
import 'package:mockito/mockito.dart';

import 'auth_interceptor_test.mocks.dart';

@GenerateMocks([TokenRepository])
void main() {
  late MockTokenRepository mockTokenRepository;
  late AuthInterceptor authInterceptor;

  setUp(() {
    mockTokenRepository = MockTokenRepository();
    authInterceptor = AuthInterceptor(
      tokenRepository: mockTokenRepository,
      baseUrl: ApiConfig.fullBaseUrl,
    );
  });

  group('AuthInterceptor - Token Attachment', () {
    test('should attach access token to authenticated requests', () async {
      // Arrange
      const testToken = 'test-access-token';
      when(
        mockTokenRepository.getAccessToken(),
      ).thenAnswer((_) async => testToken);

      final requestOptions = RequestOptions(
        path: '/tasks',
        baseUrl: ApiConfig.fullBaseUrl,
      );
      final handler = _MockRequestInterceptorHandler();

      // Act
      await authInterceptor.onRequest(requestOptions, handler);

      // Assert
      expect(requestOptions.headers['Authorization'], 'Bearer $testToken');
      verify(mockTokenRepository.getAccessToken()).called(1);
    });

    test('should not attach token to login endpoint', () async {
      // Arrange
      final requestOptions = RequestOptions(
        path: '/auth/login',
        baseUrl: ApiConfig.fullBaseUrl,
      );
      final handler = _MockRequestInterceptorHandler();

      // Act
      await authInterceptor.onRequest(requestOptions, handler);

      // Assert
      expect(requestOptions.headers.containsKey('Authorization'), false);
      verifyNever(mockTokenRepository.getAccessToken());
    });

    test('should not attach token to register endpoint', () async {
      // Arrange
      final requestOptions = RequestOptions(
        path: '/auth/register',
        baseUrl: ApiConfig.fullBaseUrl,
      );
      final handler = _MockRequestInterceptorHandler();

      // Act
      await authInterceptor.onRequest(requestOptions, handler);

      // Assert
      expect(requestOptions.headers.containsKey('Authorization'), false);
      verifyNever(mockTokenRepository.getAccessToken());
    });

    test('should not attach token to refresh endpoint', () async {
      // Arrange
      final requestOptions = RequestOptions(
        path: '/auth/refresh',
        baseUrl: ApiConfig.fullBaseUrl,
      );
      final handler = _MockRequestInterceptorHandler();

      // Act
      await authInterceptor.onRequest(requestOptions, handler);

      // Assert
      expect(requestOptions.headers.containsKey('Authorization'), false);
      verifyNever(mockTokenRepository.getAccessToken());
    });

    test('should handle null access token gracefully', () async {
      // Arrange
      when(mockTokenRepository.getAccessToken()).thenAnswer((_) async => null);

      final requestOptions = RequestOptions(
        path: '/tasks',
        baseUrl: ApiConfig.fullBaseUrl,
      );
      final handler = _MockRequestInterceptorHandler();

      // Act
      await authInterceptor.onRequest(requestOptions, handler);

      // Assert
      expect(requestOptions.headers.containsKey('Authorization'), false);
      verify(mockTokenRepository.getAccessToken()).called(1);
    });
  });

  group('AuthInterceptor - Error Handling', () {
    test('should pass through non-401 errors', () async {
      // Arrange
      final error = DioException(
        requestOptions: RequestOptions(path: '/tasks'),
        response: Response(
          requestOptions: RequestOptions(path: '/tasks'),
          statusCode: 500,
        ),
      );
      final handler = _MockErrorInterceptorHandler();

      // Act
      await authInterceptor.onError(error, handler);

      // Assert - error should be passed through unchanged
      expect(handler.rejectedError, error);
    });

    test('should pass through 401 errors on auth endpoints', () async {
      // Arrange
      final error = DioException(
        requestOptions: RequestOptions(path: '/auth/login'),
        response: Response(
          requestOptions: RequestOptions(path: '/auth/login'),
          statusCode: 401,
        ),
      );
      final handler = _MockErrorInterceptorHandler();

      // Act
      await authInterceptor.onError(error, handler);

      // Assert - error should be passed through unchanged
      expect(handler.rejectedError, error);
    });
  });

  group('AuthInterceptor - Token Expiry', () {
    test('AuthTokens.isExpired() returns true for expired token', () {
      // Arrange
      final expiredTokens = AuthTokens(
        accessToken: 'expired-token',
        refreshToken: 'refresh-token',
        tokenType: 'bearer',
        expiresIn: 1800, // 30 minutes
        issuedAt: DateTime.now().subtract(const Duration(hours: 1)),
      );

      // Act & Assert
      expect(expiredTokens.isExpired(), true);
    });

    test('AuthTokens.isExpired() returns false for valid token', () {
      // Arrange
      final validTokens = AuthTokens(
        accessToken: 'valid-token',
        refreshToken: 'refresh-token',
        tokenType: 'bearer',
        expiresIn: 1800, // 30 minutes
        issuedAt: DateTime.now(),
      );

      // Act & Assert
      expect(validTokens.isExpired(), false);
    });

    test('AuthTokens.isNearExpiry() returns true when close to expiry', () {
      // Arrange
      final nearExpiryTokens = AuthTokens(
        accessToken: 'near-expiry-token',
        refreshToken: 'refresh-token',
        tokenType: 'bearer',
        expiresIn: 1800, // 30 minutes
        issuedAt: DateTime.now().subtract(const Duration(minutes: 27)),
      );

      // Act & Assert
      expect(nearExpiryTokens.isNearExpiry(), true);
    });

    test('AuthTokens.isNearExpiry() returns false when not near expiry', () {
      // Arrange
      final validTokens = AuthTokens(
        accessToken: 'valid-token',
        refreshToken: 'refresh-token',
        tokenType: 'bearer',
        expiresIn: 1800, // 30 minutes
        issuedAt: DateTime.now(),
      );

      // Act & Assert
      expect(validTokens.isNearExpiry(), false);
    });

    test('AuthTokens.timeUntilExpiry returns correct duration', () {
      // Arrange
      final tokens = AuthTokens(
        accessToken: 'token',
        refreshToken: 'refresh-token',
        tokenType: 'bearer',
        expiresIn: 1800, // 30 minutes
        issuedAt: DateTime.now(),
      );

      // Act
      final timeUntilExpiry = tokens.timeUntilExpiry;

      // Assert - should be approximately 30 minutes (allowing for test execution time)
      expect(timeUntilExpiry.inSeconds, greaterThan(1790));
      expect(timeUntilExpiry.inSeconds, lessThanOrEqualTo(1800));
    });

    test('AuthTokens.timeUntilExpiry returns zero for expired token', () {
      // Arrange
      final expiredTokens = AuthTokens(
        accessToken: 'expired-token',
        refreshToken: 'refresh-token',
        tokenType: 'bearer',
        expiresIn: 1800,
        issuedAt: DateTime.now().subtract(const Duration(hours: 1)),
      );

      // Act
      final timeUntilExpiry = expiredTokens.timeUntilExpiry;

      // Assert
      expect(timeUntilExpiry, Duration.zero);
    });
  });
}

/// Mock request interceptor handler for testing
class _MockRequestInterceptorHandler extends RequestInterceptorHandler {
  @override
  void next(RequestOptions requestOptions) {
    // Do nothing - just for testing
  }
}

/// Mock error interceptor handler for testing
class _MockErrorInterceptorHandler extends ErrorInterceptorHandler {
  DioException? rejectedError;

  @override
  void next(DioException err) {
    rejectedError = err;
  }

  @override
  void reject(DioException error, [bool stackTrace = true]) {
    rejectedError = error;
  }

  @override
  void resolve(Response response) {
    // Do nothing - just for testing
  }
}
