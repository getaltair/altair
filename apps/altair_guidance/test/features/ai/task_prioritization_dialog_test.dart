import 'package:altair_core/altair_core.dart';
import 'package:altair_guidance/bloc/ai/ai_bloc.dart';
import 'package:altair_guidance/bloc/ai/ai_event.dart';
import 'package:altair_guidance/bloc/ai/ai_state.dart';
import 'package:altair_guidance/features/ai/task_prioritization_dialog.dart';
import 'package:altair_guidance/services/ai/models.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';

class MockAIBloc extends Mock implements AIBloc {}

void main() {
  setUpAll(() {
    registerFallbackValue(
      AITaskPrioritizationRequested(
        request: TaskPrioritizationRequest(tasks: [
          {'title': 'Test'},
        ]),
      ),
    );
  });

  group('showTaskPrioritizationDialog', () {
    late MockAIBloc mockBloc;

    setUp(() {
      mockBloc = MockAIBloc();
      when(() => mockBloc.state).thenReturn(const AIInitial());
      when(() => mockBloc.stream).thenAnswer(
        (_) => Stream.value(const AIInitial()),
      );
    });

    testWidgets('shows dialog with correct title', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AILoading(operationType: AIOperationType.prioritization),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: Builder(
              builder: (context) {
                return ElevatedButton(
                  onPressed: () {
                    showTaskPrioritizationDialog(
                      context,
                      tasks: [
                        Task(
                          id: '1',
                          title: 'Task 1',
                          status: TaskStatus.todo,
                          priority: 3,
                          createdAt: DateTime.now(),
                          updatedAt: DateTime.now(),
                        ),
                      ],
                    );
                  },
                  child: const Text('Show Dialog'),
                );
              },
            ),
          ),
        ),
      );

      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();

      expect(find.text('AI Task Prioritization'), findsOneWidget);
    });

    testWidgets('dialog is user-dismissible', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AILoading(operationType: AIOperationType.prioritization),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: Builder(
              builder: (context) {
                return ElevatedButton(
                  onPressed: () {
                    showTaskPrioritizationDialog(
                      context,
                      tasks: [
                        Task(
                          id: '1',
                          title: 'Task 1',
                          status: TaskStatus.todo,
                          priority: 3,
                          createdAt: DateTime.now(),
                          updatedAt: DateTime.now(),
                        ),
                      ],
                    );
                  },
                  child: const Text('Show Dialog'),
                );
              },
            ),
          ),
        ),
      );

      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();

      // Dismiss by tapping barrier
      await tester.tapAt(const Offset(10, 10));
      await tester.pumpAndSettle();

      expect(find.text('AI Task Prioritization'), findsNothing);
    });

    testWidgets('close button dismisses dialog', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AILoading(operationType: AIOperationType.prioritization),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: Builder(
              builder: (context) {
                return ElevatedButton(
                  onPressed: () {
                    showTaskPrioritizationDialog(
                      context,
                      tasks: [
                        Task(
                          id: '1',
                          title: 'Task 1',
                          status: TaskStatus.todo,
                          priority: 3,
                          createdAt: DateTime.now(),
                          updatedAt: DateTime.now(),
                        ),
                      ],
                    );
                  },
                  child: const Text('Show Dialog'),
                );
              },
            ),
          ),
        ),
      );

      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();

      // Find and tap close button
      await tester.tap(find.byIcon(Icons.close));
      await tester.pumpAndSettle();

      expect(find.text('AI Task Prioritization'), findsNothing);
    });
  });

  group('TaskPrioritizationDialog', () {
    late MockAIBloc mockBloc;

    setUp(() {
      mockBloc = MockAIBloc();
      when(() => mockBloc.state).thenReturn(const AIInitial());
      when(() => mockBloc.stream).thenAnswer(
        (_) => Stream.value(const AIInitial()),
      );
      when(() => mockBloc.add(any())).thenReturn(null);
    });

    testWidgets('dispatches prioritization request on init', (tester) async {
      final tasks = [
        Task(
          id: '1',
          title: 'Task 1',
          description: 'Description 1',
          status: TaskStatus.todo,
          priority: 3,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
        Task(
          id: '2',
          title: 'Task 2',
          status: TaskStatus.todo,
          priority: 3,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      ];

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: TaskPrioritizationDialog(
              tasks: tasks,
              projectContext: 'Test project',
            ),
          ),
        ),
      );

      verify(() => mockBloc.add(any<AITaskPrioritizationRequested>()))
          .called(1);
    });

    testWidgets('does not dispatch request for empty task list',
        (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const TaskPrioritizationDialog(tasks: []),
          ),
        ),
      );

      verifyNever(() => mockBloc.add(any<AITaskPrioritizationRequested>()));
    });

    testWidgets('shows loading state', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AILoading(operationType: AIOperationType.prioritization),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: TaskPrioritizationDialog(
              tasks: [
                Task(
                  id: '1',
                  title: 'Task 1',
                  status: TaskStatus.todo,
                  priority: 3,
                  createdAt: DateTime.now(),
                  updatedAt: DateTime.now(),
                ),
              ],
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      expect(find.byType(CircularProgressIndicator), findsOneWidget);
      expect(
        find.text('AI is analyzing task priorities...'),
        findsOneWidget,
      );
    });

    testWidgets('shows success state with prioritization results',
        (tester) async {
      final response = TaskPrioritizationResponse(
        recommendedOrder: ['Task 1', 'Task 2'],
        suggestions: [
          const PrioritySuggestion(
            taskTitle: 'Task 1',
            priority: PriorityLevel.high,
            urgencyScore: 0.8,
            impactScore: 0.9,
            reasoning: 'Critical for project',
          ),
          const PrioritySuggestion(
            taskTitle: 'Task 2',
            priority: PriorityLevel.medium,
            urgencyScore: 0.5,
            impactScore: 0.6,
            reasoning: 'Important but not urgent',
          ),
        ],
      );

      when(() => mockBloc.state).thenReturn(
        AITaskPrioritizationSuccess(response: response),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: TaskPrioritizationDialog(
              tasks: [
                Task(
                  id: '1',
                  title: 'Task 1',
                  status: TaskStatus.todo,
                  priority: 3,
                  createdAt: DateTime.now(),
                  updatedAt: DateTime.now(),
                ),
              ],
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      expect(find.text('Recommended Execution Order'), findsOneWidget);
      expect(find.text('Task 1'), findsOneWidget);
      expect(find.text('Task 2'), findsOneWidget);
      expect(find.text('Critical for project'), findsOneWidget);
      expect(find.text('Important but not urgent'), findsOneWidget);
    });

    testWidgets('shows error state with retry button', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AIFailure(
          message: 'Network error',
          operationType: AIOperationType.prioritization,
        ),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: TaskPrioritizationDialog(
              tasks: [
                Task(
                  id: '1',
                  title: 'Task 1',
                  status: TaskStatus.todo,
                  priority: 3,
                  createdAt: DateTime.now(),
                  updatedAt: DateTime.now(),
                ),
              ],
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      expect(find.text('Failed to get prioritization'), findsOneWidget);
      expect(find.text('Network error'), findsOneWidget);
      expect(find.text('Retry'), findsOneWidget);
    });

    testWidgets('retry button dispatches new request', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AIFailure(
          message: 'Network error',
          operationType: AIOperationType.prioritization,
        ),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: TaskPrioritizationDialog(
              tasks: [
                Task(
                  id: '1',
                  title: 'Task 1',
                  status: TaskStatus.todo,
                  priority: 3,
                  createdAt: DateTime.now(),
                  updatedAt: DateTime.now(),
                ),
              ],
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Clear previous invocation from initState
      clearInteractions(mockBloc);

      // Tap retry button
      await tester.tap(find.text('Retry'));
      await tester.pumpAndSettle();

      verify(() => mockBloc.add(any<AITaskPrioritizationRequested>()))
          .called(1);
    });

    testWidgets('displays priority badges with correct colors',
        (tester) async {
      final response = TaskPrioritizationResponse(
        recommendedOrder: ['Task 1'],
        suggestions: [
          const PrioritySuggestion(
            taskTitle: 'Task 1',
            priority: PriorityLevel.critical,
            urgencyScore: 0.9,
            impactScore: 0.9,
            reasoning: 'Critical task',
          ),
        ],
      );

      when(() => mockBloc.state).thenReturn(
        AITaskPrioritizationSuccess(response: response),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: TaskPrioritizationDialog(
              tasks: [
                Task(
                  id: '1',
                  title: 'Task 1',
                  status: TaskStatus.todo,
                  priority: 3,
                  createdAt: DateTime.now(),
                  updatedAt: DateTime.now(),
                ),
              ],
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      expect(find.text('CRITICAL'), findsOneWidget);
      expect(find.text('Urgency: 90%'), findsOneWidget);
      expect(find.text('Impact: 90%'), findsOneWidget);
    });

    testWidgets('close button has tooltip', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AILoading(operationType: AIOperationType.prioritization),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: TaskPrioritizationDialog(
              tasks: [
                Task(
                  id: '1',
                  title: 'Task 1',
                  status: TaskStatus.todo,
                  priority: 3,
                  createdAt: DateTime.now(),
                  updatedAt: DateTime.now(),
                ),
              ],
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      final closeButton =
          tester.widget<IconButton>(find.byIcon(Icons.close));
      expect(closeButton.tooltip, 'Close dialog');
    });
  });
}
