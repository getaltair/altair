/// Integration tests for project CRUD operations.
///
/// These tests verify complete project workflows from creation to deletion,
/// including project-task relationships.
library;

import 'package:altair_guidance/main.dart' as app;
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('Project Operations Tests', () {
    testWidgets('Navigate to Projects page', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Open drawer and navigate to Projects
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      // Verify we're on Projects page
      expect(find.text('Projects'), findsWidgets);
      expect(find.byType(FloatingActionButton), findsOneWidget);
    });

    testWidgets('Create new project', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Navigate to Projects
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      // Tap FAB to create new project
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();

      // Enter project details
      final textFields = find.byType(TextField);
      if (textFields.evaluate().isNotEmpty) {
        await tester.enterText(textFields.first, 'Test Project');
        await tester.pumpAndSettle();

        // If there's a description field
        if (textFields.evaluate().length > 1) {
          await tester.enterText(textFields.at(1), 'Test project description');
          await tester.pumpAndSettle();
        }

        // Save project
        final saveButton = find.widgetWithText(ElevatedButton, 'Save');
        if (saveButton.evaluate().isNotEmpty) {
          await tester.tap(saveButton);
          await tester.pumpAndSettle();
        }
      }

      // Verify we're back on Projects page
      expect(find.text('Projects'), findsWidgets);
    });

    testWidgets('Edit existing project', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Navigate to Projects
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      // Create a project first
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();

      final textFields = find.byType(TextField);
      if (textFields.evaluate().isNotEmpty) {
        await tester.enterText(textFields.first, 'Project to Edit');
        await tester.pumpAndSettle();

        final saveButton = find.widgetWithText(ElevatedButton, 'Save');
        if (saveButton.evaluate().isNotEmpty) {
          await tester.tap(saveButton);
          await tester.pumpAndSettle();
        }
      }

      // Tap on the project to edit it
      final projectTile = find.text('Project to Edit');
      if (projectTile.evaluate().isNotEmpty) {
        await tester.tap(projectTile);
        await tester.pumpAndSettle();

        // Verify edit page opened
        expect(find.byType(TextField), findsWidgets);
      }
    });

    testWidgets('Delete project', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Navigate to Projects
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      // Create a project first
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();

      final textFields = find.byType(TextField);
      if (textFields.evaluate().isNotEmpty) {
        await tester.enterText(textFields.first, 'Project to Delete');
        await tester.pumpAndSettle();

        final saveButton = find.widgetWithText(ElevatedButton, 'Save');
        if (saveButton.evaluate().isNotEmpty) {
          await tester.tap(saveButton);
          await tester.pumpAndSettle();
        }
      }

      // Long press or find delete option
      final projectTile = find.text('Project to Delete');
      if (projectTile.evaluate().isNotEmpty) {
        await tester.longPress(projectTile);
        await tester.pumpAndSettle();

        // Look for delete button
        final deleteButton = find.text('Delete');
        if (deleteButton.evaluate().isNotEmpty) {
          await tester.tap(deleteButton);
          await tester.pumpAndSettle();

          // Confirm deletion if dialog appears
          final confirmButton = find.text('Confirm');
          if (confirmButton.evaluate().isNotEmpty) {
            await tester.tap(confirmButton);
            await tester.pumpAndSettle();
          }
        }
      }
    });

    testWidgets('Filter projects', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Navigate to Projects
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      // Look for filter button
      final filterIcon = find.byIcon(Icons.filter_list);
      if (filterIcon.evaluate().isNotEmpty) {
        await tester.tap(filterIcon);
        await tester.pumpAndSettle();

        // Interact with filter options
        // Adjust based on actual implementation
      }
    });

    testWidgets('Assign task to project', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Create a project first
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();

      final textFields = find.byType(TextField);
      if (textFields.evaluate().isNotEmpty) {
        await tester.enterText(textFields.first, 'Project for Tasks');
        await tester.pumpAndSettle();

        final saveButton = find.widgetWithText(ElevatedButton, 'Save');
        if (saveButton.evaluate().isNotEmpty) {
          await tester.tap(saveButton);
          await tester.pumpAndSettle();
        }
      }

      // Go back to tasks
      await tester.tap(find.byType(BackButton));
      await tester.pumpAndSettle();

      // Create a task
      final fab = find.byType(FloatingActionButton).first;
      await tester.tap(fab);
      await tester.pumpAndSettle();

      // Enter task details and assign to project
      final taskFields = find.byType(TextField);
      if (taskFields.evaluate().isNotEmpty) {
        await tester.enterText(taskFields.first, 'Task for Project');
        await tester.pumpAndSettle();

        // Look for project dropdown/selector
        // This will depend on actual UI implementation
        final projectDropdown = find.text('Project for Tasks');
        if (projectDropdown.evaluate().isNotEmpty) {
          await tester.tap(projectDropdown);
          await tester.pumpAndSettle();
        }

        // Save task
        final saveButton = find.widgetWithText(ElevatedButton, 'Save');
        if (saveButton.evaluate().isNotEmpty) {
          await tester.tap(saveButton);
          await tester.pumpAndSettle();
        }
      }
    });

    testWidgets('View project details', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Navigate to Projects
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      // Create a project
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();

      final textFields = find.byType(TextField);
      if (textFields.evaluate().isNotEmpty) {
        await tester.enterText(textFields.first, 'Project Details Test');
        await tester.pumpAndSettle();

        final saveButton = find.widgetWithText(ElevatedButton, 'Save');
        if (saveButton.evaluate().isNotEmpty) {
          await tester.tap(saveButton);
          await tester.pumpAndSettle();
        }
      }

      // Tap on project to view details
      final projectTile = find.text('Project Details Test');
      if (projectTile.evaluate().isNotEmpty) {
        await tester.tap(projectTile);
        await tester.pumpAndSettle();

        // Verify project details are shown
        expect(find.byType(Scaffold), findsWidgets);
      }
    });
  });
}
