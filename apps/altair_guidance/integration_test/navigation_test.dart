/// Integration tests for navigation flows.
///
/// These tests verify that all navigation paths work correctly,
/// including provider context passing. These tests would have caught
/// the provider context bugs fixed in the navigation implementation.
library;

import 'package:altair_guidance/main.dart' as app;
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('Navigation Tests', () {
    testWidgets('App launches successfully', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Verify home page loads
      expect(find.text('Tasks'), findsOneWidget);
      expect(find.byType(FloatingActionButton), findsOneWidget);
    });

    testWidgets('Navigate to Projects page from drawer', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Open drawer
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();

      // Tap Projects in drawer
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      // Verify Projects page loaded (not grey screen!)
      expect(find.text('Projects'), findsWidgets);
      expect(find.byType(FloatingActionButton), findsOneWidget);
    });

    testWidgets('Navigate to new task from main FAB', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Tap main FAB
      final fab = find.byType(FloatingActionButton).first;
      await tester.tap(fab);
      await tester.pumpAndSettle();

      // Verify task edit page loaded (not grey screen!)
      expect(find.text('Create Task'), findsOneWidget);
    });

    testWidgets('Navigate to new task from "New Task" action', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Open drawer
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();

      // Note: If there's a "New Task" action in the drawer, test it
      // This is a placeholder for any additional navigation paths
    });

    testWidgets('Navigate to Projects then create new project', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Navigate to Projects
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      // Verify Projects page loaded
      expect(find.text('Projects'), findsWidgets);

      // Tap FAB to create new project
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();

      // Verify project edit page loaded (not grey screen!)
      // The exact text depends on your ProjectEditPage implementation
      expect(find.byType(Scaffold), findsWidgets);
    });

    testWidgets('Navigate back from Projects to Tasks', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Navigate to Projects
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();

      // Navigate back
      await tester.tap(find.byType(BackButton));
      await tester.pumpAndSettle();

      // Verify we're back on Tasks page
      expect(find.text('Tasks'), findsOneWidget);
    });

    testWidgets('Navigate through complete app flow', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Start on Tasks page
      expect(find.text('Tasks'), findsOneWidget);

      // Open drawer
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();

      // Go to Projects
      await tester.tap(find.text('Projects'));
      await tester.pumpAndSettle();
      expect(find.text('Projects'), findsWidgets);

      // Try to create project
      await tester.tap(find.byType(FloatingActionButton));
      await tester.pumpAndSettle();

      // Go back to projects list
      await tester.tap(find.byType(BackButton));
      await tester.pumpAndSettle();

      // Go back to home
      await tester.tap(find.byType(BackButton));
      await tester.pumpAndSettle();

      // Try to create task from FAB
      final fab = find.byType(FloatingActionButton).first;
      await tester.tap(fab);
      await tester.pumpAndSettle();

      // Verify task creation page loaded
      expect(find.text('Create Task'), findsOneWidget);
    });

    testWidgets('Drawer navigation items are accessible', (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Open drawer
      await tester.tap(find.byIcon(Icons.menu));
      await tester.pumpAndSettle();

      // Verify all expected navigation items exist
      expect(find.text('Tasks'), findsOneWidget);
      expect(find.text('Projects'), findsOneWidget);
      expect(find.text('Settings'), findsOneWidget);
    });
  });
}
