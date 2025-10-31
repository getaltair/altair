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
  });
}
