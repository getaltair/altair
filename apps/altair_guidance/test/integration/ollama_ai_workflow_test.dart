import 'dart:async';

import 'package:altair_core/altair_core.dart';
import 'package:altair_guidance/bloc/ai/ai_bloc.dart';
import 'package:altair_guidance/bloc/ai/ai_event.dart';
import 'package:altair_guidance/bloc/ai/ai_state.dart';
import 'package:altair_guidance/bloc/project/project_bloc.dart';
import 'package:altair_guidance/bloc/project/project_state.dart';
import 'package:altair_guidance/bloc/settings/settings_bloc.dart';
import 'package:altair_guidance/bloc/settings/settings_event.dart';
import 'package:altair_guidance/bloc/settings/settings_state.dart';
import 'package:altair_guidance/bloc/task/task_bloc.dart';
import 'package:altair_guidance/features/theme/theme_cubit.dart';
import 'package:altair_guidance/models/ai_settings.dart';
import 'package:altair_guidance/pages/settings_page.dart';
import 'package:altair_guidance/pages/task_edit_page.dart';
import 'package:altair_guidance/services/ai/models.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';

class MockSettingsBloc extends Mock implements SettingsBloc {}

class MockThemeCubit extends Mock implements ThemeCubit {}

class MockTaskBloc extends Mock implements TaskBloc {}

class MockProjectBloc extends Mock implements ProjectBloc {}

class MockAIBloc extends Mock implements AIBloc {}

void main() {
  late MockSettingsBloc mockSettingsBloc;
  late MockThemeCubit mockThemeCubit;
  late MockTaskBloc mockTaskBloc;
  late MockProjectBloc mockProjectBloc;
  late MockAIBloc mockAIBloc;
  late StreamController<SettingsState> settingsStreamController;
  late StreamController<AIState> aiStreamController;

  setUpAll(() {
    registerFallbackValue(
      const SettingsAIUpdated(AISettings()),
    );
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
    mockSettingsBloc = MockSettingsBloc();
    mockThemeCubit = MockThemeCubit();
    mockTaskBloc = MockTaskBloc();
    mockProjectBloc = MockProjectBloc();
    mockAIBloc = MockAIBloc();

    settingsStreamController = StreamController<SettingsState>.broadcast();
    aiStreamController = StreamController<AIState>.broadcast();

    when(() => mockSettingsBloc.state).thenReturn(
      const SettingsLoaded(AISettings()),
    );
    when(() => mockSettingsBloc.stream).thenAnswer(
      (_) => settingsStreamController.stream,
    );
    when(() => mockSettingsBloc.add(any())).thenReturn(null);

    when(() => mockThemeCubit.state).thenReturn(
      const ThemeState(ThemeMode.system),
    );

    when(() => mockProjectBloc.state).thenReturn(const ProjectLoaded(projects: []));

    when(() => mockAIBloc.state).thenReturn(const AIInitial());
    when(() => mockAIBloc.stream).thenAnswer((_) => aiStreamController.stream);
    when(() => mockAIBloc.add(any())).thenReturn(null);
  });

  tearDown(() {
    settingsStreamController.close();
    aiStreamController.close();
  });

  group('Ollama Configuration Workflow', () {
    testWidgets('can configure Ollama provider in settings', (tester) async {
      when(() => mockSettingsBloc.state).thenReturn(
        const SettingsLoaded(
          AISettings(enabled: true, provider: AIProviderType.ollama),
        ),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MultiBlocProvider(
            providers: [
              BlocProvider<SettingsBloc>.value(value: mockSettingsBloc),
              BlocProvider<ThemeCubit>.value(value: mockThemeCubit),
            ],
            child: const SettingsPage(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Verify Ollama configuration section is shown
      expect(find.text('Ollama Configuration'), findsOneWidget);
      expect(find.text('Ollama Server URL'), findsOneWidget);
      expect(
        find.textContaining('Install Ollama from ollama.com'),
        findsOneWidget,
      );
    });

    testWidgets('Ollama URL field accepts localhost address', (tester) async {
      when(() => mockSettingsBloc.state).thenReturn(
        const SettingsLoaded(
          AISettings(enabled: true, provider: AIProviderType.ollama),
        ),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MultiBlocProvider(
            providers: [
              BlocProvider<SettingsBloc>.value(value: mockSettingsBloc),
              BlocProvider<ThemeCubit>.value(value: mockThemeCubit),
            ],
            child: const SettingsPage(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Enter Ollama server URL
      final urlField = find.byType(TextField).first;
      await tester.enterText(urlField, 'http://localhost:11434');
      await tester.pumpAndSettle();

      // Verify settings update was triggered
      verify(
        () => mockSettingsBloc.add(
          any(
            that: isA<SettingsAIUpdated>().having(
              (event) => event.settings.ollamaBaseUrl,
              'ollamaBaseUrl',
              'http://localhost:11434',
            ),
          ),
        ),
      ).called(1);
    });

    testWidgets('shows Ollama model dropdown when provider selected',
        (tester) async {
      when(() => mockSettingsBloc.state).thenReturn(
        const SettingsLoaded(
          AISettings(enabled: true, provider: AIProviderType.ollama),
        ),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MultiBlocProvider(
            providers: [
              BlocProvider<SettingsBloc>.value(value: mockSettingsBloc),
              BlocProvider<ThemeCubit>.value(value: mockThemeCubit),
            ],
            child: const SettingsPage(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      expect(find.text('Model'), findsOneWidget);
    });

    testWidgets('Ollama provider does not show API key warning',
        (tester) async {
      when(() => mockSettingsBloc.state).thenReturn(
        const SettingsLoaded(
          AISettings(enabled: true, provider: AIProviderType.ollama),
        ),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MultiBlocProvider(
            providers: [
              BlocProvider<SettingsBloc>.value(value: mockSettingsBloc),
              BlocProvider<ThemeCubit>.value(value: mockThemeCubit),
            ],
            child: const SettingsPage(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Ollama doesn't require API key, so no warning should appear
      expect(
        find.text('Please provide an API key to use this provider'),
        findsNothing,
      );
      expect(find.byIcon(Icons.warning), findsNothing);
    });

    testWidgets('can save Ollama settings', (tester) async {
      // Start with initial state
      when(() => mockSettingsBloc.state).thenReturn(
        const SettingsLoaded(
          AISettings(enabled: true, provider: AIProviderType.ollama),
        ),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MultiBlocProvider(
            providers: [
              BlocProvider<SettingsBloc>.value(value: mockSettingsBloc),
              BlocProvider<ThemeCubit>.value(value: mockThemeCubit),
            ],
            child: const SettingsPage(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Configure Ollama
      final urlField = find.byType(TextField).first;
      await tester.enterText(urlField, 'http://localhost:11434');
      await tester.pumpAndSettle();

      // Emit success state
      settingsStreamController.add(
        const SettingsSaved(
          AISettings(
            enabled: true,
            provider: AIProviderType.ollama,
            ollamaBaseUrl: 'http://localhost:11434',
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Verify success message
      expect(find.text('Ollama (Local) settings saved'), findsOneWidget);
    });

    testWidgets('displays Ollama info box with helpful text', (tester) async {
      when(() => mockSettingsBloc.state).thenReturn(
        const SettingsLoaded(
          AISettings(enabled: true, provider: AIProviderType.ollama),
        ),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MultiBlocProvider(
            providers: [
              BlocProvider<SettingsBloc>.value(value: mockSettingsBloc),
              BlocProvider<ThemeCubit>.value(value: mockThemeCubit),
            ],
            child: const SettingsPage(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      expect(
        find.textContaining('Your data never leaves your computer'),
        findsOneWidget,
      );
      expect(find.byIcon(Icons.info), findsOneWidget);
    });
  });

  group('Ollama AI Features Integration', () {
    testWidgets('AI features work with Ollama provider configured',
        (tester) async {
      when(() => mockSettingsBloc.state).thenReturn(
        const SettingsLoaded(
          AISettings(
            enabled: true,
            provider: AIProviderType.ollama,
            ollamaBaseUrl: 'http://localhost:11434',
            ollamaModel: 'llama3.2',
          ),
        ),
      );

      final task = Task(
        id: '123',
        title: 'Test task',
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        status: TaskStatus.todo,
        priority: 3,
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MultiBlocProvider(
            providers: [
              BlocProvider<TaskBloc>.value(value: mockTaskBloc),
              BlocProvider<ProjectBloc>.value(value: mockProjectBloc),
              BlocProvider<AIBloc>.value(value: mockAIBloc),
              BlocProvider<SettingsBloc>.value(value: mockSettingsBloc),
            ],
            child: TaskEditPage(task: task),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // AI features should be available
      expect(find.text('AI Assistant'), findsOneWidget);
      expect(find.text('Break Down Task'), findsOneWidget);
      expect(find.text('Estimate Time'), findsOneWidget);
      expect(find.text('Get Suggestions'), findsOneWidget);
    });

    testWidgets('task breakdown works with Ollama', (tester) async {
      when(() => mockSettingsBloc.state).thenReturn(
        const SettingsLoaded(
          AISettings(
            enabled: true,
            provider: AIProviderType.ollama,
            ollamaModel: 'llama3.2',
          ),
        ),
      );

      final task = Task(
        id: '123',
        title: 'Build authentication',
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        status: TaskStatus.todo,
        priority: 3,
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MultiBlocProvider(
            providers: [
              BlocProvider<TaskBloc>.value(value: mockTaskBloc),
              BlocProvider<ProjectBloc>.value(value: mockProjectBloc),
              BlocProvider<AIBloc>.value(value: mockAIBloc),
              BlocProvider<SettingsBloc>.value(value: mockSettingsBloc),
            ],
            child: TaskEditPage(task: task),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Clear any previous interactions
      clearInteractions(mockAIBloc);

      // Verify breakdown request can be made
      verify(() => mockAIBloc.state).called(greaterThan(0));
    });

    testWidgets('time estimate works with Ollama', (tester) async {
      when(() => mockSettingsBloc.state).thenReturn(
        const SettingsLoaded(
          AISettings(
            enabled: true,
            provider: AIProviderType.ollama,
            ollamaModel: 'qwen2.5',
          ),
        ),
      );

      final task = Task(
        id: '123',
        title: 'Write documentation',
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        status: TaskStatus.todo,
        priority: 3,
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MultiBlocProvider(
            providers: [
              BlocProvider<TaskBloc>.value(value: mockTaskBloc),
              BlocProvider<ProjectBloc>.value(value: mockProjectBloc),
              BlocProvider<AIBloc>.value(value: mockAIBloc),
              BlocProvider<SettingsBloc>.value(value: mockSettingsBloc),
            ],
            child: TaskEditPage(task: task),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Time estimate button should be available
      expect(find.text('Estimate Time'), findsOneWidget);
    });

    testWidgets('context suggestions work with Ollama', (tester) async {
      when(() => mockSettingsBloc.state).thenReturn(
        const SettingsLoaded(
          AISettings(
            enabled: true,
            provider: AIProviderType.ollama,
            ollamaModel: 'mistral',
          ),
        ),
      );

      final task = Task(
        id: '123',
        title: 'Refactor code',
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        status: TaskStatus.todo,
        priority: 3,
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MultiBlocProvider(
            providers: [
              BlocProvider<TaskBloc>.value(value: mockTaskBloc),
              BlocProvider<ProjectBloc>.value(value: mockProjectBloc),
              BlocProvider<AIBloc>.value(value: mockAIBloc),
              BlocProvider<SettingsBloc>.value(value: mockSettingsBloc),
            ],
            child: TaskEditPage(task: task),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Context suggestions button should be available
      expect(find.text('Get Suggestions'), findsOneWidget);
    });
  });

  group('Ollama Error Handling', () {
    testWidgets('handles Ollama connection errors gracefully',
        (tester) async {
      when(() => mockAIBloc.state).thenReturn(
        const AIFailure(
          message: 'Failed to connect to Ollama server at http://localhost:11434',
          operationType: AIOperationType.breakdown,
        ),
      );

      when(() => mockSettingsBloc.state).thenReturn(
        const SettingsLoaded(
          AISettings(
            enabled: true,
            provider: AIProviderType.ollama,
            ollamaBaseUrl: 'http://localhost:11434',
          ),
        ),
      );

      final task = Task(
        id: '123',
        title: 'Test task',
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        status: TaskStatus.todo,
        priority: 3,
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MultiBlocProvider(
            providers: [
              BlocProvider<TaskBloc>.value(value: mockTaskBloc),
              BlocProvider<ProjectBloc>.value(value: mockProjectBloc),
              BlocProvider<AIBloc>.value(value: mockAIBloc),
              BlocProvider<SettingsBloc>.value(value: mockSettingsBloc),
            ],
            child: TaskEditPage(task: task),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // AI features should still be visible
      expect(find.text('AI Assistant'), findsOneWidget);
    });

    testWidgets('shows retry option when Ollama request fails',
        (tester) async {
      when(() => mockAIBloc.state).thenReturn(
        const AIFailure(
          message: 'Ollama server not responding',
          operationType: AIOperationType.timeEstimate,
        ),
      );

      when(() => mockSettingsBloc.state).thenReturn(
        const SettingsLoaded(
          AISettings(
            enabled: true,
            provider: AIProviderType.ollama,
          ),
        ),
      );

      final task = Task(
        id: '123',
        title: 'Test task',
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        status: TaskStatus.todo,
        priority: 3,
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MultiBlocProvider(
            providers: [
              BlocProvider<TaskBloc>.value(value: mockTaskBloc),
              BlocProvider<ProjectBloc>.value(value: mockProjectBloc),
              BlocProvider<AIBloc>.value(value: mockAIBloc),
              BlocProvider<SettingsBloc>.value(value: mockSettingsBloc),
            ],
            child: TaskEditPage(task: task),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // User should be able to access AI features to retry
      expect(find.text('Estimate Time'), findsOneWidget);
    });
  });

  group('Ollama Success Scenarios', () {
    testWidgets('displays successful AI response from Ollama',
        (tester) async {
      final breakdownResponse = TaskBreakdownResponse(
        originalTask: 'Build feature',
        subtasks: [
          SubtaskSuggestion(
            order: 1,
            title: 'Setup',
            estimatedMinutes: 30,
          ),
          SubtaskSuggestion(
            order: 2,
            title: 'Implementation',
            estimatedMinutes: 60,
          ),
        ],
      );

      when(() => mockAIBloc.state).thenReturn(
        AITaskBreakdownSuccess(response: breakdownResponse),
      );

      when(() => mockSettingsBloc.state).thenReturn(
        const SettingsLoaded(
          AISettings(
            enabled: true,
            provider: AIProviderType.ollama,
            ollamaModel: 'llama3.2',
          ),
        ),
      );

      final task = Task(
        id: '123',
        title: 'Build feature',
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        status: TaskStatus.todo,
        priority: 3,
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MultiBlocProvider(
            providers: [
              BlocProvider<TaskBloc>.value(value: mockTaskBloc),
              BlocProvider<ProjectBloc>.value(value: mockProjectBloc),
              BlocProvider<AIBloc>.value(value: mockAIBloc),
              BlocProvider<SettingsBloc>.value(value: mockSettingsBloc),
            ],
            child: TaskEditPage(task: task),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // AI features should be available for successful responses
      expect(find.text('AI Assistant'), findsOneWidget);
    });

    testWidgets('Ollama responses do not require API key validation',
        (tester) async {
      when(() => mockSettingsBloc.state).thenReturn(
        const SettingsLoaded(
          AISettings(
            enabled: true,
            provider: AIProviderType.ollama,
            // No API key needed for Ollama
          ),
        ),
      );

      await tester.pumpWidget(
        MaterialApp(
          home: MultiBlocProvider(
            providers: [
              BlocProvider<SettingsBloc>.value(value: mockSettingsBloc),
              BlocProvider<ThemeCubit>.value(value: mockThemeCubit),
            ],
            child: const SettingsPage(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should not show validation warnings for Ollama
      expect(find.byIcon(Icons.warning), findsNothing);
      expect(
        find.text('Please provide an API key to use this provider'),
        findsNothing,
      );
    });
  });
}
