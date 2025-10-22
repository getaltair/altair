/// Theme management cubit.
library;

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

/// Theme mode state.
class ThemeState {
  const ThemeState(this.themeMode);

  /// The current theme mode.
  final ThemeMode themeMode;

  /// Whether dark mode is currently active.
  bool get isDark => themeMode == ThemeMode.dark;

  /// Whether light mode is currently active.
  bool get isLight => themeMode == ThemeMode.light;

  /// Whether system theme is being used.
  bool get isSystem => themeMode == ThemeMode.system;
}

/// Cubit for managing app theme.
class ThemeCubit extends Cubit<ThemeState> {
  /// Creates a theme cubit with system theme as default.
  ThemeCubit() : super(const ThemeState(ThemeMode.system));

  /// Toggle between light and dark theme.
  void toggle() {
    if (state.themeMode == ThemeMode.light) {
      emit(const ThemeState(ThemeMode.dark));
    } else {
      emit(const ThemeState(ThemeMode.light));
    }
  }

  /// Set theme mode explicitly.
  void setThemeMode(ThemeMode mode) {
    emit(ThemeState(mode));
  }

  /// Use system theme.
  void useSystemTheme() {
    emit(const ThemeState(ThemeMode.system));
  }
}
