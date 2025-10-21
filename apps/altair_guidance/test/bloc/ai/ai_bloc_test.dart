import 'package:altair_guidance/bloc/ai/ai_bloc.dart';
import 'package:altair_guidance/bloc/ai/ai_event.dart';
import 'package:altair_guidance/bloc/ai/ai_state.dart';
import 'package:altair_guidance/services/ai/ai_service.dart';
import 'package:altair_guidance/services/ai/models.dart';
import 'package:bloc_test/bloc_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';

class MockAIService extends Mock implements AIService {}

void main() {
  group('AIBloc', () {
    late MockAIService mockAIService;

    setUp(() {
      mockAIService = MockAIService();

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
        final bloc = AIBloc(aiService: mockAIService);
        expect(bloc.state, const AIInitial());
        bloc.close();
      });
    });

    group('AITaskBreakdownRequested', () {
      blocTest<AIBloc, AIState>(
        'emits [AILoading, AITaskBreakdownSuccess] when breakdown succeeds',
        build: () {
          when(() => mockAIService.breakdownTask(any())).thenAnswer(
            (_) async => const TaskBreakdownResponse(
              originalTask: 'Build app',
              subtasks: [
                SubtaskSuggestion(
                  title: 'Setup project',
                  description: 'Initialize project',
                  estimatedMinutes: 30,
                  order: 1,
                ),
                SubtaskSuggestion(
                  title: 'Create UI',
                  description: 'Build interface',
                  estimatedMinutes: 60,
                  order: 2,
                ),
              ],
              totalEstimatedMinutes: 90,
              reasoning: 'Logical breakdown',
            ),
          );
          return AIBloc(aiService: mockAIService);
        },
        act: (bloc) => bloc.add(
          AITaskBreakdownRequested(
            request: TaskBreakdownRequest(
              taskTitle: 'Build app',
              maxSubtasks: 3,
            ),
          ),
        ),
        expect: () => [
          const AILoading(operationType: AIOperationType.breakdown),
          isA<AITaskBreakdownSuccess>()
              .having(
                (s) => s.response.originalTask,
                'originalTask',
                'Build app',
              )
              .having(
                (s) => s.response.subtasks.length,
                'subtasks.length',
                2,
              )
              .having(
                (s) => s.response.totalEstimatedMinutes,
                'totalEstimatedMinutes',
                90,
              ),
        ],
        verify: (_) {
          verify(() => mockAIService.breakdownTask(any())).called(1);
        },
      );

      blocTest<AIBloc, AIState>(
        'emits [AILoading, AIFailure] when breakdown fails with AIServiceException',
        build: () {
          when(() => mockAIService.breakdownTask(any())).thenThrow(
            const AIServiceException(
              'Service unavailable',
              statusCode: 503,
            ),
          );
          return AIBloc(aiService: mockAIService);
        },
        act: (bloc) => bloc.add(
          AITaskBreakdownRequested(
            request: TaskBreakdownRequest(taskTitle: 'Test'),
          ),
        ),
        expect: () => [
          const AILoading(operationType: AIOperationType.breakdown),
          const AIFailure(
            message: 'Service unavailable',
            operationType: AIOperationType.breakdown,
          ),
        ],
      );

      blocTest<AIBloc, AIState>(
        'emits [AILoading, AIFailure] when breakdown fails with unexpected error',
        build: () {
          when(() => mockAIService.breakdownTask(any())).thenThrow(
            Exception('Unexpected error'),
          );
          return AIBloc(aiService: mockAIService);
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
                contains('Unexpected error'),
              )
              .having(
                (f) => f.operationType,
                'operationType',
                AIOperationType.breakdown,
              ),
        ],
      );
    });

    group('AITaskPrioritizationRequested', () {
      blocTest<AIBloc, AIState>(
        'emits [AILoading, AITaskPrioritizationSuccess] when prioritization succeeds',
        build: () {
          when(() => mockAIService.prioritizeTasks(any())).thenAnswer(
            (_) async => const TaskPrioritizationResponse(
              suggestions: [
                PrioritySuggestion(
                  taskTitle: 'Task 1',
                  priority: PriorityLevel.high,
                  reasoning: 'Critical',
                  urgencyScore: 0.9,
                  impactScore: 0.8,
                ),
                PrioritySuggestion(
                  taskTitle: 'Task 2',
                  priority: PriorityLevel.medium,
                  reasoning: 'Important',
                  urgencyScore: 0.5,
                  impactScore: 0.6,
                ),
              ],
              recommendedOrder: ['Task 1', 'Task 2'],
            ),
          );
          return AIBloc(aiService: mockAIService);
        },
        act: (bloc) => bloc.add(
          AITaskPrioritizationRequested(
            request: TaskPrioritizationRequest(
              tasks: [
                {'title': 'Task 1'},
                {'title': 'Task 2'},
              ],
            ),
          ),
        ),
        expect: () => [
          const AILoading(operationType: AIOperationType.prioritization),
          isA<AITaskPrioritizationSuccess>()
              .having(
                (s) => s.response.suggestions.length,
                'suggestions.length',
                2,
              )
              .having(
                (s) => s.response.recommendedOrder,
                'recommendedOrder',
                ['Task 1', 'Task 2'],
              ),
        ],
        verify: (_) {
          verify(() => mockAIService.prioritizeTasks(any())).called(1);
        },
      );

      blocTest<AIBloc, AIState>(
        'emits [AILoading, AIFailure] when prioritization fails',
        build: () {
          when(() => mockAIService.prioritizeTasks(any())).thenThrow(
            AIServiceException.timeout('task prioritization'),
          );
          return AIBloc(aiService: mockAIService);
        },
        act: (bloc) => bloc.add(
          AITaskPrioritizationRequested(
            request: TaskPrioritizationRequest(
              tasks: [
                {'title': 'Task 1'},
              ],
            ),
          ),
        ),
        expect: () => [
          const AILoading(operationType: AIOperationType.prioritization),
          isA<AIFailure>()
              .having(
                (f) => f.message,
                'message',
                contains('taking longer than expected'),
              )
              .having(
                (f) => f.operationType,
                'operationType',
                AIOperationType.prioritization,
              ),
        ],
      );
    });

    group('AITimeEstimateRequested', () {
      blocTest<AIBloc, AIState>(
        'emits [AILoading, AITimeEstimateSuccess] when estimation succeeds',
        build: () {
          when(() => mockAIService.estimateTime(any())).thenAnswer(
            (_) async => const TimeEstimateResponse(
              taskTitle: 'Build feature',
              estimate: TimeEstimate(
                optimisticMinutes: 60,
                realisticMinutes: 90,
                pessimisticMinutes: 120,
                confidence: 0.7,
              ),
              factors: ['Complexity'],
              assumptions: ['No blockers'],
            ),
          );
          return AIBloc(aiService: mockAIService);
        },
        act: (bloc) => bloc.add(
          AITimeEstimateRequested(
            request: TimeEstimateRequest(
              taskTitle: 'Build feature',
              skillLevel: SkillLevel.intermediate,
            ),
          ),
        ),
        expect: () => [
          const AILoading(operationType: AIOperationType.timeEstimate),
          isA<AITimeEstimateSuccess>()
              .having(
                (s) => s.response.taskTitle,
                'taskTitle',
                'Build feature',
              )
              .having(
                (s) => s.response.estimate.realisticMinutes,
                'realisticMinutes',
                90,
              )
              .having(
                (s) => s.response.estimate.confidence,
                'confidence',
                0.7,
              ),
        ],
        verify: (_) {
          verify(() => mockAIService.estimateTime(any())).called(1);
        },
      );

      blocTest<AIBloc, AIState>(
        'emits [AILoading, AIFailure] when estimation fails',
        build: () {
          when(() => mockAIService.estimateTime(any())).thenThrow(
            AIServiceException.network('Connection lost'),
          );
          return AIBloc(aiService: mockAIService);
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
                contains('Network error'),
              )
              .having(
                (f) => f.operationType,
                'operationType',
                AIOperationType.timeEstimate,
              ),
        ],
      );
    });

    group('AIContextSuggestionsRequested', () {
      blocTest<AIBloc, AIState>(
        'emits [AILoading, AIContextSuggestionsSuccess] when suggestions succeed',
        build: () {
          when(() => mockAIService.getSuggestions(any())).thenAnswer(
            (_) async => const ContextSuggestionResponse(
              taskTitle: 'Learn Flutter',
              suggestions: [
                ContextSuggestion(
                  title: 'Read docs',
                  description: 'Official documentation',
                  category: 'resource',
                ),
                ContextSuggestion(
                  title: 'Build app',
                  description: 'Practice project',
                  category: 'tip',
                ),
              ],
              summary: 'Start with docs',
            ),
          );
          return AIBloc(aiService: mockAIService);
        },
        act: (bloc) => bloc.add(
          AIContextSuggestionsRequested(
            request: ContextSuggestionRequest(
              taskTitle: 'Learn Flutter',
              suggestionType: 'resources',
            ),
          ),
        ),
        expect: () => [
          const AILoading(operationType: AIOperationType.contextSuggestions),
          isA<AIContextSuggestionsSuccess>()
              .having(
                (s) => s.response.taskTitle,
                'taskTitle',
                'Learn Flutter',
              )
              .having(
                (s) => s.response.suggestions.length,
                'suggestions.length',
                2,
              )
              .having(
                (s) => s.response.summary,
                'summary',
                'Start with docs',
              ),
        ],
        verify: (_) {
          verify(() => mockAIService.getSuggestions(any())).called(1);
        },
      );

      blocTest<AIBloc, AIState>(
        'emits [AILoading, AIFailure] when suggestions fail',
        build: () {
          when(() => mockAIService.getSuggestions(any())).thenThrow(
            const AIServiceException('Invalid request', statusCode: 400),
          );
          return AIBloc(aiService: mockAIService);
        },
        act: (bloc) => bloc.add(
          AIContextSuggestionsRequested(
            request: ContextSuggestionRequest(taskTitle: 'Test'),
          ),
        ),
        expect: () => [
          const AILoading(operationType: AIOperationType.contextSuggestions),
          const AIFailure(
            message: 'Invalid request',
            operationType: AIOperationType.contextSuggestions,
          ),
        ],
      );
    });

    group('AIClearState', () {
      blocTest<AIBloc, AIState>(
        'emits [AIInitial] when clear state is requested',
        build: () => AIBloc(aiService: mockAIService),
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
        'clears failure state',
        build: () => AIBloc(aiService: mockAIService),
        seed: () => const AIFailure(
          message: 'Error',
          operationType: AIOperationType.breakdown,
        ),
        act: (bloc) => bloc.add(const AIClearState()),
        expect: () => [const AIInitial()],
      );
    });

    group('close', () {
      test('disposes AI service', () async {
        when(() => mockAIService.dispose()).thenReturn(null);

        final bloc = AIBloc(aiService: mockAIService);
        await bloc.close();

        verify(() => mockAIService.dispose()).called(1);
      });

      test('handles disposal errors gracefully', () async {
        when(() => mockAIService.dispose()).thenThrow(Exception('Error'));

        final bloc = AIBloc(aiService: mockAIService);

        // Should not throw even if dispose fails
        expect(() => bloc.close(), returnsNormally);
      });
    });
  });
}
