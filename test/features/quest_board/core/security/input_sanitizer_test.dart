import 'package:flutter_test/flutter_test.dart';
import 'package:altair/core/security/input_sanitizer.dart';

void main() {
  group('InputSanitizer', () {
    test('should sanitize HTML tags from text', () {
      const input = '<script>alert("xss")</script>Hello';
      final sanitized = InputSanitizer.sanitizeText(input);
      expect(sanitized, isNot(contains('<script>')));
      expect(sanitized, isNot(contains('</script>')));
    });

    test('should escape special characters', () {
      const input = 'Test & < > " \' /';
      final sanitized = InputSanitizer.sanitizeText(input);
      expect(sanitized, contains('&amp;'));
      expect(sanitized, contains('&lt;'));
      expect(sanitized, contains('&gt;'));
    });

    test('should limit title to 200 characters', () {
      final longTitle = 'a' * 250;
      final sanitized = InputSanitizer.sanitizeTitle(longTitle);
      expect(sanitized.length, lessThanOrEqualTo(200));
    });

    test('should limit description to 5000 characters', () {
      final longDesc = 'a' * 6000;
      final sanitized = InputSanitizer.sanitizeDescription(longDesc);
      expect(sanitized.length, lessThanOrEqualTo(5000));
    });

    test('should sanitize tags to alphanumeric, hyphens, underscores only', () {
      const input = 'tag-with<script>bad</script>_chars123';
      final sanitized = InputSanitizer.sanitizeTag(input);
      expect(sanitized, equals('tag-with_bad_chars123'));
    });

    test('should clamp energy points to 1-5 range', () {
      expect(InputSanitizer.sanitizeEnergyPoints(0), equals(1));
      expect(InputSanitizer.sanitizeEnergyPoints(3), equals(3));
      expect(InputSanitizer.sanitizeEnergyPoints(6), equals(5));
    });

    test('should validate UUID format for quest IDs', () {
      const validId = '123e4567-e89b-12d3-a456-426614174000';
      const invalidId = 'not-a-uuid';
      
      expect(InputSanitizer.isValidQuestId(validId), isTrue);
      expect(InputSanitizer.isValidQuestId(invalidId), isFalse);
    });
  });
}

