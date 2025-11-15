/// Epic entity representing a high-level goal or project
class Epic {
  final String id;
  final String title;
  final String? description;
  final DateTime createdAt;
  final DateTime? updatedAt;
  final List<String> tags;
  final String? assigneeId;

  Epic({
    required this.id,
    required this.title,
    this.description,
    required this.createdAt,
    this.updatedAt,
    this.tags = const [],
    this.assigneeId,
  });

  Epic copyWith({
    String? id,
    String? title,
    String? description,
    DateTime? createdAt,
    DateTime? updatedAt,
    List<String>? tags,
    String? assigneeId,
  }) {
    return Epic(
      id: id ?? this.id,
      title: title ?? this.title,
      description: description ?? this.description,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
      tags: tags ?? this.tags,
      assigneeId: assigneeId ?? this.assigneeId,
    );
  }
}
