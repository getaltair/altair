/// States for settings BLoC.
library;

import 'package:equatable/equatable.dart';

import '../../models/ai_settings.dart';

/// Base class for settings states.
abstract class SettingsState extends Equatable {
  const SettingsState();

  @override
  List<Object?> get props => [];
}

/// Initial state before settings are loaded.
class SettingsInitial extends SettingsState {
  const SettingsInitial();
}

/// Loading settings from storage.
class SettingsLoading extends SettingsState {
  const SettingsLoading();
}

/// Settings loaded successfully.
class SettingsLoaded extends SettingsState {
  const SettingsLoaded(this.aiSettings);

  final AISettings aiSettings;

  @override
  List<Object?> get props => [aiSettings];
}

/// Settings being saved.
class SettingsSaving extends SettingsState {
  const SettingsSaving(this.aiSettings);

  final AISettings aiSettings;

  @override
  List<Object?> get props => [aiSettings];
}

/// Settings saved successfully.
class SettingsSaved extends SettingsState {
  const SettingsSaved(this.aiSettings);

  final AISettings aiSettings;

  @override
  List<Object?> get props => [aiSettings];
}

/// Failed to load or save settings.
class SettingsFailure extends SettingsState {
  const SettingsFailure(this.message, {this.aiSettings});

  final String message;
  final AISettings? aiSettings;

  @override
  List<Object?> get props => [message, aiSettings];
}
