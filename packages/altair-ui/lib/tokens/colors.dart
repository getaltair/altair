/// Color tokens for the neo-brutalist design system.
library;

import 'package:flutter/material.dart';

/// Color constants for Altair UI.
class AltairColors {
  const AltairColors._();

  // Light theme colors
  /// Primary background color for light theme: #fafafa
  static const Color lightBgPrimary = Color(0xFFFAFAFA);

  /// Secondary background color for light theme: #ffffff
  static const Color lightBgSecondary = Color(0xFFFFFFFF);

  /// Primary text color for light theme: #000000
  static const Color lightTextPrimary = Color(0xFF000000);

  /// Secondary text color for light theme: #666666
  static const Color lightTextSecondary = Color(0xFF666666);

  /// Hover background for light theme: #f0f0f0
  static const Color lightHoverBg = Color(0xFFF0F0F0);

  // Dark theme colors
  /// Primary background color for dark theme: #1a1a1a
  static const Color darkBgPrimary = Color(0xFF1A1A1A);

  /// Secondary background color for dark theme: #2a2a2a
  static const Color darkBgSecondary = Color(0xFF2A2A2A);

  /// Primary text color for dark theme: #ffffff
  static const Color darkTextPrimary = Color(0xFFFFFFFF);

  /// Secondary text color for dark theme: #999999
  static const Color darkTextSecondary = Color(0xFF999999);

  /// Hover background for dark theme: #333333
  static const Color darkHoverBg = Color(0xFF333333);

  // Border colors
  /// Border color for light theme: #000000
  static const Color lightBorderColor = Color(0xFF000000);

  /// Border color for dark theme: #ffffff
  static const Color darkBorderColor = Color(0xFFFFFFFF);

  // Accent colors (same for both themes)
  /// Accent yellow: #ffd93d
  static const Color accentYellow = Color(0xFFFFD93D);

  /// Accent blue: #60a5fa
  static const Color accentBlue = Color(0xFF60A5FA);

  /// Accent green: #6bcb77
  static const Color accentGreen = Color(0xFF6BCB77);

  /// Accent red: #ff6b6b
  static const Color accentRed = Color(0xFFFF6B6B);

  // Semantic colors
  /// Success color (green)
  static const Color success = accentGreen;

  /// Error color (red)
  static const Color error = accentRed;

  /// Warning color (yellow)
  static const Color warning = accentYellow;

  /// Info color (blue)
  static const Color info = accentBlue;

  // Theme-agnostic colors (defaulting to light theme)
  /// Surface background color
  static const Color surface = lightBgSecondary;

  /// Border color
  static const Color border = lightBorderColor;

  /// Secondary text color
  static const Color textSecondary = lightTextSecondary;

  // Convenient aliases for theme-aware usage
  /// Light theme background (alias for lightBgPrimary)
  static const Color bgLight = lightBgPrimary;

  /// Dark theme background (alias for darkBgPrimary)
  static const Color bgDark = darkBgPrimary;

  /// Light theme text (alias for lightTextPrimary)
  static const Color textLight = darkTextPrimary;

  /// Dark theme text (alias for darkTextPrimary)
  static const Color textDark = lightTextPrimary;

  /// Light theme border (alias for lightBorderColor)
  static const Color borderLight = lightBorderColor;

  /// Dark theme border (alias for darkBorderColor)
  static const Color borderDark = darkBorderColor;
}
