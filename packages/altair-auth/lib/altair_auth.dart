/// Authentication and authorization package for Altair ecosystem.
///
/// Provides JWT-based authentication, secure token storage, and
/// authentication state management using BLoC pattern.
library altair_auth;

// Models
export 'models/auth_token.dart';
export 'models/user.dart';

// Services
export 'services/auth_service.dart';
export 'services/secure_storage_service.dart';

// Bloc
export 'bloc/auth_bloc.dart';
export 'bloc/auth_event.dart';
export 'bloc/auth_state.dart';

// Interceptors
export 'interceptors/auth_interceptor.dart';
