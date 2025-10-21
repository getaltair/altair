import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:altair_guidance/services/ai/ai_config.dart';
import 'package:altair_guidance/services/ai/ai_service.dart';
import 'package:altair_guidance/services/ai/models.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:mocktail/mocktail.dart';

class MockHttpClient extends Mock implements http.Client {}

void main() {
  group('AIService', () {
    late MockHttpClient mockClient;
    late AIConfig config;
    late AIService service;

    setUp(() {
      mockClient = MockHttpClient();
      config = const AIConfig(
        baseUrl: 'http://localhost:8001/api',
        enableSSL: false,
      );
      service = AIService(
        config: config,
        httpClient: mockClient,
      );

      // Register fallback values for mocktail
      registerFallbackValue(Uri.parse('http://localhost:8001/api'));
    });

    tearDown(() {
      service.dispose();
    });

    group('breakdownTask', () {
      test('returns TaskBreakdownResponse on successful request', () async {
        final request = TaskBreakdownRequest(
          taskTitle: 'Build a Flutter app',
          maxSubtasks: 3,
        );

        final responseBody = {
          'original_task': 'Build a Flutter app',
          'subtasks': [
            {
              'title': 'Setup project',
              'description': 'Initialize Flutter project',
              'estimated_minutes': 30,
              'order': 1,
            },
            {
              'title': 'Create UI',
              'description': 'Design the interface',
              'estimated_minutes': 60,
              'order': 2,
            },
            {
              'title': 'Test app',
              'description': 'Write and run tests',
              'estimated_minutes': 45,
              'order': 3,
            },
          ],
          'total_estimated_minutes': 135,
          'reasoning': 'Broke down into logical phases',
        };

        when(() => mockClient.post(
              any(),
              headers: any(named: 'headers'),
              body: any(named: 'body'),
            )).thenAnswer(
          (_) async => http.Response(
            jsonEncode(responseBody),
            200,
          ),
        );

        final response = await service.breakdownTask(request);

        expect(response.originalTask, 'Build a Flutter app');
        expect(response.subtasks.length, 3);
        expect(response.subtasks[0].title, 'Setup project');
        expect(response.subtasks[0].estimatedMinutes, 30);
        expect(response.totalEstimatedMinutes, 135);
        expect(response.reasoning, 'Broke down into logical phases');

        verify(
          () => mockClient.post(
            Uri.parse('http://localhost:8001/api/ai/breakdown'),
            headers: {
              'Content-Type': 'application/json',
            },
            body: jsonEncode(request.toJson()),
          ),
        ).called(1);
      });

      test('throws AIServiceException on 400 error', () async {
        final request = TaskBreakdownRequest(taskTitle: 'Test task');

        when(() => mockClient.post(
              any(),
              headers: any(named: 'headers'),
              body: any(named: 'body'),
            )).thenAnswer(
          (_) async => http.Response(
            jsonEncode({'detail': 'Invalid request'}),
            400,
          ),
        );

        expect(
          () => service.breakdownTask(request),
          throwsA(
            isA<AIServiceException>()
                .having((e) => e.message, 'message', 'Invalid request')
                .having((e) => e.statusCode, 'statusCode', 400),
          ),
        );
      });

      test('throws AIServiceException on 500 error', () async {
        final request = TaskBreakdownRequest(taskTitle: 'Test task');

        when(() => mockClient.post(
              any(),
              headers: any(named: 'headers'),
              body: any(named: 'body'),
            )).thenAnswer(
          (_) async => http.Response(
            jsonEncode({'detail': 'Internal server error'}),
            500,
          ),
        );

        expect(
          () => service.breakdownTask(request),
          throwsA(
            isA<AIServiceException>()
                .having(
                  (e) => e.message,
                  'message',
                  'Internal server error',
                )
                .having((e) => e.statusCode, 'statusCode', 500),
          ),
        );
      });

      test('throws timeout exception when request times out', () async {
        final request = TaskBreakdownRequest(taskTitle: 'Test task');

        when(() => mockClient.post(
              any(),
              headers: any(named: 'headers'),
              body: any(named: 'body'),
            )).thenAnswer(
          (_) async {
            await Future<void>.delayed(const Duration(seconds: 2));
            throw TimeoutException('Timed out');
          },
        );

        expect(
          () => service.breakdownTask(request),
          throwsA(
            isA<AIServiceException>()
                .having((e) => e.isTimeout, 'isTimeout', true)
                .having(
                  (e) => e.message,
                  'message',
                  contains('taking longer than expected'),
                ),
          ),
        );
      });

      test('throws network exception on SocketException', () async {
        final request = TaskBreakdownRequest(taskTitle: 'Test task');

        when(() => mockClient.post(
              any(),
              headers: any(named: 'headers'),
              body: any(named: 'body'),
            )).thenThrow(const SocketException('Connection refused'));

        expect(
          () => service.breakdownTask(request),
          throwsA(
            isA<AIServiceException>()
                .having((e) => e.isNetworkError, 'isNetworkError', true)
                .having((e) => e.message, 'message', contains('Network error')),
          ),
        );
      });

      test('throws network exception on ClientException', () async {
        final request = TaskBreakdownRequest(taskTitle: 'Test task');

        when(() => mockClient.post(
              any(),
              headers: any(named: 'headers'),
              body: any(named: 'body'),
            )).thenThrow(http.ClientException('Connection failed'));

        expect(
          () => service.breakdownTask(request),
          throwsA(
            isA<AIServiceException>()
                .having((e) => e.isNetworkError, 'isNetworkError', true)
                .having((e) => e.message, 'message', contains('Network error')),
          ),
        );
      });
    });

    group('prioritizeTasks', () {
      test('returns TaskPrioritizationResponse on successful request',
          () async {
        final request = TaskPrioritizationRequest(
          tasks: [
            {'title': 'Task 1', 'description': 'Description 1'},
            {'title': 'Task 2', 'description': 'Description 2'},
          ],
        );

        final responseBody = {
          'suggestions': [
            {
              'task_title': 'Task 1',
              'priority': 'high',
              'reasoning': 'Critical for project',
              'urgency_score': 0.9,
              'impact_score': 0.8,
            },
            {
              'task_title': 'Task 2',
              'priority': 'medium',
              'reasoning': 'Important but not urgent',
              'urgency_score': 0.5,
              'impact_score': 0.7,
            },
          ],
          'recommended_order': ['Task 1', 'Task 2'],
        };

        when(() => mockClient.post(
              any(),
              headers: any(named: 'headers'),
              body: any(named: 'body'),
            )).thenAnswer(
          (_) async => http.Response(
            jsonEncode(responseBody),
            200,
          ),
        );

        final response = await service.prioritizeTasks(request);

        expect(response.suggestions.length, 2);
        expect(response.suggestions[0].taskTitle, 'Task 1');
        expect(response.suggestions[0].priority, PriorityLevel.high);
        expect(response.suggestions[0].urgencyScore, 0.9);
        expect(response.recommendedOrder, ['Task 1', 'Task 2']);
      });

      test('throws timeout exception when request times out', () async {
        final request = TaskPrioritizationRequest(
          tasks: [
            {'title': 'Task 1'},
          ],
        );

        when(() => mockClient.post(
              any(),
              headers: any(named: 'headers'),
              body: any(named: 'body'),
            )).thenAnswer(
          (_) async {
            await Future<void>.delayed(const Duration(seconds: 2));
            throw TimeoutException('Timed out');
          },
        );

        expect(
          () => service.prioritizeTasks(request),
          throwsA(
            isA<AIServiceException>()
                .having((e) => e.isTimeout, 'isTimeout', true),
          ),
        );
      });
    });

    group('estimateTime', () {
      test('returns TimeEstimateResponse on successful request', () async {
        final request = TimeEstimateRequest(
          taskTitle: 'Build feature',
          skillLevel: SkillLevel.intermediate,
        );

        final responseBody = {
          'task_title': 'Build feature',
          'estimate': {
            'optimistic_minutes': 60,
            'realistic_minutes': 90,
            'pessimistic_minutes': 120,
            'confidence': 0.7,
          },
          'factors': ['Complexity', 'Dependencies'],
          'assumptions': ['No major blockers'],
        };

        when(() => mockClient.post(
              any(),
              headers: any(named: 'headers'),
              body: any(named: 'body'),
            )).thenAnswer(
          (_) async => http.Response(
            jsonEncode(responseBody),
            200,
          ),
        );

        final response = await service.estimateTime(request);

        expect(response.taskTitle, 'Build feature');
        expect(response.estimate.realisticMinutes, 90);
        expect(response.estimate.optimisticMinutes, 60);
        expect(response.estimate.pessimisticMinutes, 120);
        expect(response.estimate.confidence, 0.7);
        expect(response.factors, ['Complexity', 'Dependencies']);
        expect(response.assumptions, ['No major blockers']);
      });
    });

    group('getSuggestions', () {
      test('returns ContextSuggestionResponse on successful request', () async {
        final request = ContextSuggestionRequest(
          taskTitle: 'Learn Flutter',
          suggestionType: 'resources',
        );

        final responseBody = {
          'task_title': 'Learn Flutter',
          'suggestions': [
            {
              'title': 'Official Docs',
              'description': 'Read Flutter documentation',
              'category': 'resource',
            },
            {
              'title': 'Build sample app',
              'description': 'Practice with a small project',
              'category': 'tip',
            },
          ],
          'summary': 'Start with docs and practice',
        };

        when(() => mockClient.post(
              any(),
              headers: any(named: 'headers'),
              body: any(named: 'body'),
            )).thenAnswer(
          (_) async => http.Response(
            jsonEncode(responseBody),
            200,
          ),
        );

        final response = await service.getSuggestions(request);

        expect(response.taskTitle, 'Learn Flutter');
        expect(response.suggestions.length, 2);
        expect(response.suggestions[0].title, 'Official Docs');
        expect(response.suggestions[0].category, 'resource');
        expect(response.summary, 'Start with docs and practice');
      });
    });

    group('checkHealth', () {
      test('returns true when service is healthy', () async {
        when(() => mockClient.get(any())).thenAnswer(
          (_) async => http.Response('', 200),
        );

        final isHealthy = await service.checkHealth();

        expect(isHealthy, true);
        verify(
          () => mockClient.get(
            Uri.parse('http://localhost:8001/api/health'),
          ),
        ).called(1);
      });

      test('returns false when service is unhealthy', () async {
        when(() => mockClient.get(any())).thenAnswer(
          (_) async => http.Response('', 500),
        );

        final isHealthy = await service.checkHealth();

        expect(isHealthy, false);
      });

      test('returns false on network error', () async {
        when(() => mockClient.get(any())).thenThrow(
          const SocketException('Connection refused'),
        );

        final isHealthy = await service.checkHealth();

        expect(isHealthy, false);
      });

      test('returns false on timeout', () async {
        when(() => mockClient.get(any())).thenAnswer(
          (_) async {
            await Future<void>.delayed(const Duration(seconds: 2));
            throw TimeoutException('Timed out');
          },
        );

        final isHealthy = await service.checkHealth();

        expect(isHealthy, false);
      });
    });

    group('with authentication', () {
      test('includes Authorization header when API key is provided', () async {
        final configWithAuth = const AIConfig(
          baseUrl: 'http://localhost:8001/api',
          apiKey: 'test-key-123',
          enableSSL: false,
        );
        final serviceWithAuth = AIService(
          config: configWithAuth,
          httpClient: mockClient,
        );

        final request = TaskBreakdownRequest(taskTitle: 'Test');

        when(() => mockClient.post(
              any(),
              headers: any(named: 'headers'),
              body: any(named: 'body'),
            )).thenAnswer(
          (_) async => http.Response(
            jsonEncode({
              'original_task': 'Test',
              'subtasks': [],
            }),
            200,
          ),
        );

        await serviceWithAuth.breakdownTask(request);

        verify(
          () => mockClient.post(
            any(),
            headers: {
              'Content-Type': 'application/json',
              'Authorization': 'Bearer test-key-123',
            },
            body: any(named: 'body'),
          ),
        ).called(1);

        serviceWithAuth.dispose();
      });
    });

    group('error message extraction', () {
      test('extracts detail from JSON response', () async {
        final request = TaskBreakdownRequest(taskTitle: 'Test');

        when(() => mockClient.post(
              any(),
              headers: any(named: 'headers'),
              body: any(named: 'body'),
            )).thenAnswer(
          (_) async => http.Response(
            jsonEncode({'detail': 'Custom error message'}),
            400,
          ),
        );

        expect(
          () => service.breakdownTask(request),
          throwsA(
            isA<AIServiceException>().having(
              (e) => e.message,
              'message',
              'Custom error message',
            ),
          ),
        );
      });

      test('falls back to HTTP status text when no detail', () async {
        final request = TaskBreakdownRequest(taskTitle: 'Test');

        when(() => mockClient.post(
              any(),
              headers: any(named: 'headers'),
              body: any(named: 'body'),
            )).thenAnswer(
          (_) async => http.Response(
            'Not valid JSON',
            404,
          ),
        );

        expect(
          () => service.breakdownTask(request),
          throwsA(
            isA<AIServiceException>().having(
              (e) => e.message,
              'message',
              contains('HTTP 404'),
            ),
          ),
        );
      });
    });

    test('dispose closes HTTP client', () {
      service.dispose();
      verify(() => mockClient.close()).called(1);
    });
  });
}
