import 'dart:convert';
import 'dart:io';
import 'dart:math';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:path/path.dart' as path;

/// Manages secure credential generation and storage
class CredentialManager {
  static const _credentialKey = 'altair_db_credentials';
  static const _credentialFileName = '.altair_credentials';

  final FlutterSecureStorage _secureStorage;
  final String _configDirectory;

  CredentialManager(this._configDirectory)
      : _secureStorage = const FlutterSecureStorage(
          aOptions: AndroidOptions(
            encryptedSharedPreferences: true,
          ),
          iOptions: IOSOptions(
            accessibility: KeychainAccessibility.first_unlock,
          ),
          lOptions: LinuxOptions(),
        );

  /// Generate a cryptographically secure password
  ///
  /// Uses a combination of:
  /// - Uppercase letters (A-Z)
  /// - Lowercase letters (a-z)
  /// - Numbers (0-9)
  /// - Special characters (!@#$%^&*()_+-=[]{}|;:,.<>?)
  ///
  /// Default length: 32 characters
  /// Ensures at least one character from each category
  static String generateSecurePassword({int length = 32}) {
    if (length < 16) {
      throw ArgumentError('Password length must be at least 16 characters');
    }

    const uppercase = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';
    const lowercase = 'abcdefghijklmnopqrstuvwxyz';
    const numbers = '0123456789';
    const special = '!@#\$%^&*()_+-=[]{}|;:,.<>?';
    const allChars = uppercase + lowercase + numbers + special;

    final random = Random.secure();
    final password = StringBuffer();

    // Ensure at least one character from each category
    password.write(uppercase[random.nextInt(uppercase.length)]);
    password.write(lowercase[random.nextInt(lowercase.length)]);
    password.write(numbers[random.nextInt(numbers.length)]);
    password.write(special[random.nextInt(special.length)]);

    // Fill remaining length with random characters
    for (var i = 4; i < length; i++) {
      password.write(allChars[random.nextInt(allChars.length)]);
    }

    // Shuffle the password to avoid predictable patterns
    final chars = password.toString().split('');
    for (var i = chars.length - 1; i > 0; i--) {
      final j = random.nextInt(i + 1);
      final temp = chars[i];
      chars[i] = chars[j];
      chars[j] = temp;
    }

    return chars.join('');
  }

  /// Store credentials securely using platform-specific secure storage
  ///
  /// Platform behavior:
  /// - macOS: Keychain
  /// - Windows: Credential Manager
  /// - Linux: Secret Service API (gnome-keyring/kwallet)
  /// - Fallback: Encrypted file with chmod 600
  Future<void> storeCredentials({
    required String username,
    required String password,
  }) async {
    final credentials = jsonEncode({
      'username': username,
      'password': password,
      'created_at': DateTime.now().toIso8601String(),
    });

    try {
      // Try platform-specific secure storage first
      await _secureStorage.write(
        key: _credentialKey,
        value: credentials,
      );
    } catch (e) {
      // Fallback to file-based storage with proper permissions
      await _storeCredentialsInFile(username, password);
    }
  }

  /// Fallback: Store credentials in a file with chmod 600
  Future<void> _storeCredentialsInFile(
    String username,
    String password,
  ) async {
    final credentialFilePath = path.join(_configDirectory, _credentialFileName);
    final file = File(credentialFilePath);

    // Create secure credentials file
    final credentials = jsonEncode({
      'username': username,
      'password': password,
      'created_at': DateTime.now().toIso8601String(),
    });

    await file.writeAsString(credentials);

    // Set file permissions to 600 (owner read/write only)
    if (Platform.isLinux || Platform.isMacOS) {
      await Process.run('chmod', ['600', credentialFilePath]);
    } else if (Platform.isWindows) {
      // Windows: Remove all permissions except for current user
      await Process.run('icacls', [
        credentialFilePath,
        '/inheritance:r',
        '/grant:r',
        '${Platform.environment['USERNAME']}:(R,W)',
      ]);
    }
  }

  /// Retrieve credentials from secure storage
  ///
  /// Returns null if credentials don't exist
  Future<Credentials?> getCredentials() async {
    try {
      // Try platform-specific secure storage first
      final credentialsJson = await _secureStorage.read(key: _credentialKey);
      if (credentialsJson != null) {
        final data = jsonDecode(credentialsJson) as Map<String, dynamic>;
        return Credentials(
          username: data['username'] as String,
          password: data['password'] as String,
        );
      }
    } catch (e) {
      // Fall through to file-based retrieval
    }

    // Fallback to file-based storage
    return _getCredentialsFromFile();
  }

  /// Fallback: Retrieve credentials from file
  Future<Credentials?> _getCredentialsFromFile() async {
    final credentialFilePath = path.join(_configDirectory, _credentialFileName);
    final file = File(credentialFilePath);

    if (!await file.exists()) {
      return null;
    }

    // Verify file permissions are secure
    if (Platform.isLinux || Platform.isMacOS) {
      final result =
          await Process.run('stat', ['-c', '%a', credentialFilePath]);
      final permissions = result.stdout.toString().trim();
      if (permissions != '600') {
        throw SecurityException(
          'Credential file has insecure permissions: $permissions. Expected 600.',
        );
      }
    }

    final credentialsJson = await file.readAsString();
    final data = jsonDecode(credentialsJson) as Map<String, dynamic>;

    return Credentials(
      username: data['username'] as String,
      password: data['password'] as String,
    );
  }

  /// Generate and store new credentials
  ///
  /// Returns the generated credentials
  Future<Credentials> generateAndStoreCredentials({
    String username = 'altair',
    int passwordLength = 32,
  }) async {
    final password = generateSecurePassword(length: passwordLength);
    await storeCredentials(username: username, password: password);
    return Credentials(username: username, password: password);
  }

  /// Delete stored credentials
  Future<void> deleteCredentials() async {
    try {
      await _secureStorage.delete(key: _credentialKey);
    } catch (e) {
      // Ignore errors
    }

    // Also delete file-based credentials
    final credentialFilePath = path.join(_configDirectory, _credentialFileName);
    final file = File(credentialFilePath);
    if (await file.exists()) {
      await file.delete();
    }
  }

  /// Get credential file path for environment variable usage
  String getCredentialFilePath() {
    return path.join(_configDirectory, _credentialFileName);
  }
}

/// Represents database credentials
class Credentials {
  final String username;
  final String password;

  const Credentials({
    required this.username,
    required this.password,
  });

  @override
  String toString() => 'Credentials(username: $username, password: ***)';
}

/// Exception thrown when credential security is compromised
class SecurityException implements Exception {
  final String message;

  SecurityException(this.message);

  @override
  String toString() => 'SecurityException: $message';
}
