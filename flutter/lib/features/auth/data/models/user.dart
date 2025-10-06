/// User model matching backend's UserResponse schema.
///
/// Maps to altair/schemas/auth.py UserResponse
class User {
  final String id;
  final String email;
  final String? username;
  final DateTime createdAt;

  const User({
    required this.id,
    required this.email,
    this.username,
    required this.createdAt,
  });

  factory User.fromJson(Map<String, dynamic> json) {
    return User(
      id: json['id'] as String,
      email: json['email'] as String,
      username: json['username'] as String?,
      createdAt: DateTime.parse(json['created_at'] as String),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'email': email,
      if (username != null) 'username': username,
      'created_at': createdAt.toIso8601String(),
    };
  }

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is User &&
          runtimeType == other.runtimeType &&
          id == other.id &&
          email == other.email &&
          username == other.username &&
          createdAt == other.createdAt;

  @override
  int get hashCode =>
      id.hashCode ^ email.hashCode ^ username.hashCode ^ createdAt.hashCode;

  @override
  String toString() =>
      'User(id: $id, email: $email, username: $username, createdAt: $createdAt)';
}
