import 'package:json_annotation/json_annotation.dart';

part 'tag.g.dart';

/// Represents a tag that can be applied to tasks, projects, etc.
@JsonSerializable()
class Tag {
  /// Unique identifier for the tag
  final String id;

  /// Tag name (unique)
  final String name;

  /// Optional description
  final String? description;

  /// Tag color (hex code)
  final String? color;

  /// When the tag was created
  final DateTime createdAt;

  /// Number of times this tag is used
  final int usageCount;

  const Tag({
    required this.id,
    required this.name,
    this.description,
    this.color,
    required this.createdAt,
    this.usageCount = 0,
  });

  /// Create a copy of this tag with the given fields replaced
  Tag copyWith({
    String? id,
    String? name,
    String? description,
    String? color,
    DateTime? createdAt,
    int? usageCount,
  }) {
    return Tag(
      id: id ?? this.id,
      name: name ?? this.name,
      description: description ?? this.description,
      color: color ?? this.color,
      createdAt: createdAt ?? this.createdAt,
      usageCount: usageCount ?? this.usageCount,
    );
  }

  /// Create a Tag from JSON
  factory Tag.fromJson(Map<String, dynamic> json) => _$TagFromJson(json);

  /// Convert this Tag to JSON
  Map<String, dynamic> toJson() => _$TagToJson(this);

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;

    return other is Tag && other.id == id;
  }

  @override
  int get hashCode => id.hashCode;

  @override
  String toString() => 'Tag(id: $id, name: $name)';
}
