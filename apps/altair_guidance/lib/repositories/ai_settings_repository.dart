/// Repository for AI settings persistence.
library;

import 'dart:convert';

import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:logger/logger.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../models/ai_settings.dart';

/// Repository for managing AI settings persistence.
class AISettingsRepository {
  /// Creates an AI settings repository.
  AISettingsRepository({
    required SharedPreferences prefs,
    FlutterSecureStorage? secureStorage,
  })  : _prefs = prefs,
        _secureStorage = secureStorage ?? const FlutterSecureStorage() {
    _logger = Logger();
  }

  /// Shared preferences instance.
  final SharedPreferences _prefs;

  /// Secure storage for API keys.
  final FlutterSecureStorage _secureStorage;

  /// Logger instance.
  late final Logger _logger;

  /// Key for storing AI settings in shared preferences.
  static const String _settingsKey = 'ai_settings';

  /// Secure storage keys.
  static const String _openaiKeyStorageKey = 'ai_openai_api_key';
  static const String _anthropicKeyStorageKey = 'ai_anthropic_api_key';

  /// Loads AI settings from storage.
  ///
  /// Returns default settings if none are stored.
  Future<AISettings> load() async {
    try {
      final jsonString = _prefs.getString(_settingsKey);
      if (jsonString == null) {
        _logger.i('No stored AI settings found, using defaults');
        return const AISettings();
      }

      final json = jsonDecode(jsonString) as Map<String, dynamic>;
      final settings = AISettings.fromJson(json);

      // Load API keys from secure storage
      final openaiKey = await _secureStorage.read(key: _openaiKeyStorageKey);
      final anthropicKey = await _secureStorage.read(key: _anthropicKeyStorageKey);

      final settingsWithKeys = settings.copyWith(
        openaiApiKey: openaiKey,
        anthropicApiKey: anthropicKey,
      );

      _logger.i('Loaded AI settings: ${settingsWithKeys.provider.displayName}');
      return settingsWithKeys;
    } catch (e, stackTrace) {
      _logger.e('Failed to load AI settings', error: e, stackTrace: stackTrace);
      return const AISettings();
    }
  }

  /// Saves AI settings to storage.
  Future<void> save(AISettings settings) async {
    try {
      // Save non-sensitive settings to shared preferences
      final json = settings.toJson();
      final jsonString = jsonEncode(json);
      await _prefs.setString(_settingsKey, jsonString);

      // Save API keys to secure storage
      if (settings.openaiApiKey != null) {
        await _secureStorage.write(
          key: _openaiKeyStorageKey,
          value: settings.openaiApiKey,
        );
      } else {
        await _secureStorage.delete(key: _openaiKeyStorageKey);
      }

      if (settings.anthropicApiKey != null) {
        await _secureStorage.write(
          key: _anthropicKeyStorageKey,
          value: settings.anthropicApiKey,
        );
      } else {
        await _secureStorage.delete(key: _anthropicKeyStorageKey);
      }

      _logger.i('Saved AI settings: ${settings.provider.displayName}');
    } catch (e, stackTrace) {
      _logger.e('Failed to save AI settings', error: e, stackTrace: stackTrace);
      rethrow;
    }
  }

  /// Clears all AI settings.
  Future<void> clear() async {
    try {
      await _prefs.remove(_settingsKey);
      await _secureStorage.delete(key: _openaiKeyStorageKey);
      await _secureStorage.delete(key: _anthropicKeyStorageKey);
      _logger.i('Cleared AI settings');
    } catch (e, stackTrace) {
      _logger.e('Failed to clear AI settings', error: e, stackTrace: stackTrace);
      rethrow;
    }
  }

  /// Tests if a provider's API key is valid by checking if it's set.
  Future<bool> hasApiKey(AIProviderType provider) async {
    try {
      switch (provider) {
        case AIProviderType.openai:
          final key = await _secureStorage.read(key: _openaiKeyStorageKey);
          return key != null && key.isNotEmpty;
        case AIProviderType.anthropic:
          final key = await _secureStorage.read(key: _anthropicKeyStorageKey);
          return key != null && key.isNotEmpty;
        case AIProviderType.ollama:
          return true; // Ollama doesn't need API key
      }
    } catch (e) {
      _logger.w('Failed to check API key for $provider', error: e);
      return false;
    }
  }
}
