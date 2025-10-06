import 'package:dio/dio.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

import '../../../../core/api/api_client.dart';
import '../models/auth_tokens.dart';
import '../models/login_request.dart';
import '../models/register_request.dart';
import '../models/user.dart';
import '../repositories/token_repository.dart';

part 'auth_service.g.dart';

/// Provides the auth service instance
@riverpod
AuthService authService(Ref ref) {
  return AuthService(
    apiClient: ref.watch(apiClientProvider),
    tokenRepository: ref.watch(tokenRepositoryProvider),
  );
}

/// Authentication service for login, register, and token management.
///
/// Handles communication with the FastAPI auth endpoints and manages
/// token storage via the token repository.
class AuthService {
  final ApiClient _apiClient;
  final TokenRepository _tokenRepository;

  AuthService({
    required ApiClient apiClient,
    required TokenRepository tokenRepository,
  })  : _apiClient = apiClient,
        _tokenRepository = tokenRepository;

  /// Register a new user account
  ///
  /// Returns the created user on success.
  /// Throws [DioException] on failure (400 if email exists, etc.)
  Future<User> register(RegisterRequest request) async {
    final response = await _apiClient.rawDio.post<Map<String, dynamic>>(
      '/auth/register',
      data: request.toJson(),
    );

    if (response.data == null) {
      throw Exception('Empty response from register endpoint');
    }

    return User.fromJson(response.data!);
  }

  /// Login with email and password
  ///
  /// Returns the auth tokens on success and saves them to secure storage.
  /// Throws [DioException] on failure (401 if credentials invalid, etc.)
  Future<AuthTokens> login({
    required String email,
    required String password,
  }) async {
    final request = LoginRequest.fromEmailPassword(
      email: email,
      password: password,
    );

    // FastAPI OAuth2PasswordRequestForm expects form-encoded data
    final response = await _apiClient.rawDio.post<Map<String, dynamic>>(
      '/auth/login',
      data: request.toJson(),
      options: Options(
        contentType: Headers.formUrlEncodedContentType,
      ),
    );

    if (response.data == null) {
      throw Exception('Empty response from login endpoint');
    }

    final tokens = AuthTokens.fromJson(response.data!);

    // Save tokens to secure storage
    await _tokenRepository.saveTokens(tokens);

    return tokens;
  }

  /// Get current authenticated user info
  ///
  /// Requires valid access token in storage.
  /// Throws [DioException] if not authenticated (401)
  Future<User> getCurrentUser() async {
    final response = await _apiClient.get<Map<String, dynamic>>('/auth/me');

    if (response.data == null) {
      throw Exception('Empty response from /auth/me endpoint');
    }

    return User.fromJson(response.data!);
  }

  /// Refresh the access token using the refresh token
  ///
  /// Returns new token pair and saves them to storage.
  /// Throws [DioException] if refresh token is invalid (401)
  Future<AuthTokens> refreshTokens() async {
    final refreshToken = await _tokenRepository.getRefreshToken();

    if (refreshToken == null) {
      throw Exception('No refresh token available');
    }

    final response = await _apiClient.rawDio.post<Map<String, dynamic>>(
      '/auth/refresh',
      data: {'refresh_token': refreshToken},
    );

    if (response.data == null) {
      throw Exception('Empty response from refresh endpoint');
    }

    final tokens = AuthTokens.fromJson(response.data!);

    // Save new tokens to secure storage
    await _tokenRepository.saveTokens(tokens);

    return tokens;
  }

  /// Logout the current user
  ///
  /// Calls the backend logout endpoint to blacklist the token,
  /// then clears local token storage.
  Future<void> logout() async {
    try {
      // Try to blacklist the token on the server
      await _apiClient.post('/auth/logout');
    } catch (e) {
      // If logout fails (network error, etc.), still clear local tokens
      // This ensures user can always logout locally
    } finally {
      // Always clear local tokens
      await _tokenRepository.clearTokens();
    }
  }

  /// Check if user is currently logged in
  ///
  /// Returns true if access token exists in storage.
  /// Note: This doesn't verify if the token is still valid.
  Future<bool> isLoggedIn() async {
    return await _tokenRepository.hasTokens();
  }
}
