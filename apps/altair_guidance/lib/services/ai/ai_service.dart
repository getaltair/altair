/// AI service client for communicating with the AI backend.
library;

import 'dart:convert';

import 'package:http/http.dart' as http;
import 'package:logger/logger.dart';

import 'models.dart';

/// Exception thrown when AI service encounters an error.
class AIServiceException implements Exception {
  /// Creates an AI service exception.
  const AIServiceException(this.message, {this.statusCode});

  /// Error message.
  final String message;

  /// HTTP status code if available.
  final int? statusCode;

  @override
  String toString() => 'AIServiceException: $message'
      '${statusCode != null ? ' (HTTP $statusCode)' : ''}';
}

/// Client for AI service API.
class AIService {
  /// Creates an AI service client.
  AIService({
    required this.baseUrl,
    http.Client? httpClient,
  }) : _client = httpClient ?? http.Client() {
    _logger = Logger();
  }

  /// Base URL for the AI service.
  final String baseUrl;

  /// HTTP client.
  final http.Client _client;

  /// Logger instance.
  late final Logger _logger;

  /// Request timeout duration.
  static const Duration _timeout = Duration(seconds: 30);

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
            Uri.parse('$baseUrl/ai/breakdown'),
            headers: {'Content-Type': 'application/json'},
            body: jsonEncode(request.toJson()),
          )
          .timeout(_timeout);

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
    } on http.ClientException catch (e) {
      _logger.e('HTTP client error: $e');
      throw AIServiceException('Network error: ${e.message}');
    } catch (e) {
      if (e is AIServiceException) rethrow;
      _logger.e('Unexpected error: $e');
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
            Uri.parse('$baseUrl/ai/prioritize'),
            headers: {'Content-Type': 'application/json'},
            body: jsonEncode(request.toJson()),
          )
          .timeout(_timeout);

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
    } on http.ClientException catch (e) {
      _logger.e('HTTP client error: $e');
      throw AIServiceException('Network error: ${e.message}');
    } catch (e) {
      if (e is AIServiceException) rethrow;
      _logger.e('Unexpected error: $e');
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
            Uri.parse('$baseUrl/ai/estimate'),
            headers: {'Content-Type': 'application/json'},
            body: jsonEncode(request.toJson()),
          )
          .timeout(_timeout);

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
    } on http.ClientException catch (e) {
      _logger.e('HTTP client error: $e');
      throw AIServiceException('Network error: ${e.message}');
    } catch (e) {
      if (e is AIServiceException) rethrow;
      _logger.e('Unexpected error: $e');
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
            Uri.parse('$baseUrl/ai/suggest'),
            headers: {'Content-Type': 'application/json'},
            body: jsonEncode(request.toJson()),
          )
          .timeout(_timeout);

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
    } on http.ClientException catch (e) {
      _logger.e('HTTP client error: $e');
      throw AIServiceException('Network error: ${e.message}');
    } catch (e) {
      if (e is AIServiceException) rethrow;
      _logger.e('Unexpected error: $e');
      throw AIServiceException('Unexpected error: $e');
    }
  }

  /// Checks if the AI service is healthy.
  Future<bool> checkHealth() async {
    try {
      final response = await _client
          .get(Uri.parse('$baseUrl/health'))
          .timeout(const Duration(seconds: 5));

      return response.statusCode == 200;
    } catch (e) {
      _logger.w('Health check failed: $e');
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
