/// Events for AI BLoC.
library;

import 'package:equatable/equatable.dart';

import '../../services/ai/models.dart';

/// Base class for AI events.
sealed class AIEvent extends Equatable {
  /// Creates an AI event.
  const AIEvent();

  @override
  List<Object?> get props => [];
}

/// Event to request task breakdown.
final class AITaskBreakdownRequested extends AIEvent {
  /// Creates a task breakdown request event.
  const AITaskBreakdownRequested({
    required this.request,
  });

  /// The breakdown request.
  final TaskBreakdownRequest request;

  @override
  List<Object?> get props => [request];
}

/// Event to request task prioritization.
final class AITaskPrioritizationRequested extends AIEvent {
  /// Creates a task prioritization request event.
  const AITaskPrioritizationRequested({
    required this.request,
  });

  /// The prioritization request.
  final TaskPrioritizationRequest request;

  @override
  List<Object?> get props => [request];
}

/// Event to request time estimation.
final class AITimeEstimateRequested extends AIEvent {
  /// Creates a time estimate request event.
  const AITimeEstimateRequested({
    required this.request,
  });

  /// The time estimate request.
  final TimeEstimateRequest request;

  @override
  List<Object?> get props => [request];
}

/// Event to request context suggestions.
final class AIContextSuggestionsRequested extends AIEvent {
  /// Creates a context suggestions request event.
  const AIContextSuggestionsRequested({
    required this.request,
  });

  /// The context suggestions request.
  final ContextSuggestionRequest request;

  @override
  List<Object?> get props => [request];
}

/// Event to clear AI state.
final class AIClearState extends AIEvent {
  /// Creates a clear state event.
  const AIClearState();
}
