/// Anthropic provider implementation using anthropic_sdk_dart SDK.
library;

import 'dart:convert';

import 'package:anthropic_sdk_dart/anthropic_sdk_dart.dart';
import 'package:logger/logger.dart';

import '../models.dart';
import 'ai_provider.dart';

/// Anthropic implementation of AIProvider using anthropic_sdk_dart SDK.
class AnthropicProvider implements AIProvider {
  /// Creates an Anthropic provider.
  AnthropicProvider({
    required String apiKey,
    String model = 'claude-sonnet-4-5',
  })  : _client = AnthropicClient(apiKey: apiKey),
        _model = Model.modelId(model) {
    _logger = Logger();
  }

  /// The Anthropic API client.
  final AnthropicClient _client;

  /// The Anthropic model to use.
  final Model _model;

  /// Logger instance.
  late final Logger _logger;

  @override
  Future<TaskBreakdownResponse> breakdownTask(
    TaskBreakdownRequest request,
  ) async {
    _logger.i('[Anthropic] Breaking down task: ${request.taskTitle}');

    final systemPrompt = '''
You are a task planning expert. Break down tasks into clear, actionable subtasks.
Return your response as a valid JSON object with this exact structure:
{
  "original_task": "the original task title",
  "subtasks": [
    {
      "title": "subtask title",
      "description": "detailed description",
      "estimated_minutes": 30,
      "order": 1
    }
  ],
  "total_estimated_minutes": 90,
  "reasoning": "brief explanation of the breakdown"
}
''';

    final userPrompt = '''
Task: ${request.taskTitle}
${request.taskDescription != null ? 'Description: ${request.taskDescription}' : ''}
${request.context != null ? 'Context: ${request.context}' : ''}
Max subtasks: ${request.maxSubtasks}

Please break this down into ${request.maxSubtasks} or fewer actionable subtasks.
Return ONLY valid JSON matching the structure above.
''';

    try {
      final response = await _client.createMessage(
        request: CreateMessageRequest(
          model: _model,
          messages: [
            Message(
              role: MessageRole.user,
              content: MessageContent.text(userPrompt),
            ),
            // Prefill with { to force clean JSON output
            Message(
              role: MessageRole.assistant,
              content: MessageContent.text('{'),
            ),
          ],
          system: CreateMessageRequestSystem.text(systemPrompt),
          maxTokens: 2000,
          temperature: 0.3,
        ),
      );

      final content = response.content.text;
      if (content.isEmpty) {
        throw Exception('No response from Anthropic');
      }

      // Prepend { since it's not included in the response
      final fullJson = '{$content';
      final json = jsonDecode(fullJson) as Map<String, dynamic>;
      final result = TaskBreakdownResponse.fromJson(json);
      _logger.i('[Anthropic] Generated ${result.subtasks.length} subtasks');
      return result;
    } catch (e) {
      _logger.e('[Anthropic] Task breakdown failed: $e');
      rethrow;
    }
  }

  @override
  Future<TaskPrioritizationResponse> prioritizeTasks(
    TaskPrioritizationRequest request,
  ) async {
    _logger.i('[Anthropic] Prioritizing ${request.tasks.length} tasks');

    final systemPrompt = '''
You are a task prioritization expert. Analyze tasks and suggest priority levels.
Return your response as a valid JSON object with this exact structure:
{
  "suggestions": [
    {
      "task_title": "task title",
      "priority": "high",
      "reasoning": "why this priority",
      "urgency_score": 0.8,
      "impact_score": 0.7
    }
  ],
  "recommended_order": ["task1 title", "task2 title"]
}

Valid priority levels: "critical", "high", "medium", "low"
Scores must be between 0 and 1.
''';

    final tasksJson = request.tasks.map((task) => task['title']).join('\n');
    final userPrompt = '''
Tasks to prioritize:
$tasksJson

${request.context != null ? 'Context: ${request.context}' : ''}

Please analyze and prioritize these tasks.
Return ONLY valid JSON matching the structure above.
''';

    try {
      final response = await _client.createMessage(
        request: CreateMessageRequest(
          model: _model,
          messages: [
            Message(
              role: MessageRole.user,
              content: MessageContent.text(userPrompt),
            ),
            // Prefill with { to force clean JSON output
            Message(
              role: MessageRole.assistant,
              content: MessageContent.text('{'),
            ),
          ],
          system: CreateMessageRequestSystem.text(systemPrompt),
          maxTokens: 2000,
          temperature: 0.3,
        ),
      );

      final content = response.content.text;
      if (content.isEmpty) {
        throw Exception('No response from Anthropic');
      }

      // Prepend { since it's not included in the response
      final fullJson = '{$content';
      final json = jsonDecode(fullJson) as Map<String, dynamic>;
      final result = TaskPrioritizationResponse.fromJson(json);
      _logger.i(
        '[Anthropic] Generated ${result.suggestions.length} priority suggestions',
      );
      return result;
    } catch (e) {
      _logger.e('[Anthropic] Task prioritization failed: $e');
      rethrow;
    }
  }

  @override
  Future<TimeEstimateResponse> estimateTime(
    TimeEstimateRequest request,
  ) async {
    _logger.i('[Anthropic] Estimating time for task: ${request.taskTitle}');

    final systemPrompt = '''
You are a time estimation expert. Provide realistic time estimates for tasks.
Return your response as a valid JSON object with this exact structure:
{
  "task_title": "the task title",
  "estimate": {
    "optimistic_minutes": 30,
    "realistic_minutes": 60,
    "pessimistic_minutes": 90,
    "confidence": 0.7
  },
  "factors": ["factor1", "factor2"],
  "assumptions": ["assumption1", "assumption2"]
}

Confidence must be between 0 and 1.
''';

    final userPrompt = '''
Task: ${request.taskTitle}
${request.taskDescription != null ? 'Description: ${request.taskDescription}' : ''}
${request.subtasks != null ? 'Subtasks:\n${request.subtasks!.join('\n')}' : ''}
Skill level: ${request.skillLevel.name}

Please provide time estimates.
Return ONLY valid JSON matching the structure above.
''';

    try {
      final response = await _client.createMessage(
        request: CreateMessageRequest(
          model: _model,
          messages: [
            Message(
              role: MessageRole.user,
              content: MessageContent.text(userPrompt),
            ),
            // Prefill with { to force clean JSON output
            Message(
              role: MessageRole.assistant,
              content: MessageContent.text('{'),
            ),
          ],
          system: CreateMessageRequestSystem.text(systemPrompt),
          maxTokens: 1500,
          temperature: 0.3,
        ),
      );

      final content = response.content.text;
      if (content.isEmpty) {
        throw Exception('No response from Anthropic');
      }

      // Prepend { since it's not included in the response
      final fullJson = '{$content';
      final json = jsonDecode(fullJson) as Map<String, dynamic>;
      final result = TimeEstimateResponse.fromJson(json);
      _logger.i(
        '[Anthropic] Generated time estimate: ${result.estimate.realisticMinutes} minutes',
      );
      return result;
    } catch (e) {
      _logger.e('[Anthropic] Time estimation failed: $e');
      rethrow;
    }
  }

  @override
  Future<ContextSuggestionResponse> getSuggestions(
    ContextSuggestionRequest request,
  ) async {
    _logger.i('[Anthropic] Getting suggestions for task: ${request.taskTitle}');

    final systemPrompt = '''
You are a helpful task assistant. Provide contextual suggestions for tasks.
Return your response as a valid JSON object with this exact structure:
{
  "task_title": "the task title",
  "suggestions": [
    {
      "title": "suggestion title",
      "description": "detailed suggestion",
      "category": "resource"
    }
  ],
  "summary": "brief overall summary"
}

Valid categories: "resource", "tip", "blocker", "general"
''';

    final userPrompt = '''
Task: ${request.taskTitle}
${request.taskDescription != null ? 'Description: ${request.taskDescription}' : ''}
${request.projectContext != null ? 'Project context: ${request.projectContext}' : ''}
Suggestion type: ${request.suggestionType}

Please provide helpful ${request.suggestionType} suggestions.
Return ONLY valid JSON matching the structure above.
''';

    try {
      final response = await _client.createMessage(
        request: CreateMessageRequest(
          model: _model,
          messages: [
            Message(
              role: MessageRole.user,
              content: MessageContent.text(userPrompt),
            ),
            // Prefill with { to force clean JSON output
            Message(
              role: MessageRole.assistant,
              content: MessageContent.text('{'),
            ),
          ],
          system: CreateMessageRequestSystem.text(systemPrompt),
          maxTokens: 2000,
          temperature: 0.2,
        ),
      );

      final content = response.content.text;
      if (content.isEmpty) {
        throw Exception('No response from Anthropic');
      }

      // Prepend { since it's not included in the response
      final fullJson = '{$content';
      final json = jsonDecode(fullJson) as Map<String, dynamic>;
      final result = ContextSuggestionResponse.fromJson(json);
      _logger.i(
        '[Anthropic] Generated ${result.suggestions.length} suggestions',
      );
      return result;
    } catch (e) {
      _logger.e('[Anthropic] Context suggestions failed: $e');
      rethrow;
    }
  }

  @override
  void dispose() {
    // anthropic_sdk_dart doesn't require explicit cleanup
    _logger.d('[Anthropic] Provider disposed');
  }
}
