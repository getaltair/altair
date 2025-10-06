/// Login request for OAuth2 password flow.
///
/// The FastAPI backend uses OAuth2PasswordRequestForm which expects
/// 'username' and 'password' fields. We use email for the username field.
class LoginRequest {
  final String username;
  final String password;

  const LoginRequest({
    required this.username,
    required this.password,
  });

  /// Create from email and password
  factory LoginRequest.fromEmailPassword({
    required String email,
    required String password,
  }) {
    return LoginRequest(
      username: email, // OAuth2 form uses 'username' field
      password: password,
    );
  }

  factory LoginRequest.fromJson(Map<String, dynamic> json) {
    return LoginRequest(
      username: json['username'] as String,
      password: json['password'] as String,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'username': username,
      'password': password,
    };
  }

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LoginRequest &&
          runtimeType == other.runtimeType &&
          username == other.username &&
          password == other.password;

  @override
  int get hashCode => username.hashCode ^ password.hashCode;

  @override
  String toString() => 'LoginRequest(username: $username)';
}
