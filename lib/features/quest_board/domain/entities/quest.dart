import 'subquest.dart';

/// Column types for the Quest-Based Agile Board
enum QuestColumn {
  ideaGreenhouse,
  questLog,
  thisCycle,
  nextUp,
  inProgress,
  harvested,
}

/// Quest entity representing a medium-level task or feature
class Quest {
  final String id;
  final String? epicId;
  final String title;
  final String? description;
  final int energyPoints; // 1-5
  final QuestColumn column;
  final DateTime createdAt;
  final DateTime? updatedAt;
  final DateTime? completedAt;
  final List<String> tags;
  final String? assigneeId;
  final List<Subquest> subquests;
  final bool isArchived;

  Quest({
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
  }) : assert(energyPoints >= 1 && energyPoints <= 5,
            'Energy points must be between 1 and 5');

  Quest copyWith({
    String? id,
    String? epicId,
    String? title,
    String? description,
    int? energyPoints,
    QuestColumn? column,
    DateTime? createdAt,
    DateTime? updatedAt,
    DateTime? completedAt,
    List<String>? tags,
    String? assigneeId,
    List<Subquest>? subquests,
    bool? isArchived,
  }) {
    return Quest(
      id: id ?? this.id,
      epicId: epicId ?? this.epicId,
      title: title ?? this.title,
      description: description ?? this.description,
      energyPoints: energyPoints ?? this.energyPoints,
      column: column ?? this.column,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
      completedAt: completedAt ?? this.completedAt,
      tags: tags ?? this.tags,
      assigneeId: assigneeId ?? this.assigneeId,
      subquests: subquests ?? this.subquests,
      isArchived: isArchived ?? this.isArchived,
    );
  }

  /// Get total energy points including subquests
  int get totalEnergyPoints {
    return energyPoints +
        subquests.fold(0, (sum, subquest) => sum + subquest.energyPoints);
  }

  /// Check if quest is completed (all subquests completed)
  bool get isCompleted {
    if (subquests.isEmpty) {
      return column == QuestColumn.harvested;
    }
    return subquests.every((s) => s.isCompleted);
  }
}
