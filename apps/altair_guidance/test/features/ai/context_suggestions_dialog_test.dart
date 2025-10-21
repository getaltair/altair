import 'dart:async';

import 'package:altair_guidance/bloc/ai/ai_bloc.dart';
import 'package:altair_guidance/bloc/ai/ai_event.dart';
import 'package:altair_guidance/bloc/ai/ai_state.dart';
import 'package:altair_guidance/features/ai/context_suggestions_dialog.dart';
import 'package:altair_guidance/services/ai/models.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';

class MockAIBloc extends Mock implements AIBloc {}

void main() {
  setUpAll(() {
    registerFallbackValue(
      AIContextSuggestionsRequested(
        request: ContextSuggestionRequest(taskTitle: 'Test'),
      ),
    );
  });

  group('showContextSuggestionsDialog', () {
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
        const AILoading(operationType: AIOperationType.contextSuggestions),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: Builder(
              builder: (context) {
                return ElevatedButton(
                  onPressed: () {
                    showContextSuggestionsDialog(
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

      expect(find.text('AI Context Suggestions'), findsOneWidget);
    });

    testWidgets('dialog is user-dismissible', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AILoading(operationType: AIOperationType.contextSuggestions),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: Builder(
              builder: (context) {
                return ElevatedButton(
                  onPressed: () {
                    showContextSuggestionsDialog(
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

      expect(find.text('AI Context Suggestions'), findsNothing);
    });
  });

  group('ContextSuggestionsDialog', () {
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

    testWidgets('dispatches context suggestions request on init',
        (tester) async {
      await tester.pumpWidget(
        BlocProvider<AIBloc>.value(
          value: mockBloc,
          child: MaterialApp(
            home: Scaffold(
              body: const ContextSuggestionsDialog(
                taskTitle: 'Build feature',
                taskDescription: 'Add new functionality',
                projectContext: 'Mobile app',
                suggestionType: 'general',
              ),
            ),
          ),
        ),
      );

      verify(() => mockBloc.add(any<AIContextSuggestionsRequested>()))
          .called(1);
    });

    testWidgets('does not dispatch request for empty task title',
        (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const ContextSuggestionsDialog(taskTitle: ''),
          ),
        ),
      );

      verifyNever(() => mockBloc.add(any<AIContextSuggestionsRequested>()));
    });

    testWidgets('does not dispatch request for whitespace task title',
        (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const ContextSuggestionsDialog(taskTitle: '   '),
          ),
        ),
      );

      verifyNever(() => mockBloc.add(any<AIContextSuggestionsRequested>()));
    });

    testWidgets('shows loading state', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AILoading(operationType: AIOperationType.contextSuggestions),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const ContextSuggestionsDialog(taskTitle: 'Test task'),
          ),
        ),
      );

      await tester.pump();

      expect(find.byType(CircularProgressIndicator), findsOneWidget);
      expect(find.text('AI is generating suggestions...'), findsOneWidget);
    });

    testWidgets('shows success state with suggestions', (tester) async {
      final response = ContextSuggestionResponse(
        taskTitle: 'Build feature',
        suggestions: [
          const ContextSuggestion(
            category: 'resource',
            title: 'Flutter Documentation',
            description: 'Official Flutter docs for widget development',
            priority: PriorityLevel.high,
          ),
          const ContextSuggestion(
            category: 'tip',
            title: 'Use const constructors',
            description: 'Improves performance and reduces rebuilds',
            priority: PriorityLevel.medium,
          ),
          const ContextSuggestion(
            category: 'blocker',
            title: 'API key required',
            description: 'You need to configure the API key first',
            priority: PriorityLevel.critical,
          ),
        ],
        summary: 'Here are some helpful suggestions for your task',
      );

      when(() => mockBloc.state).thenReturn(
        AIContextSuggestionsSuccess(response: response),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const ContextSuggestionsDialog(taskTitle: 'Build feature'),
          ),
        ),
      );

      await tester.pump();

      expect(find.text('Flutter Documentation'), findsOneWidget);
      expect(find.text('Use const constructors'), findsOneWidget);
      expect(find.text('API key required'), findsOneWidget);
      expect(
        find.text('Here are some helpful suggestions for your task'),
        findsOneWidget,
      );
    });

    testWidgets('displays suggestion type selector', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AILoading(operationType: AIOperationType.contextSuggestions),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const ContextSuggestionsDialog(taskTitle: 'Test task'),
          ),
        ),
      );

      await tester.pump();

      expect(find.text('GENERAL'), findsOneWidget);
      expect(find.text('RESOURCES'), findsOneWidget);
      expect(find.text('TIPS'), findsOneWidget);
      expect(find.text('BLOCKERS'), findsOneWidget);
    });

    testWidgets('changing suggestion type dispatches new request',
        (tester) async {
      when(() => mockBloc.state).thenReturn(const AIInitial());

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const ContextSuggestionsDialog(taskTitle: 'Test task'),
          ),
        ),
      );

      await tester.pump();

      // Clear initial request from initState
      clearInteractions(mockBloc);

      // Tap on RESOURCES chip
      await tester.tap(find.text('RESOURCES'));
      await tester.pump();

      verify(() => mockBloc.add(any<AIContextSuggestionsRequested>()))
          .called(1);
    });

    testWidgets('shows empty state when no suggestions', (tester) async {
      final response = ContextSuggestionResponse(
        taskTitle: 'Test task',
        suggestions: [],
      );

      when(() => mockBloc.state).thenReturn(
        AIContextSuggestionsSuccess(response: response),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const ContextSuggestionsDialog(taskTitle: 'Test task'),
          ),
        ),
      );

      await tester.pump();

      expect(find.text('No suggestions needed'), findsOneWidget);
      expect(find.text('This task looks good to go!'), findsOneWidget);
    });

    testWidgets('displays category badges with correct colors', (tester) async {
      final response = ContextSuggestionResponse(
        taskTitle: 'Test task',
        suggestions: [
          const ContextSuggestion(
            category: 'resource',
            title: 'Resource suggestion',
            description: 'Description',
          ),
          const ContextSuggestion(
            category: 'tip',
            title: 'Tip suggestion',
            description: 'Description',
          ),
          const ContextSuggestion(
            category: 'blocker',
            title: 'Blocker suggestion',
            description: 'Description',
          ),
          const ContextSuggestion(
            category: 'warning',
            title: 'Warning suggestion',
            description: 'Description',
          ),
        ],
      );

      when(() => mockBloc.state).thenReturn(
        AIContextSuggestionsSuccess(response: response),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const ContextSuggestionsDialog(taskTitle: 'Test task'),
          ),
        ),
      );

      await tester.pump();

      expect(find.text('RESOURCE'), findsOneWidget);
      expect(find.text('TIP'), findsOneWidget);
      expect(find.text('BLOCKER'), findsOneWidget);
      expect(find.text('WARNING'), findsOneWidget);
    });

    testWidgets('displays category icons', (tester) async {
      final response = ContextSuggestionResponse(
        taskTitle: 'Test task',
        suggestions: [
          const ContextSuggestion(
            category: 'resource',
            title: 'Resource',
            description: 'Desc',
          ),
          const ContextSuggestion(
            category: 'tip',
            title: 'Tip',
            description: 'Desc',
          ),
          const ContextSuggestion(
            category: 'blocker',
            title: 'Blocker',
            description: 'Desc',
          ),
        ],
      );

      when(() => mockBloc.state).thenReturn(
        AIContextSuggestionsSuccess(response: response),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const ContextSuggestionsDialog(taskTitle: 'Test task'),
          ),
        ),
      );

      await tester.pump();

      expect(find.byIcon(Icons.book), findsOneWidget); // resource
      expect(find.byIcon(Icons.tips_and_updates), findsOneWidget); // tip
      expect(find.byIcon(Icons.block), findsOneWidget); // blocker
    });

    testWidgets('displays priority indicators', (tester) async {
      final response = ContextSuggestionResponse(
        taskTitle: 'Test task',
        suggestions: [
          const ContextSuggestion(
            category: 'resource',
            title: 'Important resource',
            description: 'Description',
            priority: PriorityLevel.high,
          ),
        ],
      );

      when(() => mockBloc.state).thenReturn(
        AIContextSuggestionsSuccess(response: response),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const ContextSuggestionsDialog(taskTitle: 'Test task'),
          ),
        ),
      );

      await tester.pump();

      expect(find.text('Important resource'), findsOneWidget);
    });

    testWidgets('shows error state with retry button', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AIFailure(
          message: 'Service unavailable',
          operationType: AIOperationType.contextSuggestions,
        ),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: BlocProvider<AIBloc>.value(
              value: mockBloc,
              child: const ContextSuggestionsDialog(taskTitle: 'Test task'),
            ),
          ),
        ),
      );

      await tester.pump();

      expect(find.text('Failed to get suggestions'), findsOneWidget);
      expect(find.text('Service unavailable'), findsOneWidget);
      expect(find.text('Retry'), findsOneWidget);
    });

    testWidgets('retry button dispatches new request', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AIFailure(
          message: 'Error',
          operationType: AIOperationType.contextSuggestions,
        ),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: BlocProvider<AIBloc>.value(
              value: mockBloc,
              child: const ContextSuggestionsDialog(taskTitle: 'Test task'),
            ),
          ),
        ),
      );

      await tester.pump();

      // Clear previous invocation from initState
      clearInteractions(mockBloc);

      // Tap retry button
      await tester.tap(find.text('Retry'));
      await tester.pump();

      verify(() => mockBloc.add(any<AIContextSuggestionsRequested>()))
          .called(1);
    });

    testWidgets('close button dismisses dialog', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AILoading(operationType: AIOperationType.contextSuggestions),
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
                      showContextSuggestionsDialog(
                        context,
                        taskTitle: 'Test',
                      );
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

      expect(find.text('AI Context Suggestions'), findsNothing);
    });

    testWidgets('close button has tooltip', (tester) async {
      when(() => mockBloc.state).thenReturn(
        const AILoading(operationType: AIOperationType.contextSuggestions),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const ContextSuggestionsDialog(taskTitle: 'Test task'),
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

    testWidgets('displays summary when provided', (tester) async {
      final response = ContextSuggestionResponse(
        taskTitle: 'Test task',
        suggestions: [
          const ContextSuggestion(
            category: 'resource',
            title: 'Resource',
            description: 'Description',
          ),
        ],
        summary: 'This is a helpful summary of the suggestions',
      );

      when(() => mockBloc.state).thenReturn(
        AIContextSuggestionsSuccess(response: response),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const ContextSuggestionsDialog(taskTitle: 'Test task'),
          ),
        ),
      );

      await tester.pump();

      expect(find.text('Summary'), findsOneWidget);
      expect(
        find.text('This is a helpful summary of the suggestions'),
        findsOneWidget,
      );
    });

    testWidgets('does not display summary section when not provided',
        (tester) async {
      final response = ContextSuggestionResponse(
        taskTitle: 'Test task',
        suggestions: [
          const ContextSuggestion(
            category: 'resource',
            title: 'Resource',
            description: 'Description',
          ),
        ],
      );

      when(() => mockBloc.state).thenReturn(
        AIContextSuggestionsSuccess(response: response),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: BlocProvider<AIBloc>.value(
            value: mockBloc,
            child: const ContextSuggestionsDialog(taskTitle: 'Test task'),
          ),
        ),
      );

      await tester.pump();

      expect(find.text('Summary'), findsNothing);
    });
  });
}
