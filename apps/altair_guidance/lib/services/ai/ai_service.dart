/// AI service client for communicating with AI providers.
library;

import 'dart:async';
import 'dart:io';

import 'package:logger/logger.dart';

import 'models.dart';
import 'providers/ai_provider.dart';

/// Exception thrown when AI service encounters an error.
class AIServiceException implements Exception {
  /// Creates an AI service exception.
  const AIServiceException(
    this.message, {
    this.statusCode,
    this.isTimeout = false,
    this.isNetworkError = false,
  });

  /// Error message.
  final String message;

  /// HTTP status code if available.
  final int? statusCode;

  /// Whether this was a timeout error.
  final bool isTimeout;

  /// Whether this was a network connectivity error.
  final bool isNetworkError;

  /// Creates a timeout exception.
  factory AIServiceException.timeout(String operation) {
    return AIServiceException(
      'The AI service is taking longer than expected for $operation. '
      'Please try again.',
      isTimeout: true,
    );
  }

  /// Creates a network error exception.
  factory AIServiceException.network(String details) {
    return AIServiceException(
      'Network error: $details. Please check your connection.',
      isNetworkError: true,
    );
  }

  @override
  String toString() => 'AIServiceException: $message'
      '${statusCode != null ? ' (HTTP $statusCode)' : ''}'
      '${isTimeout ? ' [TIMEOUT]' : ''}'
      '${isNetworkError ? ' [NETWORK]' : ''}';
}

/// Client for AI service using provider pattern.
class AIService {
  /// Creates an AI service client.
  AIService({
    required AIProvider provider,
  }) : _provider = provider {
    _logger = Logger();
    _logger.i('AI Service initialized with ${provider.runtimeType}');
  }

  /// The AI provider to use.
  final AIProvider _provider;

  /// Logger instance.
  late final Logger _logger;

  /// Breaks down a task into subtasks.
  ///
  /// Throws [AIServiceException] if the request fails.
  Future<TaskBreakdownResponse> breakdownTask(
    TaskBreakdownRequest request,
  ) async {
    _logger.i('Breaking down task: ${request.taskTitle}');

    try {
      return await _provider.breakdownTask(request);
    } on TimeoutException {
      _logger.w('Task breakdown timed out');
      throw AIServiceException.timeout('task breakdown');
    } on SocketException catch (e) {
      _logger.e('Network error: $e');
      throw AIServiceException.network(e.message);
    } catch (e) {
      if (e is AIServiceException) rethrow;
      _logger.e('Unexpected error during task breakdown', error: e);
      throw AIServiceException('Unexpected error: $e');
    }
  }

  /// Gets prioritization suggestions for tasks.
  ///
  /// Throws [AIServiceException] if the request fails.
  Future<TaskPrioritizationResponse> prioritizeTasks(
    TaskPrioritizationRequest request,
  ) async {
    _logger.i('Prioritizing ${request.tasks.length} tasks');

    try {
      return await _provider.prioritizeTasks(request);
    } on TimeoutException {
      _logger.w('Task prioritization timed out');
      throw AIServiceException.timeout('task prioritization');
    } on SocketException catch (e) {
      _logger.e('Network error: $e');
      throw AIServiceException.network(e.message);
    } catch (e) {
      if (e is AIServiceException) rethrow;
      _logger.e('Unexpected error during prioritization', error: e);
      throw AIServiceException('Unexpected error: $e');
    }
  }

  /// Estimates time for a task.
  ///
  /// Throws [AIServiceException] if the request fails.
  Future<TimeEstimateResponse> estimateTime(
    TimeEstimateRequest request,
  ) async {
    _logger.i('Estimating time for task: ${request.taskTitle}');

    try {
      return await _provider.estimateTime(request);
    } on TimeoutException {
      _logger.w('Time estimation timed out');
      throw AIServiceException.timeout('time estimation');
    } on SocketException catch (e) {
      _logger.e('Network error: $e');
      throw AIServiceException.network(e.message);
    } catch (e) {
      if (e is AIServiceException) rethrow;
      _logger.e('Unexpected error during time estimation', error: e);
      throw AIServiceException('Unexpected error: $e');
    }
  }

  /// Gets contextual suggestions for a task.
  ///
  /// Throws [AIServiceException] if the request fails.
  Future<ContextSuggestionResponse> getSuggestions(
    ContextSuggestionRequest request,
  ) async {
    _logger.i('Getting suggestions for task: ${request.taskTitle}');

    try {
      return await _provider.getSuggestions(request);
    } on TimeoutException {
      _logger.w('Context suggestions timed out');
      throw AIServiceException.timeout('context suggestions');
    } on SocketException catch (e) {
      _logger.e('Network error: $e');
      throw AIServiceException.network(e.message);
    } catch (e) {
      if (e is AIServiceException) rethrow;
      _logger.e('Unexpected error during context suggestions', error: e);
      throw AIServiceException('Unexpected error: $e');
    }
  }

  /// Disposes of AI service resources.
  void dispose() {
    _provider.dispose();
  }
}
