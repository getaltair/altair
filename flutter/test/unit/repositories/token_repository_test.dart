import 'package:flutter_test/flutter_test.dart';
import 'package:mockito/annotations.dart';
import 'package:mockito/mockito.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:altair/features/auth/data/models/auth_tokens.dart';
import 'package:altair/features/auth/data/repositories/token_repository.dart';

import 'token_repository_test.mocks.dart';

@GenerateMocks([FlutterSecureStorage])
void main() {
  group('SecureTokenRepository', () {
    late MockFlutterSecureStorage mockStorage;
    late SecureTokenRepository repository;

    final testTokens = AuthTokens(
      accessToken: 'test_access_token',
      refreshToken: 'test_refresh_token',
      tokenType: 'bearer',
      expiresIn: 3600,
      issuedAt: DateTime.now(),
    );

    setUp(() {
      mockStorage = MockFlutterSecureStorage();
      repository = SecureTokenRepository(mockStorage);
    });

    group('saveTokens', () {
      test('should save all token fields to secure storage', () async {
        // Act
        await repository.saveTokens(testTokens);

        // Assert
        verify(mockStorage.write(
          key: 'access_token',
          value: 'test_access_token',
        )).called(1);
        verify(mockStorage.write(
          key: 'refresh_token',
          value: 'test_refresh_token',
        )).called(1);
        verify(mockStorage.write(
          key: 'token_type',
          value: 'bearer',
        )).called(1);
        verify(mockStorage.write(
          key: 'expires_in',
          value: '3600',
        )).called(1);
        verify(mockStorage.write(
          key: 'issued_at',
          value: anyNamed('value'),
        )).called(1);
        verifyNoMoreInteractions(mockStorage);
      });

      test('should cache tokens in memory', () async {
        // Act
        await repository.saveTokens(testTokens);

        // Assert - subsequent calls should use cache
        final accessToken = await repository.getAccessToken();
        expect(accessToken, 'test_access_token');

        // Verify no additional reads from storage (uses cache)
        verifyNever(mockStorage.read(key: anyNamed('key')));
      });
    });

    group('getAccessToken', () {
      test('should return access token from cache when available', () async {
        // Arrange - save tokens to populate cache
        await repository.saveTokens(testTokens);

        // Act
        final result = await repository.getAccessToken();

        // Assert
        expect(result, 'test_access_token');
        // Should not read from storage when cache is available
        verifyNever(mockStorage.read(key: anyNamed('key')));
      });

      test('should load from storage when cache is empty', () async {
        // Arrange
        when(mockStorage.read(key: 'access_token'))
            .thenAnswer((_) async => 'stored_access_token');
        when(mockStorage.read(key: 'refresh_token'))
            .thenAnswer((_) async => 'stored_refresh_token');
        when(mockStorage.read(key: 'token_type'))
            .thenAnswer((_) async => 'bearer');
        when(mockStorage.read(key: 'expires_in'))
            .thenAnswer((_) async => '3600');
        when(mockStorage.read(key: 'issued_at'))
            .thenAnswer((_) async => DateTime.now().toIso8601String());

        // Act
        final result = await repository.getAccessToken();

        // Assert
        expect(result, 'stored_access_token');
        verify(mockStorage.read(key: 'access_token')).called(1);
      });

      test('should return null when no tokens exist', () async {
        // Arrange
        when(mockStorage.read(key: anyNamed('key')))
            .thenAnswer((_) async => null);

        // Act
        final result = await repository.getAccessToken();

        // Assert
        expect(result, isNull);
      });
    });

    group('getRefreshToken', () {
      test('should return refresh token from cache when available', () async {
        // Arrange
        await repository.saveTokens(testTokens);

        // Act
        final result = await repository.getRefreshToken();

        // Assert
        expect(result, 'test_refresh_token');
        verifyNever(mockStorage.read(key: anyNamed('key')));
      });

      test('should load from storage when cache is empty', () async {
        // Arrange
        when(mockStorage.read(key: 'access_token'))
            .thenAnswer((_) async => 'stored_access_token');
        when(mockStorage.read(key: 'refresh_token'))
            .thenAnswer((_) async => 'stored_refresh_token');
        when(mockStorage.read(key: 'token_type'))
            .thenAnswer((_) async => 'bearer');
        when(mockStorage.read(key: 'expires_in'))
            .thenAnswer((_) async => '3600');
        when(mockStorage.read(key: 'issued_at'))
            .thenAnswer((_) async => DateTime.now().toIso8601String());

        // Act
        final result = await repository.getRefreshToken();

        // Assert
        expect(result, 'stored_refresh_token');
      });
    });

    group('getTokens', () {
      test('should return full token object from cache', () async {
        // Arrange
        await repository.saveTokens(testTokens);

        // Act
        final result = await repository.getTokens();

        // Assert
        expect(result, equals(testTokens));
        verifyNever(mockStorage.read(key: anyNamed('key')));
      });

      test('should load and construct tokens from storage', () async {
        // Arrange
        when(mockStorage.read(key: 'access_token'))
            .thenAnswer((_) async => 'stored_access');
        when(mockStorage.read(key: 'refresh_token'))
            .thenAnswer((_) async => 'stored_refresh');
        when(mockStorage.read(key: 'token_type'))
            .thenAnswer((_) async => 'bearer');
        when(mockStorage.read(key: 'expires_in'))
            .thenAnswer((_) async => '7200');
        when(mockStorage.read(key: 'issued_at'))
            .thenAnswer((_) async => DateTime.now().toIso8601String());

        // Act
        final result = await repository.getTokens();

        // Assert
        expect(result?.accessToken, 'stored_access');
        expect(result?.refreshToken, 'stored_refresh');
        expect(result?.tokenType, 'bearer');
        expect(result?.expiresIn, 7200);
      });

      test('should return null when any required field is missing', () async {
        // Arrange - missing access token
        when(mockStorage.read(key: 'access_token'))
            .thenAnswer((_) async => null);
        when(mockStorage.read(key: 'refresh_token'))
            .thenAnswer((_) async => 'stored_refresh');
        when(mockStorage.read(key: 'token_type'))
            .thenAnswer((_) async => 'bearer');
        when(mockStorage.read(key: 'expires_in'))
            .thenAnswer((_) async => '3600');
        when(mockStorage.read(key: 'issued_at'))
            .thenAnswer((_) async => DateTime.now().toIso8601String());

        // Act
        final result = await repository.getTokens();

        // Assert
        expect(result, isNull);
      });

      test('should cache tokens loaded from storage', () async {
        // Arrange
        when(mockStorage.read(key: 'access_token'))
            .thenAnswer((_) async => 'stored_access');
        when(mockStorage.read(key: 'refresh_token'))
            .thenAnswer((_) async => 'stored_refresh');
        when(mockStorage.read(key: 'token_type'))
            .thenAnswer((_) async => 'bearer');
        when(mockStorage.read(key: 'expires_in'))
            .thenAnswer((_) async => '3600');
        when(mockStorage.read(key: 'issued_at'))
            .thenAnswer((_) async => DateTime.now().toIso8601String());

        // Act - first call loads from storage
        final result1 = await repository.getTokens();
        // Second call should use cache
        final result2 = await repository.getTokens();

        // Assert
        expect(result1, equals(result2));
        // Should only read once (first time)
        verify(mockStorage.read(key: anyNamed('key'))).called(5); // 5 fields (including issued_at)
      });
    });

    group('clearTokens', () {
      test('should clear in-memory cache', () async {
        // Arrange - save tokens first
        await repository.saveTokens(testTokens);

        // Verify tokens are cached
        final beforeClear = await repository.getAccessToken();
        expect(beforeClear, isNotNull);

        // Act
        await repository.clearTokens();

        // Assert - cache should be cleared
        when(mockStorage.read(key: anyNamed('key')))
            .thenAnswer((_) async => null);
        final afterClear = await repository.getAccessToken();
        expect(afterClear, isNull);
      });

      test('should delete all data from secure storage', () async {
        // Act
        await repository.clearTokens();

        // Assert
        verify(mockStorage.deleteAll()).called(1);
      });
    });

    group('hasTokens', () {
      test('should return true when tokens exist in cache', () async {
        // Arrange
        await repository.saveTokens(testTokens);

        // Act
        final result = await repository.hasTokens();

        // Assert
        expect(result, isTrue);
      });

      test('should return true when tokens exist in storage', () async {
        // Arrange
        when(mockStorage.read(key: 'access_token'))
            .thenAnswer((_) async => 'stored_access');
        when(mockStorage.read(key: 'refresh_token'))
            .thenAnswer((_) async => 'stored_refresh');
        when(mockStorage.read(key: 'token_type'))
            .thenAnswer((_) async => 'bearer');
        when(mockStorage.read(key: 'expires_in'))
            .thenAnswer((_) async => '3600');
        when(mockStorage.read(key: 'issued_at'))
            .thenAnswer((_) async => DateTime.now().toIso8601String());

        // Act
        final result = await repository.hasTokens();

        // Assert
        expect(result, isTrue);
      });

      test('should return false when no tokens exist', () async {
        // Arrange
        when(mockStorage.read(key: anyNamed('key')))
            .thenAnswer((_) async => null);

        // Act
        final result = await repository.hasTokens();

        // Assert
        expect(result, isFalse);
      });

      test('should return false after clearing tokens', () async {
        // Arrange
        await repository.saveTokens(testTokens);
        await repository.clearTokens();

        when(mockStorage.read(key: anyNamed('key')))
            .thenAnswer((_) async => null);

        // Act
        final result = await repository.hasTokens();

        // Assert
        expect(result, isFalse);
      });
    });

    group('edge cases', () {
      test('should handle invalid expires_in format gracefully', () async {
        // Arrange
        when(mockStorage.read(key: 'access_token'))
            .thenAnswer((_) async => 'stored_access');
        when(mockStorage.read(key: 'refresh_token'))
            .thenAnswer((_) async => 'stored_refresh');
        when(mockStorage.read(key: 'token_type'))
            .thenAnswer((_) async => 'bearer');
        when(mockStorage.read(key: 'expires_in'))
            .thenAnswer((_) async => 'invalid_number');
        when(mockStorage.read(key: 'issued_at'))
            .thenAnswer((_) async => DateTime.now().toIso8601String());

        // Act & Assert
        expect(
          () => repository.getTokens(),
          throwsA(isA<FormatException>()),
        );
      });

      test('should handle storage errors gracefully', () async {
        // Arrange
        when(mockStorage.read(key: anyNamed('key')))
            .thenThrow(Exception('Storage error'));

        // Act & Assert
        expect(
          () => repository.getAccessToken(),
          throwsException,
        );
      });
    });
  });
}
