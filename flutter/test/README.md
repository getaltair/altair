# Altair Flutter Tests

This directory contains comprehensive tests for the Altair Flutter application.

## Test Structure

```
test/
├── unit/                          # Unit tests (fast, isolated)
│   ├── models/                    # Data model tests
│   │   ├── auth_tokens_test.dart
│   │   ├── login_request_test.dart
│   │   ├── register_request_test.dart
│   │   └── user_test.dart
│   ├── repositories/              # Repository layer tests
│   │   └── token_repository_test.dart
│   └── services/                  # Service layer tests
│       └── auth_service_test.dart
├── integration/                   # Integration tests (requires backend)
│   └── auth_integration_test.dart
└── README.md                      # This file
```

## Test Types

### Unit Tests (`test/unit/`)

Unit tests are **fast, isolated tests** that verify individual components work correctly in isolation. They use mocks for all external dependencies.

**Characteristics:**
- ✅ Run in milliseconds
- ✅ No external dependencies (database, API, etc.)
- ✅ Use mocks for dependencies
- ✅ Test edge cases and error conditions
- ✅ Run on every commit

**Coverage:**
- **Models**: JSON serialization, validation, equality
- **Repositories**: Token storage, caching, error handling
- **Services**: Business logic, API calls (mocked), state management

### Integration Tests (`test/integration/`)

Integration tests verify that **multiple components work together correctly** with real external services.

**Characteristics:**
- ⏱️ Slower (seconds to minutes)
- 🔌 Require running backend server
- 🌐 Make real HTTP requests
- 🔄 Test end-to-end flows
- 📊 Run before releases

**Current coverage:**
- Full authentication flow (register → login → getCurrentUser → logout)
- Error scenarios (invalid credentials, duplicate registration)

## Running Tests

### Run All Unit Tests (Recommended for Development)

```bash
flutter test test/unit/
```

Expected output: ~70 tests passing in ~2 seconds

### Run Specific Test File

```bash
flutter test test/unit/models/auth_tokens_test.dart
flutter test test/unit/repositories/token_repository_test.dart
flutter test test/unit/services/auth_service_test.dart
```

### Run Integration Tests (Requires Backend)

**Prerequisites:**
1. Start the FastAPI backend:
   ```bash
   cd backend && uv run uvicorn altair.main:app --reload
   ```
2. Ensure database is running and migrated

**Run the tests:**
```bash
flutter test test/integration/auth_integration_test.dart
```

### Run All Tests

```bash
flutter test
```

### Run Tests with Coverage

```bash
flutter test --coverage
genhtml coverage/lcov.info -o coverage/html
open coverage/html/index.html
```

## Writing New Tests

### Unit Test Example (Models)

```dart
import 'package:flutter_test/flutter_test.dart';
import 'package:altair/features/auth/data/models/user.dart';

void main() {
  group('User', () {
    test('should deserialize from JSON', () {
      // Arrange
      final json = {'id': '123', 'email': 'test@example.com'};

      // Act
      final user = User.fromJson(json);

      // Assert
      expect(user.id, '123');
      expect(user.email, 'test@example.com');
    });
  });
}
```

### Unit Test Example (With Mocks)

```dart
import 'package:flutter_test/flutter_test.dart';
import 'package:mockito/annotations.dart';
import 'package:mockito/mockito.dart';

import 'my_test.mocks.dart';

@GenerateMocks([MyDependency])
void main() {
  late MockMyDependency mockDependency;
  late MyService service;

  setUp(() {
    mockDependency = MockMyDependency();
    service = MyService(dependency: mockDependency);
  });

  test('should call dependency correctly', () async {
    // Arrange
    when(mockDependency.doSomething())
        .thenAnswer((_) async => 'result');

    // Act
    final result = await service.performAction();

    // Assert
    expect(result, 'result');
    verify(mockDependency.doSomething()).called(1);
  });
}
```

### Generating Mocks

After adding `@GenerateMocks([ClassName])` to your test file:

```bash
flutter pub run build_runner build --delete-conflicting-outputs
```

## Test Guidelines

### DO ✅

- **Write unit tests for all new code**
- **Test edge cases and error conditions**
- **Use descriptive test names** (e.g., "should return null when tokens are missing")
- **Follow AAA pattern** (Arrange, Act, Assert)
- **Mock external dependencies** (API, storage, etc.)
- **Keep tests independent** (no shared state between tests)
- **Use `setUp()` for common initialization**
- **Test both success and failure paths**

### DON'T ❌

- **Don't make real API calls in unit tests** (use mocks)
- **Don't share state between tests** (causes flaky tests)
- **Don't test implementation details** (test behavior)
- **Don't skip error cases**
- **Don't write brittle tests** (overly specific assertions)

## Current Test Coverage

### Models (100% coverage)
- ✅ AuthTokens: Serialization, equality, validation
- ✅ User: Serialization, optional fields, date parsing
- ✅ RegisterRequest: Serialization, security (password not in toString)
- ✅ LoginRequest: OAuth2 format, email mapping

### Repositories (100% coverage)
- ✅ SecureTokenRepository: Storage, caching, error handling

### Services (100% coverage)
- ✅ AuthService: Registration, login, token refresh, logout

### Integration (E2E flows)
- ✅ Full auth flow
- ✅ Invalid credentials handling
- ✅ Duplicate registration handling

## CI/CD Integration

Unit tests run automatically on:
- Every pull request
- Every push to main
- Pre-commit hook (optional)

Integration tests run:
- Before releases
- Nightly (scheduled)
- Manual trigger

## Troubleshooting

### Mock Generation Fails

```bash
# Clean and regenerate
flutter clean
flutter pub get
flutter pub run build_runner build --delete-conflicting-outputs
```

### Tests Fail After Adding Dependency

```bash
flutter pub get
# Regenerate mocks if needed
flutter pub run build_runner build --delete-conflicting-outputs
```

### Integration Tests Fail

1. Check backend is running: `curl http://localhost:8000/health`
2. Check database is migrated: `cd backend && uv run alembic current`
3. Check for port conflicts: `lsof -i :8000`

## Future Test Additions

- [ ] Widget tests for auth screens
- [ ] API client unit tests
- [ ] Provider/state management tests
- [ ] E2E tests with widget interaction
- [ ] Performance tests
- [ ] Accessibility tests

## Resources

- [Flutter Testing Documentation](https://docs.flutter.dev/testing)
- [Mockito Package](https://pub.dev/packages/mockito)
- [Testing Best Practices](https://flutter.dev/docs/cookbook/testing)
