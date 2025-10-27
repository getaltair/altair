import 'package:altair_core/database/database.dart';
import 'package:altair_core/models/project.dart';
import 'package:altair_core/models/task.dart';
import 'package:altair_core/repositories/project_repository.dart';
import 'package:altair_core/repositories/task_repository.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  // Initialize database for testing
  setUpAll(() {
    AltairDatabase.enableTestMode();
  });

  group('TaskRepository', () {
    late TaskRepository taskRepository;

    setUp(() {
      taskRepository = TaskRepository();
    });

    tearDown(() async {
      // Reset database between tests for isolation
      await AltairDatabase().close();
      AltairDatabase.reset();
      AltairDatabase.enableTestMode();
    });

    group('CRUD Operations', () {
      test('create() should create a new task', () async {
        final now = DateTime.now();
        final task = Task(
          id: '',
          title: 'Test Task',
          description: 'A test task',
          status: TaskStatus.todo,
          tags: ['test', 'sample'],
          priority: 1,
          createdAt: now,
          updatedAt: now,
        );

        final createdTask = await taskRepository.create(task);

        expect(createdTask.id, isNotEmpty);
        expect(createdTask.title, 'Test Task');
        expect(createdTask.description, 'A test task');
        expect(createdTask.status, TaskStatus.todo);
        expect(createdTask.tags, ['test', 'sample']);
        expect(createdTask.priority, 1);
      });

      test('create() generates UUID if id is empty', () async {
        final now = DateTime.now();
        final task = Task(
          id: '',
          title: 'Auto ID Task',
          createdAt: now,
          updatedAt: now,
        );

        final createdTask = await taskRepository.create(task);

        expect(createdTask.id, isNotEmpty);
        expect(createdTask.id.length, 36); // UUID length
      });

      test('findById() should return task when it exists', () async {
        final now = DateTime.now();
        final task = Task(
          id: '',
          title: 'Find Me',
          createdAt: now,
          updatedAt: now,
        );

        final created = await taskRepository.create(task);
        final found = await taskRepository.findById(created.id);

        expect(found, isNotNull);
        expect(found!.id, created.id);
        expect(found.title, 'Find Me');
      });

      test('findById() should return null when task does not exist', () async {
        final found = await taskRepository.findById('non-existent-id');

        expect(found, isNull);
      });

      test('findAll() should return all tasks', () async {
        final now = DateTime.now();

        final task1 = Task(
          id: '',
          title: 'Task 1',
          createdAt: now,
          updatedAt: now,
        );

        final task2 = Task(
          id: '',
          title: 'Task 2',
          createdAt: now,
          updatedAt: now,
        );

        await taskRepository.create(task1);
        await taskRepository.create(task2);

        final tasks = await taskRepository.findAll();

        expect(tasks.length, greaterThanOrEqualTo(2));
        expect(tasks.any((t) => t.title == 'Task 1'), isTrue);
        expect(tasks.any((t) => t.title == 'Task 2'), isTrue);
      });

      test('findAll() with status filter should return filtered tasks',
          () async {
        final now = DateTime.now();

        final todoTask = Task(
          id: '',
          title: 'Todo Task',
          status: TaskStatus.todo,
          createdAt: now,
          updatedAt: now,
        );

        final completedTask = Task(
          id: '',
          title: 'Completed Task',
          status: TaskStatus.completed,
          createdAt: now,
          updatedAt: now,
        );

        await taskRepository.create(todoTask);
        await taskRepository.create(completedTask);

        final todoTasks = await taskRepository.findAll(status: TaskStatus.todo);

        expect(todoTasks.every((t) => t.status == TaskStatus.todo), isTrue);
      });

      test('findAll() with projectId filter should return project tasks',
          () async {
        final now = DateTime.now();

        // Create a project first (required for foreign key constraint)
        final projectRepository = ProjectRepository();
        final project = await projectRepository.create(Project(
          id: '',
          name: 'Test Project',
          createdAt: now,
          updatedAt: now,
        ));

        final projectTask = Task(
          id: '',
          title: 'Project Task',
          projectId: project.id,
          createdAt: now,
          updatedAt: now,
        );

        final otherTask = Task(
          id: '',
          title: 'Other Task',
          createdAt: now,
          updatedAt: now,
        );

        await taskRepository.create(projectTask);
        await taskRepository.create(otherTask);

        final projectTasks =
            await taskRepository.findAll(projectId: project.id);

        expect(projectTasks.every((t) => t.projectId == project.id), isTrue);
      });

      test('update() should update existing task', () async {
        final now = DateTime.now();
        final task = Task(
          id: '',
          title: 'Original Title',
          status: TaskStatus.todo,
          createdAt: now,
          updatedAt: now,
        );

        final created = await taskRepository.create(task);
        final updated = created.copyWith(
          title: 'Updated Title',
          status: TaskStatus.completed,
          completedAt: now,
        );

        final result = await taskRepository.update(updated);

        expect(result.title, 'Updated Title');
        expect(result.status, TaskStatus.completed);

        final found = await taskRepository.findById(created.id);
        expect(found!.title, 'Updated Title');
        expect(found.status, TaskStatus.completed);
      });

      test('delete() should remove task from database', () async {
        final now = DateTime.now();
        final task = Task(
          id: '',
          title: 'To Delete',
          createdAt: now,
          updatedAt: now,
        );

        final created = await taskRepository.create(task);
        await taskRepository.delete(created.id);

        final found = await taskRepository.findById(created.id);
        expect(found, isNull);
      });
    });

    group('Search and Query Operations', () {
      test('search() should find tasks by title', () async {
        final now = DateTime.now();
        final task = Task(
          id: '',
          title: 'Unique Search Title',
          createdAt: now,
          updatedAt: now,
        );

        await taskRepository.create(task);
        final results = await taskRepository.search('Unique Search');

        expect(results.isNotEmpty, isTrue);
        expect(results.any((t) => t.title == 'Unique Search Title'), isTrue);
      });

      test('search() should find tasks by description', () async {
        final now = DateTime.now();
        final task = Task(
          id: '',
          title: 'Task with Description',
          description: 'Unique description text',
          createdAt: now,
          updatedAt: now,
        );

        await taskRepository.create(task);
        final results = await taskRepository.search('Unique description');

        expect(results.isNotEmpty, isTrue);
        expect(
          results.any(
              (t) => t.description?.contains('Unique description') ?? false),
          isTrue,
        );
      });

      test('search() should return empty list when no matches', () async {
        final results =
            await taskRepository.search('NonExistentSearchTerm123456');

        expect(results, isEmpty);
      });

      test('findSubtasks() should return child tasks', () async {
        final now = DateTime.now();
        final parentTask = Task(
          id: '',
          title: 'Parent Task',
          createdAt: now,
          updatedAt: now,
        );

        final created = await taskRepository.create(parentTask);

        final childTask1 = Task(
          id: '',
          title: 'Child Task 1',
          parentTaskId: created.id,
          createdAt: now,
          updatedAt: now,
        );

        final childTask2 = Task(
          id: '',
          title: 'Child Task 2',
          parentTaskId: created.id,
          createdAt: now.add(const Duration(seconds: 1)),
          updatedAt: now.add(const Duration(seconds: 1)),
        );

        await taskRepository.create(childTask1);
        await taskRepository.create(childTask2);

        final subtasks = await taskRepository.findSubtasks(created.id);

        expect(subtasks.length, 2);
        expect(subtasks.every((t) => t.parentTaskId == created.id), isTrue);
      });
    });

    group('Edge Cases', () {
      test('handles task with all fields populated', () async {
        final now = DateTime.now();
        final completedAt = now.add(const Duration(hours: 2));

        // Create project first (required for foreign key constraint)
        final projectRepository = ProjectRepository();
        final project = await projectRepository.create(Project(
          id: '',
          name: 'Test Project',
          createdAt: now,
          updatedAt: now,
        ));

        // Create parent task first (required for foreign key constraint)
        final parentTask = await taskRepository.create(Task(
          id: '',
          title: 'Parent Task',
          createdAt: now,
          updatedAt: now,
        ));

        final task = Task(
          id: '',
          title: 'Full Task',
          description: 'Complete description',
          status: TaskStatus.completed,
          tags: ['tag1', 'tag2', 'tag3'],
          projectId: project.id,
          parentTaskId: parentTask.id,
          createdAt: now,
          updatedAt: now,
          completedAt: completedAt,
          estimatedMinutes: 60,
          actualMinutes: 75,
          priority: 1,
          metadata: {'key1': 'value1', 'key2': 123},
        );

        final created = await taskRepository.create(task);
        final found = await taskRepository.findById(created.id);

        expect(found, isNotNull);
        expect(found!.title, 'Full Task');
        expect(found.description, 'Complete description');
        expect(found.status, TaskStatus.completed);
        expect(found.tags, ['tag1', 'tag2', 'tag3']);
        expect(found.projectId, project.id);
        expect(found.parentTaskId, parentTask.id);
        expect(found.estimatedMinutes, 60);
        expect(found.actualMinutes, 75);
        expect(found.priority, 1);
        expect(found.metadata, {'key1': 'value1', 'key2': 123});
      });

      test('handles task with minimal fields', () async {
        final now = DateTime.now();
        final task = Task(
          id: '',
          title: 'Minimal Task',
          createdAt: now,
          updatedAt: now,
        );

        final created = await taskRepository.create(task);
        final found = await taskRepository.findById(created.id);

        expect(found, isNotNull);
        expect(found!.title, 'Minimal Task');
        expect(found.description, isNull);
        expect(found.status, TaskStatus.todo); // Default value
        expect(found.tags, isEmpty);
        expect(found.projectId, isNull);
        expect(found.parentTaskId, isNull);
        expect(found.completedAt, isNull);
        expect(found.estimatedMinutes, isNull);
        expect(found.actualMinutes, isNull);
        expect(found.priority, 3); // Default value
        expect(found.metadata, isNull);
      });

      test('handles empty tags list correctly', () async {
        final now = DateTime.now();
        final task = Task(
          id: '',
          title: 'No Tags',
          tags: [],
          createdAt: now,
          updatedAt: now,
        );

        final created = await taskRepository.create(task);
        final found = await taskRepository.findById(created.id);

        expect(found!.tags, isEmpty);
      });

      test('handles special characters in title and description', () async {
        final now = DateTime.now();
        final task = Task(
          id: '',
          title: 'Task with "quotes" and \'apostrophes\'',
          description: 'Description with special chars: @#\$%^&*()',
          createdAt: now,
          updatedAt: now,
        );

        final created = await taskRepository.create(task);
        final found = await taskRepository.findById(created.id);

        expect(found!.title, contains('quotes'));
        expect(found.description, contains('special chars'));
      });
    });

    group('Status Transitions', () {
      test('can transition task through all statuses', () async {
        final now = DateTime.now();
        final task = Task(
          id: '',
          title: 'Status Test',
          status: TaskStatus.todo,
          createdAt: now,
          updatedAt: now,
        );

        final created = await taskRepository.create(task);

        // Todo -> In Progress
        var updated = await taskRepository.update(
          created.copyWith(status: TaskStatus.inProgress),
        );
        expect(updated.status, TaskStatus.inProgress);

        // In Progress -> Completed
        updated = await taskRepository.update(
          updated.copyWith(status: TaskStatus.completed, completedAt: now),
        );
        expect(updated.status, TaskStatus.completed);

        // Completed -> Cancelled
        updated = await taskRepository.update(
          updated.copyWith(status: TaskStatus.cancelled),
        );
        expect(updated.status, TaskStatus.cancelled);
      });
    });

    group('Priority Handling', () {
      test('handles different priority levels', () async {
        final now = DateTime.now();

        for (var priority = 1; priority <= 5; priority++) {
          final task = Task(
            id: '',
            title: 'Priority $priority Task',
            priority: priority,
            createdAt: now,
            updatedAt: now,
          );

          final created = await taskRepository.create(task);
          final found = await taskRepository.findById(created.id);

          expect(found!.priority, priority);
        }
      });
    });

    group('Time Tracking', () {
      test('handles time estimates and actuals', () async {
        final now = DateTime.now();
        final task = Task(
          id: '',
          title: 'Timed Task',
          estimatedMinutes: 60,
          actualMinutes: 75,
          createdAt: now,
          updatedAt: now,
        );

        final created = await taskRepository.create(task);
        final found = await taskRepository.findById(created.id);

        expect(found!.estimatedMinutes, 60);
        expect(found.actualMinutes, 75);
      });

      test('updates actual time when task completed', () async {
        final now = DateTime.now();
        final task = Task(
          id: '',
          title: 'Complete with Time',
          estimatedMinutes: 60,
          createdAt: now,
          updatedAt: now,
        );

        final created = await taskRepository.create(task);
        final completed = await taskRepository.update(
          created.copyWith(
            status: TaskStatus.completed,
            actualMinutes: 90,
            completedAt: now,
          ),
        );

        expect(completed.actualMinutes, 90);
        expect(completed.completedAt, isNotNull);
      });
    });
  });
}
