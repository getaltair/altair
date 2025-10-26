import 'dart:convert';

import 'package:altair_guidance/models/ai_settings.dart';
import 'package:altair_guidance/repositories/ai_settings_repository.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:logger/logger.dart';
import 'package:mocktail/mocktail.dart';
import 'package:shared_preferences/shared_preferences.dart';

class MockSharedPreferences extends Mock implements SharedPreferences {}

class MockFlutterSecureStorage extends Mock implements FlutterSecureStorage {}

class MockLogger extends Mock implements Logger {}

void main() {
  late MockSharedPreferences mockPrefs;
  late MockFlutterSecureStorage mockSecureStorage;
  late AISettingsRepository repository;

  setUp(() {
    mockPrefs = MockSharedPreferences();
    mockSecureStorage = MockFlutterSecureStorage();
    repository = AISettingsRepository(
      prefs: mockPrefs,
      secureStorage: mockSecureStorage,
    );
  });

  group('AISettingsRepository', () {
    group('load', () {
      test('returns default settings when no data is stored', () async {
        when(() => mockPrefs.getString(any())).thenReturn(null);

        final settings = await repository.load();

        expect(settings, const AISettings());
        verify(() => mockPrefs.getString('ai_settings')).called(1);
      });

      test('loads settings from shared preferences', () async {
        const testSettings = AISettings(
          enabled: true,
          provider: AIProviderType.openai,
          openaiModel: 'gpt-4',
        );
        final jsonString = jsonEncode(testSettings.toJson());

        when(() => mockPrefs.getString('ai_settings')).thenReturn(jsonString);
        when(() => mockSecureStorage.read(key: 'ai_openai_api_key'))
            .thenAnswer((_) async => null);
        when(() => mockSecureStorage.read(key: 'ai_anthropic_api_key'))
            .thenAnswer((_) async => null);

        final settings = await repository.load();

        expect(settings.enabled, true);
        expect(settings.provider, AIProviderType.openai);
        expect(settings.openaiModel, 'gpt-4');
        verify(() => mockPrefs.getString('ai_settings')).called(1);
      });

      test('loads API keys from secure storage', () async {
        const testSettings = AISettings(
          enabled: true,
          provider: AIProviderType.openai,
        );
        final jsonString = jsonEncode(testSettings.toJson());

        when(() => mockPrefs.getString('ai_settings')).thenReturn(jsonString);
        when(() => mockSecureStorage.read(key: 'ai_openai_api_key'))
            .thenAnswer((_) async => 'openai-secret-key');
        when(() => mockSecureStorage.read(key: 'ai_anthropic_api_key'))
            .thenAnswer((_) async => 'anthropic-secret-key');

        final settings = await repository.load();

        expect(settings.openaiApiKey, 'openai-secret-key');
        expect(settings.anthropicApiKey, 'anthropic-secret-key');
        verify(() => mockSecureStorage.read(key: 'ai_openai_api_key'))
            .called(1);
        verify(() => mockSecureStorage.read(key: 'ai_anthropic_api_key'))
            .called(1);
      });

      test('returns default settings on JSON decode error', () async {
        when(() => mockPrefs.getString('ai_settings'))
            .thenReturn('invalid-json');

        final settings = await repository.load();

        expect(settings, const AISettings());
      });

      test('returns default settings on exception', () async {
        when(() => mockPrefs.getString(any()))
            .thenThrow(Exception('Storage error'));

        final settings = await repository.load();

        expect(settings, const AISettings());
      });
    });

    group('save', () {
      test('saves settings to shared preferences', () async {
        const testSettings = AISettings(
          enabled: true,
          provider: AIProviderType.anthropic,
          anthropicModel: 'claude-3-opus',
        );

        when(() => mockPrefs.setString(any(), any()))
            .thenAnswer((_) async => true);
        when(() => mockSecureStorage.delete(key: any(named: 'key')))
            .thenAnswer((_) async {});

        await repository.save(testSettings);

        final captured = verify(
          () => mockPrefs.setString('ai_settings', captureAny()),
        ).captured;

        expect(captured.length, 1);
        final json = jsonDecode(captured.first as String);
        expect(json['enabled'], true);
        expect(json['provider'], 'anthropic');
        expect(json['anthropicModel'], 'claude-3-opus');
        expect(json.containsKey('anthropicApiKey'), false);
      });

      test('saves OpenAI API key to secure storage', () async {
        const testSettings = AISettings(
          provider: AIProviderType.openai,
          openaiApiKey: 'secret-key',
        );

        when(() => mockPrefs.setString(any(), any()))
            .thenAnswer((_) async => true);
        when(
          () => mockSecureStorage.write(
            key: any(named: 'key'),
            value: any(named: 'value'),
          ),
        ).thenAnswer((_) async {});
        when(() => mockSecureStorage.delete(key: any(named: 'key')))
            .thenAnswer((_) async {});

        await repository.save(testSettings);

        verify(
          () => mockSecureStorage.write(
            key: 'ai_openai_api_key',
            value: 'secret-key',
          ),
        ).called(1);
      });

      test('saves Anthropic API key to secure storage', () async {
        const testSettings = AISettings(
          provider: AIProviderType.anthropic,
          anthropicApiKey: 'secret-key',
        );

        when(() => mockPrefs.setString(any(), any()))
            .thenAnswer((_) async => true);
        when(
          () => mockSecureStorage.write(
            key: any(named: 'key'),
            value: any(named: 'value'),
          ),
        ).thenAnswer((_) async {});
        when(() => mockSecureStorage.delete(key: any(named: 'key')))
            .thenAnswer((_) async {});

        await repository.save(testSettings);

        verify(
          () => mockSecureStorage.write(
            key: 'ai_anthropic_api_key',
            value: 'secret-key',
          ),
        ).called(1);
      });

      test('deletes OpenAI API key when null', () async {
        const testSettings = AISettings(
          provider: AIProviderType.openai,
          openaiApiKey: null,
        );

        when(() => mockPrefs.setString(any(), any()))
            .thenAnswer((_) async => true);
        when(() => mockSecureStorage.delete(key: any(named: 'key')))
            .thenAnswer((_) async {});

        await repository.save(testSettings);

        verify(() => mockSecureStorage.delete(key: 'ai_openai_api_key'))
            .called(1);
      });

      test('deletes Anthropic API key when null', () async {
        const testSettings = AISettings(
          provider: AIProviderType.anthropic,
          anthropicApiKey: null,
        );

        when(() => mockPrefs.setString(any(), any()))
            .thenAnswer((_) async => true);
        when(() => mockSecureStorage.delete(key: any(named: 'key')))
            .thenAnswer((_) async {});

        await repository.save(testSettings);

        verify(() => mockSecureStorage.delete(key: 'ai_anthropic_api_key'))
            .called(1);
      });

      test('rethrows exception on save failure', () async {
        const testSettings = AISettings();

        when(() => mockPrefs.setString(any(), any()))
            .thenThrow(Exception('Storage error'));

        expect(
          () => repository.save(testSettings),
          throwsException,
        );
      });
    });

    group('clear', () {
      test('removes all settings from storage', () async {
        when(() => mockPrefs.remove(any())).thenAnswer((_) async => true);
        when(() => mockSecureStorage.delete(key: any(named: 'key')))
            .thenAnswer((_) async {});

        await repository.clear();

        verify(() => mockPrefs.remove('ai_settings')).called(1);
        verify(() => mockSecureStorage.delete(key: 'ai_openai_api_key'))
            .called(1);
        verify(() => mockSecureStorage.delete(key: 'ai_anthropic_api_key'))
            .called(1);
      });

      test('rethrows exception on clear failure', () async {
        when(() => mockPrefs.remove(any()))
            .thenThrow(Exception('Storage error'));

        expect(
          () => repository.clear(),
          throwsException,
        );
      });
    });

    group('hasApiKey', () {
      test('returns true when OpenAI API key exists', () async {
        when(() => mockSecureStorage.read(key: 'ai_openai_api_key'))
            .thenAnswer((_) async => 'test-key');

        final result = await repository.hasApiKey(AIProviderType.openai);

        expect(result, true);
        verify(() => mockSecureStorage.read(key: 'ai_openai_api_key'))
            .called(1);
      });

      test('returns false when OpenAI API key is null', () async {
        when(() => mockSecureStorage.read(key: 'ai_openai_api_key'))
            .thenAnswer((_) async => null);

        final result = await repository.hasApiKey(AIProviderType.openai);

        expect(result, false);
      });

      test('returns false when OpenAI API key is empty', () async {
        when(() => mockSecureStorage.read(key: 'ai_openai_api_key'))
            .thenAnswer((_) async => '');

        final result = await repository.hasApiKey(AIProviderType.openai);

        expect(result, false);
      });

      test('returns true when Anthropic API key exists', () async {
        when(() => mockSecureStorage.read(key: 'ai_anthropic_api_key'))
            .thenAnswer((_) async => 'test-key');

        final result = await repository.hasApiKey(AIProviderType.anthropic);

        expect(result, true);
        verify(() => mockSecureStorage.read(key: 'ai_anthropic_api_key'))
            .called(1);
      });

      test('returns false when Anthropic API key is null', () async {
        when(() => mockSecureStorage.read(key: 'ai_anthropic_api_key'))
            .thenAnswer((_) async => null);

        final result = await repository.hasApiKey(AIProviderType.anthropic);

        expect(result, false);
      });

      test('returns true for Ollama (no API key needed)', () async {
        final result = await repository.hasApiKey(AIProviderType.ollama);

        expect(result, true);
        verifyNever(() => mockSecureStorage.read(key: any(named: 'key')));
      });

      test('returns false on exception', () async {
        when(() => mockSecureStorage.read(key: any(named: 'key')))
            .thenThrow(Exception('Storage error'));

        final result = await repository.hasApiKey(AIProviderType.openai);

        expect(result, false);
      });
    });
  });
}
