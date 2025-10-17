/// Authentication states.
library;

import 'package:equatable/equatable.dart';

import '../models/user.dart';

/// Base class for authentication states.
sealed class AuthState extends Equatable {
  const AuthState();

  @override
  List<Object?> get props => [];
}

/// Initial state before authentication check.
final class AuthInitial extends AuthState {
  const AuthInitial();
}

/// State while checking authentication status.
final class AuthLoading extends AuthState {
  const AuthLoading();
}

/// State when user is authenticated.
final class AuthAuthenticated extends AuthState {
  const AuthAuthenticated({required this.user});

  final User user;

  @override
  List<Object?> get props => [user];
}

/// State when user is not authenticated.
final class AuthUnauthenticated extends AuthState {
  const AuthUnauthenticated();
}

/// State when authentication fails.
final class AuthFailure extends AuthState {
  const AuthFailure({required this.message});

  final String message;

  @override
  List<Object?> get props => [message];
}
