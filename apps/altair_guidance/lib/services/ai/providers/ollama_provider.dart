/// Ollama provider implementation using ollama_dart SDK.
library;

import 'dart:convert';

import 'package:logger/logger.dart';
import 'package:ollama_dart/ollama_dart.dart';

import '../models.dart';
import 'ai_provider.dart';

/// Ollama implementation of AIProvider using ollama_dart SDK.
class OllamaProvider implements AIProvider {
  /// Creates an Ollama provider.
  OllamaProvider({
    required String baseUrl,
    String model = 'llama3.1:8b',
  })  : _model = model,
        _client = OllamaClient(
            baseUrl: baseUrl.endsWith('/api') ? baseUrl : '$baseUrl/api') {
    _logger = Logger();
  }

  /// The Ollama model to use.
  final String _model;

  /// Ollama client instance.
  final OllamaClient _client;

  /// Logger instance.
  late final Logger _logger;

  /// Helper to parse JSON from response, handling markdown code blocks.
  Map<String, dynamic> _parseJsonResponse(String response) {
    // Remove markdown code blocks if present
    var cleaned = response.trim();
    if (cleaned.startsWith('```json')) {
      cleaned = cleaned.substring(7);
    }
    if (cleaned.startsWith('```')) {
      cleaned = cleaned.substring(3);
    }
    if (cleaned.endsWith('```')) {
      cleaned = cleaned.substring(0, cleaned.length - 3);
    }

    try {
      return jsonDecode(cleaned.trim()) as Map<String, dynamic>;
    } catch (e) {
      _logger.e('[Ollama] Failed to parse JSON: $e\nResponse: $response');
      rethrow;
    }
  }

  @override
  Future<TaskBreakdownResponse> breakdownTask(
    TaskBreakdownRequest request,
  ) async {
    _logger.i('[Ollama] Breaking down task: ${request.taskTitle}');

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
Return ONLY the JSON object, no additional text.
''';

    try {
      final completion = await _client.generateChatCompletion(
        request: GenerateChatCompletionRequest(
          model: _model,
          messages: [
            Message(
              role: MessageRole.system,
              content: systemPrompt,
            ),
            Message(
              role: MessageRole.user,
              content: userPrompt,
            ),
          ],
          options: RequestOptions(
            temperature: 0.3,
            numPredict: 2000,
          ),
          format: GenerateChatCompletionRequestFormat.json(
            GenerateChatCompletionRequestFormatEnum.json,
          ),
        ),
      );

      final content = completion.message.content;

      final json = _parseJsonResponse(content);
      final response = TaskBreakdownResponse.fromJson(json);
      _logger.i('[Ollama] Generated ${response.subtasks.length} subtasks');
      return response;
    } catch (e) {
      _logger.e('[Ollama] Task breakdown failed: $e');
      rethrow;
    }
  }

  @override
  Future<TaskPrioritizationResponse> prioritizeTasks(
    TaskPrioritizationRequest request,
  ) async {
    _logger.i('[Ollama] Prioritizing ${request.tasks.length} tasks');

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
Return ONLY the JSON object, no additional text.
''';

    try {
      final completion = await _client.generateChatCompletion(
        request: GenerateChatCompletionRequest(
          model: _model,
          messages: [
            Message(
              role: MessageRole.system,
              content: systemPrompt,
            ),
            Message(
              role: MessageRole.user,
              content: userPrompt,
            ),
          ],
          options: RequestOptions(
            temperature: 0.3,
            numPredict: 2000,
          ),
          format: GenerateChatCompletionRequestFormat.json(
            GenerateChatCompletionRequestFormatEnum.json,
          ),
        ),
      );

      final content = completion.message.content;

      final json = _parseJsonResponse(content);
      final response = TaskPrioritizationResponse.fromJson(json);
      _logger.i(
        '[Ollama] Generated ${response.suggestions.length} priority suggestions',
      );
      return response;
    } catch (e) {
      _logger.e('[Ollama] Task prioritization failed: $e');
      rethrow;
    }
  }

  @override
  Future<TimeEstimateResponse> estimateTime(
    TimeEstimateRequest request,
  ) async {
    _logger.i('[Ollama] Estimating time for task: ${request.taskTitle}');

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
Return ONLY the JSON object, no additional text.
''';

    try {
      final completion = await _client.generateChatCompletion(
        request: GenerateChatCompletionRequest(
          model: _model,
          messages: [
            Message(
              role: MessageRole.system,
              content: systemPrompt,
            ),
            Message(
              role: MessageRole.user,
              content: userPrompt,
            ),
          ],
          options: RequestOptions(
            temperature: 0.3,
            numPredict: 1500,
          ),
          format: GenerateChatCompletionRequestFormat.json(
            GenerateChatCompletionRequestFormatEnum.json,
          ),
        ),
      );

      final content = completion.message.content;

      final json = _parseJsonResponse(content);
      final response = TimeEstimateResponse.fromJson(json);
      _logger.i(
        '[Ollama] Generated time estimate: ${response.estimate.realisticMinutes} minutes',
      );
      return response;
    } catch (e) {
      _logger.e('[Ollama] Time estimation failed: $e');
      rethrow;
    }
  }

  @override
  Future<ContextSuggestionResponse> getSuggestions(
    ContextSuggestionRequest request,
  ) async {
    _logger.i('[Ollama] Getting suggestions for task: ${request.taskTitle}');

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
Return ONLY the JSON object, no additional text.
''';

    try {
      final completion = await _client.generateChatCompletion(
        request: GenerateChatCompletionRequest(
          model: _model,
          messages: [
            Message(
              role: MessageRole.system,
              content: systemPrompt,
            ),
            Message(
              role: MessageRole.user,
              content: userPrompt,
            ),
          ],
          options: RequestOptions(
            temperature: 0.5,
            numPredict: 2000,
          ),
          format: GenerateChatCompletionRequestFormat.json(
            GenerateChatCompletionRequestFormatEnum.json,
          ),
        ),
      );

      final content = completion.message.content;

      final json = _parseJsonResponse(content);
      final response = ContextSuggestionResponse.fromJson(json);
      _logger.i(
        '[Ollama] Generated ${response.suggestions.length} suggestions',
      );
      return response;
    } catch (e) {
      _logger.e('[Ollama] Context suggestions failed: $e');
      rethrow;
    }
  }

  @override
  void dispose() {
    _client.endSession();
    _logger.d('[Ollama] Provider disposed');
  }
}
