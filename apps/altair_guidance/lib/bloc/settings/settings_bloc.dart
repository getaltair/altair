/// BLoC for managing application settings.
library;

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/logger.dart';

import '../../models/ai_settings.dart';
import '../../repositories/ai_settings_repository.dart';
import 'settings_event.dart';
import 'settings_state.dart';

/// BLoC for managing settings.
class SettingsBloc extends Bloc<SettingsEvent, SettingsState> {
  /// Creates a settings BLoC.
  SettingsBloc({
    required AISettingsRepository aiSettingsRepository,
  })  : _aiSettingsRepository = aiSettingsRepository,
        super(const SettingsInitial()) {
    _logger = Logger();

    on<SettingsLoadRequested>(_onLoadRequested);
    on<SettingsAIUpdated>(_onAIUpdated);
    on<SettingsAIToggled>(_onAIToggled);
    on<SettingsSaveRequested>(_onSaveRequested);
    on<SettingsClearRequested>(_onClearRequested);
  }

  /// AI settings repository.
  final AISettingsRepository _aiSettingsRepository;

  /// Logger instance.
  late final Logger _logger;

  /// Current AI settings (cached).
  AISettings? _currentAISettings;

  /// Handles loading settings from storage.
  Future<void> _onLoadRequested(
    SettingsLoadRequested event,
    Emitter<SettingsState> emit,
  ) async {
    _logger.i('Loading settings...');
    emit(const SettingsLoading());

    try {
      final aiSettings = await _aiSettingsRepository.load();
      _currentAISettings = aiSettings;

      _logger.i('Settings loaded successfully');
      emit(SettingsLoaded(aiSettings));
    } catch (e, stackTrace) {
      _logger.e('Failed to load settings', error: e, stackTrace: stackTrace);
      emit(SettingsFailure('Failed to load settings: $e'));
    }
  }

  /// Handles updating AI settings.
  Future<void> _onAIUpdated(
    SettingsAIUpdated event,
    Emitter<SettingsState> emit,
  ) async {
    _logger.i('Updating AI settings: ${event.settings.provider.displayName}');

    _currentAISettings = event.settings;
    emit(SettingsLoaded(event.settings));

    // Auto-save after update
    add(const SettingsSaveRequested());
  }

  /// Handles toggling AI features on/off.
  Future<void> _onAIToggled(
    SettingsAIToggled event,
    Emitter<SettingsState> emit,
  ) async {
    _logger.i('Toggling AI features: ${event.enabled}');

    if (_currentAISettings == null) {
      _logger.w('Cannot toggle AI - settings not loaded');
      return;
    }

    final updated = _currentAISettings!.copyWith(enabled: event.enabled);
    _currentAISettings = updated;
    emit(SettingsLoaded(updated));

    // Auto-save after toggle
    add(const SettingsSaveRequested());
  }

  /// Handles saving settings to storage.
  Future<void> _onSaveRequested(
    SettingsSaveRequested event,
    Emitter<SettingsState> emit,
  ) async {
    if (_currentAISettings == null) {
      _logger.w('Cannot save - no settings loaded');
      return;
    }

    _logger.i('Saving settings...');
    emit(SettingsSaving(_currentAISettings!));

    try {
      await _aiSettingsRepository.save(_currentAISettings!);
      _logger.i('Settings saved successfully');
      emit(SettingsSaved(_currentAISettings!));

      // Return to loaded state
      emit(SettingsLoaded(_currentAISettings!));
    } catch (e, stackTrace) {
      _logger.e('Failed to save settings', error: e, stackTrace: stackTrace);
      emit(SettingsFailure(
        'Failed to save settings: $e',
        aiSettings: _currentAISettings,
      ));

      // Return to loaded state with unsaved settings
      if (_currentAISettings != null) {
        emit(SettingsLoaded(_currentAISettings!));
      }
    }
  }

  /// Handles clearing all settings.
  Future<void> _onClearRequested(
    SettingsClearRequested event,
    Emitter<SettingsState> emit,
  ) async {
    _logger.i('Clearing all settings...');
    emit(const SettingsLoading());

    try {
      await _aiSettingsRepository.clear();
      _currentAISettings = const AISettings();

      _logger.i('Settings cleared successfully');
      emit(SettingsLoaded(_currentAISettings!));
    } catch (e, stackTrace) {
      _logger.e('Failed to clear settings', error: e, stackTrace: stackTrace);
      emit(SettingsFailure('Failed to clear settings: $e'));
    }
  }
}
