import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'package:altair/core/api/api_client.dart';
import 'package:altair/features/auth/data/models/auth_tokens.dart';
import 'package:altair/features/auth/data/models/register_request.dart';
import 'package:altair/features/auth/data/repositories/token_repository.dart';
import 'package:altair/features/auth/data/services/auth_service.dart';

/// Simple in-memory token repository for testing
class InMemoryTokenRepository implements TokenRepository {
  AuthTokens? _tokens;

  @override
  Future<void> saveTokens(AuthTokens tokens) async {
    _tokens = tokens;
  }

  @override
  Future<String?> getAccessToken() async => _tokens?.accessToken;

  @override
  Future<String?> getRefreshToken() async => _tokens?.refreshToken;

  @override
  Future<AuthTokens?> getTokens() async => _tokens;

  @override
  Future<void> clearTokens() async {
    _tokens = null;
  }

  @override
  Future<bool> hasTokens() async => _tokens != null;
}

/// Integration test for auth flow.
///
/// PREREQUISITES:
/// 1. Start the FastAPI backend server:
///    cd backend && uv run uvicorn altair.main:app --reload
/// 2. Ensure the database is running and migrated
/// 3. Run this test: flutter test test/auth_integration_test.dart
///
/// This test will:
/// - Register a new user with a random email
/// - Login with that user
/// - Fetch the current user info
/// - Logout
void main() {
  // Don't initialize TestWidgetsFlutterBinding - it blocks HTTP requests
  // Just run tests without it

  group('Auth Integration Tests', () {
    late ProviderContainer container;
    late AuthService authService;

    setUp(() {
      // Create a new provider container with in-memory token storage
      container = ProviderContainer(
        overrides: [
          // Override token repository with in-memory version for testing
          tokenRepositoryProvider.overrideWithValue(InMemoryTokenRepository()),
        ],
      );
      authService = container.read(authServiceProvider);
    });

    tearDown(() {
      // Clean up
      container.dispose();
    });

    test('Full auth flow: register -> login -> getCurrentUser -> logout',
        () async {
      // Generate a unique email for this test run
      final timestamp = DateTime.now().millisecondsSinceEpoch;
      final testEmail = 'test_$timestamp@example.com';
      final testPassword = 'testpassword123';

      print('\n🧪 Starting auth integration test...');
      print('📧 Test email: $testEmail');

      // Step 1: Register a new user
      print('\n1️⃣ Registering new user...');
      final registerRequest = RegisterRequest(
        email: testEmail,
        password: testPassword,
        username: 'testuser_$timestamp',
      );

      final user = await authService.register(registerRequest);
      print('✅ Registered user: ${user.email} (ID: ${user.id})');

      expect(user.email, testEmail);
      expect(user.username, 'testuser_$timestamp');

      // Step 2: Login with the new user
      print('\n2️⃣ Logging in...');
      final tokens = await authService.login(
        email: testEmail,
        password: testPassword,
      );

      print('✅ Logged in successfully');
      print('   Access token: ${tokens.accessToken.substring(0, 20)}...');
      print('   Refresh token: ${tokens.refreshToken.substring(0, 20)}...');
      print('   Token type: ${tokens.tokenType}');
      print('   Expires in: ${tokens.expiresIn}s');

      expect(tokens.accessToken, isNotEmpty);
      expect(tokens.refreshToken, isNotEmpty);
      expect(tokens.tokenType, 'bearer');

      // Step 3: Verify we can make authenticated requests
      print('\n3️⃣ Fetching current user info...');
      final currentUser = await authService.getCurrentUser();

      print('✅ Got current user: ${currentUser.email}');
      expect(currentUser.email, testEmail);
      expect(currentUser.id, user.id);

      // Step 4: Verify we're logged in
      print('\n4️⃣ Checking login status...');
      final isLoggedIn = await authService.isLoggedIn();
      print('✅ Is logged in: $isLoggedIn');
      expect(isLoggedIn, true);

      // Step 5: Logout
      print('\n5️⃣ Logging out...');
      await authService.logout();
      print('✅ Logged out successfully');

      // Step 6: Verify we're logged out
      final isStillLoggedIn = await authService.isLoggedIn();
      print('✅ Is still logged in: $isStillLoggedIn');
      expect(isStillLoggedIn, false);

      print('\n🎉 All tests passed!\n');
    });

    test('Login with invalid credentials should fail', () async {
      print('\n🧪 Testing invalid login...');

      expect(
        () async => await authService.login(
          email: 'nonexistent@example.com',
          password: 'wrongpassword',
        ),
        throwsException,
      );

      print('✅ Invalid login correctly rejected\n');
    });

    test('Register with duplicate email should fail', () async {
      print('\n🧪 Testing duplicate registration...');

      // First registration
      final timestamp = DateTime.now().millisecondsSinceEpoch;
      final testEmail = 'duplicate_test_$timestamp@example.com';

      final request = RegisterRequest(
        email: testEmail,
        password: 'password123',
      );

      await authService.register(request);
      print('✅ First registration succeeded');

      // Try to register again with same email
      expect(
        () async => await authService.register(request),
        throwsException,
      );

      print('✅ Duplicate registration correctly rejected\n');
    });
  });
}
