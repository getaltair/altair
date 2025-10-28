import 'package:altair_core/models/task.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('Task', () {
    final now = DateTime.now();
    final testTask = Task(
      id: 'test-id',
      title: 'Test Task',
      description: 'Test Description',
      status: TaskStatus.todo,
      tags: ['urgent', 'work'],
      createdAt: now,
      updatedAt: now,
      priority: 1,
    );

    test('creates task with required fields', () {
      expect(testTask.id, 'test-id');
      expect(testTask.title, 'Test Task');
      expect(testTask.description, 'Test Description');
      expect(testTask.status, TaskStatus.todo);
      expect(testTask.tags, ['urgent', 'work']);
      expect(testTask.priority, 1);
    });

    test('has default values for optional fields', () {
      final minimal = Task(
        id: 'minimal',
        title: 'Minimal Task',
        createdAt: now,
        updatedAt: now,
      );

      expect(minimal.status, TaskStatus.todo);
      expect(minimal.tags, isEmpty);
      expect(minimal.priority, 3);
      expect(minimal.description, isNull);
      expect(minimal.projectId, isNull);
      expect(minimal.parentTaskId, isNull);
      expect(minimal.completedAt, isNull);
      expect(minimal.estimatedMinutes, isNull);
      expect(minimal.actualMinutes, isNull);
      expect(minimal.metadata, isNull);
    });

    test('copyWith creates new task with updated fields', () {
      final updated = testTask.copyWith(
        title: 'Updated Title',
        status: TaskStatus.inProgress,
        priority: 2,
      );

      expect(updated.id, testTask.id);
      expect(updated.title, 'Updated Title');
      expect(updated.status, TaskStatus.inProgress);
      expect(updated.priority, 2);
      expect(updated.description, testTask.description);
      expect(updated.tags, testTask.tags);
    });

    test('copyWith preserves original when no fields provided', () {
      final copied = testTask.copyWith();

      expect(copied.id, testTask.id);
      expect(copied.title, testTask.title);
      expect(copied.status, testTask.status);
      expect(copied.description, testTask.description);
    });

    test('equality is based on id', () {
      final task1 = Task(
        id: 'same-id',
        title: 'Task 1',
        createdAt: now,
        updatedAt: now,
      );

      final task2 = Task(
        id: 'same-id',
        title: 'Task 2',
        createdAt: now,
        updatedAt: now,
      );

      final task3 = Task(
        id: 'different-id',
        title: 'Task 1',
        createdAt: now,
        updatedAt: now,
      );

      expect(task1, equals(task2));
      expect(task1, isNot(equals(task3)));
      expect(task1.hashCode, equals(task2.hashCode));
    });

    test('toString returns readable format', () {
      final str = testTask.toString();
      expect(str, contains('test-id'));
      expect(str, contains('Test Task'));
      expect(str, contains('TaskStatus.todo'));
    });

    test('handles all task statuses', () {
      expect(TaskStatus.values.length, 4);
      expect(TaskStatus.values, contains(TaskStatus.todo));
      expect(TaskStatus.values, contains(TaskStatus.inProgress));
      expect(TaskStatus.values, contains(TaskStatus.completed));
      expect(TaskStatus.values, contains(TaskStatus.cancelled));
    });

    test('validates priority range in practice', () {
      final highPriority = Task(
        id: 'high',
        title: 'High Priority',
        priority: 1,
        createdAt: now,
        updatedAt: now,
      );

      final lowPriority = Task(
        id: 'low',
        title: 'Low Priority',
        priority: 5,
        createdAt: now,
        updatedAt: now,
      );

      expect(highPriority.priority, 1);
      expect(lowPriority.priority, 5);
    });

    test('handles parent-child task relationships', () {
      final parent = Task(
        id: 'parent-id',
        title: 'Parent Task',
        createdAt: now,
        updatedAt: now,
      );

      final child = Task(
        id: 'child-id',
        title: 'Child Task',
        parentTaskId: parent.id,
        createdAt: now,
        updatedAt: now,
      );

      expect(child.parentTaskId, parent.id);
      expect(parent.parentTaskId, isNull);
    });

    test('handles project association', () {
      final task = Task(
        id: 'task-id',
        title: 'Project Task',
        projectId: 'project-123',
        createdAt: now,
        updatedAt: now,
      );

      expect(task.projectId, 'project-123');
    });

    test('handles completion tracking', () {
      final completedTime = DateTime.now();
      final completed = testTask.copyWith(
        status: TaskStatus.completed,
        completedAt: completedTime,
      );

      expect(completed.status, TaskStatus.completed);
      expect(completed.completedAt, completedTime);
    });

    test('handles time estimates and actuals', () {
      final timedTask = testTask.copyWith(
        estimatedMinutes: 60,
        actualMinutes: 75,
      );

      expect(timedTask.estimatedMinutes, 60);
      expect(timedTask.actualMinutes, 75);
    });

    test('handles metadata as flexible field', () {
      final metadata = {'color': 'blue', 'icon': 'star', 'customField': 123};

      final taskWithMeta = testTask.copyWith(metadata: metadata);

      expect(taskWithMeta.metadata, metadata);
      expect(taskWithMeta.metadata!['color'], 'blue');
      expect(taskWithMeta.metadata!['customField'], 123);
    });
  });
}
