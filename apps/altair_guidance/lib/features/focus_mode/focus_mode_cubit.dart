/// Focus mode state management.
library;

import 'package:flutter_bloc/flutter_bloc.dart';

/// Cubit to manage focus mode state.
class FocusModeCubit extends Cubit<FocusModeState> {
  /// Creates a focus mode cubit.
  FocusModeCubit() : super(const FocusModeState(isEnabled: false));

  /// Toggles focus mode on/off.
  void toggle() {
    emit(FocusModeState(isEnabled: !state.isEnabled));
  }

  /// Enables focus mode.
  void enable() {
    if (!state.isEnabled) {
      emit(const FocusModeState(isEnabled: true));
    }
  }

  /// Disables focus mode.
  void disable() {
    if (state.isEnabled) {
      emit(const FocusModeState(isEnabled: false));
    }
  }
}

/// State for focus mode.
class FocusModeState {
  /// Creates a focus mode state.
  const FocusModeState({required this.isEnabled});

  /// Whether focus mode is currently enabled.
  final bool isEnabled;

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is FocusModeState && other.isEnabled == isEnabled;
  }

  @override
  int get hashCode => isEnabled.hashCode;
}
