import 'dart:async';

import 'package:altair_guidance/bloc/ai/ai_bloc.dart';
import 'package:altair_guidance/bloc/ai/ai_event.dart';
import 'package:altair_guidance/bloc/ai/ai_state.dart';
import 'package:altair_guidance/bloc/project/project_bloc.dart';
import 'package:altair_guidance/bloc/project/project_state.dart';
import 'package:altair_guidance/bloc/settings/settings_bloc.dart';
import 'package:altair_guidance/bloc/settings/settings_event.dart';
import 'package:altair_guidance/bloc/settings/settings_state.dart';
import 'package:altair_guidance/features/theme/theme_cubit.dart';
import 'package:altair_guidance/models/ai_settings.dart';
import 'package:altair_guidance/pages/settings_page.dart';
import 'package:altair_guidance/services/ai/models.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';

class MockSettingsBloc extends Mock implements SettingsBloc {}

class MockThemeCubit extends Mock implements ThemeCubit {}

class MockProjectBloc extends Mock implements ProjectBloc {}

class MockAIBloc extends Mock implements AIBloc {}

void main() {
  late MockSettingsBloc mockSettingsBloc;
  late MockThemeCubit mockThemeCubit;
  late MockProjectBloc mockProjectBloc;
  late MockAIBloc mockAIBloc;
  late StreamController<SettingsState> settingsStreamController;
  late StreamController<ThemeState> themeStreamController;
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
    mockProjectBloc = MockProjectBloc();
    mockAIBloc = MockAIBloc();

    settingsStreamController = StreamController<SettingsState>.broadcast();
    themeStreamController = StreamController<ThemeState>.broadcast();
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
    when(() => mockThemeCubit.stream).thenAnswer(
      (_) => themeStreamController.stream,
    );

    when(() => mockProjectBloc.state)
        .thenReturn(const ProjectLoaded(projects: []));

    when(() => mockAIBloc.state).thenReturn(const AIInitial());
    when(() => mockAIBloc.stream).thenAnswer((_) => aiStreamController.stream);
    when(() => mockAIBloc.add(any())).thenReturn(null);
  });

  tearDown(() {
    settingsStreamController.close();
    themeStreamController.close();
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

      await tester.pump(const Duration(milliseconds: 100));

      // Verify Ollama configuration section is shown
      expect(find.text('Ollama Configuration'), findsOneWidget);
      expect(find.text('Ollama Server URL'), findsOneWidget);
      expect(
        find.textContaining('Install Ollama from ollama.com'),
        findsOneWidget,
      );
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

      await tester.pump(const Duration(milliseconds: 100));

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

      await tester.pump(const Duration(milliseconds: 100));

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

      await tester.pump(const Duration(milliseconds: 100));

      // Configure Ollama
      final urlField = find.byType(TextField).first;
      await tester.enterText(urlField, 'http://localhost:11434');
      await tester.pump(const Duration(milliseconds: 100));

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

      await tester.pump(const Duration(milliseconds: 100));

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

      await tester.pump(const Duration(milliseconds: 100));

      expect(
        find.textContaining('Your data never leaves your computer'),
        findsOneWidget,
      );
      expect(find.byIcon(Icons.info), findsOneWidget);
    });
  });
}
