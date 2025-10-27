import 'package:altair_core/altair_core.dart';
import 'package:altair_guidance/bloc/task/task_bloc.dart';
import 'package:altair_guidance/bloc/task/task_event.dart';
import 'package:altair_guidance/bloc/task/task_state.dart';
import 'package:bloc_test/bloc_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:logger/logger.dart';
import 'package:mocktail/mocktail.dart';

class MockTaskRepositorySurrealDB extends Mock
    implements TaskRepositorySurrealDB {}

class MockLogger extends Mock implements Logger {}

class FakeTask extends Fake implements Task {}

void main() {
  late MockTaskRepositorySurrealDB mockTaskRepository;
  late MockLogger mockLogger;

  setUpAll(() {
    registerFallbackValue(FakeTask());
  });

  // Sample test data with SurrealDB ID format
  final now = DateTime.now();
  final task1 = Task(
    id: 'task:1',
    title: 'Test Task 1',
    createdAt: now,
    updatedAt: now,
    status: TaskStatus.todo,
    priority: 3,
  );

  final task2 = Task(
    id: 'task:2',
    title: 'Test Task 2',
    description: 'Task description',
    createdAt: now,
    updatedAt: now,
    status: TaskStatus.completed,
    priority: 1,
    completedAt: now,
  );

  final task3 = Task(
    id: 'task:3',
    title: 'In Progress Task',
    createdAt: now,
    updatedAt: now,
    status: TaskStatus.inProgress,
    priority: 2,
  );

  setUp(() {
    mockTaskRepository = MockTaskRepositorySurrealDB();
    mockLogger = MockLogger();
  });

  group('TaskBloc', () {
    test('initial state is TaskInitial', () {
      final bloc = TaskBloc(
        taskRepository: mockTaskRepository,
        logger: mockLogger,
      );

      expect(bloc.state, const TaskInitial());
    });

    group('TaskLoadRequested', () {
      blocTest<TaskBloc, TaskState>(
        'emits [TaskLoading, TaskLoaded] when tasks are loaded successfully',
        build: () {
          when(() => mockTaskRepository.findAll())
              .thenAnswer((_) async => [task1, task2, task3]);
          return TaskBloc(
            taskRepository: mockTaskRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TaskLoadRequested()),
        expect: () => [
          const TaskLoading(),
          TaskLoaded(tasks: [task1, task2, task3]),
        ],
        verify: (_) {
          verify(() => mockTaskRepository.findAll()).called(1);
        },
      );

      blocTest<TaskBloc, TaskState>(
        'emits [TaskLoading, TaskFailure] when loading fails',
        build: () {
          when(() => mockTaskRepository.findAll())
              .thenThrow(Exception('Failed to load tasks'));
          return TaskBloc(
            taskRepository: mockTaskRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TaskLoadRequested()),
        expect: () => [
          const TaskLoading(),
          const TaskFailure(message: 'Exception: Failed to load tasks'),
        ],
        verify: (_) {
          verify(() => mockTaskRepository.findAll()).called(1);
        },
      );
    });

    group('TaskQuickCaptureRequested', () {
      final captureTitle = 'Quick captured task';
      final capturedTask = Task(
        id: 'task:4',
        title: captureTitle,
        createdAt: now,
        updatedAt: now,
        status: TaskStatus.todo,
        priority: 3,
      );

      blocTest<TaskBloc, TaskState>(
        'emits [TaskCaptured, TaskLoaded] when quick capture succeeds',
        build: () {
          when(() => mockTaskRepository.create(any()))
              .thenAnswer((_) async => capturedTask);
          when(() => mockTaskRepository.findAll())
              .thenAnswer((_) async => [capturedTask, task1, task2]);
          return TaskBloc(
            taskRepository: mockTaskRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(
          TaskQuickCaptureRequested(title: captureTitle),
        ),
        expect: () => [
          TaskCaptured(task: capturedTask),
          TaskLoaded(tasks: [capturedTask, task1, task2]),
        ],
        verify: (_) {
          verify(() => mockTaskRepository.create(any())).called(1);
          verify(() => mockTaskRepository.findAll()).called(1);
        },
      );

      blocTest<TaskBloc, TaskState>(
        'emits [TaskFailure] when quick capture fails',
        build: () {
          when(() => mockTaskRepository.create(any()))
              .thenThrow(Exception('Failed to create task'));
          return TaskBloc(
            taskRepository: mockTaskRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(
          const TaskQuickCaptureRequested(title: 'Test'),
        ),
        expect: () => [
          const TaskFailure(message: 'Exception: Failed to create task'),
        ],
        verify: (_) {
          verify(() => mockTaskRepository.create(any())).called(1);
        },
      );
    });

    group('TaskCreateRequested', () {
      blocTest<TaskBloc, TaskState>(
        'emits [TaskLoading, TaskLoaded] when task is created successfully',
        build: () {
          when(() => mockTaskRepository.create(task1))
              .thenAnswer((_) async => task1);
          when(() => mockTaskRepository.findAll())
              .thenAnswer((_) async => [task1, task2]);
          return TaskBloc(
            taskRepository: mockTaskRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(TaskCreateRequested(task: task1)),
        expect: () => [
          const TaskLoading(),
          TaskLoaded(tasks: [task1, task2]),
        ],
        verify: (_) {
          verify(() => mockTaskRepository.create(task1)).called(1);
          verify(() => mockTaskRepository.findAll()).called(1);
        },
      );

      blocTest<TaskBloc, TaskState>(
        'emits [TaskLoading, TaskFailure] when creation fails',
        build: () {
          when(() => mockTaskRepository.create(task1))
              .thenThrow(Exception('Failed to create'));
          return TaskBloc(
            taskRepository: mockTaskRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(TaskCreateRequested(task: task1)),
        expect: () => [
          const TaskLoading(),
          const TaskFailure(message: 'Exception: Failed to create'),
        ],
        verify: (_) {
          verify(() => mockTaskRepository.create(task1)).called(1);
        },
      );
    });

    group('TaskUpdateRequested', () {
      final updatedTask = task1.copyWith(
        title: 'Updated title',
        status: TaskStatus.completed,
      );

      blocTest<TaskBloc, TaskState>(
        'emits [TaskLoading, TaskLoaded] when task is updated successfully',
        build: () {
          when(() => mockTaskRepository.update(updatedTask))
              .thenAnswer((_) async => updatedTask);
          when(() => mockTaskRepository.findAll())
              .thenAnswer((_) async => [updatedTask, task2]);
          return TaskBloc(
            taskRepository: mockTaskRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(TaskUpdateRequested(task: updatedTask)),
        expect: () => [
          const TaskLoading(),
          TaskLoaded(tasks: [updatedTask, task2]),
        ],
        verify: (_) {
          verify(() => mockTaskRepository.update(updatedTask)).called(1);
          verify(() => mockTaskRepository.findAll()).called(1);
        },
      );

      blocTest<TaskBloc, TaskState>(
        'emits [TaskLoading, TaskFailure] when update fails',
        build: () {
          when(() => mockTaskRepository.update(updatedTask))
              .thenThrow(Exception('Failed to update'));
          return TaskBloc(
            taskRepository: mockTaskRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(TaskUpdateRequested(task: updatedTask)),
        expect: () => [
          const TaskLoading(),
          const TaskFailure(message: 'Exception: Failed to update'),
        ],
        verify: (_) {
          verify(() => mockTaskRepository.update(updatedTask)).called(1);
        },
      );
    });

    group('TaskDeleteRequested', () {
      blocTest<TaskBloc, TaskState>(
        'emits [TaskLoading, TaskLoaded] when task is deleted successfully',
        build: () {
          when(() => mockTaskRepository.delete('task:1'))
              .thenAnswer((_) async => {});
          when(() => mockTaskRepository.findAll())
              .thenAnswer((_) async => [task2]);
          return TaskBloc(
            taskRepository: mockTaskRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TaskDeleteRequested(taskId: 'task:1')),
        expect: () => [
          const TaskLoading(),
          TaskLoaded(tasks: [task2]),
        ],
        verify: (_) {
          verify(() => mockTaskRepository.delete('task:1')).called(1);
          verify(() => mockTaskRepository.findAll()).called(1);
        },
      );

      blocTest<TaskBloc, TaskState>(
        'emits [TaskLoading, TaskFailure] when deletion fails',
        build: () {
          when(() => mockTaskRepository.delete('task:1'))
              .thenThrow(Exception('Failed to delete'));
          return TaskBloc(
            taskRepository: mockTaskRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TaskDeleteRequested(taskId: 'task:1')),
        expect: () => [
          const TaskLoading(),
          const TaskFailure(message: 'Exception: Failed to delete'),
        ],
        verify: (_) {
          verify(() => mockTaskRepository.delete('task:1')).called(1);
        },
      );
    });

    group('TaskSearchRequested', () {
      const searchQuery = 'test';

      blocTest<TaskBloc, TaskState>(
        'emits [TaskLoading, TaskLoaded] when search succeeds',
        build: () {
          when(() => mockTaskRepository.search(searchQuery))
              .thenAnswer((_) async => [task1]);
          return TaskBloc(
            taskRepository: mockTaskRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TaskSearchRequested(query: searchQuery)),
        expect: () => [
          const TaskLoading(),
          TaskLoaded(tasks: [task1]),
        ],
        verify: (_) {
          verify(() => mockTaskRepository.search(searchQuery)).called(1);
        },
      );

      blocTest<TaskBloc, TaskState>(
        'emits [TaskLoading, TaskFailure] when search fails',
        build: () {
          when(() => mockTaskRepository.search(searchQuery))
              .thenThrow(Exception('Search failed'));
          return TaskBloc(
            taskRepository: mockTaskRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TaskSearchRequested(query: searchQuery)),
        expect: () => [
          const TaskLoading(),
          const TaskFailure(message: 'Exception: Search failed'),
        ],
        verify: (_) {
          verify(() => mockTaskRepository.search(searchQuery)).called(1);
        },
      );
    });

    group('TaskFilterByStatusRequested', () {
      blocTest<TaskBloc, TaskState>(
        'emits [TaskLoading, TaskLoaded] with filter when filtering succeeds',
        build: () {
          when(() => mockTaskRepository.findAll(status: TaskStatus.completed))
              .thenAnswer((_) async => [task2]);
          return TaskBloc(
            taskRepository: mockTaskRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(
          const TaskFilterByStatusRequested(status: TaskStatus.completed),
        ),
        expect: () => [
          const TaskLoading(),
          TaskLoaded(tasks: [task2], filter: TaskStatus.completed),
        ],
        verify: (_) {
          verify(
            () => mockTaskRepository.findAll(status: TaskStatus.completed),
          ).called(1);
        },
      );

      blocTest<TaskBloc, TaskState>(
        'emits [TaskLoading, TaskFailure] when filtering fails',
        build: () {
          when(() => mockTaskRepository.findAll(status: TaskStatus.todo))
              .thenThrow(Exception('Filter failed'));
          return TaskBloc(
            taskRepository: mockTaskRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(
          const TaskFilterByStatusRequested(status: TaskStatus.todo),
        ),
        expect: () => [
          const TaskLoading(),
          const TaskFailure(message: 'Exception: Filter failed'),
        ],
        verify: (_) {
          verify(() => mockTaskRepository.findAll(status: TaskStatus.todo))
              .called(1);
        },
      );
    });

    group('TaskClearFiltersRequested', () {
      blocTest<TaskBloc, TaskState>(
        'emits [TaskLoading, TaskLoaded] without filter when clearing succeeds',
        build: () {
          when(() => mockTaskRepository.findAll())
              .thenAnswer((_) async => [task1, task2, task3]);
          return TaskBloc(
            taskRepository: mockTaskRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TaskClearFiltersRequested()),
        expect: () => [
          const TaskLoading(),
          TaskLoaded(tasks: [task1, task2, task3]),
        ],
        verify: (_) {
          verify(() => mockTaskRepository.findAll()).called(1);
        },
      );

      blocTest<TaskBloc, TaskState>(
        'emits [TaskLoading, TaskFailure] when clearing filters fails',
        build: () {
          when(() => mockTaskRepository.findAll())
              .thenThrow(Exception('Failed to clear filters'));
          return TaskBloc(
            taskRepository: mockTaskRepository,
            logger: mockLogger,
          );
        },
        act: (bloc) => bloc.add(const TaskClearFiltersRequested()),
        expect: () => [
          const TaskLoading(),
          const TaskFailure(message: 'Exception: Failed to clear filters'),
        ],
        verify: (_) {
          verify(() => mockTaskRepository.findAll()).called(1);
        },
      );
    });
  });
}
