import 'package:altair_guidance/bloc/settings/settings_bloc.dart';
import 'package:altair_guidance/bloc/settings/settings_event.dart';
import 'package:altair_guidance/bloc/settings/settings_state.dart';
import 'package:altair_guidance/features/theme/theme_cubit.dart';
import 'package:altair_guidance/models/ai_settings.dart';
import 'package:altair_guidance/pages/settings_page.dart';
import 'package:altair_guidance/repositories/ai_settings_repository.dart';
import 'package:bloc_test/bloc_test.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';

class MockSettingsBloc extends MockBloc<SettingsEvent, SettingsState>
    implements SettingsBloc {}

class MockThemeCubit extends MockCubit<ThemeState> implements ThemeCubit {}

class MockAISettingsRepository extends Mock implements AISettingsRepository {}

class FakeSettingsEvent extends Fake implements SettingsEvent {}

class FakeThemeState extends Fake implements ThemeState {}

void main() {
  late MockSettingsBloc mockSettingsBloc;
  late MockThemeCubit mockThemeCubit;

  setUpAll(() {
    registerFallbackValue(FakeSettingsEvent());
    registerFallbackValue(FakeThemeState());
  });

  setUp(() {
    mockSettingsBloc = MockSettingsBloc();
    mockThemeCubit = MockThemeCubit();
  });

  Widget createSettingsPage() {
    return MaterialApp(
      home: MultiBlocProvider(
        providers: [
          BlocProvider<SettingsBloc>.value(value: mockSettingsBloc),
          BlocProvider<ThemeCubit>.value(value: mockThemeCubit),
        ],
        child: const SettingsPage(),
      ),
    );
  }

  group('SettingsPage', () {
    testWidgets('renders with all main sections', (tester) async {
      when(() => mockSettingsBloc.state).thenReturn(
        const SettingsLoaded(AISettings()),
      );
      when(() => mockThemeCubit.state).thenReturn(
        const ThemeState(ThemeMode.system),
      );

      await tester.pumpWidget(createSettingsPage());

      // Check for main sections
      expect(find.text('Settings'), findsOneWidget);
      expect(find.text('APPEARANCE'), findsOneWidget);
      expect(find.text('AI FEATURES'), findsOneWidget);

      // Scroll to find ABOUT section
      await tester.drag(find.byType(ListView), const Offset(0, -500));
      await tester.pumpAndSettle();
      expect(find.text('ABOUT'), findsOneWidget);
    });

    group('Theme Section', () {
      testWidgets('displays all theme options', (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(AISettings()),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        expect(find.text('Light'), findsOneWidget);
        expect(find.text('Always use light theme'), findsOneWidget);
        expect(find.text('Dark'), findsOneWidget);
        expect(find.text('Always use dark theme'), findsOneWidget);
        expect(find.text('Follow System'), findsOneWidget);
        expect(find.text('Match your device settings'), findsOneWidget);
      });

      testWidgets('shows light theme as selected', (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(AISettings()),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.light),
        );

        await tester.pumpWidget(createSettingsPage());

        // Light theme option should have a checkmark
        expect(find.byIcon(Icons.check_circle), findsWidgets);
      });

      testWidgets('taps light theme option', (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(AISettings()),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.dark),
        );

        await tester.pumpWidget(createSettingsPage());
        await tester.tap(find.text('Light'));
        await tester.pumpAndSettle();

        verify(() => mockThemeCubit.setThemeMode(ThemeMode.light)).called(1);
      });

      testWidgets('taps dark theme option', (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(AISettings()),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.light),
        );

        await tester.pumpWidget(createSettingsPage());
        await tester.tap(find.text('Dark'));
        await tester.pumpAndSettle();

        verify(() => mockThemeCubit.setThemeMode(ThemeMode.dark)).called(1);
      });

      testWidgets('taps follow system option', (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(AISettings()),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.light),
        );

        await tester.pumpWidget(createSettingsPage());
        await tester.tap(find.text('Follow System'));
        await tester.pumpAndSettle();

        verify(() => mockThemeCubit.useSystemTheme()).called(1);
      });
    });

    group('AI Features Section', () {
      testWidgets('shows loading indicator when settings are loading',
          (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(const SettingsLoading());
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        expect(find.byType(CircularProgressIndicator), findsOneWidget);
      });

      testWidgets('displays AI features toggle when settings loaded',
          (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(AISettings(enabled: false)),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        expect(find.text('Enable AI Features'), findsOneWidget);
        expect(
          find.text('Task breakdown, time estimates, and suggestions'),
          findsOneWidget,
        );
        expect(find.byType(Switch), findsOneWidget);
      });

      testWidgets('toggle switch reflects enabled state', (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(AISettings(enabled: true)),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        final switchWidget = tester.widget<Switch>(find.byType(Switch));
        expect(switchWidget.value, true);
      });

      testWidgets('toggling switch updates settings', (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(AISettings(enabled: false)),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());
        await tester.tap(find.byType(Switch));
        await tester.pumpAndSettle();

        verify(
          () => mockSettingsBloc.add(
            any(
              that: isA<SettingsAIUpdated>().having(
                (event) => event.settings.enabled,
                'enabled',
                true,
              ),
            ),
          ),
        ).called(1);
      });

      testWidgets('hides provider options when AI is disabled',
          (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(AISettings(enabled: false)),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        expect(find.text('AI Provider'), findsNothing);
        expect(find.text('OpenAI'), findsNothing);
      });

      testWidgets('shows provider options when AI is enabled',
          (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(AISettings(enabled: true)),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        expect(find.text('AI Provider'), findsOneWidget);
        expect(find.text('OpenAI'), findsOneWidget);
        expect(find.text('Anthropic'), findsOneWidget);
        expect(find.text('Ollama (Local)'), findsOneWidget);
      });

      testWidgets('shows provider descriptions', (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(AISettings(enabled: true)),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        expect(
          find.text('GPT models from OpenAI (requires API key)'),
          findsOneWidget,
        );
        expect(
          find.text('Claude models from Anthropic (requires API key)'),
          findsOneWidget,
        );
        expect(
          find.text('Run AI models locally (free, no API key needed)'),
          findsOneWidget,
        );
      });

      testWidgets('tapping provider option updates settings', (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(
            AISettings(enabled: true, provider: AIProvider.ollama),
          ),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        // Scroll to bring the provider options into view
        await tester.drag(find.byType(ListView), const Offset(0, -400));
        await tester.pumpAndSettle();

        await tester.tap(find.text('OpenAI'));
        await tester.pumpAndSettle();

        verify(
          () => mockSettingsBloc.add(
            any(
              that: isA<SettingsAIUpdated>().having(
                (event) => event.settings.provider,
                'provider',
                AIProvider.openai,
              ),
            ),
          ),
        ).called(1);
      });
    });

    group('OpenAI Configuration', () {
      testWidgets('shows OpenAI config when provider selected',
          (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(
            AISettings(enabled: true, provider: AIProvider.openai),
          ),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        expect(find.text('OpenAI Configuration'), findsOneWidget);
        expect(find.text('API Key'), findsOneWidget);
        expect(find.text('Model'), findsOneWidget);
        expect(
          find.text('Get your API key from platform.openai.com'),
          findsOneWidget,
        );
      });

      testWidgets('entering API key updates settings', (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(
            AISettings(enabled: true, provider: AIProvider.openai),
          ),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        // Find text fields by type and enter text in the first one (API key field)
        final textFields = find.byType(TextField);
        expect(textFields, findsAtLeastNWidgets(1));
        await tester.enterText(textFields.first, 'sk-test-key');
        await tester.pumpAndSettle();

        verify(
          () => mockSettingsBloc.add(
            any(
              that: isA<SettingsAIUpdated>().having(
                (event) => event.settings.openaiApiKey,
                'openaiApiKey',
                'sk-test-key',
              ),
            ),
          ),
        ).called(1);
      });

      testWidgets('selecting model updates settings', (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(
            AISettings(
              enabled: true,
              provider: AIProvider.openai,
              openaiModel: 'gpt-4-turbo-preview',
            ),
          ),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        // Scroll to bring the dropdown into view
        await tester.drag(find.byType(ListView), const Offset(0, -800));
        await tester.pumpAndSettle();

        // Tap the dropdown to open it
        await tester.tap(find.byType(DropdownButtonFormField<String>));
        await tester.pumpAndSettle();

        // Tap the GPT-3.5 Turbo option
        await tester.tap(find.text('GPT-3.5 Turbo').last);
        await tester.pumpAndSettle();

        verify(
          () => mockSettingsBloc.add(
            any(
              that: isA<SettingsAIUpdated>().having(
                (event) => event.settings.openaiModel,
                'openaiModel',
                'gpt-3.5-turbo',
              ),
            ),
          ),
        ).called(1);
      });
    });

    group('Anthropic Configuration', () {
      testWidgets('shows Anthropic config when provider selected',
          (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(
            AISettings(enabled: true, provider: AIProvider.anthropic),
          ),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        expect(find.text('Anthropic Configuration'), findsOneWidget);
        expect(find.text('API Key'), findsOneWidget);
        expect(find.text('Model'), findsOneWidget);
        expect(
          find.text('Get your API key from console.anthropic.com'),
          findsOneWidget,
        );
      });

      testWidgets('entering API key updates settings', (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(
            AISettings(enabled: true, provider: AIProvider.anthropic),
          ),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        // Find text fields and enter text in the first one (API key field)
        final textFields = find.byType(TextField);
        expect(textFields, findsAtLeastNWidgets(1));
        await tester.enterText(textFields.first, 'sk-ant-test-key');
        await tester.pumpAndSettle();

        verify(
          () => mockSettingsBloc.add(
            any(
              that: isA<SettingsAIUpdated>().having(
                (event) => event.settings.anthropicApiKey,
                'anthropicApiKey',
                'sk-ant-test-key',
              ),
            ),
          ),
        ).called(1);
      });
    });

    group('Ollama Configuration', () {
      testWidgets('shows Ollama config when provider selected',
          (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(
            AISettings(enabled: true, provider: AIProvider.ollama),
          ),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        expect(find.text('Ollama Configuration'), findsOneWidget);
        expect(find.text('Ollama Server URL'), findsOneWidget);
        expect(find.text('Model'), findsOneWidget);
        expect(
          find.textContaining('Install Ollama from ollama.com'),
          findsOneWidget,
        );
      });

      testWidgets('entering URL updates settings', (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(
            AISettings(enabled: true, provider: AIProvider.ollama),
          ),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        // Find text fields and enter URL in the first one (Ollama URL field)
        final textFields = find.byType(TextField);
        expect(textFields, findsAtLeastNWidgets(1));
        await tester.enterText(textFields.first, 'http://192.168.1.100:11434');
        await tester.pumpAndSettle();

        verify(
          () => mockSettingsBloc.add(
            any(
              that: isA<SettingsAIUpdated>().having(
                (event) => event.settings.ollamaBaseUrl,
                'ollamaBaseUrl',
                'http://192.168.1.100:11434',
              ),
            ),
          ),
        ).called(1);
      });
    });

    group('Validation Warning', () {
      testWidgets('shows warning when OpenAI settings invalid',
          (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(
            AISettings(
              enabled: true,
              provider: AIProvider.openai,
              // No API key provided
            ),
          ),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        expect(find.byIcon(Icons.warning), findsOneWidget);
        expect(
          find.text('Please provide an API key to use this provider'),
          findsOneWidget,
        );
      });

      testWidgets('hides warning when settings valid', (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(
            AISettings(
              enabled: true,
              provider: AIProvider.openai,
              openaiApiKey: 'sk-valid-key',
            ),
          ),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        expect(find.byIcon(Icons.warning), findsNothing);
        expect(
          find.text('Please provide an API key to use this provider'),
          findsNothing,
        );
      });

      testWidgets('shows warning when Anthropic settings invalid',
          (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(
            AISettings(
              enabled: true,
              provider: AIProvider.anthropic,
              // No API key provided
            ),
          ),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        expect(find.byIcon(Icons.warning), findsOneWidget);
      });

      testWidgets('no warning for Ollama (no API key needed)', (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(
            AISettings(enabled: true, provider: AIProvider.ollama),
          ),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        expect(find.byIcon(Icons.warning), findsNothing);
      });
    });

    group('About Section', () {
      testWidgets('displays app information', (tester) async {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(AISettings()),
        );
        when(() => mockThemeCubit.state).thenReturn(
          const ThemeState(ThemeMode.system),
        );

        await tester.pumpWidget(createSettingsPage());

        // Scroll to bottom to find About section
        await tester.drag(find.byType(ListView), const Offset(0, -1000));
        await tester.pumpAndSettle();

        expect(find.text('Altair Guidance'), findsOneWidget);
        expect(find.text('ADHD-friendly task management'), findsOneWidget);
        expect(find.text('Version 0.1.0'), findsOneWidget);
      });
    });
  });
}
