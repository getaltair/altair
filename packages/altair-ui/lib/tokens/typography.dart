/// Typography tokens for the neo-brutalist design system.
library;

import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';

/// Typography constants using JetBrains Mono.
class AltairTypography {
  const AltairTypography._();

  /// Base font family: JetBrains Mono
  static TextStyle get baseStyle => GoogleFonts.jetBrainsMono();

  /// Display large: 48px, weight 800
  static TextStyle get displayLarge => baseStyle.copyWith(
        fontSize: 48,
        fontWeight: FontWeight.w800,
        letterSpacing: -2,
      );

  /// Display medium: 36px, weight 700
  static TextStyle get displayMedium => baseStyle.copyWith(
        fontSize: 36,
        fontWeight: FontWeight.w700,
        letterSpacing: -1.5,
      );

  /// Display small: 28px, weight 700
  static TextStyle get displaySmall => baseStyle.copyWith(
        fontSize: 28,
        fontWeight: FontWeight.w700,
        letterSpacing: -1,
      );

  /// Headline large: 24px, weight 600
  static TextStyle get headlineLarge =>
      baseStyle.copyWith(fontSize: 24, fontWeight: FontWeight.w600);

  /// Headline medium: 20px, weight 600
  static TextStyle get headlineMedium =>
      baseStyle.copyWith(fontSize: 20, fontWeight: FontWeight.w600);

  /// Headline small: 18px, weight 600
  static TextStyle get headlineSmall =>
      baseStyle.copyWith(fontSize: 18, fontWeight: FontWeight.w600);

  /// Body large: 16px, weight 500
  static TextStyle get bodyLarge =>
      baseStyle.copyWith(fontSize: 16, fontWeight: FontWeight.w500);

  /// Body medium: 14px, weight 500
  static TextStyle get bodyMedium =>
      baseStyle.copyWith(fontSize: 14, fontWeight: FontWeight.w500);

  /// Body small: 12px, weight 400
  static TextStyle get bodySmall =>
      baseStyle.copyWith(fontSize: 12, fontWeight: FontWeight.w400);

  /// Label large: 14px, weight 700
  static TextStyle get labelLarge =>
      baseStyle.copyWith(fontSize: 14, fontWeight: FontWeight.w700);

  /// Label medium: 12px, weight 700
  static TextStyle get labelMedium =>
      baseStyle.copyWith(fontSize: 12, fontWeight: FontWeight.w700);

  /// Label small: 10px, weight 700
  static TextStyle get labelSmall =>
      baseStyle.copyWith(fontSize: 10, fontWeight: FontWeight.w700);
}
