/// Events for settings BLoC.
library;

import 'package:equatable/equatable.dart';

import '../../models/ai_settings.dart';

/// Base class for settings events.
abstract class SettingsEvent extends Equatable {
  const SettingsEvent();

  @override
  List<Object?> get props => [];
}

/// Event to load settings from storage.
class SettingsLoadRequested extends SettingsEvent {
  const SettingsLoadRequested();
}

/// Event to update AI settings.
class SettingsAIUpdated extends SettingsEvent {
  const SettingsAIUpdated(this.settings);

  final AISettings settings;

  @override
  List<Object?> get props => [settings];
}

/// Event to toggle AI features on/off.
class SettingsAIToggled extends SettingsEvent {
  const SettingsAIToggled(this.enabled);

  final bool enabled;

  @override
  List<Object?> get props => [enabled];
}

/// Event to save current settings.
class SettingsSaveRequested extends SettingsEvent {
  const SettingsSaveRequested();
}

/// Event to clear all settings.
class SettingsClearRequested extends SettingsEvent {
  const SettingsClearRequested();
}
