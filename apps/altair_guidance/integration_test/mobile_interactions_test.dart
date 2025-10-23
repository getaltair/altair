/// Integration tests for mobile touch interactions.
///
/// These tests verify core mobile gestures that can be reliably tested.
/// Complex interactions with dialogs and animations should be manually tested.
///
/// **Manual Testing Required:**
/// - Swipe-to-delete confirmation workflow
/// - Long-press menu delete with confirmation
/// - Settings navigation and theme switching
/// - Keyboard dismissal on different platforms
library;

import 'package:altair_guidance/main.dart' as app;
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('Mobile Touch Interactions', () {
    testWidgets('Pull-to-refresh reloads task list', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Create a task first so we have something to refresh
      final quickCaptureField = find.byType(TextField).first;
      await tester.enterText(quickCaptureField, 'Task for refresh test');
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pumpAndSettle(const Duration(seconds: 1));

      // Find the RefreshIndicator - it wraps the scrollable content
      final refreshIndicator = find.byType(RefreshIndicator);
      expect(refreshIndicator, findsOneWidget);

      // Perform pull-to-refresh gesture
      await tester.drag(refreshIndicator, const Offset(0, 300));
      await tester.pumpAndSettle();

      // Verify the task is still there (shows refresh worked)
      expect(find.text('Task for refresh test'), findsOneWidget);
    });

    testWidgets('Long-press shows context menu with actions', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Create a task
      final quickCaptureField = find.byType(TextField).first;
      await tester.enterText(quickCaptureField, 'Task for long press');
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pumpAndSettle(const Duration(seconds: 1));

      // Long press on the task
      final taskText = find.text('Task for long press');
      await tester.longPress(taskText);
      await tester.pumpAndSettle();

      // Verify bottom sheet appears with actions
      expect(find.text('Edit Task'), findsOneWidget);
      expect(find.text('Mark as Complete'), findsOneWidget);
      expect(find.text('Delete Task'), findsOneWidget);

      // Close the bottom sheet
      await tester.tapAt(const Offset(10, 10));
      await tester.pumpAndSettle();
    });

    testWidgets('Long-press menu mark as complete works', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Create a task
      final quickCaptureField = find.byType(TextField).first;
      await tester.enterText(quickCaptureField, 'Task to complete via menu');
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pumpAndSettle(const Duration(seconds: 1));

      // Long press on the task
      final taskText = find.text('Task to complete via menu');
      await tester.longPress(taskText);
      await tester.pumpAndSettle();

      // Tap "Mark as Complete"
      final completeButton = find.text('Mark as Complete');
      await tester.tap(completeButton);
      await tester.pumpAndSettle();

      // Verify the task is now marked as complete
      final checkbox = find.byType(Checkbox).first;
      final checkboxWidget = tester.widget<Checkbox>(checkbox);
      expect(checkboxWidget.value, isTrue);
    });

    testWidgets('Long-press menu shows mark as incomplete for completed tasks',
        (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Create and complete a task
      final quickCaptureField = find.byType(TextField).first;
      await tester.enterText(quickCaptureField, 'Task to uncomplete via menu');
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pumpAndSettle(const Duration(seconds: 1));

      // Complete the task via checkbox
      final checkbox = find.byType(Checkbox).first;
      await tester.tap(checkbox);
      await tester.pumpAndSettle();

      // Long press on the completed task
      final taskText = find.text('Task to uncomplete via menu');
      await tester.longPress(taskText);
      await tester.pumpAndSettle();

      // Verify "Mark as Incomplete" is shown
      expect(find.text('Mark as Incomplete'), findsOneWidget);

      // Close menu
      await tester.tapAt(const Offset(10, 10));
      await tester.pumpAndSettle();
    });

    testWidgets('Checkbox toggle completes and uncompletes task',
        (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Create a task
      final quickCaptureField = find.byType(TextField).first;
      await tester.enterText(quickCaptureField, 'Task to toggle');
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pumpAndSettle(const Duration(seconds: 1));

      // Find and tap checkbox to complete
      final checkbox = find.byType(Checkbox).first;
      await tester.tap(checkbox);
      await tester.pumpAndSettle();

      // Verify completed
      var checkboxWidget = tester.widget<Checkbox>(checkbox);
      expect(checkboxWidget.value, isTrue);

      // Tap again to uncomplete
      await tester.tap(checkbox);
      await tester.pumpAndSettle();

      // Verify uncompleted
      checkboxWidget = tester.widget<Checkbox>(checkbox);
      expect(checkboxWidget.value, isFalse);
    });
  });

  group('Quick Capture Workflow', () {
    testWidgets('Creates task via quick capture and Enter key', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Find quick capture field
      final quickCaptureField = find.byType(TextField).first;
      expect(quickCaptureField, findsOneWidget);

      // Enter task title
      await tester.enterText(quickCaptureField, 'Test task via enter');
      await tester.pumpAndSettle();

      // Submit with Enter key
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pumpAndSettle(const Duration(seconds: 1));

      // Verify task appears in list
      expect(find.text('Test task via enter'), findsOneWidget);
    });

    testWidgets('Quick capture clears after task creation', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Create a task
      final quickCaptureField = find.byType(TextField).first;
      await tester.enterText(quickCaptureField, 'Task to verify clear');
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pumpAndSettle(const Duration(seconds: 1));

      // Verify quick capture field is empty
      final textField = tester.widget<TextField>(quickCaptureField);
      expect(textField.controller?.text ?? '', isEmpty);
    });
  });
}
