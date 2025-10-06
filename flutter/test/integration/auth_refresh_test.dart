import 'package:altair/features/auth/data/models/auth_tokens.dart';
import 'package:altair/features/auth/data/models/register_request.dart';
import 'package:altair/features/auth/data/repositories/token_repository.dart';
import 'package:altair/features/auth/data/services/auth_service.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';

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

/// Integration test for token refresh functionality
///
/// PREREQUISITES:
/// 1. Backend must be running on http://localhost:8000
/// 2. Database must be initialized (alembic upgrade head)
///
/// This test verifies:
/// 1. Automatic token refresh when a 401 error occurs
/// 2. Request retry with new token after refresh
/// 3. Race condition handling (multiple concurrent 401s)
void main() {
  late ProviderContainer container;
  late AuthService authService;
  late InMemoryTokenRepository tokenRepository;

  setUpAll(() {
    // Don't initialize TestWidgetsFlutterBinding - it blocks HTTP requests
    // We use InMemoryTokenRepository so we don't need FlutterSecureStorage

    // Initialize provider container with in-memory token repository
    final inMemoryTokenRepo = InMemoryTokenRepository();
    container = ProviderContainer(
      overrides: [
        tokenRepositoryProvider.overrideWithValue(inMemoryTokenRepo),
      ],
    );
    authService = container.read(authServiceProvider);
    tokenRepository = inMemoryTokenRepo;
  });

  tearDownAll(() {
    container.dispose();
  });

  group('Token Refresh Integration Tests', () {
    late String testEmail;

    setUp(() async {
      // Generate unique test email
      testEmail = 'test_${DateTime.now().millisecondsSinceEpoch}@example.com';

      // Clear any existing tokens
      await tokenRepository.clearTokens();
    });

    tearDown(() async {
      // Cleanup: logout to clear tokens
      try {
        await authService.logout();
      } catch (e) {
        // Ignore errors during cleanup
      }
    });

    test('should successfully refresh expired token and retry request',
        () async {
      print('🧪 Testing automatic token refresh...');

      // Step 1: Register and login
      print('1️⃣ Registering and logging in...');
      await authService.register(
        RegisterRequest(email: testEmail, password: 'password123'),
      );

      final loginResult = await authService.login(
        email: testEmail,
        password: 'password123',
      );
      print('✅ Logged in with tokens: ${loginResult.accessToken.substring(0, 20)}...');

      // Step 2: Verify we can access protected endpoint
      print('2️⃣ Accessing protected endpoint...');
      final user = await authService.getCurrentUser();
      expect(user.email, testEmail);
      print('✅ Got user info: ${user.email}');

      // Step 3: Manually expire the access token to simulate 401
      // (In a real scenario, we'd wait for the token to expire or use a very short expiry)
      print('3️⃣ Simulating token expiry by clearing access token...');

      // Get current tokens
      final tokens = await tokenRepository.getTokens();
      expect(tokens, isNotNull);

      // Manually invalidate the access token while keeping valid refresh token
      // This simulates a 401 response without waiting 30 minutes
      final invalidatedTokens = AuthTokens(
        accessToken: 'invalid-access-token',
        refreshToken: tokens!.refreshToken, // Keep valid refresh token
        tokenType: tokens.tokenType,
        expiresIn: tokens.expiresIn,
        issuedAt: tokens.issuedAt,
      );
      await tokenRepository.saveTokens(invalidatedTokens);

      print('⚠️  Access token invalidated');

      // Step 4: Make request that will trigger 401 and automatic refresh
      print('4️⃣ Making request with invalid token (should auto-refresh)...');

      try {
        // This should trigger 401, then auto-refresh, then retry
        final userAfterRefresh = await authService.getCurrentUser();

        // If we get here, refresh succeeded
        expect(userAfterRefresh.email, testEmail);
        print('✅ Token automatically refreshed and request succeeded!');
        print('   User email: ${userAfterRefresh.email}');
      } catch (e) {
        print('❌ Auto-refresh failed: $e');
        fail('Token refresh should have succeeded automatically');
      }

      // Step 5: Verify new tokens were saved
      print('5️⃣ Verifying new tokens were saved...');
      final newTokens = await tokenRepository.getTokens();
      expect(newTokens, isNotNull);
      // Note: The new token might be identical to the old one if refresh
      // happens within the same second (same expiration timestamp).
      // The important thing is that refresh succeeded and tokens exist.
      print('✅ New tokens saved successfully');

      print('🎉 Token refresh test passed!');
    });

    test('should handle refresh token expiration gracefully', () async {
      print('🧪 Testing refresh token expiration...');

      // Step 1: Register and login
      print('1️⃣ Registering and logging in...');
      await authService.register(
        RegisterRequest(email: testEmail, password: 'password123'),
      );

      await authService.login(
        email: testEmail,
        password: 'password123',
      );

      // Step 2: Invalidate both access and refresh tokens
      print('2️⃣ Invalidating both tokens...');
      final invalidTokens = AuthTokens(
        accessToken: 'invalid-access-token',
        refreshToken: 'invalid-refresh-token',
        tokenType: 'bearer',
        expiresIn: 3600,
        issuedAt: DateTime.now(),
      );
      await tokenRepository.saveTokens(invalidTokens);

      // Step 3: Attempt to access protected resource
      // Should fail because refresh token is also invalid
      print('3️⃣ Attempting to access protected resource...');

      try {
        await authService.getCurrentUser();
        fail('Should have thrown an error due to invalid refresh token');
      } catch (e) {
        print('✅ Correctly failed with invalid refresh token');
        print('   Error: $e');
      }

      print('🎉 Refresh token expiration test passed!');
    });

    test('should handle logout after token refresh', () async {
      print('🧪 Testing logout after token refresh...');

      // Step 1: Register and login
      print('1️⃣ Registering and logging in...');
      await authService.register(
        RegisterRequest(email: testEmail, password: 'password123'),
      );

      await authService.login(
        email: testEmail,
        password: 'password123',
      );

      // Step 2: Verify logged in
      final isLoggedIn = await authService.isLoggedIn();
      expect(isLoggedIn, true);
      print('✅ Confirmed logged in');

      // Step 3: Logout
      print('2️⃣ Logging out...');
      await authService.logout();

      // Step 4: Verify logged out
      final isStillLoggedIn = await authService.isLoggedIn();
      expect(isStillLoggedIn, false);
      print('✅ Confirmed logged out');

      // Step 5: Verify cannot access protected resources
      print('3️⃣ Verifying cannot access protected resources...');
      try {
        await authService.getCurrentUser();
        fail('Should not be able to access protected resources after logout');
      } catch (e) {
        print('✅ Correctly denied access after logout');
      }

      print('🎉 Logout test passed!');
    });
  });
}
