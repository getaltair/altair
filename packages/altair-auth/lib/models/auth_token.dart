/// Authentication token model.
library;

import 'package:json_annotation/json_annotation.dart';

part 'auth_token.g.dart';

/// Represents an authentication token response from the server.
@JsonSerializable()
class AuthToken {
  /// Creates an authentication token.
  const AuthToken({
    required this.accessToken,
    required this.tokenType,
    this.expiresIn,
    this.refreshToken,
  });

  /// The JWT access token.
  @JsonKey(name: 'access_token')
  final String accessToken;

  /// The token type (usually 'bearer').
  @JsonKey(name: 'token_type')
  final String tokenType;

  /// Token expiration time in seconds.
  @JsonKey(name: 'expires_in')
  final int? expiresIn;

  /// Optional refresh token for token renewal.
  @JsonKey(name: 'refresh_token')
  final String? refreshToken;

  /// Creates an AuthToken from JSON.
  factory AuthToken.fromJson(Map<String, dynamic> json) =>
      _$AuthTokenFromJson(json);

  /// Converts this AuthToken to JSON.
  Map<String, dynamic> toJson() => _$AuthTokenToJson(this);

  /// Creates a copy with the specified fields replaced.
  AuthToken copyWith({
    String? accessToken,
    String? tokenType,
    int? expiresIn,
    String? refreshToken,
  }) {
    return AuthToken(
      accessToken: accessToken ?? this.accessToken,
      tokenType: tokenType ?? this.tokenType,
      expiresIn: expiresIn ?? this.expiresIn,
      refreshToken: refreshToken ?? this.refreshToken,
    );
  }
}
