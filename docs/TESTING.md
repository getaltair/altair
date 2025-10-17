# Testing Guide

This document provides comprehensive testing guidelines for the Altair monorepo.

## Table of Contents

- [Overview](#overview)
- [Test Structure](#test-structure)
- [Running Tests](#running-tests)
- [Testing Strategies](#testing-strategies)
- [Best Practices](#best-practices)
- [Coverage](#coverage)

## Overview

The Altair project uses Flutter's built-in testing framework with additional packages for enhanced testing capabilities:

- **flutter_test**: Core testing framework (widget and unit tests)
- **mockito**: Mocking dependencies for unit tests
- **bloc_test**: Testing BLoC state management
- **sqflite_common_ffi**: In-memory database for repository tests

## Test Structure

```
altair/
├── apps/
│   └── altair_guidance/
│       └── test/
│           └── app_test.dart
├── packages/
│   ├── altair-ui/
│   │   └── test/
│   │       └── widgets/
│   │           ├── altair_button_test.dart
│   │           ├── altair_card_test.dart
│   │           └── altair_text_field_test.dart
│   ├── altair-core/
│   │   └── test/
│   │       ├── models/
│   │       │   └── task_test.dart
│   │       └── repositories/
│   │           └── task_repository_test.dart
│   └── altair-auth/
│       └── test/
│           └── bloc/
│               └── auth_bloc_test.dart
```

## Running Tests

### Run All Tests

From the monorepo root:

```bash
# Run all tests in all packages
./scripts/test-all.sh

# Or manually test each package
cd apps/altair_guidance && flutter test
cd packages/altair-ui && flutter test
cd packages/altair-core && flutter test
cd packages/altair-auth && flutter test
```

### Run Specific Tests

```bash
# Run tests in a specific package
cd packages/altair-ui
flutter test

# Run a specific test file
flutter test test/widgets/altair_button_test.dart

# Run tests with a name pattern
flutter test --name "renders with child"
```

### Run Tests with Coverage

```bash
# Generate coverage report
flutter test --coverage

# View coverage in HTML (requires lcov)
genhtml coverage/lcov.info -o coverage/html
open coverage/html/index.html
```

## Testing Strategies

### 1. Widget Tests (altair-ui)

Widget tests verify UI components render correctly and respond to user interactions.

**Example: Testing a Button**

```dart
testWidgets('calls onPressed when tapped', (WidgetTester tester) async {
  var pressed = false;

  await tester.pumpWidget(
    MaterialApp(
      home: Scaffold(
        body: AltairButton(
          onPressed: () => pressed = true,
          child: const Text('Test'),
        ),
      ),
    ),
  );

  await tester.tap(find.byType(AltairButton));
  await tester.pump();

  expect(pressed, isTrue);
});
```

**Key Points:**

- Wrap widgets in `MaterialApp` and `Scaffold` for proper context
- Use `pumpWidget()` to build the widget tree
- Use `pump()` or `pumpAndSettle()` after interactions
- Use finders (`find.byType`, `find.text`, etc.) to locate widgets

### 2. Unit Tests (Models)

Unit tests verify business logic, data models, and utilities.

**Example: Testing a Model**

```dart
test('copyWith creates new task with updated fields', () {
  final task = Task(
    id: 'test-id',
    title: 'Original',
    createdAt: DateTime.now(),
    updatedAt: DateTime.now(),
  );

  final updated = task.copyWith(title: 'Updated');

  expect(updated.id, task.id);
  expect(updated.title, 'Updated');
});
```

**Key Points:**

- Test public API and edge cases
- Verify equality operators and hashCode
- Test serialization (toJson/fromJson)
- Validate copyWith preserves unchanged fields

### 3. Repository Tests (altair-core)

Repository tests verify database interactions using in-memory SQLite.

**Example: Testing CRUD Operations**

```dart
setUp(() {
  // Initialize FFI for in-memory database
  sqfliteFfiInit();
  databaseFactory = databaseFactoryFfi;
  repository = TaskRepository();
});

test('creates a new task', () async {
  final task = Task(
    id: '',
    title: 'Test Task',
    createdAt: DateTime.now(),
    updatedAt: DateTime.now(),
  );

  final created = await repository.create(task);

  expect(created.id, isNotEmpty);
  expect(created.title, 'Test Task');
});
```

**Key Points:**

- Use `sqflite_common_ffi` for testing
- Initialize database in `setUp()`
- Test all CRUD operations
- Test query filters and pagination
- Verify data persistence

### 4. BLoC Tests (altair-auth)

BLoC tests verify state management logic using `bloc_test` package.

**Example: Testing Authentication Flow**

```dart
blocTest<AuthBloc, AuthState>(
  'emits [AuthLoading, AuthAuthenticated] when login succeeds',
  build: () {
    when(authService.login(
      email: anyNamed('email'),
      password: anyNamed('password'),
    )).thenAnswer((_) async => {});
    when(authService.getCurrentUser())
        .thenAnswer((_) async => testUser);
    return AuthBloc(authService: authService);
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
  },
);
```

**Key Points:**

- Use `@GenerateMocks` annotation for mock generation
- Run `flutter pub run build_runner build` to generate mocks
- Use `blocTest` for testing state transitions
- Verify all expected states are emitted
- Verify dependencies are called correctly

### 5. Integration Tests

Integration tests verify complete user flows across the app.

**TODO**: Add integration tests when features are implemented.

## Best Practices

### General

1. **Test Naming**: Use descriptive test names that explain what is being tested
   - ✅ `'displays error text when provided'`
   - ❌ `'test 1'`

2. **Arrange-Act-Assert**: Structure tests clearly

   ```dart
   test('description', () {
     // Arrange: Set up test data
     final task = Task(...);

     // Act: Perform the action
     final result = task.copyWith(title: 'New');

     // Assert: Verify the outcome
     expect(result.title, 'New');
   });
   ```

3. **One Assertion Per Test**: Focus tests on a single behavior
   - When practical, test one thing at a time
   - Use groups to organize related tests

4. **Avoid Test Interdependence**: Tests should run independently
   - Don't rely on test execution order
   - Clean up state in `setUp()` or `tearDown()`

### Widget Tests

1. **Wrap in MaterialApp**: Provide proper context for Material widgets
2. **Use pumpAndSettle**: Wait for animations to complete
3. **Test Accessibility**: Verify screen reader support where applicable
4. **Test Different States**: Loading, error, empty, populated

### Unit Tests

1. **Test Edge Cases**: Null values, empty lists, boundary conditions
2. **Test Error Handling**: Verify exceptions are thrown when expected
3. **Mock External Dependencies**: Isolate the unit being tested
4. **Verify Immutability**: Ensure data models are immutable when expected

### BLoC Tests

1. **Test All Events**: Cover every event handler
2. **Test State Transitions**: Verify correct state emission order
3. **Test Error Scenarios**: Ensure errors are handled gracefully
4. **Mock All Dependencies**: Use mocks for services and repositories

## Generating Mocks

For tests that use Mockito, you need to generate mock classes:

```bash
# In the package directory (e.g., altair-auth)
cd packages/altair-auth

# Generate mocks
flutter pub run build_runner build

# Or watch for changes
flutter pub run build_runner watch
```

This creates `.mocks.dart` files alongside your test files.

## Coverage

### Target Coverage

- **Overall**: Aim for 80%+ code coverage
- **Critical paths**: 90%+ (auth, data persistence)
- **UI components**: 70%+ (focus on interactions)

### Viewing Coverage

```bash
# Generate coverage
flutter test --coverage

# Install lcov (macOS)
brew install lcov

# Generate HTML report
genhtml coverage/lcov.info -o coverage/html
open coverage/html/index.html
```

### CI Integration

Coverage reports are generated automatically in CI and uploaded to code coverage services.

## Continuous Integration

Tests run automatically on:

- Every push to any branch
- Every pull request
- Before merging to main

See `.github/workflows/ci.yml` for CI configuration.

## Common Issues

### Issue: "Bad state: No element"

**Cause**: Widget finder couldn't locate the widget.

**Solution**:

- Use `findsNothing` to verify absence
- Add `await tester.pumpAndSettle()` to wait for animations
- Check widget key or type matches exactly

### Issue: "MissingPluginException"

**Cause**: Native plugin not initialized in tests.

**Solution**:

- Use `TestWidgetsFlutterBinding.ensureInitialized()`
- Mock platform channels or use packages with test implementations

### Issue: "Cannot generate mocks"

**Cause**: Missing `@GenerateMocks` annotation or build_runner not run.

**Solution**:

```bash
flutter pub run build_runner build --delete-conflicting-outputs
```

## Resources

- [Flutter Testing Documentation](https://docs.flutter.dev/testing)
- [bloc_test Package](https://pub.dev/packages/bloc_test)
- [Mockito Documentation](https://pub.dev/packages/mockito)
- [Widget Testing Best Practices](https://docs.flutter.dev/cookbook/testing/widget/introduction)

## Contributing

When adding new features:

1. Write tests alongside code (TDD preferred)
2. Ensure tests pass locally before pushing
3. Maintain or improve coverage percentage
4. Update this guide if introducing new testing patterns
