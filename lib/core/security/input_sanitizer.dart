/// Input sanitization utilities to prevent XSS and injection attacks
class InputSanitizer {
  /// Sanitize text input by removing HTML tags and escaping special characters
  static String sanitizeText(String input) {
    if (input.isEmpty) return input;
    
    return input
        .replaceAll(RegExp(r'<[^>]*>'), '') // Remove HTML tags
        .replaceAll('&', '&amp;')
        .replaceAll('<', '&lt;')
        .replaceAll('>', '&gt;')
        .replaceAll('"', '&quot;')
        .replaceAll("'", '&#x27;')
        .replaceAll('/', '&#x2F;')
        .trim();
  }

  /// Sanitize quest title (max 200 characters)
  static String sanitizeTitle(String input) {
    final sanitized = sanitizeText(input);
    if (sanitized.length > 200) {
      return sanitized.substring(0, 200);
    }
    return sanitized;
  }

  /// Sanitize quest description (max 5000 characters)
  static String sanitizeDescription(String? input) {
    if (input == null || input.isEmpty) return '';
    final sanitized = sanitizeText(input);
    if (sanitized.length > 5000) {
      return sanitized.substring(0, 5000);
    }
    return sanitized;
  }

  /// Sanitize tag (alphanumeric, hyphens, underscores only, max 50 chars)
  static String sanitizeTag(String input) {
    final sanitized = input
        .replaceAll(RegExp(r'[^a-zA-Z0-9_-]'), '')
        .trim();
    if (sanitized.length > 50) {
      return sanitized.substring(0, 50);
    }
    return sanitized;
  }

  /// Validate and sanitize energy points (1-5)
  static int sanitizeEnergyPoints(int input) {
    if (input < 1) return 1;
    if (input > 5) return 5;
    return input;
  }

  /// Validate quest ID format (UUID)
  static bool isValidQuestId(String id) {
    final uuidRegex = RegExp(
      r'^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$',
      caseSensitive: false,
    );
    return uuidRegex.hasMatch(id);
  }
}

