/// Tests for task filter menu widget.
library;

import 'package:altair_core/altair_core.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';

import '../../lib/bloc/task/task_bloc.dart';
import '../../lib/bloc/task/task_event.dart';
import '../../lib/bloc/task/task_state.dart';
import '../../lib/widgets/task_filter_menu.dart';

// Mock classes
class MockTaskBloc extends Mock implements TaskBloc {}

void main() {
  late MockTaskBloc mockTaskBloc;

  setUp(() {
    mockTaskBloc = MockTaskBloc();

    // Register fallback values for Mocktail
    registerFallbackValue(const TaskLoadRequested());
    registerFallbackValue(const TaskClearFiltersRequested());
    registerFallbackValue(const TaskFilterByStatusRequested(status: TaskStatus.todo));
    registerFallbackValue(const TaskFilterByTagsRequested(tags: []));
  });

  group('Task Filter Menu', () {
    testWidgets('shows filter menu as bottom sheet on mobile', (tester) async {
      when(() => mockTaskBloc.state).thenReturn(
        TaskLoaded(
          tasks: [
            Task(
              id: 'task1',
              title: 'Test Task',
              tags: ['work', 'urgent'],
              createdAt: DateTime.now(),
              updatedAt: DateTime.now(),
            ),
          ],
        ),
      );
      when(() => mockTaskBloc.stream).thenAnswer((_) => const Stream.empty());

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<TaskBloc>.value(
            value: mockTaskBloc,
            child: Scaffold(
              body: Builder(
                builder: (context) {
                  return ElevatedButton(
                    onPressed: () {
                      showTaskFilterMenu(context, isMobile: true);
                    },
                    child: const Text('Show Filter'),
                  );
                },
              ),
            ),
          ),
        ),
      );

      // Tap button to show filter menu
      await tester.tap(find.text('Show Filter'));
      await tester.pumpAndSettle();

      // Verify bottom sheet is shown
      expect(find.text('Filter Tasks'), findsOneWidget);
      expect(find.text('Clear All'), findsOneWidget);
    });

    testWidgets('shows filter menu as dialog on desktop', (tester) async {
      when(() => mockTaskBloc.state).thenReturn(
        TaskLoaded(
          tasks: [
            Task(
              id: 'task1',
              title: 'Test Task',
              tags: ['work'],
              createdAt: DateTime.now(),
              updatedAt: DateTime.now(),
            ),
          ],
        ),
      );
      when(() => mockTaskBloc.stream).thenAnswer((_) => const Stream.empty());

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<TaskBloc>.value(
            value: mockTaskBloc,
            child: Scaffold(
              body: Builder(
                builder: (context) {
                  return ElevatedButton(
                    onPressed: () {
                      showTaskFilterMenu(context, isMobile: false);
                    },
                    child: const Text('Show Filter'),
                  );
                },
              ),
            ),
          ),
        ),
      );

      // Tap button to show filter menu
      await tester.tap(find.text('Show Filter'));
      await tester.pumpAndSettle();

      // Verify dialog is shown
      expect(find.text('Filter Tasks'), findsOneWidget);
      expect(find.text('Apply Filters'), findsOneWidget);
    });

    testWidgets('displays all status filter options', (tester) async {
      when(() => mockTaskBloc.state).thenReturn(
        TaskLoaded(tasks: []),
      );
      when(() => mockTaskBloc.stream).thenAnswer((_) => const Stream.empty());

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<TaskBloc>.value(
            value: mockTaskBloc,
            child: Scaffold(
              body: Builder(
                builder: (context) {
                  return ElevatedButton(
                    onPressed: () {
                      showTaskFilterMenu(context, isMobile: true);
                    },
                    child: const Text('Show Filter'),
                  );
                },
              ),
            ),
          ),
        ),
      );

      await tester.tap(find.text('Show Filter'));
      await tester.pumpAndSettle();

      // Verify all status options are present
      expect(find.text('All'), findsOneWidget);
      expect(find.text('To Do'), findsOneWidget);
      expect(find.text('In Progress'), findsOneWidget);
      expect(find.text('Completed'), findsOneWidget);
      expect(find.text('Cancelled'), findsOneWidget);
    });

    testWidgets('filters tasks by status when chip is tapped', (tester) async {
      when(() => mockTaskBloc.state).thenReturn(
        TaskLoaded(tasks: []),
      );
      when(() => mockTaskBloc.stream).thenAnswer((_) => const Stream.empty());
      when(() => mockTaskBloc.add(any())).thenReturn(null);

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<TaskBloc>.value(
            value: mockTaskBloc,
            child: Scaffold(
              body: Builder(
                builder: (context) {
                  return ElevatedButton(
                    onPressed: () {
                      showTaskFilterMenu(context, isMobile: true);
                    },
                    child: const Text('Show Filter'),
                  );
                },
              ),
            ),
          ),
        ),
      );

      await tester.tap(find.text('Show Filter'));
      await tester.pumpAndSettle();

      // Tap "Completed" filter
      await tester.tap(find.text('Completed'));
      await tester.pump();

      // Verify event was sent to bloc
      verify(
        () => mockTaskBloc.add(
          const TaskFilterByStatusRequested(status: TaskStatus.completed),
        ),
      ).called(1);
    });

    testWidgets('displays tag filters from tasks', (tester) async {
      when(() => mockTaskBloc.state).thenReturn(
        TaskLoaded(
          tasks: [
            Task(
              id: 'task1',
              title: 'Task 1',
              tags: ['work', 'urgent'],
              createdAt: DateTime.now(),
              updatedAt: DateTime.now(),
            ),
            Task(
              id: 'task2',
              title: 'Task 2',
              tags: ['personal'],
              createdAt: DateTime.now(),
              updatedAt: DateTime.now(),
            ),
          ],
        ),
      );
      when(() => mockTaskBloc.stream).thenAnswer((_) => const Stream.empty());

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<TaskBloc>.value(
            value: mockTaskBloc,
            child: Scaffold(
              body: Builder(
                builder: (context) {
                  return ElevatedButton(
                    onPressed: () {
                      showTaskFilterMenu(context, isMobile: true);
                    },
                    child: const Text('Show Filter'),
                  );
                },
              ),
            ),
          ),
        ),
      );

      await tester.tap(find.text('Show Filter'));
      await tester.pumpAndSettle();

      // Verify tag filters are shown
      expect(find.text('personal'), findsOneWidget);
      expect(find.text('urgent'), findsOneWidget);
      expect(find.text('work'), findsOneWidget);
    });

    testWidgets('filters tasks by tag when chip is tapped', (tester) async {
      when(() => mockTaskBloc.state).thenReturn(
        TaskLoaded(
          tasks: [
            Task(
              id: 'task1',
              title: 'Task 1',
              tags: ['work'],
              createdAt: DateTime.now(),
              updatedAt: DateTime.now(),
            ),
          ],
        ),
      );
      when(() => mockTaskBloc.stream).thenAnswer((_) => const Stream.empty());
      when(() => mockTaskBloc.add(any())).thenReturn(null);

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<TaskBloc>.value(
            value: mockTaskBloc,
            child: Scaffold(
              body: Builder(
                builder: (context) {
                  return ElevatedButton(
                    onPressed: () {
                      showTaskFilterMenu(context, isMobile: true);
                    },
                    child: const Text('Show Filter'),
                  );
                },
              ),
            ),
          ),
        ),
      );

      await tester.tap(find.text('Show Filter'));
      await tester.pumpAndSettle();

      // Tap "work" tag filter
      await tester.tap(find.text('work'));
      await tester.pump();

      // Verify event was sent to bloc
      verify(
        () => mockTaskBloc.add(
          const TaskFilterByTagsRequested(tags: ['work']),
        ),
      ).called(1);
    });

    testWidgets('clears filters when "Clear All" is tapped', (tester) async {
      when(() => mockTaskBloc.state).thenReturn(
        TaskLoaded(
          tasks: [],
          filter: TaskStatus.completed,
        ),
      );
      when(() => mockTaskBloc.stream).thenAnswer((_) => const Stream.empty());
      when(() => mockTaskBloc.add(any())).thenReturn(null);

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<TaskBloc>.value(
            value: mockTaskBloc,
            child: Scaffold(
              body: Builder(
                builder: (context) {
                  return ElevatedButton(
                    onPressed: () {
                      showTaskFilterMenu(context, isMobile: true);
                    },
                    child: const Text('Show Filter'),
                  );
                },
              ),
            ),
          ),
        ),
      );

      await tester.tap(find.text('Show Filter'));
      await tester.pumpAndSettle();

      // Tap "Clear All"
      await tester.tap(find.text('Clear All'));
      await tester.pump();

      // Verify clear event was sent
      verify(() => mockTaskBloc.add(const TaskClearFiltersRequested())).called(1);
    });

    testWidgets('shows current filter state in UI', (tester) async {
      when(() => mockTaskBloc.state).thenReturn(
        TaskLoaded(
          tasks: [],
          filter: TaskStatus.inProgress,
        ),
      );
      when(() => mockTaskBloc.stream).thenAnswer((_) => const Stream.empty());

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<TaskBloc>.value(
            value: mockTaskBloc,
            child: Scaffold(
              body: Builder(
                builder: (context) {
                  return ElevatedButton(
                    onPressed: () {
                      showTaskFilterMenu(context, isMobile: true);
                    },
                    child: const Text('Show Filter'),
                  );
                },
              ),
            ),
          ),
        ),
      );

      await tester.tap(find.text('Show Filter'));
      await tester.pumpAndSettle();

      // Verify "In Progress" option is displayed
      expect(find.text('In Progress'), findsOneWidget);
    });

    testWidgets('allows multi-select tag filtering', (tester) async {
      when(() => mockTaskBloc.state).thenReturn(
        TaskLoaded(
          tasks: [
            Task(
              id: 'task1',
              title: 'Task 1',
              tags: ['work', 'urgent'],
              createdAt: DateTime.now(),
              updatedAt: DateTime.now(),
            ),
          ],
        ),
      );
      when(() => mockTaskBloc.stream).thenAnswer((_) => const Stream.empty());
      when(() => mockTaskBloc.add(any())).thenReturn(null);

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<TaskBloc>.value(
            value: mockTaskBloc,
            child: Scaffold(
              body: Builder(
                builder: (context) {
                  return ElevatedButton(
                    onPressed: () {
                      showTaskFilterMenu(context, isMobile: true);
                    },
                    child: const Text('Show Filter'),
                  );
                },
              ),
            ),
          ),
        ),
      );

      await tester.tap(find.text('Show Filter'));
      await tester.pumpAndSettle();

      // Tap first tag
      await tester.tap(find.text('work'));
      await tester.pump();

      // Verify tag filter event
      verify(
        () => mockTaskBloc.add(
          const TaskFilterByTagsRequested(tags: ['work']),
        ),
      ).called(1);
    });
  });
}
