import 'dart:async';

import 'package:altair_core/altair_core.dart';
import 'package:altair_guidance/bloc/ai/ai_bloc.dart';
import 'package:altair_guidance/bloc/ai/ai_event.dart';
import 'package:altair_guidance/bloc/ai/ai_state.dart';
import 'package:altair_guidance/bloc/task/task_bloc.dart';
import 'package:altair_guidance/bloc/task/task_event.dart';
import 'package:altair_guidance/features/ai/task_breakdown_dialog.dart';
import 'package:altair_guidance/services/ai/models.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';

class MockAIBloc extends Mock implements AIBloc {}

class MockTaskBloc extends Mock implements TaskBloc {}

void main() {
  setUpAll(() {
    registerFallbackValue(
      AITaskBreakdownRequested(
        request: TaskBreakdownRequest(taskTitle: 'Test'),
      ),
    );
    registerFallbackValue(
      TaskCreateRequested(
        task: Task(
          id: '',
          title: 'Test',
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
          status: TaskStatus.todo,
          priority: 3,
        ),
      ),
    );
  });


  group('TaskBreakdownDialog', () {
    late MockAIBloc mockAIBloc;
    late MockTaskBloc mockTaskBloc;
    late StreamController<AIState> streamController;

    setUp(() {
      mockAIBloc = MockAIBloc();
      mockTaskBloc = MockTaskBloc();
      streamController = StreamController<AIState>.broadcast();
      when(() => mockAIBloc.state).thenReturn(const AIInitial());
      when(() => mockAIBloc.stream).thenAnswer((_) => streamController.stream);
      when(() => mockAIBloc.add(any())).thenReturn(null);
      when(() => mockTaskBloc.add(any())).thenReturn(null);
    });

    tearDown(() {
      streamController.close();
    });

    testWidgets('dispatches task breakdown request on init', (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: MultiBlocProvider(
            providers: [
              BlocProvider<AIBloc>.value(value: mockAIBloc),
              BlocProvider<TaskBloc>.value(value: mockTaskBloc),
            ],
            child: const TaskBreakdownDialog(
              taskTitle: 'Build feature',
              taskDescription: 'Add new functionality',
              projectContext: 'Flutter app',
              maxSubtasks: 5,
            ),
          ),
        ),
      );

      verify(() => mockAIBloc.add(any<AITaskBreakdownRequested>())).called(1);
    });

    testWidgets('shows loading state', (tester) async {
      when(() => mockAIBloc.state).thenReturn(
        const AILoading(operationType: AIOperationType.breakdown),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MultiBlocProvider(
            providers: [
              BlocProvider<AIBloc>.value(value: mockAIBloc),
              BlocProvider<TaskBloc>.value(value: mockTaskBloc),
            ],
            child: const TaskBreakdownDialog(taskTitle: 'Test task'),
          ),
        ),
      );

      await tester.pump();

      expect(find.byType(CircularProgressIndicator), findsOneWidget);
      expect(find.text('AI is analyzing your task...'), findsOneWidget);
      expect(find.text('This may take a few seconds'), findsOneWidget);
    });

    testWidgets('displays subtask details correctly', (tester) async {
      final response = TaskBreakdownResponse(
        originalTask: 'Test task',
        subtasks: [
          SubtaskSuggestion(
            order: 1,
            title: 'First subtask',
            description: 'Detailed description',
            estimatedMinutes: 45,
          ),
        ],
      );

      when(() => mockAIBloc.state).thenReturn(
        AITaskBreakdownSuccess(response: response),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MultiBlocProvider(
            providers: [
              BlocProvider<AIBloc>.value(value: mockAIBloc),
              BlocProvider<TaskBloc>.value(value: mockTaskBloc),
            ],
            child: const TaskBreakdownDialog(taskTitle: 'Test task'),
          ),
        ),
      );

      await tester.pump();

      expect(find.text('First subtask'), findsOneWidget);
      expect(find.text('Detailed description'), findsOneWidget);
      expect(find.text('45m'), findsOneWidget);
      expect(find.text('1'), findsOneWidget); // Order number
    });

    testWidgets('shows error state with retry button', (tester) async {
      when(() => mockAIBloc.state).thenReturn(
        const AIFailure(
          message: 'Connection timeout',
          operationType: AIOperationType.breakdown,
        ),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MultiBlocProvider(
            providers: [
              BlocProvider<AIBloc>.value(value: mockAIBloc),
              BlocProvider<TaskBloc>.value(value: mockTaskBloc),
            ],
            child: const TaskBreakdownDialog(taskTitle: 'Test task'),
          ),
        ),
      );

      await tester.pump();

      expect(find.text('Failed to break down task'), findsOneWidget);
      expect(find.text('Connection timeout'), findsOneWidget);
      expect(find.text('Retry'), findsOneWidget);
    });

    testWidgets('retry button dispatches new request', (tester) async {
      when(() => mockAIBloc.state).thenReturn(
        const AIFailure(
          message: 'Error',
          operationType: AIOperationType.breakdown,
        ),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MultiBlocProvider(
            providers: [
              BlocProvider<AIBloc>.value(value: mockAIBloc),
              BlocProvider<TaskBloc>.value(value: mockTaskBloc),
            ],
            child: const TaskBreakdownDialog(
              taskTitle: 'Test task',
              maxSubtasks: 5,
            ),
          ),
        ),
      );

      await tester.pump();

      // Clear previous invocation from initState
      clearInteractions(mockAIBloc);

      // Tap retry button
      await tester.tap(find.text('Retry'));
      await tester.pump();

      verify(() => mockAIBloc.add(any<AITaskBreakdownRequested>())).called(1);
    });

    testWidgets('shows error snackbar when breakdown fails', (tester) async {
      when(() => mockAIBloc.state).thenReturn(const AILoading(operationType: AIOperationType.breakdown));

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: MultiBlocProvider(
              providers: [
                BlocProvider<AIBloc>.value(value: mockAIBloc),
                BlocProvider<TaskBloc>.value(value: mockTaskBloc),
              ],
              child: const TaskBreakdownDialog(taskTitle: 'Test'),
            ),
          ),
        ),
      );

      await tester.pump();

      // Emit failure state
      streamController.add(
        const AIFailure(
          message: 'Network error',
          operationType: AIOperationType.breakdown,
        ),
      );

      await tester.pump(const Duration(milliseconds: 100));

      expect(find.text('Error: Network error'), findsOneWidget);
    });
  });
}
