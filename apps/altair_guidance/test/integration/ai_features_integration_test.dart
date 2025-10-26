import 'dart:async';

import 'package:altair_core/altair_core.dart';
import 'package:altair_guidance/bloc/ai/ai_bloc.dart';
import 'package:altair_guidance/bloc/ai/ai_event.dart';
import 'package:altair_guidance/bloc/ai/ai_state.dart';
import 'package:altair_guidance/bloc/project/project_bloc.dart';
import 'package:altair_guidance/bloc/project/project_state.dart';
import 'package:altair_guidance/bloc/settings/settings_bloc.dart';
import 'package:altair_guidance/bloc/settings/settings_state.dart';
import 'package:altair_guidance/bloc/task/task_bloc.dart';
import 'package:altair_guidance/models/ai_settings.dart';
import 'package:altair_guidance/pages/task_edit_page.dart';
import 'package:altair_guidance/services/ai/models.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';

class MockTaskBloc extends Mock implements TaskBloc {}

class MockProjectBloc extends Mock implements ProjectBloc {}

class MockAIBloc extends Mock implements AIBloc {}

class MockSettingsBloc extends Mock implements SettingsBloc {}

void main() {
  late MockTaskBloc mockTaskBloc;
  late MockProjectBloc mockProjectBloc;
  late MockAIBloc mockAIBloc;
  late MockSettingsBloc mockSettingsBloc;
  late StreamController<ProjectState> projectStreamController;
  late StreamController<SettingsState> settingsStreamController;
  late StreamController<AIState> aiStreamController;

  setUpAll(() {
    registerFallbackValue(
      AITaskBreakdownRequested(
        request: TaskBreakdownRequest(taskTitle: 'Test'),
      ),
    );
    registerFallbackValue(
      AITimeEstimateRequested(
        request: TimeEstimateRequest(taskTitle: 'Test'),
      ),
    );
    registerFallbackValue(
      AIContextSuggestionsRequested(
        request: ContextSuggestionRequest(taskTitle: 'Test'),
      ),
    );
  });

  setUp(() {
    mockTaskBloc = MockTaskBloc();
    mockProjectBloc = MockProjectBloc();
    mockAIBloc = MockAIBloc();
    mockSettingsBloc = MockSettingsBloc();
    projectStreamController = StreamController<ProjectState>.broadcast();
    settingsStreamController = StreamController<SettingsState>.broadcast();
    aiStreamController = StreamController<AIState>.broadcast();

    when(() => mockProjectBloc.state).thenReturn(const ProjectLoaded(projects: []));
    when(() => mockProjectBloc.stream).thenAnswer((_) => projectStreamController.stream);

    when(() => mockSettingsBloc.state).thenReturn(
      const SettingsLoaded(AISettings(enabled: true)),
    );
    when(() => mockSettingsBloc.stream).thenAnswer((_) => settingsStreamController.stream);

    when(() => mockAIBloc.state).thenReturn(const AIInitial());
    when(() => mockAIBloc.stream).thenAnswer((_) => aiStreamController.stream);
    when(() => mockAIBloc.add(any())).thenReturn(null);
  });

  tearDown(() {
    projectStreamController.close();
    settingsStreamController.close();
    aiStreamController.close();
  });

  Widget createTaskEditPage({Task? task}) {
    return MaterialApp(
      home: MultiBlocProvider(
        providers: [
          BlocProvider<TaskBloc>.value(value: mockTaskBloc),
          BlocProvider<ProjectBloc>.value(value: mockProjectBloc),
          BlocProvider<AIBloc>.value(value: mockAIBloc),
          BlocProvider<SettingsBloc>.value(value: mockSettingsBloc),
        ],
        child: TaskEditPage(task: task),
      ),
    );
  }

  group('AI Features Integration - Break Down Task', () {
    testWidgets('clicking Break Down Task shows dialog and handles success response',
        (tester) async {
      await tester.pumpWidget(createTaskEditPage());

      // Enter task title to enable AI features
      await tester.enterText(
        find.byType(TextField).first,
        'Build a new feature',
      );
      await tester.pumpAndSettle();

      // Scroll to AI section
      await tester.drag(
        find.byType(SingleChildScrollView),
        const Offset(0, -500),
      );
      await tester.pumpAndSettle();

      // Find and tap Break Down Task button
      final breakdownButton = find.text('Break Down Task');
      expect(breakdownButton, findsOneWidget);

      await tester.tap(breakdownButton);
      await tester.pumpAndSettle();

      // Verify AIBloc received the breakdown request
      verify(() => mockAIBloc.add(any<AITaskBreakdownRequested>())).called(1);

      // Simulate AI loading state
      aiStreamController.add(
        const AILoading(operationType: AIOperationType.breakdown),
      );
      await tester.pump();

      // Verify loading indicator appears
      expect(find.byType(CircularProgressIndicator), findsWidgets);
      expect(find.text('AI is analyzing your task...'), findsOneWidget);

      // Simulate successful AI response
      final response = TaskBreakdownResponse(
        originalTask: 'Build a new feature',
        totalEstimatedMinutes: 120,
        reasoning: 'Breaking into manageable steps',
        subtasks: [
          SubtaskSuggestion(
            order: 1,
            title: 'Design the feature',
            description: 'Create mockups and specs',
            estimatedMinutes: 30,
          ),
          SubtaskSuggestion(
            order: 2,
            title: 'Implement core logic',
            description: 'Write the main functionality',
            estimatedMinutes: 60,
          ),
          SubtaskSuggestion(
            order: 3,
            title: 'Add tests',
            description: 'Write unit and integration tests',
            estimatedMinutes: 30,
          ),
        ],
      );

      aiStreamController.add(AITaskBreakdownSuccess(response: response));
      await tester.pumpAndSettle();

      // Verify success UI displays without errors
      expect(find.text('Breaking down: "Build a new feature"'), findsOneWidget);
      expect(find.text('Total estimated time: 120 minutes'), findsOneWidget);
      expect(find.text('Design the feature'), findsOneWidget);
      expect(find.text('Implement core logic'), findsOneWidget);
      expect(find.text('Add tests'), findsOneWidget);
      expect(find.text('Create Subtasks'), findsOneWidget);
    });

    testWidgets('clicking Break Down Task handles error response gracefully',
        (tester) async {
      await tester.pumpWidget(createTaskEditPage());

      // Enter task title
      await tester.enterText(
        find.byType(TextField).first,
        'Build a new feature',
      );
      await tester.pumpAndSettle();

      // Scroll to AI section
      await tester.drag(
        find.byType(SingleChildScrollView),
        const Offset(0, -500),
      );
      await tester.pumpAndSettle();

      // Tap Break Down Task button
      await tester.tap(find.text('Break Down Task'));
      await tester.pumpAndSettle();

      // Simulate AI loading
      aiStreamController.add(
        const AILoading(operationType: AIOperationType.breakdown),
      );
      await tester.pump();

      // Simulate AI error
      aiStreamController.add(
        const AIFailure(
          message: 'Connection timeout',
          operationType: AIOperationType.breakdown,
        ),
      );
      await tester.pumpAndSettle();

      // Verify error UI displays without crashing
      expect(find.text('Error: Connection timeout'), findsOneWidget);
      expect(find.text('Failed to break down task'), findsOneWidget);
      expect(find.text('Retry'), findsOneWidget);
    });
  });

  group('AI Features Integration - Estimate Time', () {
    testWidgets('clicking Estimate Time shows dialog and handles success response',
        (tester) async {
      await tester.pumpWidget(createTaskEditPage());

      // Enter task title
      await tester.enterText(
        find.byType(TextField).first,
        'Fix authentication bug',
      );
      await tester.pumpAndSettle();

      // Scroll to AI section
      await tester.drag(
        find.byType(SingleChildScrollView),
        const Offset(0, -500),
      );
      await tester.pumpAndSettle();

      // Tap Estimate Time button
      await tester.tap(find.text('Estimate Time'));
      await tester.pumpAndSettle();

      // Verify AIBloc received the estimate request
      verify(() => mockAIBloc.add(any<AITimeEstimateRequested>())).called(1);

      // Simulate loading
      aiStreamController.add(
        const AILoading(operationType: AIOperationType.timeEstimate),
      );
      await tester.pump();

      // Verify loading indicator
      expect(find.byType(CircularProgressIndicator), findsWidgets);

      // Simulate successful response
      final response = TimeEstimateResponse(
        taskTitle: 'Fix authentication bug',
        estimate: const TimeEstimate(
          optimisticMinutes: 30,
          realisticMinutes: 60,
          pessimisticMinutes: 120,
          confidence: 0.75,
        ),
        factors: [
          'Complexity of authentication system',
          'Testing requirements',
          'Documentation updates',
        ],
        assumptions: [
          'Developer has authentication knowledge',
          'Testing environment is available',
        ],
      );

      aiStreamController.add(AITimeEstimateSuccess(response: response));
      await tester.pumpAndSettle();

      // Verify success UI displays without errors
      expect(find.text('Time Estimate for "Fix authentication bug"'), findsOneWidget);
      expect(find.textContaining('30'), findsWidgets);
      expect(find.textContaining('60'), findsWidgets);
      expect(find.textContaining('120'), findsWidgets);
      expect(find.text('Based on similar tasks'), findsOneWidget);
    });

    testWidgets('clicking Estimate Time handles error response gracefully',
        (tester) async {
      await tester.pumpWidget(createTaskEditPage());

      // Enter task title
      await tester.enterText(find.byType(TextField).first, 'Test task');
      await tester.pumpAndSettle();

      // Scroll and tap
      await tester.drag(
        find.byType(SingleChildScrollView),
        const Offset(0, -500),
      );
      await tester.pumpAndSettle();
      await tester.tap(find.text('Estimate Time'));
      await tester.pumpAndSettle();

      // Simulate error
      aiStreamController.add(
        const AIFailure(
          message: 'API rate limit exceeded',
          operationType: AIOperationType.timeEstimate,
        ),
      );
      await tester.pumpAndSettle();

      // Verify error displays
      expect(find.text('Error: API rate limit exceeded'), findsOneWidget);
    });
  });

  group('AI Features Integration - Get Suggestions', () {
    testWidgets('clicking Get Suggestions shows dialog and handles success response',
        (tester) async {
      await tester.pumpWidget(createTaskEditPage());

      // Enter task title
      await tester.enterText(
        find.byType(TextField).first,
        'Optimize database queries',
      );
      await tester.pumpAndSettle();

      // Scroll to AI section
      await tester.drag(
        find.byType(SingleChildScrollView),
        const Offset(0, -500),
      );
      await tester.pumpAndSettle();

      // Tap Get Suggestions button
      await tester.tap(find.text('Get Suggestions'));
      await tester.pumpAndSettle();

      // Verify AIBloc received the suggestions request
      verify(() => mockAIBloc.add(any<AIContextSuggestionsRequested>())).called(1);

      // Simulate loading
      aiStreamController.add(
        const AILoading(operationType: AIOperationType.contextSuggestions),
      );
      await tester.pump();

      // Verify loading indicator
      expect(find.byType(CircularProgressIndicator), findsWidgets);

      // Simulate successful response
      final response = ContextSuggestionResponse(
        taskTitle: 'Optimize database queries',
        suggestions: [
          const ContextSuggestion(
            category: 'resource',
            title: 'Use indexes',
            description: 'Use indexes on frequently queried columns',
          ),
          const ContextSuggestion(
            category: 'tip',
            title: 'Check logs',
            description: 'Check database slow query log',
          ),
          const ContextSuggestion(
            category: 'tip',
            title: 'Consider caching',
            description: 'Consider query result caching',
          ),
        ],
      );

      aiStreamController.add(AIContextSuggestionsSuccess(response: response));
      await tester.pumpAndSettle();

      // Verify success UI displays without errors
      expect(find.text('Context Suggestions for "Optimize database queries"'), findsOneWidget);
      expect(find.text('Use indexes'), findsOneWidget);
      expect(find.text('Check logs'), findsOneWidget);
      expect(find.text('Consider caching'), findsOneWidget);
    });

    testWidgets('clicking Get Suggestions handles error response gracefully',
        (tester) async {
      await tester.pumpWidget(createTaskEditPage());

      // Enter task title
      await tester.enterText(find.byType(TextField).first, 'Test task');
      await tester.pumpAndSettle();

      // Scroll and tap
      await tester.drag(
        find.byType(SingleChildScrollView),
        const Offset(0, -500),
      );
      await tester.pumpAndSettle();
      await tester.tap(find.text('Get Suggestions'));
      await tester.pumpAndSettle();

      // Simulate error
      aiStreamController.add(
        const AIFailure(
          message: 'Service temporarily unavailable',
          operationType: AIOperationType.contextSuggestions,
        ),
      );
      await tester.pumpAndSettle();

      // Verify error displays
      expect(find.text('Error: Service temporarily unavailable'), findsOneWidget);
    });
  });
}
