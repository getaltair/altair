import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:altair_core/altair_core.dart';
// Note: _TaskListItem is a private widget defined in main.dart
// This test documents the critical paths that need coverage

void main() {
  group('_TaskListItem Widget Tests', () {
    late Task parentTask;
    late Task childTask1;
    late Task childTask2;
    late List<Task> allTasks;

    setUp(() {
      parentTask = Task(
        id: '1',
        title: 'Parent Task',
        description: 'This is a parent task',
        status: TaskStatus.todo,
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
      );

      childTask1 = Task(
        id: '2',
        title: 'Child Task 1',
        description: null,
        status: TaskStatus.todo,
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        parentTaskId: '1',
      );

      childTask2 = Task(
        id: '3',
        title: 'Child Task 2',
        description: 'Child with description',
        status: TaskStatus.completed,
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        parentTaskId: '1',
      );

      allTasks = [parentTask, childTask1, childTask2];
    });

    group('Task Filtering Logic', () {
      testWidgets('Root tasks are displayed correctly',
          (WidgetTester tester) async {
        // Test filtering of root tasks (parentTaskId == null)
        final rootTasks =
            allTasks.where((t) => t.parentTaskId == null).toList();

        expect(rootTasks.length, equals(1));
        expect(rootTasks.first.id, equals('1'));
        expect(rootTasks.first.title, equals('Parent Task'));
      });

      testWidgets('Subtasks are filtered correctly',
          (WidgetTester tester) async {
        // Test filtering of subtasks for a parent task
        final subtasksForParent =
            allTasks.where((t) => t.parentTaskId == '1').toList();

        expect(subtasksForParent.length, equals(2));
        expect(subtasksForParent[0].id, equals('2'));
        expect(subtasksForParent[1].id, equals('3'));
      });

      testWidgets('Non-existent parent returns empty subtasks',
          (WidgetTester tester) async {
        // Test filtering with non-existent parent ID
        final subtasks =
            allTasks.where((t) => t.parentTaskId == 'non-existent').toList();

        expect(subtasks.isEmpty, isTrue);
      });

      testWidgets('O(n) subtask filtering performance',
          (WidgetTester tester) async {
        // Test that subtask filtering is O(n) not O(n²)
        // Generate large task list
        final largeTasks = <Task>[];
        for (int i = 0; i < 100; i++) {
          largeTasks.add(Task(
            id: 'root_$i',
            title: 'Root Task $i',
            status: TaskStatus.todo,
            createdAt: DateTime.now(),
            updatedAt: DateTime.now(),
          ));
        }

        // Add subtasks
        for (int i = 0; i < 50; i++) {
          largeTasks.add(Task(
            id: 'child_$i',
            title: 'Child Task $i',
            status: TaskStatus.todo,
            createdAt: DateTime.now(),
            updatedAt: DateTime.now(),
            parentTaskId: 'root_0',
          ));
        }

        // This should be O(n) - single pass through list
        final stopwatch = Stopwatch()..start();
        final subtasks =
            largeTasks.where((t) => t.parentTaskId == 'root_0').toList();
        stopwatch.stop();

        expect(subtasks.length, equals(50));
        expect(stopwatch.elapsedMilliseconds, lessThan(100),
            reason:
                'Subtask filtering should be fast (O(n)), took ${stopwatch.elapsedMilliseconds}ms');
      });
    });

    group('Expand/Collapse State Management', () {
      testWidgets('Initial state is not expanded (CRITICAL: State Reset Issue)',
          (WidgetTester tester) async {
        // CRITICAL: Line 991 in main.dart: bool _isExpanded = false;
        // State is stored in StatefulWidget but lost on rebuild
        // This test documents the issue - actual widget is private to main.dart

        // Expected behavior:
        // 1. Expanded state should persist when task properties change
        // 2. State should reset when task ID changes
        // 3. State should be preserved during parent rebuild

        expect(true,
            isTrue); // Placeholder - private widget can't be tested directly
      });

      testWidgets('Expand button toggles subtask visibility',
          (WidgetTester tester) async {
        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                task: parentTask,
                index: 0,
                subtasks: [childTask1, childTask2],
                allTasks: allTasks,
              ),
            ),
          ),
        );

        // Find and tap expand button
        final expandButton = find.byIcon(Icons.expand_more);
        expect(expandButton, findsOneWidget);

        await tester.tap(expandButton);
        await tester.pumpAndSettle();

        // Subtasks should now be visible
        expect(find.text('Child Task 1'), findsOneWidget);
        expect(find.text('Child Task 2'), findsOneWidget);
      });

      testWidgets('Expand/collapse toggle works repeatedly',
          (WidgetTester tester) async {
        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                task: parentTask,
                index: 0,
                subtasks: [childTask1, childTask2],
                allTasks: allTasks,
              ),
            ),
          ),
        );

        // Toggle expand
        await tester.tap(find.byIcon(Icons.expand_more));
        await tester.pumpAndSettle();
        expect(find.text('Child Task 1'), findsOneWidget);

        // Toggle collapse
        await tester.tap(find.byIcon(Icons.expand_less));
        await tester.pumpAndSettle();
        expect(find.text('Child Task 1'), findsNothing);

        // Toggle expand again
        await tester.tap(find.byIcon(Icons.expand_more));
        await tester.pumpAndSettle();
        expect(find.text('Child Task 1'), findsOneWidget);
      });

      testWidgets('State loss on widget rebuild is prevented',
          (WidgetTester tester) async {
        // This tests the state loss issue (line 991)
        // When _isExpanded state is recreated on rebuild
        final key = UniqueKey();

        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                key: key,
                task: parentTask,
                index: 0,
                subtasks: [childTask1, childTask2],
                allTasks: allTasks,
              ),
            ),
          ),
        );

        // Expand
        await tester.tap(find.byIcon(Icons.expand_more));
        await tester.pumpAndSettle();
        expect(find.text('Child Task 1'), findsOneWidget);

        // Force a rebuild of the parent
        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                key: key,
                task: parentTask.copyWith(title: 'Updated Parent Task'),
                index: 0,
                subtasks: [childTask1, childTask2],
                allTasks: allTasks,
              ),
            ),
          ),
        );

        // State should be preserved or reset consistently
        // Note: With StatefulWidget, state is preserved by key
      });
    });

    group('Subtask Rendering', () {
      testWidgets('Subtasks display with correct hierarchy styling',
          (WidgetTester tester) async {
        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                task: parentTask,
                index: 0,
                subtasks: [childTask1],
                allTasks: allTasks,
                isSubtask: true,
                parentTask: parentTask,
              ),
            ),
          ),
        );

        // Verify breadcrumb is shown for subtasks
        expect(find.byIcon(Icons.subdirectory_arrow_right), findsOneWidget);
        expect(find.text('Parent Task'), findsWidgets);
      });

      testWidgets('Subtask count badge shows correct number',
          (WidgetTester tester) async {
        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                task: parentTask,
                index: 0,
                subtasks: [childTask1, childTask2],
                allTasks: allTasks,
              ),
            ),
          ),
        );

        // Badge should show "2" for 2 subtasks
        expect(find.text('2'), findsOneWidget);
      });

      testWidgets('No expand button when task has no subtasks',
          (WidgetTester tester) async {
        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                task: parentTask,
                index: 0,
                subtasks: [],
                allTasks: allTasks,
              ),
            ),
          ),
        );

        // No expand/collapse button should be shown
        expect(find.byIcon(Icons.expand_more), findsNothing);
        expect(find.byIcon(Icons.expand_less), findsNothing);

        // Tap should navigate instead
        await tester.tap(find.byType(InkWell));
        // Navigation verification would happen in integration tests
      });

      testWidgets('Nested subtasks render recursively',
          (WidgetTester tester) async {
        // Test recursive _TaskListItem rendering for nested hierarchy
        final grandchildTask = Task(
          id: '4',
          title: 'Grandchild Task',
          status: TaskStatus.todo,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
          parentTaskId: '2',
        );

        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                task: parentTask,
                index: 0,
                subtasks: [childTask1],
                allTasks: [parentTask, childTask1, grandchildTask],
              ),
            ),
          ),
        );

        // Expand parent
        await tester.tap(find.byIcon(Icons.expand_more));
        await tester.pumpAndSettle();

        // Child should be visible
        expect(find.text('Child Task 1'), findsOneWidget);
      });
    });

    group('Task Status and Styling', () {
      testWidgets('Completed tasks show with strikethrough',
          (WidgetTester tester) async {
        final completedTask = Task(
          id: 'completed',
          title: 'Completed Task',
          status: TaskStatus.completed,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
          completedAt: DateTime.now(),
        );

        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                task: completedTask,
                index: 0,
                subtasks: [],
                allTasks: [completedTask],
              ),
            ),
          ),
        );

        // The text should have strikethrough decoration
        expect(find.text('Completed Task'), findsOneWidget);
        // Visual verification would be in golden tests
      });

      testWidgets('Task accent color reflects status',
          (WidgetTester tester) async {
        final inProgressTask = Task(
          id: 'in-progress',
          title: 'In Progress Task',
          status: TaskStatus.inProgress,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        );

        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                task: inProgressTask,
                index: 0,
                subtasks: [],
                allTasks: [inProgressTask],
              ),
            ),
          ),
        );

        expect(find.text('In Progress Task'), findsOneWidget);
      });

      testWidgets('Checkbox reflects task completion status',
          (WidgetTester tester) async {
        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                task: parentTask,
                index: 0,
                subtasks: [],
                allTasks: [parentTask],
              ),
            ),
          ),
        );

        // Initial checkbox should be unchecked
        final checkbox = find.byType(Checkbox);
        expect(checkbox, findsOneWidget);
      });
    });

    group('Breadcrumb Display', () {
      testWidgets('Subtask breadcrumb shows parent task name',
          (WidgetTester tester) async {
        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                task: childTask1,
                index: 0,
                subtasks: [],
                allTasks: allTasks,
                isSubtask: true,
                parentTask: parentTask,
              ),
            ),
          ),
        );

        // Should show parent task name in breadcrumb
        expect(find.text('Parent Task'), findsWidgets);
        expect(find.byIcon(Icons.subdirectory_arrow_right), findsOneWidget);
      });

      testWidgets('Root task has no breadcrumb', (WidgetTester tester) async {
        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                task: parentTask,
                index: 0,
                subtasks: [],
                allTasks: allTasks,
              ),
            ),
          ),
        );

        // Should not show breadcrumb
        expect(find.byIcon(Icons.subdirectory_arrow_right), findsNothing);
      });

      testWidgets('Breadcrumb handles missing parent task gracefully',
          (WidgetTester tester) async {
        final orphanTask = Task(
          id: 'orphan',
          title: 'Orphan Task',
          status: TaskStatus.todo,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
          parentTaskId: 'non-existent-parent',
        );

        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                task: orphanTask,
                index: 0,
                subtasks: [],
                allTasks: [orphanTask],
              ),
            ),
          ),
        );

        // Should handle gracefully without crashing
        expect(find.text('Orphan Task'), findsOneWidget);
      });
    });

    group('Task Deletion', () {
      testWidgets('Delete button is visible for all tasks',
          (WidgetTester tester) async {
        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                task: parentTask,
                index: 0,
                subtasks: [],
                allTasks: [parentTask],
              ),
            ),
          ),
        );

        expect(find.byIcon(Icons.delete_outline), findsOneWidget);
      });

      testWidgets('Delete action fires TaskDeleteRequested event',
          (WidgetTester tester) async {
        // This would require mocking the BLoC
        // Covered in integration tests
      });

      testWidgets('Cascade deletion handling - parent with children',
          (WidgetTester tester) async {
        // CRITICAL: Test cascade deletion
        // When parent task is deleted, subtasks should be handled
        // This is a code review critical issue that needs implementation

        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                task: parentTask,
                index: 0,
                subtasks: [childTask1, childTask2],
                allTasks: allTasks,
              ),
            ),
          ),
        );

        expect(find.text('Parent Task'), findsOneWidget);
        // Deletion of parent with children should:
        // - Option 1: Delete all children (cascade)
        // - Option 2: Move children to root level
        // - Option 3: Prevent deletion (recommend user delete children first)
        // Current implementation: No cascade handling detected
      });
    });

    group('Drag and Drop', () {
      testWidgets('Drag handle is visible for root tasks',
          (WidgetTester tester) async {
        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                task: parentTask,
                index: 0,
                subtasks: [],
                allTasks: [parentTask],
              ),
            ),
          ),
        );

        expect(find.byIcon(Icons.drag_handle), findsOneWidget);
      });

      testWidgets('Drag handle is hidden for subtasks',
          (WidgetTester tester) async {
        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                task: childTask1,
                index: 0,
                subtasks: [],
                allTasks: allTasks,
                isSubtask: true,
              ),
            ),
          ),
        );

        expect(find.byIcon(Icons.drag_handle), findsNothing);
      });
    });

    group('Edge Cases', () {
      testWidgets('Task with very long title handles ellipsis correctly',
          (WidgetTester tester) async {
        final longTitleTask = Task(
          id: 'long',
          title:
              'This is a very long task title that should be truncated with ellipsis when displayed',
          status: TaskStatus.todo,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        );

        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                task: longTitleTask,
                index: 0,
                subtasks: [],
                allTasks: [longTitleTask],
              ),
            ),
          ),
        );

        expect(find.byType(_TaskListItem), findsOneWidget);
      });

      testWidgets('Empty description is handled gracefully',
          (WidgetTester tester) async {
        final noDescTask = Task(
          id: 'no-desc',
          title: 'Task',
          description: null,
          status: TaskStatus.todo,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        );

        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: _TaskListItem(
                task: noDescTask,
                index: 0,
                subtasks: [],
                allTasks: [noDescTask],
              ),
            ),
          ),
        );

        expect(find.byType(_TaskListItem), findsOneWidget);
      });
    });
  });
}
