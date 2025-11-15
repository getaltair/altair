/// Subquest entity representing a small, actionable task
class Subquest {
  final String id;
  final String questId;
  final String title;
  final String? description;
  final int energyPoints; // 1-5
  final DateTime createdAt;
  final DateTime? updatedAt;
  final DateTime? completedAt;
  final List<String> tags;
  final String? assigneeId;
  final bool isCompleted;

  Subquest({
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
  }) : assert(energyPoints >= 1 && energyPoints <= 5,
            'Energy points must be between 1 and 5');

  Subquest copyWith({
    String? id,
    String? questId,
    String? title,
    String? description,
    int? energyPoints,
    DateTime? createdAt,
    DateTime? updatedAt,
    DateTime? completedAt,
    List<String>? tags,
    String? assigneeId,
    bool? isCompleted,
  }) {
    return Subquest(
      id: id ?? this.id,
      questId: questId ?? this.questId,
      title: title ?? this.title,
      description: description ?? this.description,
      energyPoints: energyPoints ?? this.energyPoints,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
      completedAt: completedAt ?? this.completedAt,
      tags: tags ?? this.tags,
      assigneeId: assigneeId ?? this.assigneeId,
      isCompleted: isCompleted ?? this.isCompleted,
    );
  }
}
