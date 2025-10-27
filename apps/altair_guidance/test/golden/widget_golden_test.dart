/// Golden file tests for visual regression testing.
///
/// These tests capture screenshots of widgets and compare them
/// against baseline images to detect unintended visual changes.
library;

import 'package:altair_core/altair_core.dart';
import 'package:altair_guidance/bloc/task/task_bloc.dart';
import 'package:altair_guidance/bloc/task/task_state.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('Widget Golden Tests', () {
    testWidgets('Task list empty state', (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: BlocProvider(
              create: (_) =>
                  TaskBloc(taskRepository: TaskRepositorySurrealDB()),
              child: BlocBuilder<TaskBloc, TaskState>(
                builder: (context, state) {
                  if (state is TaskLoaded && state.tasks.isEmpty) {
                    return const Center(
                      child: Text('No tasks yet'),
                    );
                  }
                  return const SizedBox();
                },
              ),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Capture golden file
      await expectLater(
        find.byType(Scaffold),
        matchesGoldenFile('goldens/task_list_empty.png'),
      );
    });

    testWidgets('Task card appearance', (tester) async {
      final testTask = Task(
        id: '1',
        title: 'Sample Task',
        description: 'This is a sample task for golden testing',
        status: TaskStatus.todo,
        tags: const [],
        createdAt: DateTime(2025, 1, 1),
        updatedAt: DateTime(2025, 1, 1),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Card(
              child: ListTile(
                leading: Checkbox(
                  value: testTask.status == TaskStatus.completed,
                  onChanged: (_) {},
                ),
                title: Text(testTask.title),
                subtitle: Text(testTask.description ?? ''),
              ),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Capture golden file for task card
      await expectLater(
        find.byType(Card),
        matchesGoldenFile('goldens/task_card.png'),
      );
    });

    testWidgets('Completed task appearance', (tester) async {
      final testTask = Task(
        id: '2',
        title: 'Completed Task',
        description: 'This task is completed',
        status: TaskStatus.completed,
        tags: const [],
        createdAt: DateTime(2025, 1, 1),
        updatedAt: DateTime(2025, 1, 2),
        completedAt: DateTime(2025, 1, 2),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Card(
              child: ListTile(
                leading: Checkbox(
                  value: testTask.status == TaskStatus.completed,
                  onChanged: (_) {},
                ),
                title: Text(
                  testTask.title,
                  style: const TextStyle(
                    decoration: TextDecoration.lineThrough,
                  ),
                ),
                subtitle: Text(testTask.description ?? ''),
              ),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Capture golden file for completed task
      await expectLater(
        find.byType(Card),
        matchesGoldenFile('goldens/task_card_completed.png'),
      );
    });

    testWidgets('Priority indicators render correctly', (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Column(
              children: [
                Chip(
                  label: const Text('High Priority'),
                  backgroundColor: Colors.red.shade100,
                ),
                Chip(
                  label: const Text('Medium Priority'),
                  backgroundColor: Colors.orange.shade100,
                ),
                Chip(
                  label: const Text('Low Priority'),
                  backgroundColor: Colors.green.shade100,
                ),
              ],
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Capture golden file for priority chips
      await expectLater(
        find.byType(Column),
        matchesGoldenFile('goldens/priority_chips.png'),
      );
    });

    testWidgets('FAB renders correctly', (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            floatingActionButton: FloatingActionButton(
              onPressed: () {},
              child: const Icon(Icons.add),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Capture golden file for FAB
      await expectLater(
        find.byType(FloatingActionButton),
        matchesGoldenFile('goldens/fab.png'),
      );
    });
  });
}
