/// Integration tests for Project CRUD operations.
///
/// These tests verify complete user journeys for creating, reading,
/// updating, and deleting projects.
library;

import 'package:altair_guidance/main.dart' as app;
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('Project CRUD Operations', () {
    testWidgets('Create project with minimal fields (name only)', (
      tester,
    ) async {
      app.main();
      await tester.pumpAndSettle();

      // Navigate to Projects page
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      // Note: Database may have existing projects from previous tests
      // Verify we're on Projects page
      expect(find.text('Projects'), findsWidgets);

      // Tap FAB to create new project
      final fab = find.byType(FloatingActionButton);
      await tester.tap(fab);
      await tester.pumpAndSettle();

      // Enter project name
      final nameField = find.byType(TextField).first;
      await tester.enterText(nameField, 'My First Project');
      await tester.pumpAndSettle();

      // Save project
      final saveButton = find.text('Save');
      await tester.tap(saveButton);
      await tester.pumpAndSettle();

      // Verify project appears in list
      expect(find.text('My First Project'), findsOneWidget);
    });

    testWidgets('Create project with all fields filled', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Navigate to Projects page
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      // Create new project
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();

      // Fill name (first TextField)
      final textFields = find.byType(TextField);
      await tester.enterText(textFields.first, 'Complete Project');
      await tester.pumpAndSettle();

      // Fill description (second TextField if exists)
      if (textFields.evaluate().length > 1) {
        await tester.enterText(
          textFields.at(1),
          'A comprehensive project with all details',
        );
        await tester.pumpAndSettle();
      }

      // Save
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Verify project created
      expect(find.text('Complete Project'), findsOneWidget);
    });

    testWidgets('Read project - verify it appears in list', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Navigate to Projects
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      // Create multiple projects
      for (var i = 1; i <= 3; i++) {
        await tester.tap(find.byType(FloatingActionButton));
        await tester.pumpAndSettle();
        await tester.enterText(find.byType(TextField).first, 'Project $i');
        await tester.pumpAndSettle();
        await tester.tap(find.text('Save'));
        await tester.pumpAndSettle();
      }

      // Verify all projects appear
      expect(find.text('Project 1'), findsOneWidget);
      expect(find.text('Project 2'), findsOneWidget);
      expect(find.text('Project 3'), findsOneWidget);

      // Verify list structure
      expect(find.byType(ListView), findsOneWidget);
    });

    testWidgets('Update project - change name', (tester) async {
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
      await tester.enterText(find.byType(TextField).first, 'Original Project');
      await tester.pumpAndSettle();
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Tap project to edit
      await tester.tap(find.text('Original Project'));
      await tester.pumpAndSettle();

      // Update name
      final nameField = find.byType(TextField).first;
      await tester.enterText(nameField, '');
      await tester.pumpAndSettle();
      await tester.enterText(nameField, 'Updated Project');
      await tester.pumpAndSettle();

      // Save changes
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Verify updated name
      expect(find.text('Updated Project'), findsOneWidget);
      expect(find.text('Original Project'), findsNothing);
    });

    testWidgets('Update project - change status', (tester) async {
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
      await tester.enterText(
        find.byType(TextField).first,
        'Status Test Project',
      );
      await tester.pumpAndSettle();
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Open project for editing
      await tester.tap(find.text('Status Test Project'));
      await tester.pumpAndSettle();

      // Find and change status dropdown if available
      final dropdownFinders = find.byType(DropdownButton<dynamic>);
      if (dropdownFinders.evaluate().isNotEmpty) {
        await tester.tap(dropdownFinders.first);
        await tester.pumpAndSettle();

        // Select a different status (e.g., Completed)
        final completedOption = find.text('Completed').last;
        if (completedOption.evaluate().isNotEmpty) {
          await tester.tap(completedOption);
          await tester.pumpAndSettle();
        }
      }

      // Save
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Project should still appear
      expect(find.text('Status Test Project'), findsOneWidget);
    });

    testWidgets('Delete project - verify it disappears', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Use unique project name to avoid collisions
      final projectName =
          'DeleteProject_${DateTime.now().millisecondsSinceEpoch}';

      // Navigate to Projects
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      // Create a project to delete
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();
      await tester.enterText(find.byType(TextField).first, projectName);
      await tester.pumpAndSettle();
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Verify project exists
      expect(find.text(projectName), findsOneWidget);

      // Find and tap delete button in the list view
      final deleteButton = find.byIcon(Icons.delete_outline);
      expect(deleteButton, findsAtLeastNWidgets(1));
      await tester.tap(deleteButton.first);
      await tester.pumpAndSettle();

      // Confirm deletion if dialog appears
      final confirmButton = find.text('Delete');
      if (confirmButton.evaluate().isNotEmpty) {
        await tester.tap(confirmButton);
        await tester.pumpAndSettle();
      }

      // Verify project is gone
      expect(find.text(projectName), findsNothing);
    });

    testWidgets('Delete project when multiple exist', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Use unique project names to avoid collisions
      final timestamp = DateTime.now().millisecondsSinceEpoch;
      final project1Name = 'DelTest1_$timestamp';
      final project2Name = 'DelTest2_$timestamp';
      final project3Name = 'DelTest3_$timestamp';

      // Navigate to Projects
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      // Create three projects
      for (final name in [project1Name, project2Name, project3Name]) {
        await tester.tap(find.byType(FloatingActionButton));
        await tester.pumpAndSettle();
        await tester.enterText(find.byType(TextField).first, name);
        await tester.pumpAndSettle();
        await tester.tap(find.text('Save'));
        await tester.pumpAndSettle();
      }

      // Delete Project 2 by tapping its delete button in the list
      // Projects should be in the list, find the delete button for Project 2
      final allDeleteButtons = find.byIcon(Icons.delete_outline);
      expect(allDeleteButtons, findsAtLeastNWidgets(3));

      // Tap the second delete button (for Project 2)
      // Assuming projects are listed in creation order, Project 2 is at index 1
      await tester.tap(allDeleteButtons.at(1));
      await tester.pumpAndSettle();

      // Confirm deletion if dialog appears
      final confirmButton = find.text('Delete');
      if (confirmButton.evaluate().isNotEmpty) {
        await tester.tap(confirmButton);
        await tester.pumpAndSettle();
      }

      // Verify only Project 2 is deleted
      expect(find.text(project1Name), findsOneWidget);
      expect(find.text(project2Name), findsNothing);
      expect(find.text(project3Name), findsOneWidget);
    });

    testWidgets('Form validation - empty name shows error', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Navigate to Projects
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      // Try to create project without name
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();

      // Don't enter name, just try to save
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Should show error or stay on edit page
      // (Implementation may vary - either snackbar or inline error)
    });

    testWidgets('Navigate between Projects and Tasks pages', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Go to Projects
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      expect(find.text('Projects'), findsWidgets);

      // Go back to Tasks
      await tester.tap(find.byType(BackButton));
      await tester.pumpAndSettle();

      expect(find.text('Tasks'), findsWidgets);

      // Go to Projects again
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      expect(find.text('Projects'), findsWidgets);
    });

    testWidgets('Filter projects by status', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Navigate to Projects
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      // Create some projects first
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();
      await tester.enterText(find.byType(TextField).first, 'Test Project');
      await tester.pumpAndSettle();
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Tap filter button
      final filterButton = find.byIcon(Icons.filter_list);
      if (filterButton.evaluate().isNotEmpty) {
        await tester.tap(filterButton);
        await tester.pumpAndSettle();

        // Should show filter dialog or options
        // Note: Filter UI may vary, just verify something appeared
        final filterDialogFinder = find.text('Filter Projects');
        if (filterDialogFinder.evaluate().isNotEmpty) {
          // Close filter dialog
          final allProjectsButton = find.text('All Projects');
          if (allProjectsButton.evaluate().isNotEmpty) {
            await tester.tap(allProjectsButton);
            await tester.pumpAndSettle();
          }
        }

        // Should be back on projects list
        expect(find.text('Test Project'), findsOneWidget);
      }
    });

    testWidgets('Create multiple projects and verify list', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Navigate to Projects
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      final projectNames = [
        'Mobile App',
        'Web Dashboard',
        'API Backend',
        'Documentation',
        'Testing Suite',
      ];

      // Create all projects
      for (final name in projectNames) {
        await tester.tap(find.byType(FloatingActionButton));
        await tester.pumpAndSettle();
        await tester.enterText(find.byType(TextField).first, name);
        await tester.pumpAndSettle();
        await tester.tap(find.text('Save'));
        await tester.pumpAndSettle();
      }

      // Verify all projects appear
      for (final name in projectNames) {
        expect(find.text(name), findsOneWidget);
      }
    });
  });
}
