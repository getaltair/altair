import 'package:altair_auth/bloc/auth_bloc.dart';
import 'package:altair_auth/bloc/auth_event.dart';
import 'package:altair_auth/bloc/auth_state.dart';
import 'package:altair_auth/models/auth_token.dart';
import 'package:altair_auth/models/user.dart';
import 'package:altair_auth/services/auth_service.dart';
import 'package:bloc_test/bloc_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:logger/logger.dart';
import 'package:mockito/annotations.dart';
import 'package:mockito/mockito.dart';

import 'auth_bloc_test.mocks.dart';

@GenerateMocks([AuthService, Logger])
void main() {
  group('AuthBloc', () {
    late MockAuthService authService;
    late MockLogger logger;

    final testToken = AuthToken(
      accessToken: 'test-access-token',
      tokenType: 'bearer',
      expiresIn: 3600,
      refreshToken: 'test-refresh-token',
    );

    setUp(() {
      authService = MockAuthService();
      logger = MockLogger();
    });

    test('initial state is AuthInitial', () {
      final bloc = AuthBloc(authService: authService, logger: logger);
      expect(bloc.state, const AuthInitial());
    });

    group('AuthCheckRequested', () {
      final testUser = User(
        id: 'test-id',
        email: 'test@example.com',
        name: 'Test User',
      );

      blocTest<AuthBloc, AuthState>(
        'emits [AuthLoading, AuthAuthenticated] when user is authenticated',
        build: () {
          when(authService.isAuthenticated())
              .thenAnswer((_) async => true);
          when(authService.getCurrentUser())
              .thenAnswer((_) async => testUser);
          return AuthBloc(authService: authService, logger: logger);
        },
        act: (bloc) => bloc.add(const AuthCheckRequested()),
        expect: () => [
          const AuthLoading(),
          AuthAuthenticated(user: testUser),
        ],
        verify: (_) {
          verify(authService.isAuthenticated()).called(1);
          verify(authService.getCurrentUser()).called(1);
        },
      );

      blocTest<AuthBloc, AuthState>(
        'emits [AuthLoading, AuthUnauthenticated] when user is not authenticated',
        build: () {
          when(authService.isAuthenticated())
              .thenAnswer((_) async => false);
          return AuthBloc(authService: authService, logger: logger);
        },
        act: (bloc) => bloc.add(const AuthCheckRequested()),
        expect: () => [
          const AuthLoading(),
          const AuthUnauthenticated(),
        ],
        verify: (_) {
          verify(authService.isAuthenticated()).called(1);
          verifyNever(authService.getCurrentUser());
        },
      );

      blocTest<AuthBloc, AuthState>(
        'emits [AuthLoading, AuthUnauthenticated] when auth check fails',
        build: () {
          when(authService.isAuthenticated())
              .thenThrow(Exception('Auth check failed'));
          return AuthBloc(authService: authService, logger: logger);
        },
        act: (bloc) => bloc.add(const AuthCheckRequested()),
        expect: () => [
          const AuthLoading(),
          const AuthUnauthenticated(),
        ],
      );
    });

    group('AuthLoginRequested', () {
      final testUser = User(
        id: 'test-id',
        email: 'test@example.com',
      );

      blocTest<AuthBloc, AuthState>(
        'emits [AuthLoading, AuthAuthenticated] when login succeeds',
        build: () {
          when(authService.login(
            email: anyNamed('email'),
            password: anyNamed('password'),
          )).thenAnswer((_) async => testToken);
          when(authService.getCurrentUser())
              .thenAnswer((_) async => testUser);
          return AuthBloc(authService: authService, logger: logger);
        },
        act: (bloc) => bloc.add(const AuthLoginRequested(
          email: 'test@example.com',
          password: 'password123',
        )),
        expect: () => [
          const AuthLoading(),
          AuthAuthenticated(user: testUser),
        ],
        verify: (_) {
          verify(authService.login(
            email: 'test@example.com',
            password: 'password123',
          )).called(1);
          verify(authService.getCurrentUser()).called(1);
        },
      );

      blocTest<AuthBloc, AuthState>(
        'emits [AuthLoading, AuthFailure] when login fails',
        build: () {
          when(authService.login(
            email: anyNamed('email'),
            password: anyNamed('password'),
          )).thenThrow(Exception('Invalid credentials'));
          return AuthBloc(authService: authService, logger: logger);
        },
        act: (bloc) => bloc.add(const AuthLoginRequested(
          email: 'test@example.com',
          password: 'wrong-password',
        )),
        expect: () => [
          const AuthLoading(),
          const AuthFailure(message: 'Exception: Invalid credentials'),
        ],
      );
    });

    group('AuthRegisterRequested', () {
      final testUser = User(
        id: 'new-user-id',
        email: 'new@example.com',
      );

      blocTest<AuthBloc, AuthState>(
        'emits [AuthLoading, AuthAuthenticated] when registration succeeds',
        build: () {
          when(authService.register(
            email: anyNamed('email'),
            password: anyNamed('password'),
          )).thenAnswer((_) async => testToken);
          when(authService.getCurrentUser())
              .thenAnswer((_) async => testUser);
          return AuthBloc(authService: authService, logger: logger);
        },
        act: (bloc) => bloc.add(const AuthRegisterRequested(
          email: 'new@example.com',
          password: 'password123',
        )),
        expect: () => [
          const AuthLoading(),
          AuthAuthenticated(user: testUser),
        ],
        verify: (_) {
          verify(authService.register(
            email: 'new@example.com',
            password: 'password123',
          )).called(1);
          verify(authService.getCurrentUser()).called(1);
        },
      );

      blocTest<AuthBloc, AuthState>(
        'emits [AuthLoading, AuthFailure] when registration fails',
        build: () {
          when(authService.register(
            email: anyNamed('email'),
            password: anyNamed('password'),
          )).thenThrow(Exception('Email already exists'));
          return AuthBloc(authService: authService, logger: logger);
        },
        act: (bloc) => bloc.add(const AuthRegisterRequested(
          email: 'existing@example.com',
          password: 'password123',
        )),
        expect: () => [
          const AuthLoading(),
          const AuthFailure(message: 'Exception: Email already exists'),
        ],
      );
    });

    group('AuthLogoutRequested', () {
      blocTest<AuthBloc, AuthState>(
        'emits [AuthLoading, AuthUnauthenticated] when logout succeeds',
        build: () {
          when(authService.logout()).thenAnswer((_) async => {});
          return AuthBloc(authService: authService, logger: logger);
        },
        act: (bloc) => bloc.add(const AuthLogoutRequested()),
        expect: () => [
          const AuthLoading(),
          const AuthUnauthenticated(),
        ],
        verify: (_) {
          verify(authService.logout()).called(1);
        },
      );

      blocTest<AuthBloc, AuthState>(
        'emits [AuthLoading, AuthFailure] when logout fails',
        build: () {
          when(authService.logout())
              .thenThrow(Exception('Logout failed'));
          return AuthBloc(authService: authService, logger: logger);
        },
        act: (bloc) => bloc.add(const AuthLogoutRequested()),
        expect: () => [
          const AuthLoading(),
          const AuthFailure(message: 'Exception: Logout failed'),
        ],
      );
    });

    group('AuthRefreshRequested', () {
      final testUser = User(
        id: 'test-id',
        email: 'test@example.com',
      );

      blocTest<AuthBloc, AuthState>(
        'emits [AuthAuthenticated] when token refresh succeeds',
        build: () {
          when(authService.refreshToken()).thenAnswer((_) async => testToken);
          when(authService.getCurrentUser())
              .thenAnswer((_) async => testUser);
          return AuthBloc(authService: authService, logger: logger);
        },
        act: (bloc) => bloc.add(const AuthRefreshRequested()),
        expect: () => [
          AuthAuthenticated(user: testUser),
        ],
        verify: (_) {
          verify(authService.refreshToken()).called(1);
          verify(authService.getCurrentUser()).called(1);
        },
      );

      blocTest<AuthBloc, AuthState>(
        'emits [AuthUnauthenticated] when token refresh fails',
        build: () {
          when(authService.refreshToken())
              .thenThrow(Exception('Refresh failed'));
          return AuthBloc(authService: authService, logger: logger);
        },
        act: (bloc) => bloc.add(const AuthRefreshRequested()),
        expect: () => [
          const AuthUnauthenticated(),
        ],
      );
    });
  });
}
