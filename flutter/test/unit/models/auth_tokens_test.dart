import 'package:flutter_test/flutter_test.dart';
import 'package:altair/features/auth/data/models/auth_tokens.dart';

void main() {
  group('AuthTokens', () {
    group('fromJson', () {
      test('should correctly deserialize from JSON', () {
        // Arrange
        final json = {
          'access_token': 'test_access_token',
          'refresh_token': 'test_refresh_token',
          'token_type': 'bearer',
          'expires_in': 3600,
        };

        // Act
        final tokens = AuthTokens.fromJson(json);

        // Assert
        expect(tokens.accessToken, 'test_access_token');
        expect(tokens.refreshToken, 'test_refresh_token');
        expect(tokens.tokenType, 'bearer');
        expect(tokens.expiresIn, 3600);
      });

      test('should throw when required fields are missing', () {
        // Arrange
        final invalidJson = {
          'access_token': 'test_access_token',
          // missing other required fields
        };

        // Act & Assert
        expect(
          () => AuthTokens.fromJson(invalidJson),
          throwsA(isA<TypeError>()),
        );
      });

      test('should throw when field types are wrong', () {
        // Arrange
        final invalidJson = {
          'access_token': 'test_access_token',
          'refresh_token': 'test_refresh_token',
          'token_type': 'bearer',
          'expires_in': 'not_a_number', // Should be int
        };

        // Act & Assert
        expect(
          () => AuthTokens.fromJson(invalidJson),
          throwsA(isA<TypeError>()),
        );
      });
    });

    group('toJson', () {
      test('should correctly serialize to JSON', () {
        // Arrange
        const tokens = AuthTokens(
          accessToken: 'test_access_token',
          refreshToken: 'test_refresh_token',
          tokenType: 'bearer',
          expiresIn: 3600,
        );

        // Act
        final json = tokens.toJson();

        // Assert
        expect(json['access_token'], 'test_access_token');
        expect(json['refresh_token'], 'test_refresh_token');
        expect(json['token_type'], 'bearer');
        expect(json['expires_in'], 3600);
      });

      test('should round-trip correctly (fromJson -> toJson)', () {
        // Arrange
        final originalJson = {
          'access_token': 'test_access_token',
          'refresh_token': 'test_refresh_token',
          'token_type': 'bearer',
          'expires_in': 3600,
        };

        // Act
        final tokens = AuthTokens.fromJson(originalJson);
        final resultJson = tokens.toJson();

        // Assert
        expect(resultJson, equals(originalJson));
      });
    });

    group('equality', () {
      test('should be equal when all fields match', () {
        // Arrange
        const tokens1 = AuthTokens(
          accessToken: 'token1',
          refreshToken: 'token2',
          tokenType: 'bearer',
          expiresIn: 3600,
        );

        const tokens2 = AuthTokens(
          accessToken: 'token1',
          refreshToken: 'token2',
          tokenType: 'bearer',
          expiresIn: 3600,
        );

        // Assert
        expect(tokens1, equals(tokens2));
        expect(tokens1.hashCode, equals(tokens2.hashCode));
      });

      test('should not be equal when access token differs', () {
        // Arrange
        const tokens1 = AuthTokens(
          accessToken: 'token1',
          refreshToken: 'token2',
          tokenType: 'bearer',
          expiresIn: 3600,
        );

        const tokens2 = AuthTokens(
          accessToken: 'different_token',
          refreshToken: 'token2',
          tokenType: 'bearer',
          expiresIn: 3600,
        );

        // Assert
        expect(tokens1, isNot(equals(tokens2)));
      });

      test('should not be equal when expires_in differs', () {
        // Arrange
        const tokens1 = AuthTokens(
          accessToken: 'token1',
          refreshToken: 'token2',
          tokenType: 'bearer',
          expiresIn: 3600,
        );

        const tokens2 = AuthTokens(
          accessToken: 'token1',
          refreshToken: 'token2',
          tokenType: 'bearer',
          expiresIn: 7200,
        );

        // Assert
        expect(tokens1, isNot(equals(tokens2)));
      });
    });

    group('toString', () {
      test('should truncate tokens in string representation', () {
        // Arrange
        const tokens = AuthTokens(
          accessToken: 'this_is_a_very_long_access_token_that_should_be_truncated',
          refreshToken: 'this_is_a_very_long_refresh_token_that_should_be_truncated',
          tokenType: 'bearer',
          expiresIn: 3600,
        );

        // Act
        final str = tokens.toString();

        // Assert
        expect(str, contains('this_is_a_very_long_')); // First 20 chars
        expect(str, contains('...'));
        expect(str, contains('bearer'));
        expect(str, contains('3600'));
        // Full token should not be in string for security
        expect(
          str,
          isNot(contains('this_is_a_very_long_access_token_that_should_be_truncated')),
        );
      });
    });
  });
}
