import 'package:dio/dio.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mockito/annotations.dart';
import 'package:mockito/mockito.dart';
import 'package:altair/core/api/api_client.dart';
import 'package:altair/features/auth/data/models/register_request.dart';
import 'package:altair/features/auth/data/repositories/token_repository.dart';
import 'package:altair/features/auth/data/services/auth_service.dart';

import 'auth_service_test.mocks.dart';

@GenerateMocks([ApiClient, TokenRepository, Dio])
void main() {
  group('AuthService', () {
    late MockApiClient mockApiClient;
    late MockTokenRepository mockTokenRepository;
    late MockDio mockDio;
    late AuthService authService;

    setUp(() {
      mockApiClient = MockApiClient();
      mockTokenRepository = MockTokenRepository();
      mockDio = MockDio();
      authService = AuthService(
        apiClient: mockApiClient,
        tokenRepository: mockTokenRepository,
      );

      // Setup rawDio to return our mock
      when(mockApiClient.rawDio).thenReturn(mockDio);
    });

    group('register', () {
      test('should register user successfully', () async {
        // Arrange
        const request = RegisterRequest(
          email: 'test@example.com',
          password: 'password123',
          username: 'testuser',
        );

        when(mockDio.post<Map<String, dynamic>>(
          '/auth/register',
          data: anyNamed('data'),
        )).thenAnswer((_) async => Response(
              requestOptions: RequestOptions(path: '/auth/register'),
              data: {
                'id': 'user-123',
                'email': 'test@example.com',
                'username': 'testuser',
                'created_at': '2024-01-15T10:30:00Z',
              },
              statusCode: 200,
            ));

        // Act
        final result = await authService.register(request);

        // Assert
        expect(result.id, 'user-123');
        expect(result.email, 'test@example.com');
        expect(result.username, 'testuser');

        verify(mockDio.post<Map<String, dynamic>>(
          '/auth/register',
          data: request.toJson(),
        )).called(1);
      });

      test('should throw exception when response data is null', () async {
        // Arrange
        const request = RegisterRequest(
          email: 'test@example.com',
          password: 'password123',
        );

        when(mockDio.post<Map<String, dynamic>>(
          '/auth/register',
          data: anyNamed('data'),
        )).thenAnswer((_) async => Response(
              requestOptions: RequestOptions(path: '/auth/register'),
              data: null,
              statusCode: 200,
            ));

        // Act & Assert
        expect(
          () => authService.register(request),
          throwsA(isA<Exception>()),
        );
      });

      test('should throw DioException when registration fails', () async {
        // Arrange
        const request = RegisterRequest(
          email: 'existing@example.com',
          password: 'password123',
        );

        when(mockDio.post<Map<String, dynamic>>(
          '/auth/register',
          data: anyNamed('data'),
        )).thenThrow(DioException(
          requestOptions: RequestOptions(path: '/auth/register'),
          response: Response(
            requestOptions: RequestOptions(path: '/auth/register'),
            statusCode: 400,
            data: {'detail': 'Email already exists'},
          ),
        ));

        // Act & Assert
        expect(
          () => authService.register(request),
          throwsA(isA<DioException>()),
        );
      });
    });

    group('login', () {
      test('should login successfully and save tokens', () async {
        // Arrange
        when(mockDio.post<Map<String, dynamic>>(
          '/auth/login',
          data: anyNamed('data'),
          options: anyNamed('options'),
        )).thenAnswer((_) async => Response(
              requestOptions: RequestOptions(path: '/auth/login'),
              data: {
                'access_token': 'test_access_token',
                'refresh_token': 'test_refresh_token',
                'token_type': 'bearer',
                'expires_in': 3600,
              },
              statusCode: 200,
            ));

        when(mockTokenRepository.saveTokens(any))
            .thenAnswer((_) async => {});

        // Act
        final result = await authService.login(
          email: 'test@example.com',
          password: 'password123',
        );

        // Assert
        expect(result.accessToken, 'test_access_token');
        expect(result.refreshToken, 'test_refresh_token');
        expect(result.tokenType, 'bearer');
        expect(result.expiresIn, 3600);

        // Verify login request was made with correct format
        final captured = verify(mockDio.post<Map<String, dynamic>>(
          '/auth/login',
          data: captureAnyNamed('data'),
          options: captureAnyNamed('options'),
        )).captured;

        final requestData = captured[0] as Map<String, dynamic>;
        expect(requestData['username'], 'test@example.com');
        expect(requestData['password'], 'password123');

        final options = captured[1] as Options;
        expect(options.contentType, Headers.formUrlEncodedContentType);

        // Verify tokens were saved
        verify(mockTokenRepository.saveTokens(result)).called(1);
      });

      test('should throw exception when response data is null', () async {
        // Arrange
        when(mockDio.post<Map<String, dynamic>>(
          '/auth/login',
          data: anyNamed('data'),
          options: anyNamed('options'),
        )).thenAnswer((_) async => Response(
              requestOptions: RequestOptions(path: '/auth/login'),
              data: null,
              statusCode: 200,
            ));

        // Act & Assert
        expect(
          () => authService.login(
            email: 'test@example.com',
            password: 'password123',
          ),
          throwsA(isA<Exception>()),
        );
      });

      test('should throw DioException when credentials are invalid', () async {
        // Arrange
        when(mockDio.post<Map<String, dynamic>>(
          '/auth/login',
          data: anyNamed('data'),
          options: anyNamed('options'),
        )).thenThrow(DioException(
          requestOptions: RequestOptions(path: '/auth/login'),
          response: Response(
            requestOptions: RequestOptions(path: '/auth/login'),
            statusCode: 401,
            data: {'detail': 'Invalid credentials'},
          ),
        ));

        // Act & Assert
        expect(
          () => authService.login(
            email: 'test@example.com',
            password: 'wrongpassword',
          ),
          throwsA(isA<DioException>()),
        );
      });
    });

    group('getCurrentUser', () {
      test('should get current user successfully', () async {
        // Arrange
        when(mockApiClient.get<Map<String, dynamic>>('/auth/me'))
            .thenAnswer((_) async => Response(
                  requestOptions: RequestOptions(path: '/auth/me'),
                  data: {
                    'id': 'user-123',
                    'email': 'test@example.com',
                    'username': 'testuser',
                    'created_at': '2024-01-15T10:30:00Z',
                  },
                  statusCode: 200,
                ));

        // Act
        final result = await authService.getCurrentUser();

        // Assert
        expect(result.id, 'user-123');
        expect(result.email, 'test@example.com');
        expect(result.username, 'testuser');

        verify(mockApiClient.get<Map<String, dynamic>>('/auth/me')).called(1);
      });

      test('should throw exception when response data is null', () async {
        // Arrange
        when(mockApiClient.get<Map<String, dynamic>>('/auth/me'))
            .thenAnswer((_) async => Response(
                  requestOptions: RequestOptions(path: '/auth/me'),
                  data: null,
                  statusCode: 200,
                ));

        // Act & Assert
        expect(
          () => authService.getCurrentUser(),
          throwsA(isA<Exception>()),
        );
      });

      test('should throw DioException when not authenticated', () async {
        // Arrange
        when(mockApiClient.get<Map<String, dynamic>>('/auth/me'))
            .thenThrow(DioException(
          requestOptions: RequestOptions(path: '/auth/me'),
          response: Response(
            requestOptions: RequestOptions(path: '/auth/me'),
            statusCode: 401,
            data: {'detail': 'Not authenticated'},
          ),
        ));

        // Act & Assert
        expect(
          () => authService.getCurrentUser(),
          throwsA(isA<DioException>()),
        );
      });
    });

    group('refreshTokens', () {
      test('should refresh tokens successfully', () async {
        // Arrange
        when(mockTokenRepository.getRefreshToken())
            .thenAnswer((_) async => 'old_refresh_token');

        when(mockDio.post<Map<String, dynamic>>(
          '/auth/refresh',
          data: anyNamed('data'),
        )).thenAnswer((_) async => Response(
              requestOptions: RequestOptions(path: '/auth/refresh'),
              data: {
                'access_token': 'new_access_token',
                'refresh_token': 'new_refresh_token',
                'token_type': 'bearer',
                'expires_in': 3600,
              },
              statusCode: 200,
            ));

        when(mockTokenRepository.saveTokens(any))
            .thenAnswer((_) async => {});

        // Act
        final result = await authService.refreshTokens();

        // Assert
        expect(result.accessToken, 'new_access_token');
        expect(result.refreshToken, 'new_refresh_token');

        verify(mockTokenRepository.getRefreshToken()).called(1);
        verify(mockDio.post<Map<String, dynamic>>(
          '/auth/refresh',
          data: {'refresh_token': 'old_refresh_token'},
        )).called(1);
        verify(mockTokenRepository.saveTokens(result)).called(1);
      });

      test('should throw exception when no refresh token available', () async {
        // Arrange
        when(mockTokenRepository.getRefreshToken())
            .thenAnswer((_) async => null);

        // Act & Assert
        expect(
          () => authService.refreshTokens(),
          throwsA(isA<Exception>()),
        );

        verifyNever(mockDio.post(any, data: anyNamed('data')));
      });

      test('should throw DioException when refresh token is invalid', () async {
        // Arrange
        when(mockTokenRepository.getRefreshToken())
            .thenAnswer((_) async => 'invalid_refresh_token');

        when(mockDio.post<Map<String, dynamic>>(
          '/auth/refresh',
          data: anyNamed('data'),
        )).thenThrow(DioException(
          requestOptions: RequestOptions(path: '/auth/refresh'),
          response: Response(
            requestOptions: RequestOptions(path: '/auth/refresh'),
            statusCode: 401,
            data: {'detail': 'Invalid refresh token'},
          ),
        ));

        // Act & Assert
        expect(
          () => authService.refreshTokens(),
          throwsA(isA<DioException>()),
        );
      });
    });

    group('logout', () {
      test('should logout successfully and clear tokens', () async {
        // Arrange
        when(mockApiClient.post('/auth/logout'))
            .thenAnswer((_) async => Response(
                  requestOptions: RequestOptions(path: '/auth/logout'),
                  statusCode: 200,
                ));

        when(mockTokenRepository.clearTokens()).thenAnswer((_) async => {});

        // Act
        await authService.logout();

        // Assert
        verify(mockApiClient.post('/auth/logout')).called(1);
        verify(mockTokenRepository.clearTokens()).called(1);
      });

      test('should clear tokens even when logout request fails', () async {
        // Arrange
        when(mockApiClient.post('/auth/logout')).thenThrow(DioException(
          requestOptions: RequestOptions(path: '/auth/logout'),
          error: 'Network error',
        ));

        when(mockTokenRepository.clearTokens()).thenAnswer((_) async => {});

        // Act
        await authService.logout();

        // Assert
        verify(mockApiClient.post('/auth/logout')).called(1);
        verify(mockTokenRepository.clearTokens()).called(1);
      });

      test('should clear tokens when not authenticated', () async {
        // Arrange
        when(mockApiClient.post('/auth/logout')).thenThrow(DioException(
          requestOptions: RequestOptions(path: '/auth/logout'),
          response: Response(
            requestOptions: RequestOptions(path: '/auth/logout'),
            statusCode: 401,
          ),
        ));

        when(mockTokenRepository.clearTokens()).thenAnswer((_) async => {});

        // Act
        await authService.logout();

        // Assert
        verify(mockTokenRepository.clearTokens()).called(1);
      });
    });

    group('isLoggedIn', () {
      test('should return true when tokens exist', () async {
        // Arrange
        when(mockTokenRepository.hasTokens()).thenAnswer((_) async => true);

        // Act
        final result = await authService.isLoggedIn();

        // Assert
        expect(result, isTrue);
        verify(mockTokenRepository.hasTokens()).called(1);
      });

      test('should return false when no tokens exist', () async {
        // Arrange
        when(mockTokenRepository.hasTokens()).thenAnswer((_) async => false);

        // Act
        final result = await authService.isLoggedIn();

        // Assert
        expect(result, isFalse);
        verify(mockTokenRepository.hasTokens()).called(1);
      });
    });
  });
}
