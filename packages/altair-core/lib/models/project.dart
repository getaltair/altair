import 'package:json_annotation/json_annotation.dart';

part 'project.g.dart';

/// Status of a project
enum ProjectStatus {
  /// Project is actively being worked on
  active,

  /// Project is on hold
  onHold,

  /// Project is completed
  completed,

  /// Project is cancelled
  cancelled,
}

/// Represents a project that groups related tasks
@JsonSerializable()
class Project {
  /// Unique identifier for the project
  final String id;

  /// Name of the project
  final String name;

  /// Optional description
  final String? description;

  /// Current status
  final ProjectStatus status;

  /// Tags associated with this project
  final List<String> tags;

  /// Project color (hex code)
  final String? color;

  /// When the project was created
  final DateTime createdAt;

  /// When the project was last updated
  final DateTime updatedAt;

  /// Target completion date
  final DateTime? targetDate;

  /// When the project was completed
  final DateTime? completedAt;

  /// Project metadata (flexible JSON field)
  final Map<String, dynamic>? metadata;

  const Project({
    required this.id,
    required this.name,
    this.description,
    this.status = ProjectStatus.active,
    this.tags = const [],
    this.color,
    required this.createdAt,
    required this.updatedAt,
    this.targetDate,
    this.completedAt,
    this.metadata,
  });

  /// Create a copy of this project with the given fields replaced
  Project copyWith({
    String? id,
    String? name,
    String? description,
    ProjectStatus? status,
    List<String>? tags,
    String? color,
    DateTime? createdAt,
    DateTime? updatedAt,
    DateTime? targetDate,
    DateTime? completedAt,
    Map<String, dynamic>? metadata,
  }) {
    return Project(
      id: id ?? this.id,
      name: name ?? this.name,
      description: description ?? this.description,
      status: status ?? this.status,
      tags: tags ?? this.tags,
      color: color ?? this.color,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
      targetDate: targetDate ?? this.targetDate,
      completedAt: completedAt ?? this.completedAt,
      metadata: metadata ?? this.metadata,
    );
  }

  /// Create a Project from JSON
  factory Project.fromJson(Map<String, dynamic> json) =>
      _$ProjectFromJson(json);

  /// Convert this Project to JSON
  Map<String, dynamic> toJson() => _$ProjectToJson(this);

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;

    return other is Project && other.id == id;
  }

  @override
  int get hashCode => id.hashCode;

  @override
  String toString() => 'Project(id: $id, name: $name, status: $status)';
}
