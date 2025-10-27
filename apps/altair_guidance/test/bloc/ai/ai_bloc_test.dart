import 'package:altair_guidance/bloc/ai/ai_bloc.dart';
import 'package:altair_guidance/bloc/ai/ai_event.dart';
import 'package:altair_guidance/bloc/ai/ai_state.dart';
import 'package:altair_guidance/bloc/settings/settings_bloc.dart';
import 'package:altair_guidance/bloc/settings/settings_state.dart';
import 'package:altair_guidance/models/ai_settings.dart';
import 'package:altair_guidance/services/ai/models.dart';
import 'package:bloc_test/bloc_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';

class MockSettingsBloc extends Mock implements SettingsBloc {}

void main() {
  group('AIBloc', () {
    late MockSettingsBloc mockSettingsBloc;

    setUp(() {
      mockSettingsBloc = MockSettingsBloc();

      // Register fallback values
      registerFallbackValue(
        TaskBreakdownRequest(taskTitle: 'Test'),
      );
      registerFallbackValue(
        TaskPrioritizationRequest(tasks: [
          {'title': 'Test'},
        ]),
      );
      registerFallbackValue(
        TimeEstimateRequest(taskTitle: 'Test'),
      );
      registerFallbackValue(
        ContextSuggestionRequest(taskTitle: 'Test'),
      );
    });

    group('constructor', () {
      test('initial state is AIInitial', () {
        when(() => mockSettingsBloc.state).thenReturn(
          const SettingsLoaded(AISettings()),
        );

        final bloc = AIBloc(settingsBloc: mockSettingsBloc);
        expect(bloc.state, const AIInitial());
        bloc.close();
      });
    });

    group('Settings State Handling', () {
      blocTest<AIBloc, AIState>(
        'emits failure when settings not loaded',
        build: () {
          when(() => mockSettingsBloc.state).thenReturn(const SettingsLoading());
          return AIBloc(settingsBloc: mockSettingsBloc);
        },
        act: (bloc) => bloc.add(
          AITaskBreakdownRequested(
            request: TaskBreakdownRequest(taskTitle: 'Test'),
          ),
        ),
        expect: () => [
          const AILoading(operationType: AIOperationType.breakdown),
          isA<AIFailure>()
              .having(
                (f) => f.message,
                'message',
                contains('AI features are disabled'),
              )
              .having(
                (f) => f.operationType,
                'operationType',
                AIOperationType.breakdown,
              ),
        ],
      );

      blocTest<AIBloc, AIState>(
        'emits failure when AI is disabled in settings',
        build: () {
          when(() => mockSettingsBloc.state).thenReturn(
            const SettingsLoaded(
              AISettings(enabled: false),
            ),
          );
          return AIBloc(settingsBloc: mockSettingsBloc);
        },
        act: (bloc) => bloc.add(
          AITaskBreakdownRequested(
            request: TaskBreakdownRequest(taskTitle: 'Test'),
          ),
        ),
        expect: () => [
          const AILoading(operationType: AIOperationType.breakdown),
          isA<AIFailure>()
              .having(
                (f) => f.message,
                'message',
                contains('AI features are disabled'),
              )
              .having(
                (f) => f.operationType,
                'operationType',
                AIOperationType.breakdown,
              ),
        ],
      );

      blocTest<AIBloc, AIState>(
        'emits failure when OpenAI selected but API key missing',
        build: () {
          when(() => mockSettingsBloc.state).thenReturn(
            const SettingsLoaded(
              AISettings(
                enabled: true,
                provider: AIProviderType.openai,
                openaiApiKey: null,
              ),
            ),
          );
          return AIBloc(settingsBloc: mockSettingsBloc);
        },
        act: (bloc) => bloc.add(
          AITaskBreakdownRequested(
            request: TaskBreakdownRequest(taskTitle: 'Test'),
          ),
        ),
        expect: () => [
          const AILoading(operationType: AIOperationType.breakdown),
          isA<AIFailure>()
              .having(
                (f) => f.message,
                'message',
                contains('AI features are disabled'),
              )
              .having(
                (f) => f.operationType,
                'operationType',
                AIOperationType.breakdown,
              ),
        ],
      );

      blocTest<AIBloc, AIState>(
        'emits failure when Anthropic selected but API key missing',
        build: () {
          when(() => mockSettingsBloc.state).thenReturn(
            const SettingsLoaded(
              AISettings(
                enabled: true,
                provider: AIProviderType.anthropic,
                anthropicApiKey: null,
              ),
            ),
          );
          return AIBloc(settingsBloc: mockSettingsBloc);
        },
        act: (bloc) => bloc.add(
          AITimeEstimateRequested(
            request: TimeEstimateRequest(taskTitle: 'Test'),
          ),
        ),
        expect: () => [
          const AILoading(operationType: AIOperationType.timeEstimate),
          isA<AIFailure>()
              .having(
                (f) => f.message,
                'message',
                contains('AI features are disabled'),
              )
              .having(
                (f) => f.operationType,
                'operationType',
                AIOperationType.timeEstimate,
              ),
        ],
      );
    });

    group('AIClearState', () {
      blocTest<AIBloc, AIState>(
        'emits [AIInitial] when clear state is requested from success',
        build: () {
          when(() => mockSettingsBloc.state).thenReturn(
            const SettingsLoaded(AISettings()),
          );
          return AIBloc(settingsBloc: mockSettingsBloc);
        },
        seed: () => const AITaskBreakdownSuccess(
          response: TaskBreakdownResponse(
            originalTask: 'Test',
            subtasks: [],
          ),
        ),
        act: (bloc) => bloc.add(const AIClearState()),
        expect: () => [const AIInitial()],
      );

      blocTest<AIBloc, AIState>(
        'emits [AIInitial] when clear state is requested from failure',
        build: () {
          when(() => mockSettingsBloc.state).thenReturn(
            const SettingsLoaded(AISettings()),
          );
          return AIBloc(settingsBloc: mockSettingsBloc);
        },
        seed: () => const AIFailure(
          message: 'Error',
          operationType: AIOperationType.breakdown,
        ),
        act: (bloc) => bloc.add(const AIClearState()),
        expect: () => [const AIInitial()],
      );
    });

    group('Operation Types', () {
      blocTest<AIBloc, AIState>(
        'task prioritization shows correct operation type on failure',
        build: () {
          when(() => mockSettingsBloc.state).thenReturn(
            const SettingsLoaded(AISettings(enabled: false)),
          );
          return AIBloc(settingsBloc: mockSettingsBloc);
        },
        act: (bloc) => bloc.add(
          AITaskPrioritizationRequested(
            request: TaskPrioritizationRequest(tasks: [
              {'title': 'Task 1'},
            ]),
          ),
        ),
        expect: () => [
          const AILoading(operationType: AIOperationType.prioritization),
          isA<AIFailure>().having(
            (f) => f.operationType,
            'operationType',
            AIOperationType.prioritization,
          ),
        ],
      );

      blocTest<AIBloc, AIState>(
        'context suggestions shows correct operation type on failure',
        build: () {
          when(() => mockSettingsBloc.state).thenReturn(
            const SettingsLoaded(AISettings(enabled: false)),
          );
          return AIBloc(settingsBloc: mockSettingsBloc);
        },
        act: (bloc) => bloc.add(
          AIContextSuggestionsRequested(
            request: ContextSuggestionRequest(taskTitle: 'Test'),
          ),
        ),
        expect: () => [
          const AILoading(operationType: AIOperationType.contextSuggestions),
          isA<AIFailure>().having(
            (f) => f.operationType,
            'operationType',
            AIOperationType.contextSuggestions,
          ),
        ],
      );
    });
  });
}
