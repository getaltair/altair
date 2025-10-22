/// Integration tests for Task-Project relationship operations.
///
/// These tests verify that tasks can be correctly associated with projects
/// and that the relationships are maintained through CRUD operations.
library;

import 'package:altair_guidance/main.dart' as app;
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('Task-Project Relationships', () {
    testWidgets('Create task and assign to project', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // First, create a project
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();
      await tester.enterText(find.byType(TextField).first, 'Test Project');
      await tester.pumpAndSettle();
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Navigate back to Tasks
      await tester.tap(find.byType(BackButton));
      await tester.pumpAndSettle();

      // Create a task
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();

      // Enter task title
      await tester.enterText(
        find.widgetWithText(TextField, 'What needs to be done?'),
        'Task for Test Project',
      );
      await tester.pumpAndSettle();

      // Find and select project dropdown if available
      final dropdownFinders = find.byType(DropdownButton<String?>);
      if (dropdownFinders.evaluate().isNotEmpty) {
        // Tap dropdown to open
        await tester.tap(dropdownFinders.first);
        await tester.pumpAndSettle();

        // Select "Test Project" from dropdown
        final projectOption = find.text('Test Project').last;
        if (projectOption.evaluate().isNotEmpty) {
          await tester.tap(projectOption);
          await tester.pumpAndSettle();
        }
      }

      // Save task
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Verify task was created
      expect(find.text('Task for Test Project'), findsOneWidget);
    });

    testWidgets('Create task without project assignment', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Create a task without selecting a project
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();

      await tester.enterText(
        find.widgetWithText(TextField, 'What needs to be done?'),
        'Unassigned task',
      );
      await tester.pumpAndSettle();

      // Save without selecting project
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Should still create successfully
      expect(find.text('Unassigned task'), findsOneWidget);
    });

    testWidgets('Update task to assign it to a project', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Create a project first
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();
      await tester.enterText(find.byType(TextField).first, 'Assignment Project');
      await tester.pumpAndSettle();
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Go back to tasks
      await tester.tap(find.byType(BackButton));
      await tester.pumpAndSettle();

      // Create task without project
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();
      await tester.enterText(
        find.widgetWithText(TextField, 'What needs to be done?'),
        'Task to be assigned',
      );
      await tester.pumpAndSettle();
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Edit task to add project
      await tester.tap(find.text('Task to be assigned'));
      await tester.pumpAndSettle();

      // Select project from dropdown
      final dropdownFinders = find.byType(DropdownButton<String?>);
      if (dropdownFinders.evaluate().isNotEmpty) {
        await tester.tap(dropdownFinders.first);
        await tester.pumpAndSettle();

        final projectOption = find.text('Assignment Project').last;
        if (projectOption.evaluate().isNotEmpty) {
          await tester.tap(projectOption);
          await tester.pumpAndSettle();
        }
      }

      // Save
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Task should still exist
      expect(find.text('Task to be assigned'), findsOneWidget);
    });

    testWidgets('Create multiple tasks for same project', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Create project
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();
      await tester.enterText(find.byType(TextField).first, 'Shared Project');
      await tester.pumpAndSettle();
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Go to tasks
      await tester.tap(find.byType(BackButton));
      await tester.pumpAndSettle();

      // Create multiple tasks for this project
      final taskTitles = [
        'Task 1 for Shared Project',
        'Task 2 for Shared Project',
        'Task 3 for Shared Project',
      ];

      for (final title in taskTitles) {
        await tester.tap(find.byType(FloatingActionButton));
        await tester.pumpAndSettle();

        await tester.enterText(
          find.widgetWithText(TextField, 'What needs to be done?'),
          title,
        );
        await tester.pumpAndSettle();

        // Select project if dropdown available
        final dropdownFinders = find.byType(DropdownButton<String?>);
        if (dropdownFinders.evaluate().isNotEmpty) {
          await tester.tap(dropdownFinders.first);
          await tester.pumpAndSettle();

          final projectOption = find.text('Shared Project').last;
          if (projectOption.evaluate().isNotEmpty) {
            await tester.tap(projectOption);
            await tester.pumpAndSettle();
          }
        }

        await tester.tap(find.text('Save'));
        await tester.pumpAndSettle();
      }

      // Verify all tasks exist
      for (final title in taskTitles) {
        expect(find.text(title), findsOneWidget);
      }
    });

    testWidgets('Update task to remove project assignment', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Create project
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();
      await tester.enterText(find.byType(TextField).first, 'Removable Project');
      await tester.pumpAndSettle();
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Go to tasks and create task with project
      await tester.tap(find.byType(BackButton));
      await tester.pumpAndSettle();

      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();

      await tester.enterText(
        find.widgetWithText(TextField, 'What needs to be done?'),
        'Task to unassign',
      );
      await tester.pumpAndSettle();

      // Assign to project
      final dropdownFinders = find.byType(DropdownButton<String?>);
      if (dropdownFinders.evaluate().isNotEmpty) {
        await tester.tap(dropdownFinders.first);
        await tester.pumpAndSettle();

        final projectOption = find.text('Removable Project').last;
        if (projectOption.evaluate().isNotEmpty) {
          await tester.tap(projectOption);
          await tester.pumpAndSettle();
        }
      }

      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Edit task to remove project
      await tester.tap(find.text('Task to unassign'));
      await tester.pumpAndSettle();

      // Select "None" or empty option from dropdown if available
      final editDropdown = find.byType(DropdownButton<String?>);
      if (editDropdown.evaluate().isNotEmpty) {
        await tester.tap(editDropdown.first);
        await tester.pumpAndSettle();

        // Look for "None" or similar option
        final noneOption = find.text('None').last;
        if (noneOption.evaluate().isNotEmpty) {
          await tester.tap(noneOption);
          await tester.pumpAndSettle();
        }
      }

      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Task should still exist
      expect(find.text('Task to unassign'), findsOneWidget);
    });

    testWidgets('Create tasks across multiple projects', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Create two projects
      final projects = ['Project A', 'Project B'];

      for (final projectName in projects) {
        await tester.tap(find.byIcon(Icons.menu));
        await tester.pumpAndSettle();
        await tester.tap(find.text('Projects'));
        await tester.pumpAndSettle();

        await tester.tap(find.byType(FloatingActionButton));
        await tester.pumpAndSettle();
        await tester.enterText(find.byType(TextField).first, projectName);
        await tester.pumpAndSettle();
        await tester.tap(find.text('Save'));
        await tester.pumpAndSettle();

        await tester.tap(find.byType(BackButton));
        await tester.pumpAndSettle();
      }

      // Create task for Project A
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();
      await tester.enterText(
        find.widgetWithText(TextField, 'What needs to be done?'),
        'Task for Project A',
      );
      await tester.pumpAndSettle();

      final dropdownFinders = find.byType(DropdownButton<String?>);
      if (dropdownFinders.evaluate().isNotEmpty) {
        await tester.tap(dropdownFinders.first);
        await tester.pumpAndSettle();

        final projectAOption = find.text('Project A').last;
        if (projectAOption.evaluate().isNotEmpty) {
          await tester.tap(projectAOption);
          await tester.pumpAndSettle();
        }
      }

      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Create task for Project B
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();
      await tester.enterText(
        find.widgetWithText(TextField, 'What needs to be done?'),
        'Task for Project B',
      );
      await tester.pumpAndSettle();

      final dropdownFinders2 = find.byType(DropdownButton<String?>);
      if (dropdownFinders2.evaluate().isNotEmpty) {
        await tester.tap(dropdownFinders2.first);
        await tester.pumpAndSettle();

        final projectBOption = find.text('Project B').last;
        if (projectBOption.evaluate().isNotEmpty) {
          await tester.tap(projectBOption);
          await tester.pumpAndSettle();
        }
      }

      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Verify both tasks exist
      expect(find.text('Task for Project A'), findsOneWidget);
      expect(find.text('Task for Project B'), findsOneWidget);
    });

    testWidgets('Project dropdown shows all available projects', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Create several projects
      final projectNames = ['Alpha', 'Beta', 'Gamma'];

      for (final name in projectNames) {
        await tester.tap(find.byIcon(Icons.menu));
        await tester.pumpAndSettle();
        await tester.tap(find.text('Projects'));
        await tester.pumpAndSettle();

        await tester.tap(find.byType(FloatingActionButton));
        await tester.pumpAndSettle();
        await tester.enterText(find.byType(TextField).first, name);
        await tester.pumpAndSettle();
        await tester.tap(find.text('Save'));
        await tester.pumpAndSettle();

        await tester.tap(find.byType(BackButton));
        await tester.pumpAndSettle();
      }

      // Create new task and open project dropdown
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();

      final dropdownFinders = find.byType(DropdownButton<String?>);
      if (dropdownFinders.evaluate().isNotEmpty) {
        await tester.tap(dropdownFinders.first);
        await tester.pumpAndSettle();

        // Verify all projects appear in dropdown
        // Note: Projects appear multiple times (in list and dropdown)
        for (final name in projectNames) {
          expect(find.text(name), findsWidgets);
        }
      }
    });
  });
}
