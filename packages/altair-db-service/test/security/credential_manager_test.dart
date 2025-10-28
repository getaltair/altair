import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:altair_db_service/src/security/credential_manager.dart';
import 'package:path/path.dart' as path;

void main() {
  late Directory tempDir;
  late CredentialManager credentialManager;

  setUp(() async {
    // Create temporary directory for tests
    tempDir = await Directory.systemTemp.createTemp('credential_test_');
    credentialManager = CredentialManager(tempDir.path);
  });

  tearDown(() async {
    // Clean up
    if (await tempDir.exists()) {
      await tempDir.delete(recursive: true);
    }
  });

  group('Password Generation Security', () {
    test('generates password with minimum length of 32 characters', () {
      final password = CredentialManager.generateSecurePassword();
      expect(password.length, equals(32));
    });

    test('generates password with custom length', () {
      final password = CredentialManager.generateSecurePassword(length: 64);
      expect(password.length, equals(64));
    });

    test('throws error for password length less than 16', () {
      expect(
        () => CredentialManager.generateSecurePassword(length: 15),
        throwsA(isA<ArgumentError>()),
      );
    });

    test('generated passwords are unique (no collisions)', () {
      final passwords = <String>{};
      for (var i = 0; i < 100; i++) {
        passwords.add(CredentialManager.generateSecurePassword());
      }
      // All 100 passwords should be unique
      expect(passwords.length, equals(100));
    });

    test('password contains uppercase letters', () {
      final password = CredentialManager.generateSecurePassword();
      expect(password, matches(RegExp(r'[A-Z]')));
    });

    test('password contains lowercase letters', () {
      final password = CredentialManager.generateSecurePassword();
      expect(password, matches(RegExp(r'[a-z]')));
    });

    test('password contains numbers', () {
      final password = CredentialManager.generateSecurePassword();
      expect(password, matches(RegExp(r'[0-9]')));
    });

    test('password contains special characters', () {
      final password = CredentialManager.generateSecurePassword();
      expect(password, matches(RegExp(r'[!@#\$%^&*()\-_=+\[\]{}|;:,.<>?]')));
    });

    test('password has good entropy (character distribution)', () {
      final password = CredentialManager.generateSecurePassword(length: 100);
      final chars = password.split('');

      // Count character types
      var uppercase = 0;
      var lowercase = 0;
      var digits = 0;
      var special = 0;

      for (final char in chars) {
        if (RegExp(r'[A-Z]').hasMatch(char))
          uppercase++;
        else if (RegExp(r'[a-z]').hasMatch(char))
          lowercase++;
        else if (RegExp(r'[0-9]').hasMatch(char))
          digits++;
        else
          special++;
      }

      // Each category should have reasonable representation (at least 5%)
      expect(uppercase, greaterThan(5));
      expect(lowercase, greaterThan(5));
      expect(digits, greaterThan(5));
      expect(special, greaterThan(5));
    });

    test('password does not start with predictable pattern', () {
      // Generate 10 passwords and ensure they don't all start with same char type
      final firstChars = <String>[];
      for (var i = 0; i < 10; i++) {
        final password = CredentialManager.generateSecurePassword();
        firstChars.add(password[0]);
      }

      // Should have variety in first characters
      final uniqueFirstChars = firstChars.toSet();
      expect(uniqueFirstChars.length, greaterThan(1));
    });
  });

  group('Credential Storage', () {
    test('stores and retrieves credentials', () async {
      await credentialManager.storeCredentials(
        username: 'testuser',
        password: 'testpass123',
      );

      final credentials = await credentialManager.getCredentials();
      expect(credentials, isNotNull);
      expect(credentials!.username, equals('testuser'));
      expect(credentials.password, equals('testpass123'));
    });

    test('returns null when no credentials exist', () async {
      final credentials = await credentialManager.getCredentials();
      expect(credentials, isNull);
    });

    test('generates and stores credentials', () async {
      final credentials = await credentialManager.generateAndStoreCredentials(
        username: 'altair',
      );

      expect(credentials.username, equals('altair'));
      expect(credentials.password.length, equals(32));

      // Verify they were stored
      final retrieved = await credentialManager.getCredentials();
      expect(retrieved, isNotNull);
      expect(retrieved!.username, equals('altair'));
      expect(retrieved.password, equals(credentials.password));
    });

    test('deletes credentials', () async {
      await credentialManager.storeCredentials(
        username: 'testuser',
        password: 'testpass123',
      );

      await credentialManager.deleteCredentials();

      final credentials = await credentialManager.getCredentials();
      expect(credentials, isNull);
    });
  });

  group('File Security', () {
    test('credential file has secure permissions (600) on Unix', () async {
      if (!Platform.isLinux && !Platform.isMacOS) {
        return; // Skip on non-Unix platforms
      }

      await credentialManager.storeCredentials(
        username: 'testuser',
        password: 'testpass123',
      );

      final credentialFilePath = credentialManager.getCredentialFilePath();
      final result = await Process.run('stat', [
        '-c',
        '%a',
        credentialFilePath,
      ]);
      final permissions = result.stdout.toString().trim();

      expect(permissions, equals('600'));
    });

    test('throws SecurityException if file permissions are insecure', () async {
      if (!Platform.isLinux && !Platform.isMacOS) {
        return; // Skip on non-Unix platforms
      }

      // Create credential file with insecure permissions
      final credentialFilePath = credentialManager.getCredentialFilePath();
      final file = File(credentialFilePath);
      await file.writeAsString('{"username":"test","password":"test"}');
      await Process.run('chmod', ['644', credentialFilePath]);

      expect(
        () async => await credentialManager.getCredentials(),
        throwsA(isA<SecurityException>()),
      );
    });

    test('credential file path is within config directory', () {
      final credentialPath = credentialManager.getCredentialFilePath();
      expect(credentialPath, startsWith(tempDir.path));
      expect(path.basename(credentialPath), equals('.altair_credentials'));
    });
  });

  group('Credentials Model', () {
    test('toString masks password', () {
      const credentials = Credentials(
        username: 'testuser',
        password: 'secretpassword',
      );

      final stringRepresentation = credentials.toString();
      expect(stringRepresentation, contains('testuser'));
      expect(stringRepresentation, isNot(contains('secretpassword')));
      expect(stringRepresentation, contains('***'));
    });
  });

  group('Edge Cases', () {
    test('handles special characters in username', () async {
      await credentialManager.storeCredentials(
        username: 'test@user.com',
        password: 'testpass123',
      );

      final credentials = await credentialManager.getCredentials();
      expect(credentials, isNotNull);
      expect(credentials!.username, equals('test@user.com'));
    });

    test('handles very long passwords', () async {
      final longPassword = CredentialManager.generateSecurePassword(
        length: 128,
      );
      await credentialManager.storeCredentials(
        username: 'testuser',
        password: longPassword,
      );

      final credentials = await credentialManager.getCredentials();
      expect(credentials, isNotNull);
      expect(credentials!.password, equals(longPassword));
    });

    test('overwrites existing credentials', () async {
      await credentialManager.storeCredentials(
        username: 'user1',
        password: 'pass1',
      );

      await credentialManager.storeCredentials(
        username: 'user2',
        password: 'pass2',
      );

      final credentials = await credentialManager.getCredentials();
      expect(credentials!.username, equals('user2'));
      expect(credentials.password, equals('pass2'));
    });
  });
}
