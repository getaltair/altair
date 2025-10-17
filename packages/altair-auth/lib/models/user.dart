/// User model.
library;

import 'package:json_annotation/json_annotation.dart';

part 'user.g.dart';

/// Represents an authenticated user.
@JsonSerializable()
class User {
  /// Creates a user.
  const User({
    required this.id,
    required this.email,
    this.name,
    this.createdAt,
    this.updatedAt,
  });

  /// Unique user identifier.
  final String id;

  /// User's email address.
  final String email;

  /// User's display name (optional).
  final String? name;

  /// Account creation timestamp.
  @JsonKey(name: 'created_at')
  final DateTime? createdAt;

  /// Last update timestamp.
  @JsonKey(name: 'updated_at')
  final DateTime? updatedAt;

  /// Creates a User from JSON.
  factory User.fromJson(Map<String, dynamic> json) => _$UserFromJson(json);

  /// Converts this User to JSON.
  Map<String, dynamic> toJson() => _$UserToJson(this);

  /// Creates a copy with the specified fields replaced.
  User copyWith({
    String? id,
    String? email,
    String? name,
    DateTime? createdAt,
    DateTime? updatedAt,
  }) {
    return User(
      id: id ?? this.id,
      email: email ?? this.email,
      name: name ?? this.name,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
    );
  }
}
