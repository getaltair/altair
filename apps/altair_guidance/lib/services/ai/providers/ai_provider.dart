/// Abstract interface for AI providers.
library;

import '../models.dart';

/// Abstract interface that all AI providers must implement.
abstract class AIProvider {
  /// Breaks down a task into subtasks.
  ///
  /// Throws [Exception] if the request fails.
  Future<TaskBreakdownResponse> breakdownTask(TaskBreakdownRequest request);

  /// Gets prioritization suggestions for tasks.
  ///
  /// Throws [Exception] if the request fails.
  Future<TaskPrioritizationResponse> prioritizeTasks(
    TaskPrioritizationRequest request,
  );

  /// Estimates time for a task.
  ///
  /// Throws [Exception] if the request fails.
  Future<TimeEstimateResponse> estimateTime(TimeEstimateRequest request);

  /// Gets contextual suggestions for a task.
  ///
  /// Throws [Exception] if the request fails.
  Future<ContextSuggestionResponse> getSuggestions(
    ContextSuggestionRequest request,
  );

  /// Disposes of any resources held by the provider.
  void dispose();
}
