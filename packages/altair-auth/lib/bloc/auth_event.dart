/// Authentication events.
library;

import 'package:equatable/equatable.dart';

/// Base class for authentication events.
sealed class AuthEvent extends Equatable {
  const AuthEvent();

  @override
  List<Object?> get props => [];
}

/// Event to check if user is already authenticated.
final class AuthCheckRequested extends AuthEvent {
  const AuthCheckRequested();
}

/// Event to log in a user.
final class AuthLoginRequested extends AuthEvent {
  const AuthLoginRequested({
    required this.email,
    required this.password,
  });

  final String email;
  final String password;

  @override
  List<Object?> get props => [email, password];
}

/// Event to register a new user.
final class AuthRegisterRequested extends AuthEvent {
  const AuthRegisterRequested({
    required this.email,
    required this.password,
  });

  final String email;
  final String password;

  @override
  List<Object?> get props => [email, password];
}

/// Event to log out the current user.
final class AuthLogoutRequested extends AuthEvent {
  const AuthLogoutRequested();
}

/// Event to refresh the access token.
final class AuthRefreshRequested extends AuthEvent {
  const AuthRefreshRequested();
}
