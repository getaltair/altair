import 'package:flutter_test/flutter_test.dart';
import 'package:altair/features/auth/data/models/user.dart';

void main() {
  group('User', () {
    final testDate = DateTime.parse('2024-01-15T10:30:00Z');

    group('fromJson', () {
      test('should correctly deserialize from JSON with all fields', () {
        // Arrange
        final json = {
          'id': 'user-123',
          'email': 'test@example.com',
          'username': 'testuser',
          'created_at': '2024-01-15T10:30:00Z',
        };

        // Act
        final user = User.fromJson(json);

        // Assert
        expect(user.id, 'user-123');
        expect(user.email, 'test@example.com');
        expect(user.username, 'testuser');
        expect(user.createdAt, testDate);
      });

      test('should correctly deserialize when username is null', () {
        // Arrange
        final json = {
          'id': 'user-123',
          'email': 'test@example.com',
          'username': null,
          'created_at': '2024-01-15T10:30:00Z',
        };

        // Act
        final user = User.fromJson(json);

        // Assert
        expect(user.id, 'user-123');
        expect(user.email, 'test@example.com');
        expect(user.username, isNull);
        expect(user.createdAt, testDate);
      });

      test('should correctly deserialize when username is missing', () {
        // Arrange
        final json = {
          'id': 'user-123',
          'email': 'test@example.com',
          'created_at': '2024-01-15T10:30:00Z',
        };

        // Act
        final user = User.fromJson(json);

        // Assert
        expect(user.username, isNull);
      });

      test('should throw when required fields are missing', () {
        // Arrange
        final invalidJson = {
          'id': 'user-123',
          // missing email and created_at
        };

        // Act & Assert
        expect(
          () => User.fromJson(invalidJson),
          throwsA(isA<TypeError>()),
        );
      });

      test('should throw when created_at has invalid format', () {
        // Arrange
        final invalidJson = {
          'id': 'user-123',
          'email': 'test@example.com',
          'created_at': 'not-a-date',
        };

        // Act & Assert
        expect(
          () => User.fromJson(invalidJson),
          throwsA(isA<FormatException>()),
        );
      });
    });

    group('toJson', () {
      test('should correctly serialize to JSON with all fields', () {
        // Arrange
        final user = User(
          id: 'user-123',
          email: 'test@example.com',
          username: 'testuser',
          createdAt: testDate,
        );

        // Act
        final json = user.toJson();

        // Assert
        expect(json['id'], 'user-123');
        expect(json['email'], 'test@example.com');
        expect(json['username'], 'testuser');
        expect(json['created_at'], '2024-01-15T10:30:00.000Z');
      });

      test('should omit username when null', () {
        // Arrange
        final user = User(
          id: 'user-123',
          email: 'test@example.com',
          username: null,
          createdAt: testDate,
        );

        // Act
        final json = user.toJson();

        // Assert
        expect(json.containsKey('username'), isFalse);
        expect(json['id'], 'user-123');
        expect(json['email'], 'test@example.com');
        expect(json['created_at'], '2024-01-15T10:30:00.000Z');
      });

      test('should round-trip correctly (fromJson -> toJson)', () {
        // Arrange
        final originalJson = {
          'id': 'user-123',
          'email': 'test@example.com',
          'username': 'testuser',
          'created_at': '2024-01-15T10:30:00.000Z',
        };

        // Act
        final user = User.fromJson(originalJson);
        final resultJson = user.toJson();

        // Assert
        expect(resultJson, equals(originalJson));
      });
    });

    group('equality', () {
      test('should be equal when all fields match', () {
        // Arrange
        final user1 = User(
          id: 'user-123',
          email: 'test@example.com',
          username: 'testuser',
          createdAt: testDate,
        );

        final user2 = User(
          id: 'user-123',
          email: 'test@example.com',
          username: 'testuser',
          createdAt: testDate,
        );

        // Assert
        expect(user1, equals(user2));
        expect(user1.hashCode, equals(user2.hashCode));
      });

      test('should not be equal when id differs', () {
        // Arrange
        final user1 = User(
          id: 'user-123',
          email: 'test@example.com',
          username: 'testuser',
          createdAt: testDate,
        );

        final user2 = User(
          id: 'user-456',
          email: 'test@example.com',
          username: 'testuser',
          createdAt: testDate,
        );

        // Assert
        expect(user1, isNot(equals(user2)));
      });

      test('should not be equal when username differs', () {
        // Arrange
        final user1 = User(
          id: 'user-123',
          email: 'test@example.com',
          username: 'testuser',
          createdAt: testDate,
        );

        final user2 = User(
          id: 'user-123',
          email: 'test@example.com',
          username: 'different',
          createdAt: testDate,
        );

        // Assert
        expect(user1, isNot(equals(user2)));
      });

      test('should be equal when both usernames are null', () {
        // Arrange
        final user1 = User(
          id: 'user-123',
          email: 'test@example.com',
          username: null,
          createdAt: testDate,
        );

        final user2 = User(
          id: 'user-123',
          email: 'test@example.com',
          username: null,
          createdAt: testDate,
        );

        // Assert
        expect(user1, equals(user2));
      });
    });

    group('toString', () {
      test('should include all fields in string representation', () {
        // Arrange
        final user = User(
          id: 'user-123',
          email: 'test@example.com',
          username: 'testuser',
          createdAt: testDate,
        );

        // Act
        final str = user.toString();

        // Assert
        expect(str, contains('user-123'));
        expect(str, contains('test@example.com'));
        expect(str, contains('testuser'));
        expect(str, contains(testDate.toString()));
      });

      test('should handle null username in string representation', () {
        // Arrange
        final user = User(
          id: 'user-123',
          email: 'test@example.com',
          username: null,
          createdAt: testDate,
        );

        // Act
        final str = user.toString();

        // Assert
        expect(str, contains('user-123'));
        expect(str, contains('test@example.com'));
        expect(str, contains('null'));
      });
    });
  });
}
