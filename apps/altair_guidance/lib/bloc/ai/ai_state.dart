/// States for AI BLoC.
library;

import 'package:equatable/equatable.dart';

import '../../services/ai/models.dart';

/// Base class for AI states.
sealed class AIState extends Equatable {
  /// Creates an AI state.
  const AIState();

  @override
  List<Object?> get props => [];
}

/// Initial state before any AI operations.
final class AIInitial extends AIState {
  /// Creates an initial state.
  const AIInitial();
}

/// State when AI operation is in progress.
final class AILoading extends AIState {
  /// Creates a loading state.
  const AILoading({required this.operationType});

  /// Type of AI operation in progress.
  final AIOperationType operationType;

  @override
  List<Object?> get props => [operationType];
}

/// State when task breakdown is successful.
final class AITaskBreakdownSuccess extends AIState {
  /// Creates a task breakdown success state.
  const AITaskBreakdownSuccess({required this.response});

  /// The breakdown response.
  final TaskBreakdownResponse response;

  @override
  List<Object?> get props => [response];
}

/// State when task prioritization is successful.
final class AITaskPrioritizationSuccess extends AIState {
  /// Creates a task prioritization success state.
  const AITaskPrioritizationSuccess({required this.response});

  /// The prioritization response.
  final TaskPrioritizationResponse response;

  @override
  List<Object?> get props => [response];
}

/// State when time estimation is successful.
final class AITimeEstimateSuccess extends AIState {
  /// Creates a time estimate success state.
  const AITimeEstimateSuccess({required this.response});

  /// The time estimate response.
  final TimeEstimateResponse response;

  @override
  List<Object?> get props => [response];
}

/// State when context suggestions are successful.
final class AIContextSuggestionsSuccess extends AIState {
  /// Creates a context suggestions success state.
  const AIContextSuggestionsSuccess({required this.response});

  /// The context suggestions response.
  final ContextSuggestionResponse response;

  @override
  List<Object?> get props => [response];
}

/// State when AI operation fails.
final class AIFailure extends AIState {
  /// Creates a failure state.
  const AIFailure({
    required this.message,
    required this.operationType,
  });

  /// Error message.
  final String message;

  /// Type of AI operation that failed.
  final AIOperationType operationType;

  @override
  List<Object?> get props => [message, operationType];
}

/// Types of AI operations.
enum AIOperationType {
  /// Task breakdown operation.
  breakdown,

  /// Task prioritization operation.
  prioritization,

  /// Time estimation operation.
  timeEstimate,

  /// Context suggestions operation.
  contextSuggestions,
}
