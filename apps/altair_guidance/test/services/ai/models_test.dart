import 'package:altair_guidance/services/ai/models.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('SkillLevel', () {
    test('toJson() returns enum name', () {
      expect(SkillLevel.beginner.toJson(), 'beginner');
      expect(SkillLevel.intermediate.toJson(), 'intermediate');
      expect(SkillLevel.advanced.toJson(), 'advanced');
    });

    test('fromString() creates correct enum', () {
      expect(SkillLevel.fromString('beginner'), SkillLevel.beginner);
      expect(SkillLevel.fromString('intermediate'), SkillLevel.intermediate);
      expect(SkillLevel.fromString('advanced'), SkillLevel.advanced);
    });

    test('fromString() is case insensitive', () {
      expect(SkillLevel.fromString('BEGINNER'), SkillLevel.beginner);
      expect(SkillLevel.fromString('Intermediate'), SkillLevel.intermediate);
      expect(SkillLevel.fromString('ADVANCED'), SkillLevel.advanced);
    });

    test('fromString() defaults to intermediate for invalid input', () {
      expect(SkillLevel.fromString('invalid'), SkillLevel.intermediate);
      expect(SkillLevel.fromString(''), SkillLevel.intermediate);
    });
  });

  group('TaskBreakdownRequest', () {
    test('creates with valid required parameters', () {
      final request = TaskBreakdownRequest(
        taskTitle: 'Test Task',
      );

      expect(request.taskTitle, 'Test Task');
      expect(request.taskDescription, isNull);
      expect(request.context, isNull);
      expect(request.maxSubtasks, 5);
    });

    test('creates with all parameters', () {
      final request = TaskBreakdownRequest(
        taskTitle: 'Test Task',
        taskDescription: 'Description',
        context: 'Context',
        maxSubtasks: 10,
      );

      expect(request.taskTitle, 'Test Task');
      expect(request.taskDescription, 'Description');
      expect(request.context, 'Context');
      expect(request.maxSubtasks, 10);
    });

    test('throws when task title is empty', () {
      expect(
        () => TaskBreakdownRequest(taskTitle: ''),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'Task title cannot be empty',
          ),
        ),
      );
    });

    test('throws when task title is whitespace only', () {
      expect(
        () => TaskBreakdownRequest(taskTitle: '   '),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'Task title cannot be empty',
          ),
        ),
      );
    });

    test('throws when task title exceeds 500 characters', () {
      expect(
        () => TaskBreakdownRequest(taskTitle: 'a' * 501),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'Task title must be 500 characters or less',
          ),
        ),
      );
    });

    test('allows task title with exactly 500 characters', () {
      expect(
        () => TaskBreakdownRequest(taskTitle: 'a' * 500),
        returnsNormally,
      );
    });

    test('throws when task description exceeds 5000 characters', () {
      expect(
        () => TaskBreakdownRequest(
          taskTitle: 'Test',
          taskDescription: 'a' * 5001,
        ),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'Task description must be 5000 characters or less',
          ),
        ),
      );
    });

    test('throws when context exceeds 2000 characters', () {
      expect(
        () => TaskBreakdownRequest(
          taskTitle: 'Test',
          context: 'a' * 2001,
        ),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'Context must be 2000 characters or less',
          ),
        ),
      );
    });

    test('throws when maxSubtasks is less than 1', () {
      expect(
        () => TaskBreakdownRequest(
          taskTitle: 'Test',
          maxSubtasks: 0,
        ),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'maxSubtasks must be between 1 and 20',
          ),
        ),
      );
    });

    test('throws when maxSubtasks is greater than 20', () {
      expect(
        () => TaskBreakdownRequest(
          taskTitle: 'Test',
          maxSubtasks: 21,
        ),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'maxSubtasks must be between 1 and 20',
          ),
        ),
      );
    });

    test('toJson() includes all fields', () {
      final request = TaskBreakdownRequest(
        taskTitle: 'Test Task',
        taskDescription: 'Description',
        context: 'Context',
        maxSubtasks: 10,
      );

      final json = request.toJson();

      expect(json, {
        'task_title': 'Test Task',
        'task_description': 'Description',
        'context': 'Context',
        'max_subtasks': 10,
      });
    });

    test('toJson() omits null fields', () {
      final request = TaskBreakdownRequest(
        taskTitle: 'Test Task',
      );

      final json = request.toJson();

      expect(json, {
        'task_title': 'Test Task',
        'max_subtasks': 5,
      });
      expect(json.containsKey('task_description'), false);
      expect(json.containsKey('context'), false);
    });
  });

  group('TaskPrioritizationRequest', () {
    test('creates with valid tasks', () {
      final request = TaskPrioritizationRequest(
        tasks: [
          {'title': 'Task 1', 'description': 'Desc 1'},
          {'title': 'Task 2', 'description': 'Desc 2'},
        ],
      );

      expect(request.tasks.length, 2);
      expect(request.context, isNull);
    });

    test('throws when tasks list is empty', () {
      expect(
        () => TaskPrioritizationRequest(tasks: []),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'Tasks list cannot be empty',
          ),
        ),
      );
    });

    test('throws when tasks list exceeds 50 items', () {
      final tasks = List.generate(
        51,
        (i) => {'title': 'Task $i'},
      );

      expect(
        () => TaskPrioritizationRequest(tasks: tasks),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'Cannot prioritize more than 50 tasks at once',
          ),
        ),
      );
    });

    test('throws when context exceeds 2000 characters', () {
      expect(
        () => TaskPrioritizationRequest(
          tasks: [
            {'title': 'Task 1'},
          ],
          context: 'a' * 2001,
        ),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'Context must be 2000 characters or less',
          ),
        ),
      );
    });

    test('throws when task is missing title', () {
      expect(
        () => TaskPrioritizationRequest(
          tasks: [
            {'description': 'No title'},
          ],
        ),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'Each task must have a non-empty title',
          ),
        ),
      );
    });

    test('throws when task has empty title', () {
      expect(
        () => TaskPrioritizationRequest(
          tasks: [
            {'title': '  '},
          ],
        ),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'Each task must have a non-empty title',
          ),
        ),
      );
    });

    test('toJson() includes all fields', () {
      final request = TaskPrioritizationRequest(
        tasks: [
          {'title': 'Task 1'},
        ],
        context: 'Test context',
      );

      final json = request.toJson();

      expect(json['tasks'], [
        {'title': 'Task 1'},
      ]);
      expect(json['context'], 'Test context');
    });
  });

  group('TimeEstimateRequest', () {
    test('creates with valid parameters', () {
      final request = TimeEstimateRequest(
        taskTitle: 'Test Task',
      );

      expect(request.taskTitle, 'Test Task');
      expect(request.skillLevel, SkillLevel.intermediate);
    });

    test('creates with all parameters', () {
      final request = TimeEstimateRequest(
        taskTitle: 'Test Task',
        taskDescription: 'Description',
        subtasks: ['Subtask 1', 'Subtask 2'],
        skillLevel: SkillLevel.advanced,
      );

      expect(request.taskTitle, 'Test Task');
      expect(request.taskDescription, 'Description');
      expect(request.subtasks, ['Subtask 1', 'Subtask 2']);
      expect(request.skillLevel, SkillLevel.advanced);
    });

    test('throws when task title is empty', () {
      expect(
        () => TimeEstimateRequest(taskTitle: ''),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'Task title cannot be empty',
          ),
        ),
      );
    });

    test('throws when task title exceeds 500 characters', () {
      expect(
        () => TimeEstimateRequest(taskTitle: 'a' * 501),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'Task title must be 500 characters or less',
          ),
        ),
      );
    });

    test('throws when task description exceeds 5000 characters', () {
      expect(
        () => TimeEstimateRequest(
          taskTitle: 'Test',
          taskDescription: 'a' * 5001,
        ),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'Task description must be 5000 characters or less',
          ),
        ),
      );
    });

    test('throws when subtasks exceed 20 items', () {
      expect(
        () => TimeEstimateRequest(
          taskTitle: 'Test',
          subtasks: List.generate(21, (i) => 'Subtask $i'),
        ),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'Cannot have more than 20 subtasks',
          ),
        ),
      );
    });

    test('toJson() includes skill level as string', () {
      final request = TimeEstimateRequest(
        taskTitle: 'Test Task',
        skillLevel: SkillLevel.beginner,
      );

      final json = request.toJson();

      expect(json['skill_level'], 'beginner');
    });
  });

  group('ContextSuggestionRequest', () {
    test('creates with default suggestion type', () {
      final request = ContextSuggestionRequest(
        taskTitle: 'Test Task',
      );

      expect(request.taskTitle, 'Test Task');
      expect(request.suggestionType, 'general');
    });

    test('creates with all parameters', () {
      final request = ContextSuggestionRequest(
        taskTitle: 'Test Task',
        taskDescription: 'Description',
        projectContext: 'Project context',
        suggestionType: 'resources',
      );

      expect(request.taskTitle, 'Test Task');
      expect(request.taskDescription, 'Description');
      expect(request.projectContext, 'Project context');
      expect(request.suggestionType, 'resources');
    });

    test('throws when task title is empty', () {
      expect(
        () => ContextSuggestionRequest(taskTitle: ''),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'Task title cannot be empty',
          ),
        ),
      );
    });

    test('throws when task title exceeds 500 characters', () {
      expect(
        () => ContextSuggestionRequest(taskTitle: 'a' * 501),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'Task title must be 500 characters or less',
          ),
        ),
      );
    });

    test('throws when task description exceeds 5000 characters', () {
      expect(
        () => ContextSuggestionRequest(
          taskTitle: 'Test',
          taskDescription: 'a' * 5001,
        ),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'Task description must be 5000 characters or less',
          ),
        ),
      );
    });

    test('throws when project context exceeds 2000 characters', () {
      expect(
        () => ContextSuggestionRequest(
          taskTitle: 'Test',
          projectContext: 'a' * 2001,
        ),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            'Project context must be 2000 characters or less',
          ),
        ),
      );
    });

    test('throws for invalid suggestion type', () {
      expect(
        () => ContextSuggestionRequest(
          taskTitle: 'Test',
          suggestionType: 'invalid',
        ),
        throwsA(
          isA<ArgumentError>().having(
            (e) => e.message,
            'message',
            contains('Invalid suggestion type'),
          ),
        ),
      );
    });

    test('accepts all valid suggestion types', () {
      expect(
        () => ContextSuggestionRequest(
          taskTitle: 'Test',
          suggestionType: 'general',
        ),
        returnsNormally,
      );
      expect(
        () => ContextSuggestionRequest(
          taskTitle: 'Test',
          suggestionType: 'resources',
        ),
        returnsNormally,
      );
      expect(
        () => ContextSuggestionRequest(
          taskTitle: 'Test',
          suggestionType: 'tips',
        ),
        returnsNormally,
      );
      expect(
        () => ContextSuggestionRequest(
          taskTitle: 'Test',
          suggestionType: 'blockers',
        ),
        returnsNormally,
      );
    });

    test('toJson() includes all fields', () {
      final request = ContextSuggestionRequest(
        taskTitle: 'Test Task',
        taskDescription: 'Description',
        projectContext: 'Context',
        suggestionType: 'tips',
      );

      final json = request.toJson();

      expect(json, {
        'task_title': 'Test Task',
        'task_description': 'Description',
        'project_context': 'Context',
        'suggestion_type': 'tips',
      });
    });
  });
}
