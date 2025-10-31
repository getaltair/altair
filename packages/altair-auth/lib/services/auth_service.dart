/// Authentication service.
library;

import 'package:dio/dio.dart';
import 'package:logger/logger.dart';

import '../models/auth_token.dart';
import '../models/user.dart';
import 'secure_storage_service.dart';

/// Service for handling authentication operations.
class AuthService {
  /// Creates an authentication service.
  AuthService({
    required Dio dio,
    required SecureStorageService storage,
    required String baseUrl,
    Logger? logger,
  }) : _dio = dio,
       _storage = storage,
       _baseUrl = baseUrl,
       _logger = logger ?? Logger();

  final Dio _dio;
  final SecureStorageService _storage;
  final String _baseUrl;
  final Logger _logger;

  /// Authenticates a user with email and password.
  Future<AuthToken> login({
    required String email,
    required String password,
  }) async {
    try {
      _logger.d('Attempting login for email: $email');

      final response = await _dio.post<Map<String, dynamic>>(
        '$_baseUrl/auth/login',
        data: {'email': email, 'password': password},
      );

      if (response.data == null) {
        throw Exception('No data received from login endpoint');
      }

      final token = AuthToken.fromJson(response.data!);

      // Save tokens to secure storage
      await _storage.saveAccessToken(token.accessToken);
      await _storage.saveTokenType(token.tokenType);

      if (token.refreshToken != null) {
        await _storage.saveRefreshToken(token.refreshToken!);
      }

      if (token.expiresIn != null) {
        await _storage.saveExpiresIn(token.expiresIn!);
      }

      _logger.i('Login successful for email: $email');
      return token;
    } on DioException catch (e, stackTrace) {
      _logger.e('Login failed', error: e, stackTrace: stackTrace);
      if (e.response?.statusCode == 401) {
        throw Exception('Invalid email or password');
      }
      rethrow;
    } catch (e, stackTrace) {
      _logger.e(
        'Login failed with unexpected error',
        error: e,
        stackTrace: stackTrace,
      );
      rethrow;
    }
  }

  /// Registers a new user.
  Future<AuthToken> register({
    required String email,
    required String password,
  }) async {
    try {
      _logger.d('Attempting registration for email: $email');

      final response = await _dio.post<Map<String, dynamic>>(
        '$_baseUrl/auth/register',
        data: {'email': email, 'password': password},
      );

      if (response.data == null) {
        throw Exception('No data received from register endpoint');
      }

      final token = AuthToken.fromJson(response.data!);

      // Save tokens to secure storage
      await _storage.saveAccessToken(token.accessToken);
      await _storage.saveTokenType(token.tokenType);

      if (token.refreshToken != null) {
        await _storage.saveRefreshToken(token.refreshToken!);
      }

      if (token.expiresIn != null) {
        await _storage.saveExpiresIn(token.expiresIn!);
      }

      _logger.i('Registration successful for email: $email');
      return token;
    } on DioException catch (e, stackTrace) {
      _logger.e('Registration failed', error: e, stackTrace: stackTrace);
      if (e.response?.statusCode == 409) {
        throw Exception('Email already registered');
      }
      rethrow;
    } catch (e, stackTrace) {
      _logger.e(
        'Registration failed with unexpected error',
        error: e,
        stackTrace: stackTrace,
      );
      rethrow;
    }
  }

  /// Refreshes the access token using a refresh token.
  Future<AuthToken> refreshToken() async {
    try {
      final refreshToken = await _storage.getRefreshToken();
      if (refreshToken == null) {
        throw Exception('No refresh token available');
      }

      _logger.d('Attempting to refresh token');

      final response = await _dio.post<Map<String, dynamic>>(
        '$_baseUrl/auth/refresh',
        data: {'refresh_token': refreshToken},
      );

      if (response.data == null) {
        throw Exception('No data received from refresh endpoint');
      }

      final token = AuthToken.fromJson(response.data!);

      // Update stored tokens
      await _storage.saveAccessToken(token.accessToken);
      await _storage.saveTokenType(token.tokenType);

      if (token.refreshToken != null) {
        await _storage.saveRefreshToken(token.refreshToken!);
      }

      if (token.expiresIn != null) {
        await _storage.saveExpiresIn(token.expiresIn!);
      }

      _logger.i('Token refresh successful');
      return token;
    } catch (e, stackTrace) {
      _logger.e('Token refresh failed', error: e, stackTrace: stackTrace);
      rethrow;
    }
  }

  /// Logs out the current user.
  Future<void> logout() async {
    try {
      _logger.d('Logging out');
      await _storage.clearTokens();
      _logger.i('Logout successful');
    } catch (e, stackTrace) {
      _logger.e('Logout failed', error: e, stackTrace: stackTrace);
      rethrow;
    }
  }

  /// Retrieves the current user's information.
  Future<User> getCurrentUser() async {
    try {
      _logger.d('Fetching current user');

      final response = await _dio.get<Map<String, dynamic>>(
        '$_baseUrl/users/me',
      );

      if (response.data == null) {
        throw Exception('No data received from user endpoint');
      }

      final user = User.fromJson(response.data!);
      _logger.i('Current user fetched: ${user.email}');
      return user;
    } catch (e, stackTrace) {
      _logger.e(
        'Failed to fetch current user',
        error: e,
        stackTrace: stackTrace,
      );
      rethrow;
    }
  }

  /// Checks if the user is currently authenticated.
  Future<bool> isAuthenticated() async {
    return await _storage.hasAccessToken();
  }
}
