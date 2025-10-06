import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

import '../models/auth_tokens.dart';

part 'token_repository.g.dart';

/// Provides a singleton instance of [TokenRepository]
@riverpod
TokenRepository tokenRepository(Ref ref) {
  return SecureTokenRepository(const FlutterSecureStorage());
}

/// Abstract interface for token storage operations.
///
/// Defines the contract for saving, retrieving, and clearing JWT tokens.
/// Implementations should handle both persistence and caching.
abstract class TokenRepository {
  /// Save both access and refresh tokens securely
  Future<void> saveTokens(AuthTokens tokens);

  /// Retrieve the current access token (from cache or storage)
  Future<String?> getAccessToken();

  /// Retrieve the current refresh token (from cache or storage)
  Future<String?> getRefreshToken();

  /// Get the full token object if available
  Future<AuthTokens?> getTokens();

  /// Clear all stored tokens (logout)
  Future<void> clearTokens();

  /// Check if user has valid tokens (basic check, doesn't verify expiration)
  Future<bool> hasTokens();
}

/// Token repository using flutter_secure_storage with in-memory caching.
///
/// Uses platform keychain (iOS Keychain, Android Keystore) for secure
/// persistence, with Dart variables for fast in-memory access.
///
/// This is the recommended approach for MVP:
/// - Secure: Uses hardware-backed encryption
/// - Simple: No extra dependencies (Hive/Drift not needed for tokens)
/// - Fast: In-memory cache avoids repeated secure storage reads
class SecureTokenRepository implements TokenRepository {
  final FlutterSecureStorage _storage;

  // In-memory cache - simple Dart variables, no Hive needed
  AuthTokens? _cachedTokens;

  // Storage keys
  static const _accessTokenKey = 'access_token';
  static const _refreshTokenKey = 'refresh_token';
  static const _tokenTypeKey = 'token_type';
  static const _expiresInKey = 'expires_in';
  static const _issuedAtKey = 'issued_at';

  SecureTokenRepository(this._storage);

  @override
  Future<void> saveTokens(AuthTokens tokens) async {
    // Update in-memory cache immediately
    _cachedTokens = tokens;

    // Persist to secure storage (atomic write)
    await Future.wait([
      _storage.write(key: _accessTokenKey, value: tokens.accessToken),
      _storage.write(key: _refreshTokenKey, value: tokens.refreshToken),
      _storage.write(key: _tokenTypeKey, value: tokens.tokenType),
      _storage.write(key: _expiresInKey, value: tokens.expiresIn.toString()),
      _storage.write(
        key: _issuedAtKey,
        value: tokens.issuedAt.toIso8601String(),
      ),
    ]);
  }

  @override
  Future<String?> getAccessToken() async {
    // Return from cache if available
    if (_cachedTokens != null) {
      return _cachedTokens!.accessToken;
    }

    // Load from storage and cache
    final tokens = await getTokens();
    return tokens?.accessToken;
  }

  @override
  Future<String?> getRefreshToken() async {
    // Return from cache if available
    if (_cachedTokens != null) {
      return _cachedTokens!.refreshToken;
    }

    // Load from storage and cache
    final tokens = await getTokens();
    return tokens?.refreshToken;
  }

  @override
  Future<AuthTokens?> getTokens() async {
    // Return cache if available
    if (_cachedTokens != null) {
      return _cachedTokens;
    }

    // Load all token components from secure storage
    final results = await Future.wait([
      _storage.read(key: _accessTokenKey),
      _storage.read(key: _refreshTokenKey),
      _storage.read(key: _tokenTypeKey),
      _storage.read(key: _expiresInKey),
      _storage.read(key: _issuedAtKey),
    ]);

    final accessToken = results[0];
    final refreshToken = results[1];
    final tokenType = results[2];
    final expiresInStr = results[3];
    final issuedAtStr = results[4];

    // If any required field is missing, return null
    if (accessToken == null ||
        refreshToken == null ||
        tokenType == null ||
        expiresInStr == null ||
        issuedAtStr == null) {
      return null;
    }

    // Parse and cache the tokens
    _cachedTokens = AuthTokens(
      accessToken: accessToken,
      refreshToken: refreshToken,
      tokenType: tokenType,
      expiresIn: int.parse(expiresInStr),
      issuedAt: DateTime.parse(issuedAtStr),
    );

    return _cachedTokens;
  }

  @override
  Future<void> clearTokens() async {
    // Clear in-memory cache
    _cachedTokens = null;

    // Clear all token data from secure storage
    await _storage.deleteAll();
  }

  @override
  Future<bool> hasTokens() async {
    final accessToken = await getAccessToken();
    return accessToken != null;
  }
}
