/// User registration request.
///
/// Maps to backend's UserCreate schema from altair/schemas/auth.py
class RegisterRequest {
  final String email;
  final String password;
  final String? username;

  const RegisterRequest({
    required this.email,
    required this.password,
    this.username,
  });

  factory RegisterRequest.fromJson(Map<String, dynamic> json) {
    return RegisterRequest(
      email: json['email'] as String,
      password: json['password'] as String,
      username: json['username'] as String?,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'email': email,
      'password': password,
      if (username != null) 'username': username,
    };
  }

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RegisterRequest &&
          runtimeType == other.runtimeType &&
          email == other.email &&
          password == other.password &&
          username == other.username;

  @override
  int get hashCode => email.hashCode ^ password.hashCode ^ username.hashCode;

  @override
  String toString() => 'RegisterRequest(email: $email, username: $username)';
}
