/// Authentication bloc.
library;

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/logger.dart';

import '../services/auth_service.dart';
import 'auth_event.dart';
import 'auth_state.dart';

/// Bloc for managing authentication state.
class AuthBloc extends Bloc<AuthEvent, AuthState> {
  /// Creates an authentication bloc.
  AuthBloc({
    required AuthService authService,
    Logger? logger,
  })  : _authService = authService,
        _logger = logger ?? Logger(),
        super(const AuthInitial()) {
    on<AuthCheckRequested>(_onAuthCheckRequested);
    on<AuthLoginRequested>(_onAuthLoginRequested);
    on<AuthRegisterRequested>(_onAuthRegisterRequested);
    on<AuthLogoutRequested>(_onAuthLogoutRequested);
    on<AuthRefreshRequested>(_onAuthRefreshRequested);
  }

  final AuthService _authService;
  final Logger _logger;

  /// Handles authentication check event.
  Future<void> _onAuthCheckRequested(
    AuthCheckRequested event,
    Emitter<AuthState> emit,
  ) async {
    emit(const AuthLoading());

    try {
      final isAuthenticated = await _authService.isAuthenticated();

      if (isAuthenticated) {
        final user = await _authService.getCurrentUser();
        emit(AuthAuthenticated(user: user));
        _logger.i('User is authenticated: ${user.email}');
      } else {
        emit(const AuthUnauthenticated());
        _logger.i('User is not authenticated');
      }
    } catch (e, stackTrace) {
      _logger.e('Auth check failed', error: e, stackTrace: stackTrace);
      emit(const AuthUnauthenticated());
    }
  }

  /// Handles login event.
  Future<void> _onAuthLoginRequested(
    AuthLoginRequested event,
    Emitter<AuthState> emit,
  ) async {
    emit(const AuthLoading());

    try {
      await _authService.login(
        email: event.email,
        password: event.password,
      );

      final user = await _authService.getCurrentUser();
      emit(AuthAuthenticated(user: user));
      _logger.i('Login successful: ${user.email}');
    } catch (e, stackTrace) {
      _logger.e('Login failed', error: e, stackTrace: stackTrace);
      emit(AuthFailure(message: e.toString()));
    }
  }

  /// Handles registration event.
  Future<void> _onAuthRegisterRequested(
    AuthRegisterRequested event,
    Emitter<AuthState> emit,
  ) async {
    emit(const AuthLoading());

    try {
      await _authService.register(
        email: event.email,
        password: event.password,
      );

      final user = await _authService.getCurrentUser();
      emit(AuthAuthenticated(user: user));
      _logger.i('Registration successful: ${user.email}');
    } catch (e, stackTrace) {
      _logger.e('Registration failed', error: e, stackTrace: stackTrace);
      emit(AuthFailure(message: e.toString()));
    }
  }

  /// Handles logout event.
  Future<void> _onAuthLogoutRequested(
    AuthLogoutRequested event,
    Emitter<AuthState> emit,
  ) async {
    emit(const AuthLoading());

    try {
      await _authService.logout();
      emit(const AuthUnauthenticated());
      _logger.i('Logout successful');
    } catch (e, stackTrace) {
      _logger.e('Logout failed', error: e, stackTrace: stackTrace);
      emit(AuthFailure(message: e.toString()));
    }
  }

  /// Handles token refresh event.
  Future<void> _onAuthRefreshRequested(
    AuthRefreshRequested event,
    Emitter<AuthState> emit,
  ) async {
    try {
      await _authService.refreshToken();
      final user = await _authService.getCurrentUser();
      emit(AuthAuthenticated(user: user));
      _logger.i('Token refresh successful');
    } catch (e, stackTrace) {
      _logger.e('Token refresh failed', error: e, stackTrace: stackTrace);
      emit(const AuthUnauthenticated());
    }
  }
}
