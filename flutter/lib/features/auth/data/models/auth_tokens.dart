/// JWT token pair returned from authentication endpoints.
///
/// Contains both access token (short-lived, used for API calls) and
/// refresh token (long-lived, used to obtain new access tokens).
///
/// Maps to backend's `Token` schema from altair/schemas/auth.py
class AuthTokens {
  final String accessToken;
  final String refreshToken;
  final String tokenType;
  final int expiresIn;

  /// Timestamp when these tokens were issued (for expiry calculation)
  final DateTime issuedAt;

  const AuthTokens({
    required this.accessToken,
    required this.refreshToken,
    required this.tokenType,
    required this.expiresIn,
    required this.issuedAt,
  });

  factory AuthTokens.fromJson(Map<String, dynamic> json) {
    return AuthTokens(
      accessToken: json['access_token'] as String,
      refreshToken: json['refresh_token'] as String,
      tokenType: json['token_type'] as String,
      expiresIn: json['expires_in'] as int,
      issuedAt: json['issued_at'] != null
          ? DateTime.parse(json['issued_at'] as String)
          : DateTime.now(), // Default to now if not provided by backend
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'access_token': accessToken,
      'refresh_token': refreshToken,
      'token_type': tokenType,
      'expires_in': expiresIn,
      'issued_at': issuedAt.toIso8601String(),
    };
  }

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is AuthTokens &&
          runtimeType == other.runtimeType &&
          accessToken == other.accessToken &&
          refreshToken == other.refreshToken &&
          tokenType == other.tokenType &&
          expiresIn == other.expiresIn &&
          issuedAt == other.issuedAt;

  @override
  int get hashCode =>
      accessToken.hashCode ^
      refreshToken.hashCode ^
      tokenType.hashCode ^
      expiresIn.hashCode ^
      issuedAt.hashCode;

  @override
  String toString() {
    return 'AuthTokens(accessToken: ${accessToken.substring(0, 20)}..., refreshToken: ${refreshToken.substring(0, 20)}..., tokenType: $tokenType, expiresIn: $expiresIn, issuedAt: $issuedAt)';
  }

  /// Check if the access token has expired
  ///
  /// Returns true if the current time is past the expiration time.
  bool isExpired() {
    final expirationTime = issuedAt.add(Duration(seconds: expiresIn));
    return DateTime.now().isAfter(expirationTime);
  }

  /// Check if the access token is near expiry
  ///
  /// Returns true if the token will expire within the given buffer time.
  /// Default buffer is 5 minutes, which allows time for token refresh
  /// before actual expiry.
  bool isNearExpiry({Duration buffer = const Duration(minutes: 5)}) {
    final expirationTime = issuedAt.add(Duration(seconds: expiresIn));
    final refreshThreshold = expirationTime.subtract(buffer);
    return DateTime.now().isAfter(refreshThreshold);
  }

  /// Get the time remaining until token expiry
  ///
  /// Returns a Duration representing how much time is left.
  /// Returns Duration.zero if already expired.
  Duration get timeUntilExpiry {
    final expirationTime = issuedAt.add(Duration(seconds: expiresIn));
    final remaining = expirationTime.difference(DateTime.now());
    return remaining.isNegative ? Duration.zero : remaining;
  }
}
