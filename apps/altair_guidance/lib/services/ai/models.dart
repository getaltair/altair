/// AI service models for requests and responses.
library;

import 'package:equatable/equatable.dart';

// ============================================================================
// Enums
// ============================================================================

/// Skill level for time estimation.
enum SkillLevel {
  /// Beginner level - less experienced.
  beginner,

  /// Intermediate level - moderate experience.
  intermediate,

  /// Advanced level - highly experienced.
  advanced;

  /// Converts enum to string for API.
  String toJson() => name;

  /// Creates from string.
  static SkillLevel fromString(String value) {
    return SkillLevel.values.firstWhere(
      (e) => e.name == value.toLowerCase(),
      orElse: () => SkillLevel.intermediate,
    );
  }
}

// ============================================================================
// Request Models
// ============================================================================

/// Request to break down a task into subtasks.
class TaskBreakdownRequest extends Equatable {
  /// Creates a task breakdown request.
  ///
  /// Throws [ArgumentError] if validation fails.
  TaskBreakdownRequest({
    required this.taskTitle,
    this.taskDescription,
    this.context,
    this.maxSubtasks = 5,
    this.provider,
    this.apiKey,
  }) {
    _validate();
  }

  /// Task title.
  final String taskTitle;

  /// Optional task description.
  final String? taskDescription;

  /// Additional context about the project or goal.
  final String? context;

  /// Maximum number of subtasks to generate (1-20).
  final int maxSubtasks;

  /// AI provider to use (overrides server default).
  final String? provider;

  /// API key for the provider (required for openai/anthropic if not in server config).
  final String? apiKey;

  /// Validates the request.
  void _validate() {
    if (taskTitle.trim().isEmpty) {
      throw ArgumentError('Task title cannot be empty');
    }
    if (taskTitle.length > 500) {
      throw ArgumentError('Task title must be 500 characters or less');
    }
    if (taskDescription != null && taskDescription!.length > 5000) {
      throw ArgumentError('Task description must be 5000 characters or less');
    }
    if (context != null && context!.length > 2000) {
      throw ArgumentError('Context must be 2000 characters or less');
    }
    if (maxSubtasks < 1 || maxSubtasks > 20) {
      throw ArgumentError('maxSubtasks must be between 1 and 20');
    }
  }

  /// Converts to JSON.
  Map<String, dynamic> toJson() => {
        'task_title': taskTitle,
        if (taskDescription != null) 'task_description': taskDescription,
        if (context != null) 'context': context,
        'max_subtasks': maxSubtasks,
        if (provider != null) 'provider': provider,
        if (apiKey != null) 'api_key': apiKey,
      };

  @override
  List<Object?> get props => [taskTitle, taskDescription, context, maxSubtasks, provider, apiKey];
}

/// Request to get prioritization suggestions for tasks.
class TaskPrioritizationRequest extends Equatable {
  /// Creates a task prioritization request.
  ///
  /// Throws [ArgumentError] if validation fails.
  TaskPrioritizationRequest({
    required this.tasks,
    this.context,
    this.provider,
    this.apiKey,
  }) {
    _validate();
  }

  /// List of tasks with title and description.
  final List<Map<String, String>> tasks;

  /// Project context or goals.
  final String? context;

  /// AI provider to use (overrides server default).
  final String? provider;

  /// API key for the provider (required for openai/anthropic if not in server config).
  final String? apiKey;

  /// Validates the request.
  void _validate() {
    if (tasks.isEmpty) {
      throw ArgumentError('Tasks list cannot be empty');
    }
    if (tasks.length > 50) {
      throw ArgumentError('Cannot prioritize more than 50 tasks at once');
    }
    if (context != null && context!.length > 2000) {
      throw ArgumentError('Context must be 2000 characters or less');
    }
    // Validate each task has required fields
    for (final task in tasks) {
      if (!task.containsKey('title') || task['title']!.trim().isEmpty) {
        throw ArgumentError('Each task must have a non-empty title');
      }
    }
  }

  /// Converts to JSON.
  Map<String, dynamic> toJson() => {
        'tasks': tasks,
        if (context != null) 'context': context,
        if (provider != null) 'provider': provider,
        if (apiKey != null) 'api_key': apiKey,
      };

  @override
  List<Object?> get props => [tasks, context, provider, apiKey];
}

/// Request to estimate time for a task.
class TimeEstimateRequest extends Equatable {
  /// Creates a time estimate request.
  ///
  /// Throws [ArgumentError] if validation fails.
  TimeEstimateRequest({
    required this.taskTitle,
    this.taskDescription,
    this.subtasks,
    this.skillLevel = SkillLevel.intermediate,
    this.provider,
    this.apiKey,
  }) {
    _validate();
  }

  /// Task title.
  final String taskTitle;

  /// Optional task description.
  final String? taskDescription;

  /// List of subtasks.
  final List<String>? subtasks;

  /// User skill level.
  final SkillLevel skillLevel;

  /// AI provider to use (overrides server default).
  final String? provider;

  /// API key for the provider (required for openai/anthropic if not in server config).
  final String? apiKey;

  /// Validates the request.
  void _validate() {
    if (taskTitle.trim().isEmpty) {
      throw ArgumentError('Task title cannot be empty');
    }
    if (taskTitle.length > 500) {
      throw ArgumentError('Task title must be 500 characters or less');
    }
    if (taskDescription != null && taskDescription!.length > 5000) {
      throw ArgumentError('Task description must be 5000 characters or less');
    }
    if (subtasks != null && subtasks!.length > 20) {
      throw ArgumentError('Cannot have more than 20 subtasks');
    }
  }

  /// Converts to JSON.
  Map<String, dynamic> toJson() => {
        'task_title': taskTitle,
        if (taskDescription != null) 'task_description': taskDescription,
        if (subtasks != null) 'subtasks': subtasks,
        'skill_level': skillLevel.toJson(),
        if (provider != null) 'provider': provider,
        if (apiKey != null) 'api_key': apiKey,
      };

  @override
  List<Object?> get props => [taskTitle, taskDescription, subtasks, skillLevel, provider, apiKey];
}

/// Request for contextual suggestions.
class ContextSuggestionRequest extends Equatable {
  /// Creates a context suggestion request.
  ///
  /// Throws [ArgumentError] if validation fails.
  ContextSuggestionRequest({
    required this.taskTitle,
    this.taskDescription,
    this.projectContext,
    this.suggestionType = 'general',
    this.provider,
    this.apiKey,
  }) {
    _validate();
  }

  /// Task title.
  final String taskTitle;

  /// Optional task description.
  final String? taskDescription;

  /// Project context.
  final String? projectContext;

  /// Type of suggestions: general, resources, tips, blockers.
  final String suggestionType;

  /// AI provider to use (overrides server default).
  final String? provider;

  /// API key for the provider (required for openai/anthropic if not in server config).
  final String? apiKey;

  /// Validates the request.
  void _validate() {
    if (taskTitle.trim().isEmpty) {
      throw ArgumentError('Task title cannot be empty');
    }
    if (taskTitle.length > 500) {
      throw ArgumentError('Task title must be 500 characters or less');
    }
    if (taskDescription != null && taskDescription!.length > 5000) {
      throw ArgumentError('Task description must be 5000 characters or less');
    }
    if (projectContext != null && projectContext!.length > 2000) {
      throw ArgumentError('Project context must be 2000 characters or less');
    }
    const validTypes = ['general', 'resources', 'tips', 'blockers'];
    if (!validTypes.contains(suggestionType)) {
      throw ArgumentError(
        'Invalid suggestion type: $suggestionType. '
        'Must be one of: ${validTypes.join(", ")}',
      );
    }
  }

  /// Converts to JSON.
  Map<String, dynamic> toJson() => {
        'task_title': taskTitle,
        if (taskDescription != null) 'task_description': taskDescription,
        if (projectContext != null) 'project_context': projectContext,
        'suggestion_type': suggestionType,
        if (provider != null) 'provider': provider,
        if (apiKey != null) 'api_key': apiKey,
      };

  @override
  List<Object?> get props =>
      [taskTitle, taskDescription, projectContext, suggestionType, provider, apiKey];
}

// ============================================================================
// Response Models
// ============================================================================

/// A suggested subtask.
class SubtaskSuggestion extends Equatable {
  /// Creates a subtask suggestion.
  const SubtaskSuggestion({
    required this.title,
    this.description,
    this.estimatedMinutes,
    required this.order,
  });

  /// Creates from JSON.
  factory SubtaskSuggestion.fromJson(Map<String, dynamic> json) {
    return SubtaskSuggestion(
      title: json['title'] as String,
      description: json['description'] as String?,
      estimatedMinutes: json['estimated_minutes'] as int?,
      order: json['order'] as int,
    );
  }

  /// Subtask title.
  final String title;

  /// Detailed description.
  final String? description;

  /// Estimated time in minutes.
  final int? estimatedMinutes;

  /// Suggested order of execution.
  final int order;

  @override
  List<Object?> get props => [title, description, estimatedMinutes, order];
}

/// Response containing task breakdown suggestions.
class TaskBreakdownResponse extends Equatable {
  /// Creates a task breakdown response.
  const TaskBreakdownResponse({
    required this.originalTask,
    required this.subtasks,
    this.totalEstimatedMinutes,
    this.reasoning,
  });

  /// Creates from JSON.
  factory TaskBreakdownResponse.fromJson(Map<String, dynamic> json) {
    return TaskBreakdownResponse(
      originalTask: json['original_task'] as String,
      subtasks: (json['subtasks'] as List)
          .map((e) => SubtaskSuggestion.fromJson(e as Map<String, dynamic>))
          .toList(),
      totalEstimatedMinutes: json['total_estimated_minutes'] as int?,
      reasoning: json['reasoning'] as String?,
    );
  }

  /// Original task title.
  final String originalTask;

  /// List of suggested subtasks.
  final List<SubtaskSuggestion> subtasks;

  /// Total estimated time for all subtasks.
  final int? totalEstimatedMinutes;

  /// AI reasoning for the breakdown.
  final String? reasoning;

  @override
  List<Object?> get props =>
      [originalTask, subtasks, totalEstimatedMinutes, reasoning];
}

/// Priority levels for tasks.
enum PriorityLevel {
  /// Critical priority.
  critical,

  /// High priority.
  high,

  /// Medium priority.
  medium,

  /// Low priority.
  low;

  /// Creates from string.
  static PriorityLevel fromString(String value) {
    return PriorityLevel.values.firstWhere(
      (e) => e.name == value.toLowerCase(),
      orElse: () => PriorityLevel.medium,
    );
  }
}

/// Priority suggestion for a task.
class PrioritySuggestion extends Equatable {
  /// Creates a priority suggestion.
  const PrioritySuggestion({
    this.taskId,
    required this.taskTitle,
    required this.priority,
    required this.reasoning,
    required this.urgencyScore,
    required this.impactScore,
  });

  /// Creates from JSON.
  factory PrioritySuggestion.fromJson(Map<String, dynamic> json) {
    return PrioritySuggestion(
      taskId: json['task_id'] as String?,
      taskTitle: json['task_title'] as String,
      priority: PriorityLevel.fromString(json['priority'] as String),
      reasoning: json['reasoning'] as String,
      urgencyScore: (json['urgency_score'] as num).toDouble(),
      impactScore: (json['impact_score'] as num).toDouble(),
    );
  }

  /// Task identifier from request.
  final String? taskId;

  /// Task title.
  final String taskTitle;

  /// Suggested priority level.
  final PriorityLevel priority;

  /// Why this priority level.
  final String reasoning;

  /// Urgency score (0-1).
  final double urgencyScore;

  /// Impact score (0-1).
  final double impactScore;

  @override
  List<Object?> get props =>
      [taskId, taskTitle, priority, reasoning, urgencyScore, impactScore];
}

/// Response containing task prioritization suggestions.
class TaskPrioritizationResponse extends Equatable {
  /// Creates a task prioritization response.
  const TaskPrioritizationResponse({
    required this.suggestions,
    required this.recommendedOrder,
  });

  /// Creates from JSON.
  factory TaskPrioritizationResponse.fromJson(Map<String, dynamic> json) {
    return TaskPrioritizationResponse(
      suggestions: (json['suggestions'] as List)
          .map((e) => PrioritySuggestion.fromJson(e as Map<String, dynamic>))
          .toList(),
      recommendedOrder:
          (json['recommended_order'] as List).map((e) => e as String).toList(),
    );
  }

  /// Priority suggestions per task.
  final List<PrioritySuggestion> suggestions;

  /// Recommended execution order (task titles).
  final List<String> recommendedOrder;

  @override
  List<Object?> get props => [suggestions, recommendedOrder];
}

/// Time estimate for a task.
class TimeEstimate extends Equatable {
  /// Creates a time estimate.
  const TimeEstimate({
    required this.optimisticMinutes,
    required this.realisticMinutes,
    required this.pessimisticMinutes,
    required this.confidence,
  });

  /// Creates from JSON.
  factory TimeEstimate.fromJson(Map<String, dynamic> json) {
    return TimeEstimate(
      optimisticMinutes: json['optimistic_minutes'] as int,
      realisticMinutes: json['realistic_minutes'] as int,
      pessimisticMinutes: json['pessimistic_minutes'] as int,
      confidence: (json['confidence'] as num).toDouble(),
    );
  }

  /// Best-case estimate.
  final int optimisticMinutes;

  /// Most likely estimate.
  final int realisticMinutes;

  /// Worst-case estimate.
  final int pessimisticMinutes;

  /// Confidence in estimate (0-1).
  final double confidence;

  @override
  List<Object?> get props =>
      [optimisticMinutes, realisticMinutes, pessimisticMinutes, confidence];
}

/// Response containing time estimates.
class TimeEstimateResponse extends Equatable {
  /// Creates a time estimate response.
  const TimeEstimateResponse({
    required this.taskTitle,
    required this.estimate,
    required this.factors,
    required this.assumptions,
  });

  /// Creates from JSON.
  factory TimeEstimateResponse.fromJson(Map<String, dynamic> json) {
    return TimeEstimateResponse(
      taskTitle: json['task_title'] as String,
      estimate: TimeEstimate.fromJson(json['estimate'] as Map<String, dynamic>),
      factors: (json['factors'] as List).map((e) => e as String).toList(),
      assumptions:
          (json['assumptions'] as List).map((e) => e as String).toList(),
    );
  }

  /// Task being estimated.
  final String taskTitle;

  /// Time estimates.
  final TimeEstimate estimate;

  /// Factors considered in estimation.
  final List<String> factors;

  /// Assumptions made.
  final List<String> assumptions;

  @override
  List<Object?> get props => [taskTitle, estimate, factors, assumptions];
}

/// A contextual suggestion.
class ContextSuggestion extends Equatable {
  /// Creates a context suggestion.
  const ContextSuggestion({
    required this.title,
    required this.description,
    required this.category,
    this.priority,
  });

  /// Creates from JSON.
  factory ContextSuggestion.fromJson(Map<String, dynamic> json) {
    return ContextSuggestion(
      title: json['title'] as String,
      description: json['description'] as String,
      category: json['category'] as String,
      priority: json['priority'] != null
          ? PriorityLevel.fromString(json['priority'] as String)
          : null,
    );
  }

  /// Suggestion title.
  final String title;

  /// Detailed suggestion.
  final String description;

  /// Suggestion category: resource, tip, blocker, etc.
  final String category;

  /// Suggestion priority.
  final PriorityLevel? priority;

  @override
  List<Object?> get props => [title, description, category, priority];
}

/// Response containing contextual suggestions.
class ContextSuggestionResponse extends Equatable {
  /// Creates a context suggestion response.
  const ContextSuggestionResponse({
    required this.taskTitle,
    required this.suggestions,
    this.summary,
  });

  /// Creates from JSON.
  factory ContextSuggestionResponse.fromJson(Map<String, dynamic> json) {
    return ContextSuggestionResponse(
      taskTitle: json['task_title'] as String,
      suggestions: (json['suggestions'] as List)
          .map((e) => ContextSuggestion.fromJson(e as Map<String, dynamic>))
          .toList(),
      summary: json['summary'] as String?,
    );
  }

  /// Task being analyzed.
  final String taskTitle;

  /// List of suggestions.
  final List<ContextSuggestion> suggestions;

  /// Overall summary of suggestions.
  final String? summary;

  @override
  List<Object?> get props => [taskTitle, suggestions, summary];
}
