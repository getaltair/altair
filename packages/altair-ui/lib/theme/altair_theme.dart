/// Altair theme configuration.
library;

import 'package:flutter/material.dart';

import '../tokens/borders.dart';
import '../tokens/colors.dart';
import '../tokens/spacing.dart';
import '../tokens/typography.dart';

/// Altair theme builder for both light and dark modes.
class AltairTheme {
  const AltairTheme._();

  /// Light theme configuration.
  static ThemeData get lightTheme {
    return ThemeData(
      useMaterial3: true,
      brightness: Brightness.light,
      scaffoldBackgroundColor: AltairColors.lightBgPrimary,
      fontFamily: AltairTypography.baseStyle.fontFamily,

      // Color scheme
      colorScheme: const ColorScheme.light(
        primary: AltairColors.accentOrange,
        secondary: AltairColors.accentBlue,
        tertiary: AltairColors.accentGreen,
        error: AltairColors.error,
        surface: AltairColors.lightBgSecondary,
        onPrimary: Colors.white,
        onSecondary: AltairColors.lightTextPrimary,
        onTertiary: AltairColors.lightTextPrimary,
        onSurface: AltairColors.lightTextPrimary,
        onError: Colors.white,
        outline: AltairColors.lightBorderColor,
      ),

      // Text theme
      textTheme: TextTheme(
        displayLarge: AltairTypography.displayLarge.copyWith(
          color: AltairColors.lightTextPrimary,
        ),
        displayMedium: AltairTypography.displayMedium.copyWith(
          color: AltairColors.lightTextPrimary,
        ),
        displaySmall: AltairTypography.displaySmall.copyWith(
          color: AltairColors.lightTextPrimary,
        ),
        headlineLarge: AltairTypography.headlineLarge.copyWith(
          color: AltairColors.lightTextPrimary,
        ),
        headlineMedium: AltairTypography.headlineMedium.copyWith(
          color: AltairColors.lightTextPrimary,
        ),
        headlineSmall: AltairTypography.headlineSmall.copyWith(
          color: AltairColors.lightTextPrimary,
        ),
        bodyLarge: AltairTypography.bodyLarge.copyWith(
          color: AltairColors.lightTextPrimary,
        ),
        bodyMedium: AltairTypography.bodyMedium.copyWith(
          color: AltairColors.lightTextPrimary,
        ),
        bodySmall: AltairTypography.bodySmall.copyWith(
          color: AltairColors.lightTextSecondary,
        ),
        labelLarge: AltairTypography.labelLarge.copyWith(
          color: AltairColors.lightTextPrimary,
        ),
        labelMedium: AltairTypography.labelMedium.copyWith(
          color: AltairColors.lightTextPrimary,
        ),
        labelSmall: AltairTypography.labelSmall.copyWith(
          color: AltairColors.lightTextSecondary,
        ),
      ),

      // Card theme
      cardTheme: const CardThemeData(
        elevation: 0,
        shape: RoundedRectangleBorder(
          side: BorderSide(color: AltairColors.lightBorderColor, width: 1.0),
        ),
        color: AltairColors.lightBgSecondary,
        margin: EdgeInsets.all(AltairSpacing.md),
      ),

      // Button themes
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: AltairColors.lightTextPrimary,
          foregroundColor: AltairColors.lightBgSecondary,
          elevation: 0,
          shape: const RoundedRectangleBorder(),
          side: const BorderSide(
            color: AltairColors.lightBorderColor,
            width: 1.0,
          ),
          padding: const EdgeInsets.symmetric(
            horizontal: AltairSpacing.md,
            vertical: AltairSpacing.sm,
          ),
          textStyle: AltairTypography.labelLarge,
        ),
      ),

      outlinedButtonTheme: OutlinedButtonThemeData(
        style: OutlinedButton.styleFrom(
          foregroundColor: AltairColors.lightTextPrimary,
          elevation: 0,
          shape: const RoundedRectangleBorder(),
          side: const BorderSide(
            color: AltairColors.lightBorderColor,
            width: 1.0,
          ),
          padding: const EdgeInsets.symmetric(
            horizontal: AltairSpacing.md,
            vertical: AltairSpacing.sm,
          ),
          textStyle: AltairTypography.labelLarge,
        ),
      ),

      // Input decoration theme
      inputDecorationTheme: const InputDecorationTheme(
        filled: true,
        fillColor: AltairColors.lightBgSecondary,
        border: OutlineInputBorder(
          borderSide: BorderSide(
            color: AltairColors.lightBorderColor,
            width: 1.0,
          ),
        ),
        enabledBorder: OutlineInputBorder(
          borderSide: BorderSide(
            color: AltairColors.lightBorderColor,
            width: 1.0,
          ),
        ),
        focusedBorder: OutlineInputBorder(
          borderSide: BorderSide(color: AltairColors.accentOrange, width: 2.0),
        ),
        errorBorder: OutlineInputBorder(
          borderSide: BorderSide(color: AltairColors.error, width: 1.0),
        ),
        contentPadding: EdgeInsets.symmetric(
          horizontal: AltairSpacing.md,
          vertical: AltairSpacing.sm,
        ),
      ),

      // App bar theme
      appBarTheme: AppBarTheme(
        backgroundColor: AltairColors.lightBgSecondary,
        surfaceTintColor: Colors.transparent,
        foregroundColor: AltairColors.lightTextPrimary,
        elevation: 0,
        centerTitle: false,
        titleTextStyle: AltairTypography.headlineMedium.copyWith(
          color: AltairColors.lightTextPrimary,
        ),
        shape: const Border(
          bottom: BorderSide(color: AltairColors.lightBorderColor, width: 1.0),
        ),
      ),

      // Divider theme
      dividerTheme: const DividerThemeData(
        color: AltairColors.lightBorderColor,
        thickness: AltairBorders.thin,
        space: 0,
      ),
    );
  }

  /// Dark theme configuration.
  static ThemeData get darkTheme {
    return ThemeData(
      useMaterial3: true,
      brightness: Brightness.dark,
      scaffoldBackgroundColor: AltairColors.darkBgPrimary,
      fontFamily: AltairTypography.baseStyle.fontFamily,

      // Color scheme
      colorScheme: const ColorScheme.dark(
        primary: AltairColors.accentOrange,
        secondary: AltairColors.accentBlue,
        tertiary: AltairColors.accentGreen,
        error: AltairColors.error,
        surface: AltairColors.darkBgSecondary,
        onPrimary: Colors.white,
        onSecondary: AltairColors.darkTextPrimary,
        onTertiary: AltairColors.darkTextPrimary,
        onSurface: AltairColors.darkTextPrimary,
        onError: Colors.white,
        outline: AltairColors.darkBorderColor,
      ),

      // Text theme
      textTheme: TextTheme(
        displayLarge: AltairTypography.displayLarge.copyWith(
          color: AltairColors.darkTextPrimary,
        ),
        displayMedium: AltairTypography.displayMedium.copyWith(
          color: AltairColors.darkTextPrimary,
        ),
        displaySmall: AltairTypography.displaySmall.copyWith(
          color: AltairColors.darkTextPrimary,
        ),
        headlineLarge: AltairTypography.headlineLarge.copyWith(
          color: AltairColors.darkTextPrimary,
        ),
        headlineMedium: AltairTypography.headlineMedium.copyWith(
          color: AltairColors.darkTextPrimary,
        ),
        headlineSmall: AltairTypography.headlineSmall.copyWith(
          color: AltairColors.darkTextPrimary,
        ),
        bodyLarge: AltairTypography.bodyLarge.copyWith(
          color: AltairColors.darkTextPrimary,
        ),
        bodyMedium: AltairTypography.bodyMedium.copyWith(
          color: AltairColors.darkTextPrimary,
        ),
        bodySmall: AltairTypography.bodySmall.copyWith(
          color: AltairColors.darkTextSecondary,
        ),
        labelLarge: AltairTypography.labelLarge.copyWith(
          color: AltairColors.darkTextPrimary,
        ),
        labelMedium: AltairTypography.labelMedium.copyWith(
          color: AltairColors.darkTextPrimary,
        ),
        labelSmall: AltairTypography.labelSmall.copyWith(
          color: AltairColors.darkTextSecondary,
        ),
      ),

      // Card theme
      cardTheme: const CardThemeData(
        elevation: 0,
        shape: RoundedRectangleBorder(
          side: BorderSide(color: AltairColors.darkBorderColor, width: 1.0),
        ),
        color: AltairColors.darkBgSecondary,
        margin: EdgeInsets.all(AltairSpacing.md),
      ),

      // Button themes
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: AltairColors.darkTextPrimary,
          foregroundColor: AltairColors.darkBgSecondary,
          elevation: 0,
          shape: const RoundedRectangleBorder(),
          side: const BorderSide(
            color: AltairColors.darkBorderColor,
            width: 1.0,
          ),
          padding: const EdgeInsets.symmetric(
            horizontal: AltairSpacing.md,
            vertical: AltairSpacing.sm,
          ),
          textStyle: AltairTypography.labelLarge,
        ),
      ),

      outlinedButtonTheme: OutlinedButtonThemeData(
        style: OutlinedButton.styleFrom(
          foregroundColor: AltairColors.darkTextPrimary,
          elevation: 0,
          shape: const RoundedRectangleBorder(),
          side: const BorderSide(
            color: AltairColors.darkBorderColor,
            width: 1.0,
          ),
          padding: const EdgeInsets.symmetric(
            horizontal: AltairSpacing.md,
            vertical: AltairSpacing.sm,
          ),
          textStyle: AltairTypography.labelLarge,
        ),
      ),

      // Input decoration theme
      inputDecorationTheme: const InputDecorationTheme(
        filled: true,
        fillColor: AltairColors.darkBgSecondary,
        border: OutlineInputBorder(
          borderSide: BorderSide(
            color: AltairColors.darkBorderColor,
            width: 1.0,
          ),
        ),
        enabledBorder: OutlineInputBorder(
          borderSide: BorderSide(
            color: AltairColors.darkBorderColor,
            width: 1.0,
          ),
        ),
        focusedBorder: OutlineInputBorder(
          borderSide: BorderSide(color: AltairColors.accentOrange, width: 2.0),
        ),
        errorBorder: OutlineInputBorder(
          borderSide: BorderSide(color: AltairColors.error, width: 1.0),
        ),
        contentPadding: EdgeInsets.symmetric(
          horizontal: AltairSpacing.md,
          vertical: AltairSpacing.sm,
        ),
      ),

      // App bar theme
      appBarTheme: AppBarTheme(
        backgroundColor: AltairColors.darkBgSecondary,
        surfaceTintColor: Colors.transparent,
        foregroundColor: AltairColors.darkTextPrimary,
        elevation: 0,
        centerTitle: false,
        titleTextStyle: AltairTypography.headlineMedium.copyWith(
          color: AltairColors.darkTextPrimary,
        ),
        shape: const Border(
          bottom: BorderSide(color: AltairColors.darkBorderColor, width: 1.0),
        ),
      ),

      // Divider theme
      dividerTheme: const DividerThemeData(
        color: AltairColors.darkBorderColor,
        thickness: AltairBorders.thin,
        space: 0,
      ),
    );
  }
}
