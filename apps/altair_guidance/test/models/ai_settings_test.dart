import 'package:altair_guidance/models/ai_settings.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('AIProvider', () {
    test('has correct display names', () {
      expect(AIProvider.openai.displayName, 'OpenAI');
      expect(AIProvider.anthropic.displayName, 'Anthropic');
      expect(AIProvider.ollama.displayName, 'Ollama (Local)');
    });

    test('has correct default base URLs', () {
      expect(AIProvider.openai.defaultBaseUrl, 'https://api.openai.com/v1');
      expect(
        AIProvider.anthropic.defaultBaseUrl,
        'https://api.anthropic.com/v1',
      );
      expect(AIProvider.ollama.defaultBaseUrl, 'http://localhost:11434');
    });
  });

  group('AISettings', () {
    test('creates default settings correctly', () {
      const settings = AISettings();

      expect(settings.enabled, false);
      expect(settings.provider, AIProvider.ollama);
      expect(settings.openaiApiKey, null);
      expect(settings.anthropicApiKey, null);
      expect(settings.ollamaBaseUrl, null);
      expect(settings.openaiModel, 'gpt-4-turbo-preview');
      expect(settings.anthropicModel, 'claude-3-5-sonnet-20241022');
      expect(settings.ollamaModel, 'llama3');
      expect(settings.customBaseUrl, null);
    });

    test('creates settings with custom values', () {
      const settings = AISettings(
        enabled: true,
        provider: AIProvider.openai,
        openaiApiKey: 'test-key',
        openaiModel: 'gpt-4',
        customBaseUrl: 'https://custom.api.com',
      );

      expect(settings.enabled, true);
      expect(settings.provider, AIProvider.openai);
      expect(settings.openaiApiKey, 'test-key');
      expect(settings.openaiModel, 'gpt-4');
      expect(settings.customBaseUrl, 'https://custom.api.com');
    });

    group('currentApiKey', () {
      test('returns OpenAI API key when provider is OpenAI', () {
        const settings = AISettings(
          provider: AIProvider.openai,
          openaiApiKey: 'openai-key',
          anthropicApiKey: 'anthropic-key',
        );

        expect(settings.currentApiKey, 'openai-key');
      });

      test('returns Anthropic API key when provider is Anthropic', () {
        const settings = AISettings(
          provider: AIProvider.anthropic,
          openaiApiKey: 'openai-key',
          anthropicApiKey: 'anthropic-key',
        );

        expect(settings.currentApiKey, 'anthropic-key');
      });

      test('returns null when provider is Ollama', () {
        const settings = AISettings(
          provider: AIProvider.ollama,
          openaiApiKey: 'openai-key',
        );

        expect(settings.currentApiKey, null);
      });
    });

    group('currentModel', () {
      test('returns OpenAI model when provider is OpenAI', () {
        const settings = AISettings(
          provider: AIProvider.openai,
          openaiModel: 'gpt-4',
        );

        expect(settings.currentModel, 'gpt-4');
      });

      test('returns Anthropic model when provider is Anthropic', () {
        const settings = AISettings(
          provider: AIProvider.anthropic,
          anthropicModel: 'claude-3-opus',
        );

        expect(settings.currentModel, 'claude-3-opus');
      });

      test('returns Ollama model when provider is Ollama', () {
        const settings = AISettings(
          provider: AIProvider.ollama,
          ollamaModel: 'llama3',
        );

        expect(settings.currentModel, 'llama3');
      });
    });

    group('baseUrl', () {
      test('returns custom base URL when set', () {
        const settings = AISettings(
          provider: AIProvider.openai,
          customBaseUrl: 'https://custom.api.com',
        );

        expect(settings.baseUrl, 'https://custom.api.com');
      });

      test('returns default OpenAI URL when provider is OpenAI', () {
        const settings = AISettings(provider: AIProvider.openai);

        expect(settings.baseUrl, 'https://api.openai.com/v1');
      });

      test('returns default Anthropic URL when provider is Anthropic', () {
        const settings = AISettings(provider: AIProvider.anthropic);

        expect(settings.baseUrl, 'https://api.anthropic.com/v1');
      });

      test('returns default Ollama URL when provider is Ollama', () {
        const settings = AISettings(provider: AIProvider.ollama);

        expect(settings.baseUrl, 'http://localhost:11434');
      });

      test('returns custom Ollama URL when set', () {
        const settings = AISettings(
          provider: AIProvider.ollama,
          ollamaBaseUrl: 'http://192.168.1.100:11434',
        );

        expect(settings.baseUrl, 'http://192.168.1.100:11434');
      });
    });

    group('isValid', () {
      test('returns true when disabled', () {
        const settings = AISettings(enabled: false);

        expect(settings.isValid, true);
      });

      test('returns false when OpenAI provider has no API key', () {
        const settings = AISettings(
          enabled: true,
          provider: AIProvider.openai,
        );

        expect(settings.isValid, false);
      });

      test('returns false when OpenAI provider has empty API key', () {
        const settings = AISettings(
          enabled: true,
          provider: AIProvider.openai,
          openaiApiKey: '',
        );

        expect(settings.isValid, false);
      });

      test('returns true when OpenAI provider has API key', () {
        const settings = AISettings(
          enabled: true,
          provider: AIProvider.openai,
          openaiApiKey: 'test-key',
        );

        expect(settings.isValid, true);
      });

      test('returns false when Anthropic provider has no API key', () {
        const settings = AISettings(
          enabled: true,
          provider: AIProvider.anthropic,
        );

        expect(settings.isValid, false);
      });

      test('returns false when Anthropic provider has empty API key', () {
        const settings = AISettings(
          enabled: true,
          provider: AIProvider.anthropic,
          anthropicApiKey: '',
        );

        expect(settings.isValid, false);
      });

      test('returns true when Anthropic provider has API key', () {
        const settings = AISettings(
          enabled: true,
          provider: AIProvider.anthropic,
          anthropicApiKey: 'test-key',
        );

        expect(settings.isValid, true);
      });

      test('returns true when Ollama provider is enabled', () {
        const settings = AISettings(
          enabled: true,
          provider: AIProvider.ollama,
        );

        expect(settings.isValid, true);
      });

      test('returns false when Ollama has empty base URL', () {
        const settings = AISettings(
          enabled: true,
          provider: AIProvider.ollama,
          ollamaBaseUrl: '',
        );

        expect(settings.isValid, false);
      });
    });

    group('copyWith', () {
      test('creates copy with updated enabled', () {
        const original = AISettings();
        final updated = original.copyWith(enabled: true);

        expect(updated.enabled, true);
        expect(updated.provider, original.provider);
      });

      test('creates copy with updated provider', () {
        const original = AISettings();
        final updated = original.copyWith(provider: AIProvider.openai);

        expect(updated.provider, AIProvider.openai);
        expect(updated.enabled, original.enabled);
      });

      test('creates copy with updated API keys', () {
        const original = AISettings();
        final updated = original.copyWith(
          openaiApiKey: 'new-openai-key',
          anthropicApiKey: 'new-anthropic-key',
        );

        expect(updated.openaiApiKey, 'new-openai-key');
        expect(updated.anthropicApiKey, 'new-anthropic-key');
      });

      test('creates copy with updated models', () {
        const original = AISettings();
        final updated = original.copyWith(
          openaiModel: 'gpt-4',
          anthropicModel: 'claude-3-opus',
          ollamaModel: 'mistral',
        );

        expect(updated.openaiModel, 'gpt-4');
        expect(updated.anthropicModel, 'claude-3-opus');
        expect(updated.ollamaModel, 'mistral');
      });

      test('preserves original values when not specified', () {
        const original = AISettings(
          enabled: true,
          provider: AIProvider.openai,
          openaiApiKey: 'test-key',
        );
        final updated = original.copyWith(openaiModel: 'gpt-4');

        expect(updated.enabled, original.enabled);
        expect(updated.provider, original.provider);
        expect(updated.openaiApiKey, original.openaiApiKey);
        expect(updated.openaiModel, 'gpt-4');
      });
    });

    group('JSON serialization', () {
      test('toJson includes all non-sensitive fields', () {
        const settings = AISettings(
          enabled: true,
          provider: AIProvider.openai,
          openaiApiKey: 'secret-key',
          openaiModel: 'gpt-4',
          customBaseUrl: 'https://custom.api.com',
        );

        final json = settings.toJson();

        expect(json['enabled'], true);
        expect(json['provider'], 'openai');
        expect(json['openaiModel'], 'gpt-4');
        expect(json['customBaseUrl'], 'https://custom.api.com');
        expect(json.containsKey('openaiApiKey'), false);
        expect(json.containsKey('anthropicApiKey'), false);
      });

      test('fromJson creates settings correctly', () {
        final json = {
          'enabled': true,
          'provider': 'anthropic',
          'anthropicModel': 'claude-3-opus',
          'ollamaBaseUrl': 'http://custom:11434',
        };

        final settings = AISettings.fromJson(json);

        expect(settings.enabled, true);
        expect(settings.provider, AIProvider.anthropic);
        expect(settings.anthropicModel, 'claude-3-opus');
        expect(settings.ollamaBaseUrl, 'http://custom:11434');
        expect(settings.openaiApiKey, null);
        expect(settings.anthropicApiKey, null);
      });

      test('fromJson uses defaults for missing fields', () {
        final json = <String, dynamic>{};

        final settings = AISettings.fromJson(json);

        expect(settings.enabled, false);
        expect(settings.provider, AIProvider.ollama);
        expect(settings.openaiModel, 'gpt-4-turbo-preview');
        expect(settings.anthropicModel, 'claude-3-5-sonnet-20241022');
        expect(settings.ollamaModel, 'llama3');
      });

      test('fromJson handles unknown provider gracefully', () {
        final json = {'provider': 'unknown-provider'};

        final settings = AISettings.fromJson(json);

        expect(settings.provider, AIProvider.ollama);
      });

      test('round-trip serialization preserves data', () {
        const original = AISettings(
          enabled: true,
          provider: AIProvider.anthropic,
          anthropicModel: 'claude-3-opus',
          customBaseUrl: 'https://custom.api.com',
        );

        final json = original.toJson();
        final deserialized = AISettings.fromJson(json);

        expect(deserialized.enabled, original.enabled);
        expect(deserialized.provider, original.provider);
        expect(deserialized.anthropicModel, original.anthropicModel);
        expect(deserialized.customBaseUrl, original.customBaseUrl);
      });
    });

    group('Equatable', () {
      test('equals when all fields are the same', () {
        const settings1 = AISettings(
          enabled: true,
          provider: AIProvider.openai,
          openaiApiKey: 'test-key',
        );
        const settings2 = AISettings(
          enabled: true,
          provider: AIProvider.openai,
          openaiApiKey: 'test-key',
        );

        expect(settings1, equals(settings2));
      });

      test('not equals when fields differ', () {
        const settings1 = AISettings(enabled: true);
        const settings2 = AISettings(enabled: false);

        expect(settings1, isNot(equals(settings2)));
      });

      test('not equals when API keys differ', () {
        const settings1 = AISettings(
          provider: AIProvider.openai,
          openaiApiKey: 'key1',
        );
        const settings2 = AISettings(
          provider: AIProvider.openai,
          openaiApiKey: 'key2',
        );

        expect(settings1, isNot(equals(settings2)));
      });
    });

    group('toString', () {
      test('includes relevant information', () {
        const settings = AISettings(
          enabled: true,
          provider: AIProvider.openai,
          openaiApiKey: 'test-key',
          openaiModel: 'gpt-4',
        );

        final str = settings.toString();

        expect(str, contains('enabled: true'));
        expect(str, contains('provider: OpenAI'));
        expect(str, contains('model: gpt-4'));
        expect(str, contains('valid: true'));
        expect(str, isNot(contains('test-key'))); // Should not expose API key
      });
    });
  });
}
