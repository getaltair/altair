# Golden File Tests

Golden file tests provide visual regression testing by comparing widget screenshots against baseline images.

## What are Golden Files?

Golden files are PNG images that capture the expected appearance of widgets. When tests run, Flutter renders the widget and compares the output against the golden file. If there are visual differences, the test fails.

## Running Golden Tests

```bash
# Run all golden tests
flutter test test/golden/

# Update golden files (when you intentionally change UI)
flutter test test/golden/ --update-goldens

# Run specific golden test
flutter test test/golden/widget_golden_test.dart
```

## When to Update Golden Files

Update golden files when you:
- **Intentionally change UI** - Colors, layouts, text styles
- **Add new widgets** - New UI components
- **Change platform** - Switch between macOS, Linux, Windows (golden files are platform-specific)

## Golden File Workflow

1. **Make UI changes** to your widgets
2. **Run tests** - `flutter test test/golden/`
3. **Tests fail** if visuals changed
4. **Review changes** - Check the failure output for differences
5. **Update goldens** - If changes are intentional: `flutter test test/golden/ --update-goldens`
6. **Commit goldens** - Add updated golden files to git

## CI/CD Integration

Golden tests run in CI/CD on Linux. If you develop on macOS or Windows:
- Generate goldens on Linux (using CI or Docker)
- Or accept platform-specific differences
- Consider using `matchesGoldenFile` with platform suffixes

## Tips

- **Keep goldens small** - Test individual widgets, not full pages
- **Use consistent data** - Fixed dates, predictable content
- **Platform awareness** - Fonts and rendering differ across platforms
- **Review carefully** - Don't blindly update goldens

## Current Coverage

- Task list empty state
- Task card appearance (uncompleted)
- Task card appearance (completed)
- Priority indicators
- FloatingActionButton

## See Also

- [Flutter Golden File Testing](https://docs.flutter.dev/cookbook/testing/widget/golden-files)
- [Testing Strategy Document](../../../docs/TESTING-STRATEGY.md)
