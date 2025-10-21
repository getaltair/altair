/// AI service client for communicating with the AI backend.
library;

import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:http/http.dart' as http;
import 'package:logger/logger.dart';

import 'ai_config.dart';
import 'models.dart';

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

/// Client for AI service API.
class AIService {
  /// Creates an AI service client.
  AIService({
    required AIConfig config,
    http.Client? httpClient,
  })  : _config = config,
        _client = httpClient ?? http.Client() {
    _logger = Logger();
    _config.validate();
    _logger.i('AI Service initialized: ${_config.baseUrl}');
  }

  /// Configuration for the AI service.
  final AIConfig _config;

  /// HTTP client.
  final http.Client _client;

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
      final response = await _client
          .post(
            Uri.parse('${_config.baseUrl}/ai/breakdown'),
            headers: _config.headers,
            body: jsonEncode(request.toJson()),
          )
          .timeout(_config.breakdownTimeout);

      if (response.statusCode == 200) {
        final json = jsonDecode(response.body) as Map<String, dynamic>;
        final result = TaskBreakdownResponse.fromJson(json);
        _logger.i('Generated ${result.subtasks.length} subtasks');
        return result;
      } else {
        final error = _extractErrorMessage(response);
        _logger.e('Task breakdown failed: $error');
        throw AIServiceException(
          error,
          statusCode: response.statusCode,
        );
      }
    } on TimeoutException {
      _logger.w('Task breakdown timed out');
      throw AIServiceException.timeout('task breakdown');
    } on SocketException catch (e) {
      _logger.e('Network error: $e');
      throw AIServiceException.network(e.message);
    } on http.ClientException catch (e) {
      _logger.e('HTTP client error: $e');
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
      final response = await _client
          .post(
            Uri.parse('${_config.baseUrl}/ai/prioritize'),
            headers: _config.headers,
            body: jsonEncode(request.toJson()),
          )
          .timeout(_config.prioritizationTimeout);

      if (response.statusCode == 200) {
        final json = jsonDecode(response.body) as Map<String, dynamic>;
        final result = TaskPrioritizationResponse.fromJson(json);
        _logger.i('Generated ${result.suggestions.length} priority suggestions');
        return result;
      } else {
        final error = _extractErrorMessage(response);
        _logger.e('Task prioritization failed: $error');
        throw AIServiceException(
          error,
          statusCode: response.statusCode,
        );
      }
    } on TimeoutException {
      _logger.w('Task prioritization timed out');
      throw AIServiceException.timeout('task prioritization');
    } on SocketException catch (e) {
      _logger.e('Network error: $e');
      throw AIServiceException.network(e.message);
    } on http.ClientException catch (e) {
      _logger.e('HTTP client error: $e');
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
      final response = await _client
          .post(
            Uri.parse('${_config.baseUrl}/ai/estimate'),
            headers: _config.headers,
            body: jsonEncode(request.toJson()),
          )
          .timeout(_config.estimateTimeout);

      if (response.statusCode == 200) {
        final json = jsonDecode(response.body) as Map<String, dynamic>;
        final result = TimeEstimateResponse.fromJson(json);
        _logger.i(
          'Generated time estimate: ${result.estimate.realisticMinutes} minutes',
        );
        return result;
      } else {
        final error = _extractErrorMessage(response);
        _logger.e('Time estimation failed: $error');
        throw AIServiceException(
          error,
          statusCode: response.statusCode,
        );
      }
    } on TimeoutException {
      _logger.w('Time estimation timed out');
      throw AIServiceException.timeout('time estimation');
    } on SocketException catch (e) {
      _logger.e('Network error: $e');
      throw AIServiceException.network(e.message);
    } on http.ClientException catch (e) {
      _logger.e('HTTP client error: $e');
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
      final response = await _client
          .post(
            Uri.parse('${_config.baseUrl}/ai/suggest'),
            headers: _config.headers,
            body: jsonEncode(request.toJson()),
          )
          .timeout(_config.suggestionsTimeout);

      if (response.statusCode == 200) {
        final json = jsonDecode(response.body) as Map<String, dynamic>;
        final result = ContextSuggestionResponse.fromJson(json);
        _logger.i('Generated ${result.suggestions.length} suggestions');
        return result;
      } else {
        final error = _extractErrorMessage(response);
        _logger.e('Context suggestions failed: $error');
        throw AIServiceException(
          error,
          statusCode: response.statusCode,
        );
      }
    } on TimeoutException {
      _logger.w('Context suggestions timed out');
      throw AIServiceException.timeout('context suggestions');
    } on SocketException catch (e) {
      _logger.e('Network error: $e');
      throw AIServiceException.network(e.message);
    } on http.ClientException catch (e) {
      _logger.e('HTTP client error: $e');
      throw AIServiceException.network(e.message);
    } catch (e) {
      if (e is AIServiceException) rethrow;
      _logger.e('Unexpected error during context suggestions', error: e);
      throw AIServiceException('Unexpected error: $e');
    }
  }

  /// Checks if the AI service is healthy.
  Future<bool> checkHealth() async {
    try {
      final response = await _client
          .get(Uri.parse('${_config.baseUrl}/health'))
          .timeout(_config.healthCheckTimeout);

      return response.statusCode == 200;
    } catch (e) {
      _logger.w('Health check failed', error: e);
      return false;
    }
  }

  /// Extracts error message from HTTP response.
  String _extractErrorMessage(http.Response response) {
    try {
      final json = jsonDecode(response.body) as Map<String, dynamic>;
      return json['detail'] as String? ?? 'Unknown error';
    } catch (_) {
      return 'HTTP ${response.statusCode}: ${response.reasonPhrase}';
    }
  }

  /// Closes the HTTP client.
  void dispose() {
    _client.close();
  }
}
