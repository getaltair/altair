/// Secure storage service for sensitive data.
library;

import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:logger/logger.dart';

/// Service for securely storing sensitive data like tokens.
class SecureStorageService {
  /// Creates a secure storage service.
  SecureStorageService({
    FlutterSecureStorage? storage,
    Logger? logger,
  })  : _storage = storage ?? const FlutterSecureStorage(),
        _logger = logger ?? Logger();

  final FlutterSecureStorage _storage;
  final Logger _logger;

  // Storage keys
  static const String _accessTokenKey = 'access_token';
  static const String _refreshTokenKey = 'refresh_token';
  static const String _tokenTypeKey = 'token_type';
  static const String _expiresInKey = 'expires_in';

  /// Saves an access token.
  Future<void> saveAccessToken(String token) async {
    try {
      await _storage.write(key: _accessTokenKey, value: token);
      _logger.d('Access token saved');
    } catch (e, stackTrace) {
      _logger.e('Failed to save access token',
          error: e, stackTrace: stackTrace);
      rethrow;
    }
  }

  /// Retrieves the access token.
  Future<String?> getAccessToken() async {
    try {
      return await _storage.read(key: _accessTokenKey);
    } catch (e, stackTrace) {
      _logger.e('Failed to read access token',
          error: e, stackTrace: stackTrace);
      return null;
    }
  }

  /// Saves a refresh token.
  Future<void> saveRefreshToken(String token) async {
    try {
      await _storage.write(key: _refreshTokenKey, value: token);
      _logger.d('Refresh token saved');
    } catch (e, stackTrace) {
      _logger.e('Failed to save refresh token',
          error: e, stackTrace: stackTrace);
      rethrow;
    }
  }

  /// Retrieves the refresh token.
  Future<String?> getRefreshToken() async {
    try {
      return await _storage.read(key: _refreshTokenKey);
    } catch (e, stackTrace) {
      _logger.e('Failed to read refresh token',
          error: e, stackTrace: stackTrace);
      return null;
    }
  }

  /// Saves token type.
  Future<void> saveTokenType(String tokenType) async {
    try {
      await _storage.write(key: _tokenTypeKey, value: tokenType);
    } catch (e, stackTrace) {
      _logger.e('Failed to save token type', error: e, stackTrace: stackTrace);
      rethrow;
    }
  }

  /// Retrieves the token type.
  Future<String?> getTokenType() async {
    try {
      return await _storage.read(key: _tokenTypeKey);
    } catch (e, stackTrace) {
      _logger.e('Failed to read token type', error: e, stackTrace: stackTrace);
      return null;
    }
  }

  /// Saves token expiration time.
  Future<void> saveExpiresIn(int expiresIn) async {
    try {
      await _storage.write(key: _expiresInKey, value: expiresIn.toString());
    } catch (e, stackTrace) {
      _logger.e('Failed to save expires_in', error: e, stackTrace: stackTrace);
      rethrow;
    }
  }

  /// Retrieves the token expiration time.
  Future<int?> getExpiresIn() async {
    try {
      final value = await _storage.read(key: _expiresInKey);
      return value != null ? int.tryParse(value) : null;
    } catch (e, stackTrace) {
      _logger.e('Failed to read expires_in', error: e, stackTrace: stackTrace);
      return null;
    }
  }

  /// Clears all stored tokens.
  Future<void> clearTokens() async {
    try {
      await Future.wait([
        _storage.delete(key: _accessTokenKey),
        _storage.delete(key: _refreshTokenKey),
        _storage.delete(key: _tokenTypeKey),
        _storage.delete(key: _expiresInKey),
      ]);
      _logger.d('All tokens cleared');
    } catch (e, stackTrace) {
      _logger.e('Failed to clear tokens', error: e, stackTrace: stackTrace);
      rethrow;
    }
  }

  /// Checks if an access token exists.
  Future<bool> hasAccessToken() async {
    final token = await getAccessToken();
    return token != null && token.isNotEmpty;
  }
}
