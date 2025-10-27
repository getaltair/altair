/// BLoC for managing AI operations.
library;

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/logger.dart';

import '../../services/ai/ai_config.dart';
import '../../services/ai/providers/ai_provider.dart';
import '../settings/settings_bloc.dart';
import '../settings/settings_state.dart';
import 'ai_event.dart';
import 'ai_state.dart';

/// BLoC for AI operations.
class AIBloc extends Bloc<AIEvent, AIState> {
  /// Creates an AI BLoC.
  AIBloc({
    required SettingsBloc settingsBloc,
  })  : _settingsBloc = settingsBloc,
        super(const AIInitial()) {
    _logger = Logger();

    on<AITaskBreakdownRequested>(_onTaskBreakdownRequested);
    on<AITaskPrioritizationRequested>(_onTaskPrioritizationRequested);
    on<AITimeEstimateRequested>(_onTimeEstimateRequested);
    on<AIContextSuggestionsRequested>(_onContextSuggestionsRequested);
    on<AIClearState>(_onClearState);
  }

  final SettingsBloc _settingsBloc;
  late final Logger _logger;

  /// Gets the current AI provider based on settings.
  /// Returns null if AI is disabled or settings not loaded.
  AIProvider? _getCurrentProvider() {
    final state = _settingsBloc.state;
    if (state is! SettingsLoaded) {
      _logger.w('Settings not loaded, AI provider unavailable');
      return null;
    }

    final settings = state.aiSettings;
    if (!settings.enabled) {
      _logger.i('AI features disabled in settings');
      return null;
    }

    try {
      final provider = AIConfig.createProvider(settings);
      if (provider == null) {
        _logger.w(
            'Failed to create AI provider from settings: ${settings.provider.displayName}');
      } else {
        _logger.i('Created ${settings.provider.displayName} provider');
      }
      return provider;
    } catch (e) {
      _logger.e('Error creating AI provider: $e');
      return null;
    }
  }

  Future<void> _onTaskBreakdownRequested(
    AITaskBreakdownRequested event,
    Emitter<AIState> emit,
  ) async {
    _logger.i('Requesting task breakdown: ${event.request.taskTitle}');
    emit(const AILoading(operationType: AIOperationType.breakdown));

    try {
      // Get current AI provider from settings
      final provider = _getCurrentProvider();
      if (provider == null) {
        throw Exception(
          'AI features are disabled. Please configure an AI provider in settings.',
        );
      }

      final response = await provider.breakdownTask(event.request);
      _logger
          .i('Task breakdown successful: ${response.subtasks.length} subtasks');
      emit(AITaskBreakdownSuccess(response: response));
    } catch (e) {
      _logger.e('Task breakdown failed: $e');
      emit(
        AIFailure(
          message: e.toString(),
          operationType: AIOperationType.breakdown,
        ),
      );
    }
  }

  Future<void> _onTaskPrioritizationRequested(
    AITaskPrioritizationRequested event,
    Emitter<AIState> emit,
  ) async {
    _logger.i(
        'Requesting task prioritization for ${event.request.tasks.length} tasks');
    emit(const AILoading(operationType: AIOperationType.prioritization));

    try {
      // Get current AI provider from settings
      final provider = _getCurrentProvider();
      if (provider == null) {
        throw Exception(
          'AI features are disabled. Please configure an AI provider in settings.',
        );
      }

      final response = await provider.prioritizeTasks(event.request);
      _logger.i(
        'Task prioritization successful: ${response.suggestions.length} suggestions',
      );
      emit(AITaskPrioritizationSuccess(response: response));
    } catch (e) {
      _logger.e('Task prioritization failed: $e');
      emit(
        AIFailure(
          message: e.toString(),
          operationType: AIOperationType.prioritization,
        ),
      );
    }
  }

  Future<void> _onTimeEstimateRequested(
    AITimeEstimateRequested event,
    Emitter<AIState> emit,
  ) async {
    _logger.i('Requesting time estimate: ${event.request.taskTitle}');
    emit(const AILoading(operationType: AIOperationType.timeEstimate));

    try {
      // Get current AI provider from settings
      final provider = _getCurrentProvider();
      if (provider == null) {
        throw Exception(
          'AI features are disabled. Please configure an AI provider in settings.',
        );
      }

      final response = await provider.estimateTime(event.request);
      _logger.i(
        'Time estimate successful: ${response.estimate.realisticMinutes} minutes',
      );
      emit(AITimeEstimateSuccess(response: response));
    } catch (e) {
      _logger.e('Time estimation failed: $e');
      emit(
        AIFailure(
          message: e.toString(),
          operationType: AIOperationType.timeEstimate,
        ),
      );
    }
  }

  Future<void> _onContextSuggestionsRequested(
    AIContextSuggestionsRequested event,
    Emitter<AIState> emit,
  ) async {
    _logger.i('Requesting context suggestions: ${event.request.taskTitle}');
    emit(const AILoading(operationType: AIOperationType.contextSuggestions));

    try {
      // Get current AI provider from settings
      final provider = _getCurrentProvider();
      if (provider == null) {
        throw Exception(
          'AI features are disabled. Please configure an AI provider in settings.',
        );
      }

      final response = await provider.getSuggestions(event.request);
      _logger.i(
        'Context suggestions successful: ${response.suggestions.length} suggestions',
      );
      emit(AIContextSuggestionsSuccess(response: response));
    } catch (e) {
      _logger.e('Context suggestions failed: $e');
      emit(
        AIFailure(
          message: e.toString(),
          operationType: AIOperationType.contextSuggestions,
        ),
      );
    }
  }

  void _onClearState(
    AIClearState event,
    Emitter<AIState> emit,
  ) {
    _logger.d('Clearing AI state');
    emit(const AIInitial());
  }
}
