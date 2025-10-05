import 'package:flutter_test/flutter_test.dart';
import 'package:altair/features/auth/data/models/login_request.dart';

void main() {
  group('LoginRequest', () {
    group('fromEmailPassword factory', () {
      test('should create request with email as username', () {
        // Arrange & Act
        final request = LoginRequest.fromEmailPassword(
          email: 'test@example.com',
          password: 'securepassword123',
        );

        // Assert
        expect(request.username, 'test@example.com');
        expect(request.password, 'securepassword123');
      });

      test('should handle email with special characters', () {
        // Arrange & Act
        final request = LoginRequest.fromEmailPassword(
          email: 'user.name+tag@example.co.uk',
          password: 'password',
        );

        // Assert
        expect(request.username, 'user.name+tag@example.co.uk');
      });
    });

    group('fromJson', () {
      test('should correctly deserialize from JSON', () {
        // Arrange
        final json = {
          'username': 'test@example.com',
          'password': 'securepassword123',
        };

        // Act
        final request = LoginRequest.fromJson(json);

        // Assert
        expect(request.username, 'test@example.com');
        expect(request.password, 'securepassword123');
      });

      test('should throw when required fields are missing', () {
        // Arrange
        final invalidJson = {
          'username': 'test@example.com',
          // missing password
        };

        // Act & Assert
        expect(
          () => LoginRequest.fromJson(invalidJson),
          throwsA(isA<TypeError>()),
        );
      });
    });

    group('toJson', () {
      test('should correctly serialize to JSON', () {
        // Arrange
        const request = LoginRequest(
          username: 'test@example.com',
          password: 'securepassword123',
        );

        // Act
        final json = request.toJson();

        // Assert
        expect(json['username'], 'test@example.com');
        expect(json['password'], 'securepassword123');
      });

      test('should round-trip correctly (fromJson -> toJson)', () {
        // Arrange
        final originalJson = {
          'username': 'test@example.com',
          'password': 'securepassword123',
        };

        // Act
        final request = LoginRequest.fromJson(originalJson);
        final resultJson = request.toJson();

        // Assert
        expect(resultJson, equals(originalJson));
      });

      test('should match OAuth2 password flow format', () {
        // Arrange
        final request = LoginRequest.fromEmailPassword(
          email: 'test@example.com',
          password: 'password',
        );

        // Act
        final json = request.toJson();

        // Assert
        // OAuth2 expects 'username' and 'password' fields
        expect(json.containsKey('username'), isTrue);
        expect(json.containsKey('password'), isTrue);
        expect(json.containsKey('email'), isFalse);
      });
    });

    group('equality', () {
      test('should be equal when all fields match', () {
        // Arrange
        const request1 = LoginRequest(
          username: 'test@example.com',
          password: 'securepassword123',
        );

        const request2 = LoginRequest(
          username: 'test@example.com',
          password: 'securepassword123',
        );

        // Assert
        expect(request1, equals(request2));
        expect(request1.hashCode, equals(request2.hashCode));
      });

      test('should not be equal when username differs', () {
        // Arrange
        const request1 = LoginRequest(
          username: 'test@example.com',
          password: 'securepassword123',
        );

        const request2 = LoginRequest(
          username: 'different@example.com',
          password: 'securepassword123',
        );

        // Assert
        expect(request1, isNot(equals(request2)));
      });

      test('should not be equal when password differs', () {
        // Arrange
        const request1 = LoginRequest(
          username: 'test@example.com',
          password: 'securepassword123',
        );

        const request2 = LoginRequest(
          username: 'test@example.com',
          password: 'differentpassword',
        );

        // Assert
        expect(request1, isNot(equals(request2)));
      });
    });

    group('toString', () {
      test('should not include password in string representation', () {
        // Arrange
        const request = LoginRequest(
          username: 'test@example.com',
          password: 'securepassword123',
        );

        // Act
        final str = request.toString();

        // Assert
        expect(str, contains('test@example.com'));
        // Password should not be in string for security
        expect(str, isNot(contains('securepassword123')));
        expect(str, isNot(contains('password')));
      });
    });
  });
}
