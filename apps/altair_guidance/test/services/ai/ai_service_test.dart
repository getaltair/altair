import 'dart:async';
import 'dart:io';

import 'package:altair_guidance/services/ai/ai_service.dart';
import 'package:altair_guidance/services/ai/models.dart';
import 'package:altair_guidance/services/ai/providers/ai_provider.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mocktail/mocktail.dart';

class MockAIProvider extends Mock implements AIProvider {}

void main() {
  group('AIService', () {
    late MockAIProvider mockProvider;
    late AIService service;

    setUp(() {
      mockProvider = MockAIProvider();
      service = AIService(provider: mockProvider);

      // Register fallback values for mocktail
      registerFallbackValue(
        TaskBreakdownRequest(taskTitle: 'test', maxSubtasks: 3),
      );
      registerFallbackValue(
        TaskPrioritizationRequest(tasks: [
          {'title': 'Test Task'}
        ]),
      );
      registerFallbackValue(
        TimeEstimateRequest(
            taskTitle: 'test', skillLevel: SkillLevel.intermediate),
      );
      registerFallbackValue(
        ContextSuggestionRequest(taskTitle: 'test', suggestionType: 'general'),
      );
    });

    tearDown(() {
      service.dispose();
    });

    group('breakdownTask', () {
      test('returns TaskBreakdownResponse from provider', () async {
        final request = TaskBreakdownRequest(
          taskTitle: 'Build a Flutter app',
          maxSubtasks: 3,
        );

        final expectedResponse = TaskBreakdownResponse(
          originalTask: 'Build a Flutter app',
          subtasks: [
            const SubtaskSuggestion(
              title: 'Setup project',
              description: 'Initialize Flutter project',
              estimatedMinutes: 30,
              order: 1,
            ),
            const SubtaskSuggestion(
              title: 'Create UI',
              description: 'Design the interface',
              estimatedMinutes: 60,
              order: 2,
            ),
          ],
          totalEstimatedMinutes: 90,
          reasoning: 'Broke down into logical phases',
        );

        when(() => mockProvider.breakdownTask(any()))
            .thenAnswer((_) async => expectedResponse);

        final response = await service.breakdownTask(request);

        expect(response, equals(expectedResponse));
        verify(() => mockProvider.breakdownTask(request)).called(1);
      });

      test('throws AIServiceException on timeout', () async {
        final request = TaskBreakdownRequest(
          taskTitle: 'Build a Flutter app',
          maxSubtasks: 3,
        );

        when(() => mockProvider.breakdownTask(any()))
            .thenThrow(TimeoutException('Timeout'));

        expect(
          () => service.breakdownTask(request),
          throwsA(
            isA<AIServiceException>()
                .having((e) => e.isTimeout, 'isTimeout', true)
                .having(
                  (e) => e.message,
                  'message',
                  contains('task breakdown'),
                ),
          ),
        );
      });

      test('throws AIServiceException on network error', () async {
        final request = TaskBreakdownRequest(
          taskTitle: 'Build a Flutter app',
          maxSubtasks: 3,
        );

        when(() => mockProvider.breakdownTask(any()))
            .thenThrow(const SocketException('No internet'));

        expect(
          () => service.breakdownTask(request),
          throwsA(
            isA<AIServiceException>()
                .having((e) => e.isNetworkError, 'isNetworkError', true)
                .having(
                  (e) => e.message,
                  'message',
                  contains('No internet'),
                ),
          ),
        );
      });

      test('rethrows AIServiceException from provider', () async {
        final request = TaskBreakdownRequest(
          taskTitle: 'Build a Flutter app',
          maxSubtasks: 3,
        );

        final exception = AIServiceException('Provider error', statusCode: 400);

        when(() => mockProvider.breakdownTask(any())).thenThrow(exception);

        expect(
          () => service.breakdownTask(request),
          throwsA(equals(exception)),
        );
      });
    });

    group('prioritizeTasks', () {
      test('returns TaskPrioritizationResponse from provider', () async {
        final request = TaskPrioritizationRequest(
          tasks: [
            {'title': 'Task 1'},
            {'title': 'Task 2'},
          ],
        );

        final expectedResponse = TaskPrioritizationResponse(
          suggestions: [
            const PrioritySuggestion(
              taskTitle: 'Task 1',
              priority: PriorityLevel.high,
              reasoning: 'Important',
              urgencyScore: 0.8,
              impactScore: 0.9,
            ),
          ],
          recommendedOrder: ['Task 1', 'Task 2'],
        );

        when(() => mockProvider.prioritizeTasks(any()))
            .thenAnswer((_) async => expectedResponse);

        final response = await service.prioritizeTasks(request);

        expect(response, equals(expectedResponse));
        verify(() => mockProvider.prioritizeTasks(request)).called(1);
      });

      test('throws AIServiceException on timeout', () async {
        final request = TaskPrioritizationRequest(tasks: [
          {'title': 'Task 1'}
        ]);

        when(() => mockProvider.prioritizeTasks(any()))
            .thenThrow(TimeoutException('Timeout'));

        expect(
          () => service.prioritizeTasks(request),
          throwsA(
            isA<AIServiceException>()
                .having((e) => e.isTimeout, 'isTimeout', true)
                .having(
                  (e) => e.message,
                  'message',
                  contains('task prioritization'),
                ),
          ),
        );
      });
    });

    group('estimateTime', () {
      test('returns TimeEstimateResponse from provider', () async {
        final request = TimeEstimateRequest(
          taskTitle: 'Build feature',
          skillLevel: SkillLevel.intermediate,
        );

        final expectedResponse = TimeEstimateResponse(
          taskTitle: 'Build feature',
          estimate: const TimeEstimate(
            optimisticMinutes: 30,
            realisticMinutes: 60,
            pessimisticMinutes: 90,
            confidence: 0.7,
          ),
          factors: ['complexity', 'dependencies'],
          assumptions: ['standard setup'],
        );

        when(() => mockProvider.estimateTime(any()))
            .thenAnswer((_) async => expectedResponse);

        final response = await service.estimateTime(request);

        expect(response, equals(expectedResponse));
        verify(() => mockProvider.estimateTime(request)).called(1);
      });

      test('throws AIServiceException on timeout', () async {
        final request = TimeEstimateRequest(
          taskTitle: 'Build feature',
          skillLevel: SkillLevel.intermediate,
        );

        when(() => mockProvider.estimateTime(any()))
            .thenThrow(TimeoutException('Timeout'));

        expect(
          () => service.estimateTime(request),
          throwsA(
            isA<AIServiceException>()
                .having((e) => e.isTimeout, 'isTimeout', true)
                .having(
                  (e) => e.message,
                  'message',
                  contains('time estimation'),
                ),
          ),
        );
      });
    });

    group('getSuggestions', () {
      test('returns ContextSuggestionResponse from provider', () async {
        final request = ContextSuggestionRequest(
          taskTitle: 'Build feature',
          suggestionType: 'resources',
        );

        final expectedResponse = ContextSuggestionResponse(
          taskTitle: 'Build feature',
          suggestions: [
            const ContextSuggestion(
              title: 'Check documentation',
              description: 'Review API docs',
              category: 'resources',
            ),
          ],
          summary: 'Helpful resources found',
        );

        when(() => mockProvider.getSuggestions(any()))
            .thenAnswer((_) async => expectedResponse);

        final response = await service.getSuggestions(request);

        expect(response, equals(expectedResponse));
        verify(() => mockProvider.getSuggestions(request)).called(1);
      });

      test('throws AIServiceException on timeout', () async {
        final request = ContextSuggestionRequest(
          taskTitle: 'Build feature',
          suggestionType: 'general',
        );

        when(() => mockProvider.getSuggestions(any()))
            .thenThrow(TimeoutException('Timeout'));

        expect(
          () => service.getSuggestions(request),
          throwsA(
            isA<AIServiceException>()
                .having((e) => e.isTimeout, 'isTimeout', true)
                .having(
                  (e) => e.message,
                  'message',
                  contains('context suggestions'),
                ),
          ),
        );
      });
    });

    group('dispose', () {
      test('calls dispose on provider', () {
        when(() => mockProvider.dispose()).thenReturn(null);

        service.dispose();

        verify(() => mockProvider.dispose()).called(1);
      });
    });
  });
}
