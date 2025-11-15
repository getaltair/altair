import '../../domain/entities/quest.dart';
import '../../domain/entities/subquest.dart';

/// Data model for Quest (for serialization/deserialization)
class QuestModel {
  final String id;
  final String? epicId;
  final String title;
  final String? description;
  final int energyPoints;
  final String column; // Serialized as string
  final DateTime createdAt;
  final DateTime? updatedAt;
  final DateTime? completedAt;
  final List<String> tags;
  final String? assigneeId;
  final List<SubquestModel> subquests;
  final bool isArchived;

  QuestModel({
    required this.id,
    this.epicId,
    required this.title,
    this.description,
    required this.energyPoints,
    required this.column,
    required this.createdAt,
    this.updatedAt,
    this.completedAt,
    this.tags = const [],
    this.assigneeId,
    this.subquests = const [],
    this.isArchived = false,
  });

  /// Convert from domain entity
  factory QuestModel.fromEntity(Quest quest) {
    return QuestModel(
      id: quest.id,
      epicId: quest.epicId,
      title: quest.title,
      description: quest.description,
      energyPoints: quest.energyPoints,
      column: quest.column.name,
      createdAt: quest.createdAt,
      updatedAt: quest.updatedAt,
      completedAt: quest.completedAt,
      tags: quest.tags,
      assigneeId: quest.assigneeId,
      subquests:
          quest.subquests.map((s) => SubquestModel.fromEntity(s)).toList(),
      isArchived: quest.isArchived,
    );
  }

  /// Convert to domain entity
  Quest toEntity() {
    return Quest(
      id: id,
      epicId: epicId,
      title: title,
      description: description,
      energyPoints: energyPoints,
      column: QuestColumn.values.firstWhere(
        (c) => c.name == column,
        orElse: () => QuestColumn.ideaGreenhouse,
      ),
      createdAt: createdAt,
      updatedAt: updatedAt,
      completedAt: completedAt,
      tags: tags,
      assigneeId: assigneeId,
      subquests: subquests.map((s) => s.toEntity()).toList(),
      isArchived: isArchived,
    );
  }

  /// Convert from JSON
  factory QuestModel.fromJson(Map<String, dynamic> json) {
    return QuestModel(
      id: json['id'] as String,
      epicId: json['epicId'] as String?,
      title: json['title'] as String,
      description: json['description'] as String?,
      energyPoints: json['energyPoints'] as int,
      column: json['column'] as String,
      createdAt: DateTime.parse(json['createdAt'] as String),
      updatedAt: json['updatedAt'] != null
          ? DateTime.parse(json['updatedAt'] as String)
          : null,
      completedAt: json['completedAt'] != null
          ? DateTime.parse(json['completedAt'] as String)
          : null,
      tags: (json['tags'] as List<dynamic>?)?.cast<String>() ?? [],
      assigneeId: json['assigneeId'] as String?,
      subquests: (json['subquests'] as List<dynamic>?)
              ?.map((s) => SubquestModel.fromJson(s as Map<String, dynamic>))
              .toList() ??
          [],
      isArchived: json['isArchived'] as bool? ?? false,
    );
  }

  /// Convert to JSON
  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'epicId': epicId,
      'title': title,
      'description': description,
      'energyPoints': energyPoints,
      'column': column,
      'createdAt': createdAt.toIso8601String(),
      'updatedAt': updatedAt?.toIso8601String(),
      'completedAt': completedAt?.toIso8601String(),
      'tags': tags,
      'assigneeId': assigneeId,
      'subquests': subquests.map((s) => s.toJson()).toList(),
      'isArchived': isArchived,
    };
  }
}

/// Data model for Subquest
class SubquestModel {
  final String id;
  final String questId;
  final String title;
  final String? description;
  final int energyPoints;
  final DateTime createdAt;
  final DateTime? updatedAt;
  final DateTime? completedAt;
  final List<String> tags;
  final String? assigneeId;
  final bool isCompleted;

  SubquestModel({
    required this.id,
    required this.questId,
    required this.title,
    this.description,
    required this.energyPoints,
    required this.createdAt,
    this.updatedAt,
    this.completedAt,
    this.tags = const [],
    this.assigneeId,
    this.isCompleted = false,
  });

  factory SubquestModel.fromEntity(Subquest subquest) {
    return SubquestModel(
      id: subquest.id,
      questId: subquest.questId,
      title: subquest.title,
      description: subquest.description,
      energyPoints: subquest.energyPoints,
      createdAt: subquest.createdAt,
      updatedAt: subquest.updatedAt,
      completedAt: subquest.completedAt,
      tags: subquest.tags,
      assigneeId: subquest.assigneeId,
      isCompleted: subquest.isCompleted,
    );
  }

  Subquest toEntity() {
    return Subquest(
      id: id,
      questId: questId,
      title: title,
      description: description,
      energyPoints: energyPoints,
      createdAt: createdAt,
      updatedAt: updatedAt,
      completedAt: completedAt,
      tags: tags,
      assigneeId: assigneeId,
      isCompleted: isCompleted,
    );
  }

  factory SubquestModel.fromJson(Map<String, dynamic> json) {
    return SubquestModel(
      id: json['id'] as String,
      questId: json['questId'] as String,
      title: json['title'] as String,
      description: json['description'] as String?,
      energyPoints: json['energyPoints'] as int,
      createdAt: DateTime.parse(json['createdAt'] as String),
      updatedAt: json['updatedAt'] != null
          ? DateTime.parse(json['updatedAt'] as String)
          : null,
      completedAt: json['completedAt'] != null
          ? DateTime.parse(json['completedAt'] as String)
          : null,
      tags: (json['tags'] as List<dynamic>?)?.cast<String>() ?? [],
      assigneeId: json['assigneeId'] as String?,
      isCompleted: json['isCompleted'] as bool? ?? false,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'questId': questId,
      'title': title,
      'description': description,
      'energyPoints': energyPoints,
      'createdAt': createdAt.toIso8601String(),
      'updatedAt': updatedAt?.toIso8601String(),
      'completedAt': completedAt?.toIso8601String(),
      'tags': tags,
      'assigneeId': assigneeId,
      'isCompleted': isCompleted,
    };
  }
}
