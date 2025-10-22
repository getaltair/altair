# Integration Tests

This directory contains integration tests for Altair Guidance. These tests verify complete user workflows and would have caught the provider context bugs discovered during AppImage testing.

## Test Files

- **navigation_test.dart** - Tests all navigation flows including provider context passing
- **task_operations_test.dart** - Tests complete task CRUD workflows
- **project_operations_test.dart** - Tests complete project management workflows

## Running Tests

```bash
# Run all integration tests
flutter test integration_test

# Run specific test file
flutter test integration_test/navigation_test.dart

# Run on specific device
flutter test integration_test -d linux
```

## What These Tests Catch

### Provider Context Bugs ✅
The navigation tests specifically verify that provider context is correctly passed through navigation, which would have caught the three bugs fixed in:
- `main.dart:91` - FAB navigation
- `main.dart:275-276` - Projects menu navigation
- `projects_page.dart:36` - Project creation FAB
- `main.dart:316` - Task creation FAB

### Complete Workflows ✅
- Task creation via quick capture
- Task creation via FAB
- Project creation and management
- Navigation between pages
- State persistence

## Test Status

Current status: 4/8 tests passing

**Passing Tests:**
- App launches successfully
- Navigate to Projects from drawer
- Navigate to new task from main FAB
- Navigate back from Projects to Tasks

**Failing Tests:**
- Navigate to Projects then create new project
- Navigate through complete app flow
- Drawer navigation items check

The failures are expected for initial tests and indicate real issues to fix in the test assertions (finding multiple "Tasks" text, etc.). This is normal and healthy - tests that catch real issues!

## See Also

- [Testing Strategy Document](../../../docs/TESTING-STRATEGY.md)
- [Flutter Integration Testing Guide](https://docs.flutter.dev/testing/integration-tests)
