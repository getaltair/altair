/// OpenAI provider implementation using dart_openai SDK.
library;

import 'dart:convert';

import 'package:dart_openai/dart_openai.dart';
import 'package:logger/logger.dart';

import '../models.dart';
import 'ai_provider.dart';

/// OpenAI implementation of AIProvider using dart_openai SDK.
class OpenAIProvider implements AIProvider {
  /// Creates an OpenAI provider.
  OpenAIProvider({
    required String apiKey,
    String model = 'gpt-4o-mini',
  }) : _model = model {
    OpenAI.apiKey = apiKey;
    OpenAI.showLogs = false;
    OpenAI.showResponsesLogs = false;
    _logger = Logger();
  }

  /// The OpenAI model to use.
  final String _model;

  /// Logger instance.
  late final Logger _logger;

  @override
  Future<TaskBreakdownResponse> breakdownTask(
    TaskBreakdownRequest request,
  ) async {
    _logger.i('[OpenAI] Breaking down task: ${request.taskTitle}');

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
''';

    try {
      final completion = await OpenAI.instance.chat.create(
        model: _model,
        responseFormat: {'type': 'json_object'},
        messages: [
          OpenAIChatCompletionChoiceMessageModel(
            content: [
              OpenAIChatCompletionChoiceMessageContentItemModel.text(
                systemPrompt,
              ),
            ],
            role: OpenAIChatMessageRole.system,
          ),
          OpenAIChatCompletionChoiceMessageModel(
            content: [
              OpenAIChatCompletionChoiceMessageContentItemModel.text(
                userPrompt,
              ),
            ],
            role: OpenAIChatMessageRole.user,
          ),
        ],
        temperature: 0.3,
        maxTokens: 2000,
      );

      final content = completion.choices.first.message.content?.first.text;
      if (content == null) {
        throw Exception('No response from OpenAI');
      }

      final json = jsonDecode(content) as Map<String, dynamic>;
      final response = TaskBreakdownResponse.fromJson(json);
      _logger
          .i('[OpenAI] Generated ${response.subtasks.length} subtasks');
      return response;
    } catch (e) {
      _logger.e('[OpenAI] Task breakdown failed: $e');
      rethrow;
    }
  }

  @override
  Future<TaskPrioritizationResponse> prioritizeTasks(
    TaskPrioritizationRequest request,
  ) async {
    _logger.i('[OpenAI] Prioritizing ${request.tasks.length} tasks');

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
''';

    try {
      final completion = await OpenAI.instance.chat.create(
        model: _model,
        responseFormat: {'type': 'json_object'},
        messages: [
          OpenAIChatCompletionChoiceMessageModel(
            content: [
              OpenAIChatCompletionChoiceMessageContentItemModel.text(
                systemPrompt,
              ),
            ],
            role: OpenAIChatMessageRole.system,
          ),
          OpenAIChatCompletionChoiceMessageModel(
            content: [
              OpenAIChatCompletionChoiceMessageContentItemModel.text(
                userPrompt,
              ),
            ],
            role: OpenAIChatMessageRole.user,
          ),
        ],
        temperature: 0.3,
        maxTokens: 2000,
      );

      final content = completion.choices.first.message.content?.first.text;
      if (content == null) {
        throw Exception('No response from OpenAI');
      }

      final json = jsonDecode(content) as Map<String, dynamic>;
      final response = TaskPrioritizationResponse.fromJson(json);
      _logger.i(
        '[OpenAI] Generated ${response.suggestions.length} priority suggestions',
      );
      return response;
    } catch (e) {
      _logger.e('[OpenAI] Task prioritization failed: $e');
      rethrow;
    }
  }

  @override
  Future<TimeEstimateResponse> estimateTime(
    TimeEstimateRequest request,
  ) async {
    _logger.i('[OpenAI] Estimating time for task: ${request.taskTitle}');

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
''';

    try {
      final completion = await OpenAI.instance.chat.create(
        model: _model,
        responseFormat: {'type': 'json_object'},
        messages: [
          OpenAIChatCompletionChoiceMessageModel(
            content: [
              OpenAIChatCompletionChoiceMessageContentItemModel.text(
                systemPrompt,
              ),
            ],
            role: OpenAIChatMessageRole.system,
          ),
          OpenAIChatCompletionChoiceMessageModel(
            content: [
              OpenAIChatCompletionChoiceMessageContentItemModel.text(
                userPrompt,
              ),
            ],
            role: OpenAIChatMessageRole.user,
          ),
        ],
        temperature: 0.3,
        maxTokens: 1500,
      );

      final content = completion.choices.first.message.content?.first.text;
      if (content == null) {
        throw Exception('No response from OpenAI');
      }

      final json = jsonDecode(content) as Map<String, dynamic>;
      final response = TimeEstimateResponse.fromJson(json);
      _logger.i(
        '[OpenAI] Generated time estimate: ${response.estimate.realisticMinutes} minutes',
      );
      return response;
    } catch (e) {
      _logger.e('[OpenAI] Time estimation failed: $e');
      rethrow;
    }
  }

  @override
  Future<ContextSuggestionResponse> getSuggestions(
    ContextSuggestionRequest request,
  ) async {
    _logger.i('[OpenAI] Getting suggestions for task: ${request.taskTitle}');

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
''';

    try {
      final completion = await OpenAI.instance.chat.create(
        model: _model,
        responseFormat: {'type': 'json_object'},
        messages: [
          OpenAIChatCompletionChoiceMessageModel(
            content: [
              OpenAIChatCompletionChoiceMessageContentItemModel.text(
                systemPrompt,
              ),
            ],
            role: OpenAIChatMessageRole.system,
          ),
          OpenAIChatCompletionChoiceMessageModel(
            content: [
              OpenAIChatCompletionChoiceMessageContentItemModel.text(
                userPrompt,
              ),
            ],
            role: OpenAIChatMessageRole.user,
          ),
        ],
        temperature: 0.5,
        maxTokens: 2000,
      );

      final content = completion.choices.first.message.content?.first.text;
      if (content == null) {
        throw Exception('No response from OpenAI');
      }

      final json = jsonDecode(content) as Map<String, dynamic>;
      final response = ContextSuggestionResponse.fromJson(json);
      _logger.i(
        '[OpenAI] Generated ${response.suggestions.length} suggestions',
      );
      return response;
    } catch (e) {
      _logger.e('[OpenAI] Context suggestions failed: $e');
      rethrow;
    }
  }

  @override
  void dispose() {
    // dart_openai doesn't require explicit cleanup
    _logger.d('[OpenAI] Provider disposed');
  }
}
