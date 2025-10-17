# altair_auth

Authentication and authorization package for the Altair ecosystem.

## Features

- JWT-based authentication
- Secure token storage using flutter_secure_storage
- Authentication state management with BLoC pattern
- Automatic token injection via HTTP interceptor
- Token refresh handling
- Login, registration, and logout flows

## Installation

Add to your `pubspec.yaml`:

```yaml
dependencies:
  altair_auth:
    path: ../altair-auth
```

## Usage

### Setup

Initialize the authentication services:

```dart
import 'package:altair_auth/altair_auth.dart';
import 'package:dio/dio.dart';

// Create secure storage
final storage = SecureStorageService();

// Create HTTP client
final dio = Dio(BaseOptions(
  baseUrl: 'https://api.altair.com',
));

// Add auth interceptor
dio.interceptors.add(AuthInterceptor(storage: storage));

// Create auth service
final authService = AuthService(
  dio: dio,
  storage: storage,
  baseUrl: 'https://api.altair.com',
);

// Create auth bloc
final authBloc = AuthBloc(authService: authService);
```

### Login

```dart
authBloc.add(AuthLoginRequested(
  email: 'user@example.com',
  password: 'password123',
));
```

### Register

```dart
authBloc.add(AuthRegisterRequested(
  email: 'newuser@example.com',
  password: 'password123',
));
```

### Logout

```dart
authBloc.add(const AuthLogoutRequested());
```

### Check Authentication Status

```dart
authBloc.add(const AuthCheckRequested());
```

### Listen to Auth State

```dart
BlocListener<AuthBloc, AuthState>(
  listener: (context, state) {
    if (state is AuthAuthenticated) {
      // User is logged in
      print('Logged in as: ${state.user.email}');
    } else if (state is AuthUnauthenticated) {
      // User is logged out
      print('Please log in');
    } else if (state is AuthFailure) {
      // Authentication failed
      print('Error: ${state.message}');
    }
  },
  child: YourWidget(),
);
```

## Architecture

### Models

- `User`: Represents an authenticated user
- `AuthToken`: JWT token response from server

### Services

- `AuthService`: Handles authentication API calls
- `SecureStorageService`: Securely stores tokens

### BLoC

- `AuthBloc`: Manages authentication state
- `AuthEvent`: Authentication events (login, logout, etc.)
- `AuthState`: Authentication states (authenticated, unauthenticated, etc.)

### Interceptors

- `AuthInterceptor`: Automatically adds JWT tokens to HTTP requests

## Testing

Run tests:

```bash
flutter test
```

## License

AGPL-3.0-or-later
