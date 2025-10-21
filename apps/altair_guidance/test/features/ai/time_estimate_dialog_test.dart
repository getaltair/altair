import 'dart:async';

import 'package:altair_guidance/bloc/ai/ai_bloc.dart';
import 'package:altair_guidance/bloc/ai/ai_event.dart';
import 'package:altair_guidance/bloc/ai/ai_state.dart';
import 'package:altair_guidance/features/ai/time_estimate_dialog.dart';
import 'package:altair_guidance/services/ai/models.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';

class MockAIBloc extends Mock implements AIBloc {}

void main() {
  setUpAll(() {
    registerFallbackValue(
      AITimeEstimateRequested(
        request: TimeEstimateRequest(taskTitle: 'Test'),
      ),
    );
  });

  group('showTimeEstimateDialog', () {
    late MockAIBloc mockBloc;
    late StreamController<AIState> streamController;

    setUp(() {
      mockBloc = MockAIBloc();
      streamController = StreamController<AIState>.broadcast();
      when(() => mockBloc.state).thenReturn(const AIInitial());
      when(() => mockBloc.stream).thenAnswer((_) => streamController.stream);
    });

    tearDown(() {
      streamController.close();
    });

    testWidgets('shows dialog with correct title', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AILoading(operationType: AIOperationType.timeEstimate),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: Builder(
              builder: (context) {
                return ElevatedButton(
                  onPressed: () {
                    showTimeEstimateDialog(
                      context,
                      taskTitle: 'Test task',
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
      await tester.pump();

      expect(find.text('AI Time Estimate'), findsOneWidget);
    });

    testWidgets('dialog is user-dismissible', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AILoading(operationType: AIOperationType.timeEstimate),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: Builder(
              builder: (context) {
                return ElevatedButton(
                  onPressed: () {
                    showTimeEstimateDialog(
                      context,
                      taskTitle: 'Test task',
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
      await tester.pump();

      // Dismiss by tapping barrier
      await tester.tapAt(const Offset(10, 10));
      await tester.pump();

      expect(find.text('AI Time Estimate'), findsNothing);
    });
  });

  group('TimeEstimateDialog', () {
    late MockAIBloc mockBloc;
    late StreamController<AIState> streamController;

    setUp(() {
      mockBloc = MockAIBloc();
      streamController = StreamController<AIState>.broadcast();
      when(() => mockBloc.state).thenReturn(const AIInitial());
      when(() => mockBloc.stream).thenAnswer((_) => streamController.stream);
      when(() => mockBloc.add(any())).thenReturn(null);
    });

    tearDown(() {
      streamController.close();
    });

    testWidgets('dispatches time estimate request on init', (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const TimeEstimateDialog(
              taskTitle: 'Build feature',
              taskDescription: 'Add new functionality',
              subtasks: ['Setup', 'Implementation', 'Testing'],
              skillLevel: SkillLevel.intermediate,
            ),
          ),
        ),
      );

      verify(() => mockBloc.add(any<AITimeEstimateRequested>())).called(1);
    });

    testWidgets('does not dispatch request for empty task title',
        (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const TimeEstimateDialog(taskTitle: ''),
          ),
        ),
      );

      verifyNever(() => mockBloc.add(any<AITimeEstimateRequested>()));
    });

    testWidgets('does not dispatch request for whitespace task title',
        (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const TimeEstimateDialog(taskTitle: '   '),
          ),
        ),
      );

      verifyNever(() => mockBloc.add(any<AITimeEstimateRequested>()));
    });

    testWidgets('shows loading state', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AILoading(operationType: AIOperationType.timeEstimate),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const TimeEstimateDialog(taskTitle: 'Test task'),
          ),
        ),
      );

      await tester.pump();

      expect(find.byType(CircularProgressIndicator), findsOneWidget);
      expect(find.text('AI is calculating time estimate...'), findsOneWidget);
    });

    testWidgets('shows success state with time estimates', (tester) async {
      final response = TimeEstimateResponse(
        taskTitle: 'Build feature',
        estimate: const TimeEstimate(
          optimisticMinutes: 60,
          realisticMinutes: 120,
          pessimisticMinutes: 180,
          confidence: 0.85,
        ),
        factors: [
          'Task complexity',
          'Required dependencies',
          'Testing requirements',
        ],
        assumptions: [
          'Developer has basic knowledge',
          'No major blockers',
        ],
      );

      when(() => mockBloc.state).thenReturn(
        AITimeEstimateSuccess(response: response),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const TimeEstimateDialog(taskTitle: 'Build feature'),
          ),
        ),
      );

      await tester.pump();

      expect(find.text('Task: Build feature'), findsOneWidget);
      expect(find.text('Optimistic (Best Case)'), findsOneWidget);
      expect(find.text('Realistic (Most Likely)'), findsOneWidget);
      expect(find.text('Pessimistic (Worst Case)'), findsOneWidget);
      expect(find.text('1h 0m'), findsOneWidget); // Optimistic
      expect(find.text('2h 0m'), findsOneWidget); // Realistic
      expect(find.text('3h 0m'), findsOneWidget); // Pessimistic
      expect(find.text('Confidence: 85%'), findsOneWidget);
    });

    testWidgets('formats time correctly for minutes only', (tester) async {
      final response = TimeEstimateResponse(
        taskTitle: 'Small task',
        estimate: const TimeEstimate(
          optimisticMinutes: 15,
          realisticMinutes: 30,
          pessimisticMinutes: 45,
          confidence: 0.9,
        ),
        factors: ['Simple task'],
        assumptions: ['Quick implementation'],
      );

      when(() => mockBloc.state).thenReturn(
        AITimeEstimateSuccess(response: response),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const TimeEstimateDialog(taskTitle: 'Small task'),
          ),
        ),
      );

      await tester.pump();

      expect(find.text('15m'), findsOneWidget);
      expect(find.text('30m'), findsOneWidget);
      expect(find.text('45m'), findsOneWidget);
    });

    testWidgets('displays factors considered', (tester) async {
      final response = TimeEstimateResponse(
        taskTitle: 'Test task',
        estimate: const TimeEstimate(
          optimisticMinutes: 60,
          realisticMinutes: 90,
          pessimisticMinutes: 120,
          confidence: 0.8,
        ),
        factors: [
          'Code complexity',
          'Testing requirements',
          'Documentation needs',
        ],
        assumptions: ['Has required tools'],
      );

      when(() => mockBloc.state).thenReturn(
        AITimeEstimateSuccess(response: response),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const TimeEstimateDialog(taskTitle: 'Test task'),
          ),
        ),
      );

      await tester.pump();

      expect(find.text('Factors Considered'), findsOneWidget);
      expect(find.text('Code complexity'), findsOneWidget);
      expect(find.text('Testing requirements'), findsOneWidget);
      expect(find.text('Documentation needs'), findsOneWidget);
    });

    testWidgets('displays assumptions', (tester) async {
      final response = TimeEstimateResponse(
        taskTitle: 'Test task',
        estimate: const TimeEstimate(
          optimisticMinutes: 60,
          realisticMinutes: 90,
          pessimisticMinutes: 120,
          confidence: 0.8,
        ),
        factors: ['Complexity'],
        assumptions: [
          'Developer has experience',
          'Environment is set up',
          'No external dependencies',
        ],
      );

      when(() => mockBloc.state).thenReturn(
        AITimeEstimateSuccess(response: response),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const TimeEstimateDialog(taskTitle: 'Test task'),
          ),
        ),
      );

      await tester.pump();

      expect(find.text('Assumptions'), findsOneWidget);
      expect(find.text('Developer has experience'), findsOneWidget);
      expect(find.text('Environment is set up'), findsOneWidget);
      expect(find.text('No external dependencies'), findsOneWidget);
    });

    testWidgets('shows error state with retry button', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AIFailure(
          message: 'Connection timeout',
          operationType: AIOperationType.timeEstimate,
        ),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const TimeEstimateDialog(taskTitle: 'Test task'),
          ),
        ),
      );

      await tester.pump();

      expect(find.text('Failed to get time estimate'), findsOneWidget);
      expect(find.text('Connection timeout'), findsOneWidget);
      expect(find.text('Retry'), findsOneWidget);
    });

    testWidgets('retry button dispatches new request', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AIFailure(
          message: 'Error',
          operationType: AIOperationType.timeEstimate,
        ),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const TimeEstimateDialog(taskTitle: 'Test task'),
          ),
        ),
      );

      await tester.pump();

      // Clear previous invocation from initState
      clearInteractions(mockBloc);

      // Tap retry button
      await tester.tap(find.text('Retry'));
      await tester.pump();

      verify(() => mockBloc.add(any<AITimeEstimateRequested>())).called(1);
    });

    testWidgets('close button dismisses dialog', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AILoading(operationType: AIOperationType.timeEstimate),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: BlocProvider<AIBloc>.value(
              value: mockBloc,
              child: Builder(
                builder: (context) {
                  return ElevatedButton(
                    onPressed: () {
                      showTimeEstimateDialog(context, taskTitle: 'Test');
                    },
                    child: const Text('Show'),
                  );
                },
              ),
            ),
          ),
        ),
      );

      await tester.tap(find.text('Show'));
      await tester.pump();

      await tester.tap(find.byIcon(Icons.close));
      await tester.pump();

      expect(find.text('AI Time Estimate'), findsNothing);
    });

    testWidgets('close button has tooltip', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AILoading(operationType: AIOperationType.timeEstimate),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const TimeEstimateDialog(taskTitle: 'Test task'),
          ),
        ),
      );

      await tester.pump();

      final closeButton = tester.widget<IconButton>(
        find.ancestor(
          of: find.byIcon(Icons.close),
          matching: find.byType(IconButton),
        ),
      );
      expect(closeButton.tooltip, 'Close dialog');
    });

    testWidgets('realistic estimate is highlighted', (tester) async {
      final response = TimeEstimateResponse(
        taskTitle: 'Test task',
        estimate: const TimeEstimate(
          optimisticMinutes: 60,
          realisticMinutes: 120,
          pessimisticMinutes: 180,
          confidence: 0.8,
        ),
        factors: ['Complexity'],
        assumptions: ['Has tools'],
      );

      when(() => mockBloc.state).thenReturn(
        AITimeEstimateSuccess(response: response),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const TimeEstimateDialog(taskTitle: 'Test task'),
          ),
        ),
      );

      await tester.pump();

      // The realistic estimate should be highlighted
      expect(find.text('Realistic (Most Likely)'), findsOneWidget);
    });
  });
}
