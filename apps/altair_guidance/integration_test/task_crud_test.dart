/// Integration tests for Task CRUD operations.
///
/// These tests verify complete user journeys for creating, reading,
/// updating, and deleting tasks - the core functionality of the app.
library;

import 'package:altair_guidance/main.dart' as app;
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('Task CRUD Operations', () {
    testWidgets('Create task with minimal fields (title only)', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Note: Database may have existing tasks from previous tests
      // We just verify the app loaded successfully
      expect(find.byType(FloatingActionButton), findsOneWidget);

      // Open new task page via FAB
      final fab = find.byType(FloatingActionButton);
      await tester.tap(fab);
      await tester.pumpAndSettle();

      // Verify task edit page loaded
      expect(find.text('New Task'), findsOneWidget);

      // Enter only a title
      final titleField = find.widgetWithText(TextField, 'What needs to be done?');
      expect(titleField, findsOneWidget);
      await tester.enterText(titleField, 'Buy groceries');
      await tester.pumpAndSettle();

      // Save the task
      final saveButton = find.text('Save');
      await tester.tap(saveButton);
      await tester.pumpAndSettle();

      // Verify we're back on main page
      expect(find.text('Tasks'), findsWidgets);

      // Verify task appears in list
      expect(find.text('Buy groceries'), findsOneWidget);
    });

    testWidgets('Create task with all fields filled', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Open new task page
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();

      // Fill title
      final titleField = find.widgetWithText(TextField, 'What needs to be done?');
      await tester.enterText(titleField, 'Complete integration tests');
      await tester.pumpAndSettle();

      // Fill description
      final descriptionField = find.widgetWithText(TextField, 'Add more details...');
      await tester.enterText(descriptionField, 'Add comprehensive CRUD tests for task management');
      await tester.pumpAndSettle();

      // Set estimated time
      final estimatedTimeField = find.widgetWithText(TextField, 'e.g., 30');
      await tester.enterText(estimatedTimeField, '120');
      await tester.pumpAndSettle();

      // Save task
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Verify task appears with title
      expect(find.text('Complete integration tests'), findsOneWidget);

      // Description should be visible in list (if implemented)
      // Note: Description might be truncated in list view
    });

    testWidgets('Read task - verify it appears in list', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Create first task
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();
      final titleField1 = find.widgetWithText(TextField, 'What needs to be done?');
      await tester.enterText(titleField1, 'Task 1');
      await tester.pumpAndSettle();
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Create second task
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();
      final titleField2 = find.widgetWithText(TextField, 'What needs to be done?');
      await tester.enterText(titleField2, 'Task 2');
      await tester.pumpAndSettle();
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Verify both tasks appear in list
      expect(find.text('Task 1'), findsOneWidget);
      expect(find.text('Task 2'), findsOneWidget);

      // Verify list structure
      expect(find.byType(ReorderableListView), findsOneWidget);
    });

    testWidgets('Update task - change title and verify', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Create a task
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();
      await tester.enterText(
        find.widgetWithText(TextField, 'What needs to be done?'),
        'Original title',
      );
      await tester.pumpAndSettle();
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Verify original task appears
      expect(find.text('Original title'), findsOneWidget);

      // Tap on the task card to edit it
      await tester.tap(find.text('Original title'));
      await tester.pumpAndSettle();

      // Verify edit page opened
      expect(find.text('Edit Task'), findsOneWidget);

      // Clear and update title
      final titleField = find.widgetWithText(TextField, 'What needs to be done?');
      await tester.enterText(titleField, '');
      await tester.pumpAndSettle();
      await tester.enterText(titleField, 'Updated title');
      await tester.pumpAndSettle();

      // Save changes
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Verify updated title appears in list
      expect(find.text('Updated title'), findsOneWidget);
      expect(find.text('Original title'), findsNothing);
    });

    testWidgets('Update task - change status to completed', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Create a task
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();
      await tester.enterText(
        find.widgetWithText(TextField, 'What needs to be done?'),
        'Task to complete',
      );
      await tester.pumpAndSettle();
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Find and tap the task's checkbox
      final checkbox = find.byType(Checkbox).first;
      expect(checkbox, findsOneWidget);

      // Verify initially unchecked
      final checkboxWidget = tester.widget<Checkbox>(checkbox);
      expect(checkboxWidget.value, isFalse);

      // Tap checkbox to complete task
      await tester.tap(checkbox);
      await tester.pumpAndSettle();

      // Verify checkbox is now checked
      final updatedCheckbox = tester.widget<Checkbox>(checkbox);
      expect(updatedCheckbox.value, isTrue);

      // Verify task title still appears (might have strikethrough style)
      expect(find.text('Task to complete'), findsOneWidget);
    });

    testWidgets('Update task - add description', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Create task without description
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();
      await tester.enterText(
        find.widgetWithText(TextField, 'What needs to be done?'),
        'Task without description',
      );
      await tester.pumpAndSettle();
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Open task for editing
      await tester.tap(find.text('Task without description'));
      await tester.pumpAndSettle();

      // Add description
      final descField = find.widgetWithText(TextField, 'Add more details...');
      await tester.enterText(descField, 'This is the added description');
      await tester.pumpAndSettle();

      // Save
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Description should now appear in list (if visible in list view)
      // At minimum, task should still be in list
      expect(find.text('Task without description'), findsOneWidget);
    });

    testWidgets('Delete task - verify it disappears', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Use unique task name to avoid collisions
      final taskName = 'DeleteSingle_${DateTime.now().millisecondsSinceEpoch}';

      // Create a task to delete
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();
      await tester.enterText(
        find.widgetWithText(TextField, 'What needs to be done?'),
        taskName,
      );
      await tester.pumpAndSettle();
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Verify task exists
      expect(find.text(taskName), findsOneWidget);

      // Find and tap delete button (IconButton with delete_outline icon)
      final deleteButton = find.byIcon(Icons.delete_outline);
      expect(deleteButton, findsAtLeastNWidgets(1));
      await tester.tap(deleteButton.first);
      await tester.pumpAndSettle();

      // Verify task is gone
      expect(find.text(taskName), findsNothing);
    });

    testWidgets('Delete task when multiple exist - verify only one deleted', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Use unique task names to avoid collisions with previous test runs
      final timestamp = DateTime.now().millisecondsSinceEpoch;
      final task1Name = 'DeleteTest1_$timestamp';
      final task2Name = 'DeleteTest2_$timestamp';
      final task3Name = 'DeleteTest3_$timestamp';

      // Create three tasks
      for (final name in [task1Name, task2Name, task3Name]) {
        await tester.tap(find.byType(FloatingActionButton));
        await tester.pumpAndSettle();
        await tester.enterText(
          find.widgetWithText(TextField, 'What needs to be done?'),
          name,
        );
        await tester.pumpAndSettle();
        await tester.tap(find.text('Save'));
        await tester.pumpAndSettle();
      }

      // Verify all three tasks exist
      expect(find.text(task1Name), findsOneWidget);
      expect(find.text(task2Name), findsOneWidget);
      expect(find.text(task3Name), findsOneWidget);

      // Count delete buttons before deletion
      final deleteButtonsBefore = find.byIcon(Icons.delete_outline);
      final countBefore = tester.widgetList(deleteButtonsBefore).length;

      // Find the delete button for task2 by finding its index
      // The tasks should be in the list in reverse creation order (newest first)
      final task2Finder = find.text(task2Name);
      expect(task2Finder, findsOneWidget);

      // Get all delete buttons and tap the one corresponding to task2
      // Since we just created these tasks, they should be at the top of the list
      final allDeleteButtons = find.byIcon(Icons.delete_outline);
      // Task2 should be second in the list (0-indexed position 1)
      await tester.tap(allDeleteButtons.at(1));
      await tester.pumpAndSettle();

      // Verify Task 2 is gone but others remain
      expect(find.text(task1Name), findsOneWidget);
      expect(find.text(task2Name), findsNothing);
      expect(find.text(task3Name), findsOneWidget);

      // Verify one less delete button exists
      final deleteButtonsAfter = find.byIcon(Icons.delete_outline);
      expect(tester.widgetList(deleteButtonsAfter).length, equals(countBefore - 1));
    });

    testWidgets('Form validation - empty title shows error', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Open new task page
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();

      // Try to save without entering title
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Should show error snackbar
      expect(find.text('Task title cannot be empty'), findsOneWidget);

      // Should still be on edit page (not navigated back)
      expect(find.text('New Task'), findsOneWidget);
    });

    testWidgets('Unsaved changes warning when navigating back', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Open new task page
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();

      // Enter some text
      await tester.enterText(
        find.widgetWithText(TextField, 'What needs to be done?'),
        'Unsaved task',
      );
      await tester.pumpAndSettle();

      // Try to go back without saving
      final backButton = find.byType(BackButton);
      await tester.tap(backButton);
      await tester.pumpAndSettle();

      // Should show discard changes dialog
      expect(find.text('Discard changes?'), findsOneWidget);
      expect(find.text('You have unsaved changes. Are you sure you want to discard them?'), findsOneWidget);

      // Cancel the discard
      await tester.tap(find.text('Cancel'));
      await tester.pumpAndSettle();

      // Should still be on edit page
      expect(find.text('New Task'), findsOneWidget);
      expect(find.text('Unsaved task'), findsOneWidget);

      // Now discard changes
      await tester.tap(backButton);
      await tester.pumpAndSettle();
      await tester.tap(find.text('Discard'));
      await tester.pumpAndSettle();

      // Should be back on tasks page
      expect(find.text('Tasks'), findsWidgets);
      expect(find.text('Unsaved task'), findsNothing); // Task not saved
    });

    testWidgets('Create multiple tasks and verify list updates', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      final taskTitles = [
        'Morning routine',
        'Work on project',
        'Exercise',
        'Read book',
        'Prepare dinner',
      ];

      // Create all tasks
      for (final title in taskTitles) {
        await tester.tap(find.byType(FloatingActionButton));
        await tester.pumpAndSettle();
        await tester.enterText(
          find.widgetWithText(TextField, 'What needs to be done?'),
          title,
        );
        await tester.pumpAndSettle();
        await tester.tap(find.text('Save'));
        await tester.pumpAndSettle();
      }

      // Verify all tasks appear in list
      for (final title in taskTitles) {
        expect(find.text(title), findsOneWidget);
      }

      // Verify at least the number of tasks we created exist
      expect(find.byIcon(Icons.delete_outline), findsAtLeastNWidgets(taskTitles.length));
    });

    testWidgets('Quick capture task from main page', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Find quick capture field
      final quickCaptureField = find.widgetWithText(TextField, 'Quick capture (Ctrl/Cmd + K)...');
      expect(quickCaptureField, findsOneWidget);

      // Enter task via quick capture
      await tester.enterText(quickCaptureField, 'Quick captured task');
      await tester.pumpAndSettle();

      // Submit (press enter or wait for auto-submit)
      await tester.testTextInput.receiveAction(TextInputAction.done);
      await tester.pumpAndSettle();

      // Verify task was captured and appears in list
      expect(find.text('Quick captured task'), findsOneWidget);

      // Should show success snackbar
      expect(find.textContaining('Task captured'), findsOneWidget);
    });
  });
}
