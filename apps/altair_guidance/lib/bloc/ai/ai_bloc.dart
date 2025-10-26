/// BLoC for managing AI operations.
library;

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/logger.dart';

import '../../services/ai/ai_service.dart';
import '../../services/ai/models.dart';
import '../settings/settings_bloc.dart';
import '../settings/settings_state.dart';
import 'ai_event.dart';
import 'ai_state.dart';

/// BLoC for AI operations.
class AIBloc extends Bloc<AIEvent, AIState> {
  /// Creates an AI BLoC.
  AIBloc({
    required AIService aiService,
    required SettingsBloc settingsBloc,
  })  : _aiService = aiService,
        _settingsBloc = settingsBloc,
        super(const AIInitial()) {
    _logger = Logger();

    on<AITaskBreakdownRequested>(_onTaskBreakdownRequested);
    on<AITaskPrioritizationRequested>(_onTaskPrioritizationRequested);
    on<AITimeEstimateRequested>(_onTimeEstimateRequested);
    on<AIContextSuggestionsRequested>(_onContextSuggestionsRequested);
    on<AIClearState>(_onClearState);
  }

  final AIService _aiService;
  final SettingsBloc _settingsBloc;
  late final Logger _logger;

  /// Extracts provider and API key from settings.
  ({String? provider, String? apiKey}) _getProviderConfig() {
    final state = _settingsBloc.state;
    if (state is! SettingsLoaded) {
      return (provider: null, apiKey: null);
    }

    final settings = state.aiSettings;
    if (!settings.enabled) {
      return (provider: null, apiKey: null);
    }

    return (
      provider: settings.provider.name,
      apiKey: settings.currentApiKey,
    );
  }

  Future<void> _onTaskBreakdownRequested(
    AITaskBreakdownRequested event,
    Emitter<AIState> emit,
  ) async {
    _logger.i('Requesting task breakdown: ${event.request.taskTitle}');
    emit(const AILoading(operationType: AIOperationType.breakdown));

    try {
      // Inject provider and API key from settings
      final config = _getProviderConfig();
      final request = TaskBreakdownRequest(
        taskTitle: event.request.taskTitle,
        taskDescription: event.request.taskDescription,
        context: event.request.context,
        maxSubtasks: event.request.maxSubtasks,
        provider: config.provider,
        apiKey: config.apiKey,
      );

      final response = await _aiService.breakdownTask(request);
      _logger
          .i('Task breakdown successful: ${response.subtasks.length} subtasks');
      emit(AITaskBreakdownSuccess(response: response));
    } on AIServiceException catch (e) {
      _logger.e('Task breakdown failed: ${e.message}');
      emit(
        AIFailure(
          message: e.message,
          operationType: AIOperationType.breakdown,
        ),
      );
    } catch (e) {
      _logger.e('Unexpected error during task breakdown: $e');
      emit(
        AIFailure(
          message: 'Unexpected error: $e',
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
      // Inject provider and API key from settings
      final config = _getProviderConfig();
      final request = TaskPrioritizationRequest(
        tasks: event.request.tasks,
        context: event.request.context,
        provider: config.provider,
        apiKey: config.apiKey,
      );

      final response = await _aiService.prioritizeTasks(request);
      _logger.i(
        'Task prioritization successful: ${response.suggestions.length} suggestions',
      );
      emit(AITaskPrioritizationSuccess(response: response));
    } on AIServiceException catch (e) {
      _logger.e('Task prioritization failed: ${e.message}');
      emit(
        AIFailure(
          message: e.message,
          operationType: AIOperationType.prioritization,
        ),
      );
    } catch (e) {
      _logger.e('Unexpected error during task prioritization: $e');
      emit(
        AIFailure(
          message: 'Unexpected error: $e',
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
      // Inject provider and API key from settings
      final config = _getProviderConfig();
      final request = TimeEstimateRequest(
        taskTitle: event.request.taskTitle,
        taskDescription: event.request.taskDescription,
        subtasks: event.request.subtasks,
        skillLevel: event.request.skillLevel,
        provider: config.provider,
        apiKey: config.apiKey,
      );

      final response = await _aiService.estimateTime(request);
      _logger.i(
        'Time estimate successful: ${response.estimate.realisticMinutes} minutes',
      );
      emit(AITimeEstimateSuccess(response: response));
    } on AIServiceException catch (e) {
      _logger.e('Time estimation failed: ${e.message}');
      emit(
        AIFailure(
          message: e.message,
          operationType: AIOperationType.timeEstimate,
        ),
      );
    } catch (e) {
      _logger.e('Unexpected error during time estimation: $e');
      emit(
        AIFailure(
          message: 'Unexpected error: $e',
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
      // Inject provider and API key from settings
      final config = _getProviderConfig();
      final request = ContextSuggestionRequest(
        taskTitle: event.request.taskTitle,
        taskDescription: event.request.taskDescription,
        projectContext: event.request.projectContext,
        suggestionType: event.request.suggestionType,
        provider: config.provider,
        apiKey: config.apiKey,
      );

      final response = await _aiService.getSuggestions(request);
      _logger.i(
        'Context suggestions successful: ${response.suggestions.length} suggestions',
      );
      emit(AIContextSuggestionsSuccess(response: response));
    } on AIServiceException catch (e) {
      _logger.e('Context suggestions failed: ${e.message}');
      emit(
        AIFailure(
          message: e.message,
          operationType: AIOperationType.contextSuggestions,
        ),
      );
    } catch (e) {
      _logger.e('Unexpected error during context suggestions: $e');
      emit(
        AIFailure(
          message: 'Unexpected error: $e',
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

  @override
  Future<void> close() {
    try {
      _aiService.dispose();
    } catch (e) {
      _logger.w('Error disposing AI service', error: e);
    }
    return super.close();
  }
}
