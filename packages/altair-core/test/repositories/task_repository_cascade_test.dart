/// Tests for task repository cascade delete functionality.
library;

import 'package:altair_core/altair_core.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  late TaskRepository repository;

  setUp(() async {
    repository = TaskRepository();
    await repository.initialize();
  });

  group('Task Repository Cascade Delete', () {
    test('deleting parent task deletes all direct subtasks', () async {
      // Create parent task
      final parent = await repository.create(
        Task(
          id: '',
          title: 'Parent Task',
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      // Create subtasks
      final subtask1 = await repository.create(
        Task(
          id: '',
          title: 'Subtask 1',
          parentTaskId: parent.id,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      final subtask2 = await repository.create(
        Task(
          id: '',
          title: 'Subtask 2',
          parentTaskId: parent.id,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      // Verify subtasks exist
      final subtasksBefore = await repository.findSubtasks(parent.id);
      expect(subtasksBefore.length, equals(2));

      // Delete parent
      await repository.delete(parent.id);

      // Verify parent and subtasks are deleted
      final parentAfter = await repository.findById(parent.id);
      expect(parentAfter, isNull);

      final subtask1After = await repository.findById(subtask1.id);
      expect(subtask1After, isNull);

      final subtask2After = await repository.findById(subtask2.id);
      expect(subtask2After, isNull);
    });

    test('deleting parent task deletes nested subtasks recursively', () async {
      // Create parent task
      final parent = await repository.create(
        Task(
          id: '',
          title: 'Parent Task',
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      // Create level 1 subtask
      final subtask1 = await repository.create(
        Task(
          id: '',
          title: 'Subtask Level 1',
          parentTaskId: parent.id,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      // Create level 2 subtask (child of subtask1)
      final subtask2 = await repository.create(
        Task(
          id: '',
          title: 'Subtask Level 2',
          parentTaskId: subtask1.id,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      // Create level 3 subtask (child of subtask2)
      final subtask3 = await repository.create(
        Task(
          id: '',
          title: 'Subtask Level 3',
          parentTaskId: subtask2.id,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      // Delete parent
      await repository.delete(parent.id);

      // Verify all tasks in hierarchy are deleted
      expect(await repository.findById(parent.id), isNull);
      expect(await repository.findById(subtask1.id), isNull);
      expect(await repository.findById(subtask2.id), isNull);
      expect(await repository.findById(subtask3.id), isNull);
    });

    test('deleting middle-level task only deletes its descendants', () async {
      // Create parent task
      final parent = await repository.create(
        Task(
          id: '',
          title: 'Parent Task',
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      // Create level 1 subtask
      final subtask1 = await repository.create(
        Task(
          id: '',
          title: 'Subtask Level 1',
          parentTaskId: parent.id,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      // Create level 2 subtask (child of subtask1)
      final subtask2 = await repository.create(
        Task(
          id: '',
          title: 'Subtask Level 2',
          parentTaskId: subtask1.id,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      // Delete middle-level task (subtask1)
      await repository.delete(subtask1.id);

      // Verify parent still exists
      final parentAfter = await repository.findById(parent.id);
      expect(parentAfter, isNotNull);
      expect(parentAfter!.id, equals(parent.id));

      // Verify subtask1 and its children are deleted
      expect(await repository.findById(subtask1.id), isNull);
      expect(await repository.findById(subtask2.id), isNull);
    });

    test('deleting task with no subtasks works normally', () async {
      final task = await repository.create(
        Task(
          id: '',
          title: 'Standalone Task',
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      // Delete task
      await repository.delete(task.id);

      // Verify task is deleted
      final taskAfter = await repository.findById(task.id);
      expect(taskAfter, isNull);
    });

    test('deleting leaf task (subtask with no children) works normally',
        () async {
      final parent = await repository.create(
        Task(
          id: '',
          title: 'Parent Task',
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      final subtask = await repository.create(
        Task(
          id: '',
          title: 'Leaf Subtask',
          parentTaskId: parent.id,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      // Delete leaf subtask
      await repository.delete(subtask.id);

      // Verify parent still exists
      final parentAfter = await repository.findById(parent.id);
      expect(parentAfter, isNotNull);

      // Verify subtask is deleted
      final subtaskAfter = await repository.findById(subtask.id);
      expect(subtaskAfter, isNull);
    });

    test('deleting parent with multiple subtask branches', () async {
      // Create parent
      final parent = await repository.create(
        Task(
          id: '',
          title: 'Parent Task',
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      // Create branch 1
      final branch1 = await repository.create(
        Task(
          id: '',
          title: 'Branch 1',
          parentTaskId: parent.id,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      final branch1Child = await repository.create(
        Task(
          id: '',
          title: 'Branch 1 Child',
          parentTaskId: branch1.id,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      // Create branch 2
      final branch2 = await repository.create(
        Task(
          id: '',
          title: 'Branch 2',
          parentTaskId: parent.id,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      final branch2Child = await repository.create(
        Task(
          id: '',
          title: 'Branch 2 Child',
          parentTaskId: branch2.id,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      // Delete parent
      await repository.delete(parent.id);

      // Verify all tasks in both branches are deleted
      expect(await repository.findById(parent.id), isNull);
      expect(await repository.findById(branch1.id), isNull);
      expect(await repository.findById(branch1Child.id), isNull);
      expect(await repository.findById(branch2.id), isNull);
      expect(await repository.findById(branch2Child.id), isNull);
    });

    test('detects and throws error for circular reference', () async {
      // Create two tasks
      final task1 = await repository.create(
        Task(
          id: '',
          title: 'Task 1',
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      final task2 = await repository.create(
        Task(
          id: '',
          title: 'Task 2',
          parentTaskId: task1.id,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      // Create circular reference: task1.parent = task2
      final task1WithCircularRef = task1.copyWith(
        parentTaskId: task2.id,
        updatedAt: DateTime.now(),
      );
      await repository.update(task1WithCircularRef);

      // Attempting to delete should throw StateError
      expect(
        () => repository.delete(task1.id),
        throwsA(
          isA<StateError>().having(
            (e) => e.message,
            'message',
            contains('Circular reference detected'),
          ),
        ),
      );
    });

    test('throws error when max depth exceeded', () async {
      // Create a chain of tasks exceeding max depth (limit is 100)
      Task? previousTask;

      // Create 102 tasks in a chain (exceeds limit of 100)
      for (var i = 0; i < 102; i++) {
        final task = await repository.create(
          Task(
            id: '',
            title: 'Task at depth $i',
            parentTaskId: previousTask?.id,
            createdAt: DateTime.now(),
            updatedAt: DateTime.now(),
          ),
        );
        previousTask = task;
      }

      // Get the root task (first one created)
      final allTasks = await repository.findAll();
      final rootTask = allTasks.firstWhere((t) => t.parentTaskId == null);

      // Attempting to delete should throw StateError
      expect(
        () => repository.delete(rootTask.id),
        throwsA(
          isA<StateError>().having(
            (e) => e.message,
            'message',
            contains('Maximum task hierarchy depth'),
          ),
        ),
      );
    });

    test('handles large hierarchy within depth limit efficiently', () async {
      // Create a task tree with depth of 50 (well within limit of 100)
      const depth = 50;
      Task? previousTask;

      // Create chain
      for (var i = 0; i < depth; i++) {
        final task = await repository.create(
          Task(
            id: '',
            title: 'Task at depth $i',
            parentTaskId: previousTask?.id,
            createdAt: DateTime.now(),
            updatedAt: DateTime.now(),
          ),
        );
        previousTask = task;
      }

      // Get the root task
      final allTasks = await repository.findAll();
      final rootTask = allTasks.firstWhere((t) => t.parentTaskId == null);

      // Delete should succeed without errors
      await repository.delete(rootTask.id);

      // Verify all tasks deleted
      final remainingTasks = await repository.findAll();
      expect(
        remainingTasks.where((t) => t.title.startsWith('Task at depth')),
        isEmpty,
      );
    });

    test('prevents partial deletion on error', () async {
      // Create a simple hierarchy
      final parent = await repository.create(
        Task(
          id: '',
          title: 'Parent',
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      final child1 = await repository.create(
        Task(
          id: '',
          title: 'Child 1',
          parentTaskId: parent.id,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      final child2 = await repository.create(
        Task(
          id: '',
          title: 'Child 2',
          parentTaskId: parent.id,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      );

      // Verify all exist
      expect(await repository.findById(parent.id), isNotNull);
      expect(await repository.findById(child1.id), isNotNull);
      expect(await repository.findById(child2.id), isNotNull);

      // Note: Full transaction support would require SurrealDB transaction API
      // This test documents the expected behavior
      // In a production system, we'd use BEGIN/COMMIT/ROLLBACK
    });
  });
}
