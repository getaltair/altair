# Altair Guidance - Comprehensive Testing Strategy

**Version:** 1.0
**Last Updated:** 2025-01-21
**Author:** Development Team

---

## Table of Contents

1. [Testing Philosophy](#testing-philosophy)
2. [Testing Pyramid](#testing-pyramid)
3. [Test Types](#test-types)
4. [Unit Testing](#unit-testing)
5. [Widget Testing](#widget-testing)
6. [Integration Testing](#integration-testing)
7. [End-to-End Testing](#end-to-end-testing)
8. [Test Organization](#test-organization)
9. [Best Practices](#best-practices)
10. [Running Tests](#running-tests)
11. [Coverage Goals](#coverage-goals)
12. [CI/CD Integration](#cicd-integration)
13. [Troubleshooting](#troubleshooting)

---

## Testing Philosophy

### Core Principles

1. **Test Behavior, Not Implementation**
   - Focus on what the code does, not how it does it
   - Tests should survive refactoring

2. **Fast Feedback**
   - Unit tests run in milliseconds
   - Widget tests run in seconds
   - Integration tests run in minutes

3. **Comprehensive Coverage**
   - 80%+ code coverage for business logic
   - 100% coverage for critical paths (auth, data persistence)
   - All user-facing flows have integration tests

4. **Prevent Regressions**
   - Every bug fix includes a test
   - Breaking changes caught before deployment

5. **Living Documentation**
   - Tests serve as usage examples
   - Test names clearly describe behavior

---

## Testing Pyramid

```
        ╱─────────╲
       ╱    E2E    ╲       ~10% (Slow, Expensive)
      ╱─────────────╲
     ╱  Integration  ╲     ~20% (Moderate Speed)
    ╱─────────────────╲
   ╱     Widget        ╲   ~30% (Fast)
  ╱─────────────────────╲
 ╱        Unit           ╲ ~40% (Very Fast)
╱─────────────────────────╲
```

### Distribution

- **Unit Tests (40%)**: Business logic, state management, utilities
- **Widget Tests (30%)**: Individual UI components
- **Integration Tests (20%)**: User workflows, navigation
- **E2E Tests (10%)**: Critical user journeys

---

## Test Types

### 1. Unit Tests

**Purpose:** Test individual functions, classes, and business logic in isolation

**Location:** `test/<feature>/*_test.dart`

**Tools:**
- `flutter_test` - Core testing framework
- `bloc_test` - Testing BLoCs
- `mocktail` - Mocking dependencies

**Example:**
```dart
test('TaskBloc emits [TaskLoading, TaskLoaded] when tasks are loaded', () {
  final repository = MockTaskRepository();
  when(() => repository.getAllTasks()).thenAnswer((_) async => [task1, task2]);

  final bloc = TaskBloc(taskRepository: repository);

  expect(
    bloc.stream,
    emitsInOrder([
      TaskLoading(),
      TaskLoaded(tasks: [task1, task2]),
    ]),
  );

  bloc.add(TaskLoadRequested());
});
```

**What to Test:**
- BLoC state transitions
- Repository CRUD operations
- Data model transformations
- Validation logic
- Error handling
- Edge cases

### 2. Widget Tests

**Purpose:** Test UI components and interactions in isolation

**Location:** `test/widgets/*_test.dart`

**Tools:**
- `flutter_test`
- `mocktail` for mocking providers

**Example:**
```dart
testWidgets('TaskCard displays task title and description', (tester) async {
  final task = Task(title: 'Test Task', description: 'Test Description');

  await tester.pumpWidget(
    MaterialApp(
      home: Scaffold(
        body: TaskCard(task: task),
      ),
    ),
  );

  expect(find.text('Test Task'), findsOneWidget);
  expect(find.text('Test Description'), findsOneWidget);
});
```

**What to Test:**
- Widget rendering
- User interactions (taps, swipes, etc.)
- Widget state changes
- Accessibility
- Layout constraints
- Theme variations

### 3. Integration Tests

**Purpose:** Test complete user workflows and feature interactions

**Location:** `integration_test/*_test.dart`

**Tools:**
- `integration_test` package
- `flutter_test`

**Example:**
```dart
testWidgets('Complete task creation workflow', (tester) async {
  app.main();
  await tester.pumpAndSettle();

  // Navigate to task creation
  await tester.tap(find.byType(FloatingActionButton));
  await tester.pumpAndSettle();

  // Fill in task details
  await tester.enterText(find.byType(TextField).first, 'Integration Test Task');
  await tester.pumpAndSettle();

  // Save task
  await tester.tap(find.text('Save'));
  await tester.pumpAndSettle();

  // Verify task appears in list
  expect(find.text('Integration Test Task'), findsOneWidget);
});
```

**What to Test:**
- Navigation flows
- Multi-step workflows
- State persistence
- Cross-feature interactions
- Error scenarios
- Provider context passing (critical!)

### 4. End-to-End Tests

**Purpose:** Test complete user journeys from start to finish

**Location:** `integration_test/e2e/*_test.dart`

**Tools:**
- `integration_test` package
- Real backend services (or test doubles)

**Example:**
```dart
testWidgets('New user onboarding journey', (tester) async {
  app.main();
  await tester.pumpAndSettle();

  // Create first task
  // Set up first project
  // Complete onboarding checklist
  // Verify dashboard shows completed onboarding
});
```

**What to Test:**
- Critical user paths
- Authentication flows
- Data synchronization
- Offline/online transitions

---

## Unit Testing

### BLoC Testing

**Pattern:**
1. Arrange: Set up dependencies and initial state
2. Act: Add event to BLoC
3. Assert: Verify state emissions

**Example:**
```dart
blocTest<TaskBloc, TaskState>(
  'emits [TaskLoading, TaskLoaded] when tasks load successfully',
  build: () {
    when(() => repository.getAllTasks()).thenAnswer((_) async => tasks);
    return TaskBloc(taskRepository: repository);
  },
  act: (bloc) => bloc.add(TaskLoadRequested()),
  expect: () => [
    TaskLoading(),
    TaskLoaded(tasks: tasks),
  ],
  verify: (_) {
    verify(() => repository.getAllTasks()).called(1);
  },
);
```

### Repository Testing

**Pattern:**
1. Mock data source
2. Call repository method
3. Verify correct transformations

**Example:**
```dart
test('getAllTasks returns parsed Task objects', () async {
  final dataSource = MockDataSource();
  when(() => dataSource.fetch()).thenAnswer((_) async => rawData);

  final repository = TaskRepository(dataSource: dataSource);
  final tasks = await repository.getAllTasks();

  expect(tasks, hasLength(2));
  expect(tasks.first, isA<Task>());
});
```

### Model Testing

**Pattern:**
1. Test serialization/deserialization
2. Test validation
3. Test equality

**Example:**
```dart
group('Task Model', () {
  test('fromJson creates valid Task', () {
    final task = Task.fromJson(taskJson);
    expect(task.title, equals('Test'));
  });

  test('toJson produces correct map', () {
    final json = task.toJson();
    expect(json['title'], equals(task.title));
  });

  test('copyWith creates new instance with updated fields', () {
    final updated = task.copyWith(title: 'New Title');
    expect(updated.title, equals('New Title'));
    expect(updated.id, equals(task.id));
  });
});
```

---

## Widget Testing

### Component Testing

**Pattern:**
1. Wrap widget in MaterialApp/Scaffold
2. Provide necessary providers/blocs
3. Pump widget
4. Verify rendering and interactions

**Example:**
```dart
testWidgets('QuickCapture submits on Enter key', (tester) async {
  var submitted = false;

  await tester.pumpWidget(
    MaterialApp(
      home: Scaffold(
        body: QuickCapture(
          onSubmit: (text) => submitted = true,
        ),
      ),
    ),
  );

  await tester.enterText(find.byType(TextField), 'Quick task');
  await tester.testTextInput.receiveAction(TextInputAction.done);
  await tester.pump();

  expect(submitted, isTrue);
});
```

### Provider Testing

**Pattern:**
1. Mock provider values
2. Wrap widget with provider
3. Verify widget responds to provider state

**Example:**
```dart
testWidgets('TaskList shows loading indicator', (tester) async {
  final mockBloc = MockTaskBloc();
  when(() => mockBloc.state).thenReturn(TaskLoading());

  await tester.pumpWidget(
    MaterialApp(
      home: BlocProvider.value(
        value: mockBloc,
        child: TaskList(),
      ),
    ),
  );

  expect(find.byType(CircularProgressIndicator), findsOneWidget);
});
```

---

## Integration Testing

### Setup

Integration tests run the full app and test real user interactions.

**File Structure:**
```
integration_test/
├── navigation_test.dart         # Navigation flows
├── task_operations_test.dart    # Task CRUD
├── project_operations_test.dart # Project CRUD
└── app_flow_test.dart          # End-to-end journeys
```

### Running Integration Tests

```bash
# Run on desktop
flutter test integration_test

# Run on specific device
flutter test integration_test -d <device_id>

# Run specific test file
flutter test integration_test/navigation_test.dart
```

### Critical Tests

#### 1. Navigation Tests
- Verify all navigation paths work
- Test provider context passing
- Test back navigation
- Test deep linking (future)

**Why Critical:** Provider context bugs cause grey screens and app crashes

#### 2. Data Persistence
- Create/update/delete operations persist
- App restores state after restart
- Offline data syncs on reconnect

#### 3. State Management
- BLoC events trigger correct state changes
- UI updates reflect state changes
- Multiple widgets share state correctly

### Integration Test Best Practices

1. **Test Real User Flows**
   ```dart
   // Good: Test complete workflow
   testWidgets('User creates and completes task', (tester) async {
     // Create task
     // Find task in list
     // Mark as complete
     // Verify completion
   });

   // Avoid: Testing implementation details
   ```

2. **Use Semantic Finders**
   ```dart
   // Good: Semantic/accessible
   find.text('Create Task')
   find.byIcon(Icons.add)
   find.byType(FloatingActionButton)

   // Avoid: Implementation-dependent
   find.byKey(Key('specific-key-123'))
   ```

3. **Wait for Animations**
   ```dart
   await tester.pumpAndSettle(); // Wait for all animations
   ```

4. **Test Negative Paths**
   ```dart
   testWidgets('Shows error when network fails', (tester) async {
     // Simulate network error
     // Verify error message shown
   });
   ```

---

## End-to-End Testing

### Critical Journeys

1. **New User Onboarding**
   - First launch
   - Create first task
   - Complete first task
   - Create first project

2. **Daily Usage Flow**
   - Launch app
   - Review today's tasks
   - Add new task via quick capture
   - Mark tasks complete
   - Review progress

3. **Project Management**
   - Create project
   - Add tasks to project
   - View project progress
   - Complete project

4. **AI Features**
   - Task breakdown
   - Time estimation
   - Smart prioritization
   - Context suggestions

### E2E Test Template

```dart
testWidgets('Complete new user journey', (tester) async {
  app.main();
  await tester.pumpAndSettle();

  // Step 1: Initial state
  expect(find.text('Tasks'), findsOneWidget);

  // Step 2: Create first task
  await tester.enterText(find.byType(TextField).first, 'My first task');
  await tester.testTextInput.receiveAction(TextInputAction.done);
  await tester.pumpAndSettle();

  // Step 3: Verify task appears
  expect(find.text('My first task'), findsOneWidget);

  // Step 4: Complete task
  await tester.tap(find.byType(Checkbox).first);
  await tester.pumpAndSettle();

  // Step 5: Verify completion
  // Add assertions for completion state
});
```

---

## Test Organization

### Directory Structure

```
apps/altair_guidance/
├── test/                           # Unit & Widget Tests
│   ├── bloc/
│   │   ├── task/
│   │   │   └── task_bloc_test.dart
│   │   ├── project/
│   │   └── ai/
│   ├── services/
│   │   └── ai/
│   │       ├── ai_service_test.dart
│   │       └── ai_config_test.dart
│   ├── features/
│   │   └── ai/
│   │       ├── task_prioritization_dialog_test.dart
│   │       └── time_estimate_dialog_test.dart
│   └── widgets/
│       ├── task_card_test.dart
│       └── quick_capture_test.dart
│
└── integration_test/               # Integration Tests
    ├── navigation_test.dart
    ├── task_operations_test.dart
    ├── project_operations_test.dart
    └── app_flow_test.dart
```

### Naming Conventions

- **Test Files:** `<feature>_test.dart`
- **Test Groups:** Describe the component/feature being tested
- **Test Cases:** Use descriptive names that explain behavior

```dart
group('TaskBloc', () {
  group('TaskLoadRequested', () {
    test('emits [TaskLoading, TaskLoaded] when successful', () {
      // test implementation
    });

    test('emits [TaskLoading, TaskFailure] when repository throws', () {
      // test implementation
    });
  });
});
```

---

## Best Practices

### 1. Test Isolation

✅ **Good:**
```dart
setUp(() {
  repository = MockTaskRepository();
  bloc = TaskBloc(taskRepository: repository);
});

tearDown(() {
  bloc.close();
});
```

❌ **Bad:**
```dart
// Shared state between tests
final bloc = TaskBloc(taskRepository: repository);
```

### 2. Clear Assertions

✅ **Good:**
```dart
expect(task.title, equals('Expected Title'));
expect(tasks, hasLength(5));
expect(state, isA<TaskLoaded>());
```

❌ **Bad:**
```dart
expect(task.title == 'Expected Title', true);
```

### 3. Descriptive Test Names

✅ **Good:**
```dart
test('emits TaskFailure when repository throws NetworkException', () {});
```

❌ **Bad:**
```dart
test('test1', () {});
```

### 4. Test One Thing

✅ **Good:**
```dart
test('task title is required', () {
  expect(() => Task(title: ''), throwsValidationException);
});

test('task description is optional', () {
  final task = Task(title: 'Title');
  expect(task.description, isNull);
});
```

❌ **Bad:**
```dart
test('task validation', () {
  // Tests multiple validation rules
});
```

### 5. Avoid Test Interdependence

✅ **Good:**
```dart
test('creates task successfully', () {
  // Self-contained test
});

test('updates task successfully', () {
  // Creates own test data
});
```

❌ **Bad:**
```dart
test('creates task successfully', () {
  createdTaskId = await repository.create(task);
});

test('updates task successfully', () {
  // Depends on createdTaskId from previous test
});
```

---

## Running Tests

### Unit Tests

```bash
# Run all unit tests
flutter test

# Run specific test file
flutter test test/bloc/task/task_bloc_test.dart

# Run with coverage
flutter test --coverage

# Watch mode
flutter test --watch
```

### Widget Tests

```bash
# Run all widget tests
flutter test test/widgets

# Run specific widget test
flutter test test/widgets/task_card_test.dart
```

### Integration Tests

```bash
# Run all integration tests
flutter test integration_test

# Run on specific device
flutter test integration_test -d macos

# Run specific integration test
flutter test integration_test/navigation_test.dart
```

### Test Options

```bash
# Verbose output
flutter test --verbose

# Update goldens (for widget tests with screenshots)
flutter test --update-goldens

# Run tests matching pattern
flutter test --name "TaskBloc"

# Exclude tests
flutter test --exclude-tags slow
```

---

## Coverage Goals

### Minimum Coverage Targets

| Component | Coverage Goal | Priority |
|-----------|--------------|----------|
| BLoCs | 90% | Critical |
| Repositories | 85% | High |
| Models | 80% | High |
| Services | 85% | High |
| Widgets | 70% | Medium |
| Utils | 80% | Medium |
| UI Components | 60% | Low |

### Generating Coverage Reports

```bash
# Generate coverage
flutter test --coverage

# Convert to HTML (requires lcov)
genhtml coverage/lcov.info -o coverage/html

# Open in browser
open coverage/html/index.html
```

### Coverage Exemptions

Some code can be excluded from coverage requirements:

```dart
// coverage:ignore-file  - Ignore entire file
// coverage:ignore-start - Start ignoring
// coverage:ignore-end   - Stop ignoring
// coverage:ignore-line  - Ignore single line
```

**Valid Exemptions:**
- Generated code (`.g.dart` files)
- Platform-specific code with fallbacks
- Debug-only code
- UI positioning/animation code

---

## CI/CD Integration

### GitHub Actions Workflow

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: subosito/flutter-action@v2
        with:
          flutter-version: '3.27.1'

      - name: Install dependencies
        run: flutter pub get

      - name: Analyze code
        run: flutter analyze

      - name: Run unit tests
        run: flutter test --coverage

      - name: Run integration tests
        run: flutter test integration_test

      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: ./coverage/lcov.info
```

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

echo "Running tests..."
flutter test || exit 1

echo "Running analyzer..."
flutter analyze || exit 1

echo "All checks passed!"
```

---

## Troubleshooting

### Common Issues

#### 1. "Provider not found" errors

**Problem:** Widget tests fail with provider errors

**Solution:**
```dart
// Wrap widget with necessary providers
await tester.pumpWidget(
  MaterialApp(
    home: BlocProvider.value(
      value: mockBloc,
      child: YourWidget(),
    ),
  ),
);
```

#### 2. Tests timeout

**Problem:** Integration tests hang

**Solution:**
```dart
// Increase timeout
testWidgets('slow test', (tester) async {
  // test code
}, timeout: Timeout(Duration(minutes: 5)));

// Ensure pumpAndSettle completes
await tester.pumpAndSettle(Duration(seconds: 10));
```

#### 3. Flaky tests

**Problem:** Tests pass/fail inconsistently

**Solutions:**
- Add proper `await` statements
- Use `pumpAndSettle()` instead of `pump()`
- Mock time-dependent code
- Avoid testing animations directly

#### 4. Golden file mismatches

**Problem:** Screenshot tests fail on different platforms

**Solution:**
```bash
# Update goldens on same platform as CI
flutter test --update-goldens

# Or use tolerance
matchesGoldenFile('golden.png', version: 1.0)
```

---

## Test Coverage Analysis

### Current Coverage (as of 2025-01-21)

- **Unit Tests:** ✅ Good coverage of BLoCs, repositories, models
- **Widget Tests:** ✅ Basic widget testing exists
- **Integration Tests:** 🔴 **MISSING** - Added in this PR
- **E2E Tests:** 🔴 **MISSING** - To be added

### Critical Gaps Addressed

1. **Navigation Flow Testing** ✅
   - Tests added for all navigation paths
   - Provider context bugs now caught

2. **User Workflow Testing** ✅
   - Task creation workflows
   - Project management workflows

3. **Error Scenario Testing** ⚠️
   - Partially covered in unit tests
   - Need more integration-level error testing

---

## Future Improvements

### Short Term
- [ ] Add E2E tests for AI features
- [ ] Implement visual regression testing
- [ ] Add performance benchmarks
- [ ] Set up automated coverage reporting

### Medium Term
- [ ] Add accessibility testing
- [ ] Implement load testing for data operations
- [ ] Add security testing for AI service
- [ ] Test offline functionality

### Long Term
- [ ] Implement cross-platform integration tests
- [ ] Add user analytics to track real-world usage patterns
- [ ] Automated UI testing in CI/CD
- [ ] A/B testing framework

---

## Resources

### Flutter Testing Docs
- [Testing Flutter Apps](https://docs.flutter.dev/testing)
- [Integration Testing](https://docs.flutter.dev/testing/integration-tests)
- [Widget Testing](https://docs.flutter.dev/cookbook/testing/widget/introduction)

### Tools & Libraries
- [flutter_test](https://api.flutter.dev/flutter/flutter_test/flutter_test-library.html)
- [integration_test](https://pub.dev/packages/integration_test)
- [bloc_test](https://pub.dev/packages/bloc_test)
- [mocktail](https://pub.dev/packages/mocktail)

### Best Practices
- [Effective Dart: Testing](https://dart.dev/guides/language/effective-dart/testing)
- [Flutter Testing Best Practices](https://flutter.dev/docs/testing/best-practices)

---

**Document Status:** Living document - update as testing strategy evolves
**Next Review:** 2025-02-21
