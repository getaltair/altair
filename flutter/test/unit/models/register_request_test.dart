import 'package:flutter_test/flutter_test.dart';
import 'package:altair/features/auth/data/models/register_request.dart';

void main() {
  group('RegisterRequest', () {
    group('fromJson', () {
      test('should correctly deserialize from JSON with all fields', () {
        // Arrange
        final json = {
          'email': 'test@example.com',
          'password': 'securepassword123',
          'username': 'testuser',
        };

        // Act
        final request = RegisterRequest.fromJson(json);

        // Assert
        expect(request.email, 'test@example.com');
        expect(request.password, 'securepassword123');
        expect(request.username, 'testuser');
      });

      test('should correctly deserialize when username is null', () {
        // Arrange
        final json = {
          'email': 'test@example.com',
          'password': 'securepassword123',
          'username': null,
        };

        // Act
        final request = RegisterRequest.fromJson(json);

        // Assert
        expect(request.email, 'test@example.com');
        expect(request.password, 'securepassword123');
        expect(request.username, isNull);
      });

      test('should correctly deserialize when username is missing', () {
        // Arrange
        final json = {
          'email': 'test@example.com',
          'password': 'securepassword123',
        };

        // Act
        final request = RegisterRequest.fromJson(json);

        // Assert
        expect(request.username, isNull);
      });

      test('should throw when required fields are missing', () {
        // Arrange
        final invalidJson = {
          'email': 'test@example.com',
          // missing password
        };

        // Act & Assert
        expect(
          () => RegisterRequest.fromJson(invalidJson),
          throwsA(isA<TypeError>()),
        );
      });
    });

    group('toJson', () {
      test('should correctly serialize to JSON with all fields', () {
        // Arrange
        const request = RegisterRequest(
          email: 'test@example.com',
          password: 'securepassword123',
          username: 'testuser',
        );

        // Act
        final json = request.toJson();

        // Assert
        expect(json['email'], 'test@example.com');
        expect(json['password'], 'securepassword123');
        expect(json['username'], 'testuser');
      });

      test('should omit username when null', () {
        // Arrange
        const request = RegisterRequest(
          email: 'test@example.com',
          password: 'securepassword123',
          username: null,
        );

        // Act
        final json = request.toJson();

        // Assert
        expect(json.containsKey('username'), isFalse);
        expect(json['email'], 'test@example.com');
        expect(json['password'], 'securepassword123');
      });

      test('should round-trip correctly (fromJson -> toJson)', () {
        // Arrange
        final originalJson = {
          'email': 'test@example.com',
          'password': 'securepassword123',
          'username': 'testuser',
        };

        // Act
        final request = RegisterRequest.fromJson(originalJson);
        final resultJson = request.toJson();

        // Assert
        expect(resultJson, equals(originalJson));
      });
    });

    group('equality', () {
      test('should be equal when all fields match', () {
        // Arrange
        const request1 = RegisterRequest(
          email: 'test@example.com',
          password: 'securepassword123',
          username: 'testuser',
        );

        const request2 = RegisterRequest(
          email: 'test@example.com',
          password: 'securepassword123',
          username: 'testuser',
        );

        // Assert
        expect(request1, equals(request2));
        expect(request1.hashCode, equals(request2.hashCode));
      });

      test('should not be equal when email differs', () {
        // Arrange
        const request1 = RegisterRequest(
          email: 'test@example.com',
          password: 'securepassword123',
          username: 'testuser',
        );

        const request2 = RegisterRequest(
          email: 'different@example.com',
          password: 'securepassword123',
          username: 'testuser',
        );

        // Assert
        expect(request1, isNot(equals(request2)));
      });

      test('should not be equal when password differs', () {
        // Arrange
        const request1 = RegisterRequest(
          email: 'test@example.com',
          password: 'securepassword123',
          username: 'testuser',
        );

        const request2 = RegisterRequest(
          email: 'test@example.com',
          password: 'differentpassword',
          username: 'testuser',
        );

        // Assert
        expect(request1, isNot(equals(request2)));
      });

      test('should be equal when both usernames are null', () {
        // Arrange
        const request1 = RegisterRequest(
          email: 'test@example.com',
          password: 'securepassword123',
          username: null,
        );

        const request2 = RegisterRequest(
          email: 'test@example.com',
          password: 'securepassword123',
          username: null,
        );

        // Assert
        expect(request1, equals(request2));
      });
    });

    group('toString', () {
      test('should not include password in string representation', () {
        // Arrange
        const request = RegisterRequest(
          email: 'test@example.com',
          password: 'securepassword123',
          username: 'testuser',
        );

        // Act
        final str = request.toString();

        // Assert
        expect(str, contains('test@example.com'));
        expect(str, contains('testuser'));
        // Password should not be in string for security
        expect(str, isNot(contains('securepassword123')));
        expect(str, isNot(contains('password')));
      });

      test('should handle null username in string representation', () {
        // Arrange
        const request = RegisterRequest(
          email: 'test@example.com',
          password: 'securepassword123',
          username: null,
        );

        // Act
        final str = request.toString();

        // Assert
        expect(str, contains('test@example.com'));
        expect(str, contains('null'));
      });
    });

    group('validation scenarios', () {
      test('should create request with valid email format', () {
        // Arrange & Act
        const request = RegisterRequest(
          email: 'valid.email+tag@example.com',
          password: 'password',
        );

        // Assert
        expect(request.email, 'valid.email+tag@example.com');
      });

      test(
        'should create request with short password (validation on backend)',
        () {
          // Arrange & Act
          // Note: Password validation is handled on backend
          const request = RegisterRequest(
            email: 'test@example.com',
            password: '123',
          );

          // Assert
          expect(request.password, '123');
        },
      );

      test('should create request with special characters in username', () {
        // Arrange & Act
        const request = RegisterRequest(
          email: 'test@example.com',
          password: 'password',
          username: 'user_name-123',
        );

        // Assert
        expect(request.username, 'user_name-123');
      });
    });
  });
}
