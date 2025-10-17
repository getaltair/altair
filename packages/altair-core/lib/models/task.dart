import 'package:json_annotation/json_annotation.dart';

part 'task.g.dart';

/// Status of a task
enum TaskStatus {
  /// Task is yet to be started
  todo,

  /// Task is currently being worked on
  inProgress,

  /// Task is completed
  completed,

  /// Task is cancelled
  cancelled,
}

/// Represents a task in the Altair system
@JsonSerializable()
class Task {
  /// Unique identifier for the task
  final String id;

  /// Title of the task
  final String title;

  /// Optional detailed description
  final String? description;

  /// Current status of the task
  final TaskStatus status;

  /// Tags associated with this task
  final List<String> tags;

  /// ID of the parent project (if any)
  final String? projectId;

  /// ID of the parent task (for subtasks)
  final String? parentTaskId;

  /// When the task was created
  final DateTime createdAt;

  /// When the task was last updated
  final DateTime updatedAt;

  /// When the task was completed (if applicable)
  final DateTime? completedAt;

  /// Estimated time to complete in minutes
  final int? estimatedMinutes;

  /// Actual time spent in minutes
  final int? actualMinutes;

  /// Priority level (1-5, with 1 being highest)
  final int priority;

  /// Task metadata (flexible JSON field)
  final Map<String, dynamic>? metadata;

  const Task({
    required this.id,
    required this.title,
    this.description,
    this.status = TaskStatus.todo,
    this.tags = const [],
    this.projectId,
    this.parentTaskId,
    required this.createdAt,
    required this.updatedAt,
    this.completedAt,
    this.estimatedMinutes,
    this.actualMinutes,
    this.priority = 3,
    this.metadata,
  });

  /// Create a copy of this task with the given fields replaced
  Task copyWith({
    String? id,
    String? title,
    String? description,
    TaskStatus? status,
    List<String>? tags,
    String? projectId,
    String? parentTaskId,
    DateTime? createdAt,
    DateTime? updatedAt,
    DateTime? completedAt,
    int? estimatedMinutes,
    int? actualMinutes,
    int? priority,
    Map<String, dynamic>? metadata,
  }) {
    return Task(
      id: id ?? this.id,
      title: title ?? this.title,
      description: description ?? this.description,
      status: status ?? this.status,
      tags: tags ?? this.tags,
      projectId: projectId ?? this.projectId,
      parentTaskId: parentTaskId ?? this.parentTaskId,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
      completedAt: completedAt ?? this.completedAt,
      estimatedMinutes: estimatedMinutes ?? this.estimatedMinutes,
      actualMinutes: actualMinutes ?? this.actualMinutes,
      priority: priority ?? this.priority,
      metadata: metadata ?? this.metadata,
    );
  }

  /// Create a Task from JSON
  factory Task.fromJson(Map<String, dynamic> json) => _$TaskFromJson(json);

  /// Convert this Task to JSON
  Map<String, dynamic> toJson() => _$TaskToJson(this);

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;

    return other is Task && other.id == id;
  }

  @override
  int get hashCode => id.hashCode;

  @override
  String toString() => 'Task(id: $id, title: $title, status: $status)';
}
