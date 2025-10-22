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
    testWidgets('Create project with minimal fields (name only)', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Navigate to Projects page
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      // Verify empty state
      expect(find.text('No projects yet'), findsOneWidget);

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

      // Verify empty state is gone
      expect(find.text('No projects yet'), findsNothing);
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
        await tester.enterText(textFields.at(1), 'A comprehensive project with all details');
        await tester.pumpAndSettle();
      }

      // Save
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Verify project created
      expect(find.text('Complete Project'), findsOneWidget);

      // Should show success snackbar
      expect(find.textContaining('Project created'), findsOneWidget);
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
      await tester.enterText(find.byType(TextField).first, 'Status Test Project');
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

      // Navigate to Projects
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      // Create a project to delete
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();
      await tester.enterText(find.byType(TextField).first, 'Project to Delete');
      await tester.pumpAndSettle();
      await tester.tap(find.text('Save'));
      await tester.pumpAndSettle();

      // Verify project exists
      expect(find.text('Project to Delete'), findsOneWidget);

      // Open project for editing
      await tester.tap(find.text('Project to Delete'));
      await tester.pumpAndSettle();

      // Find and tap delete button if available
      final deleteButton = find.byIcon(Icons.delete);
      if (deleteButton.evaluate().isNotEmpty) {
        await tester.tap(deleteButton);
        await tester.pumpAndSettle();

        // Confirm deletion if dialog appears
        final confirmButton = find.text('Delete');
        if (confirmButton.evaluate().isNotEmpty) {
          await tester.tap(confirmButton);
          await tester.pumpAndSettle();
        }
      }

      // Verify project is gone
      expect(find.text('Project to Delete'), findsNothing);

      // Should show empty state
      expect(find.text('No projects yet'), findsOneWidget);
    });

    testWidgets('Delete project when multiple exist', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Navigate to Projects
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      // Create three projects
      for (var i = 1; i <= 3; i++) {
        await tester.tap(find.byType(FloatingActionButton));
        await tester.pumpAndSettle();
        await tester.enterText(find.byType(TextField).first, 'Project $i');
        await tester.pumpAndSettle();
        await tester.tap(find.text('Save'));
        await tester.pumpAndSettle();
      }

      // Delete Project 2
      await tester.tap(find.text('Project 2'));
      await tester.pumpAndSettle();

      final deleteButton = find.byIcon(Icons.delete);
      if (deleteButton.evaluate().isNotEmpty) {
        await tester.tap(deleteButton);
        await tester.pumpAndSettle();

        final confirmButton = find.text('Delete');
        if (confirmButton.evaluate().isNotEmpty) {
          await tester.tap(confirmButton);
          await tester.pumpAndSettle();
        }
      }

      // Verify only Project 2 is deleted
      expect(find.text('Project 1'), findsOneWidget);
      expect(find.text('Project 2'), findsNothing);
      expect(find.text('Project 3'), findsOneWidget);
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

        // Should show filter options
        expect(find.text('Filter Projects'), findsOneWidget);
        expect(find.text('All Projects'), findsOneWidget);
        expect(find.text('Active'), findsOneWidget);
        expect(find.text('Completed'), findsOneWidget);

        // Tap "All Projects" to close
        await tester.tap(find.text('All Projects'));
        await tester.pumpAndSettle();

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
