import 'package:altair_guidance/models/ai_settings.dart';
import 'package:altair_guidance/services/ai/ai_config.dart';
import 'package:altair_guidance/services/ai/providers/anthropic_provider.dart';
import 'package:altair_guidance/services/ai/providers/openai_provider.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('AIConfig.createProvider', () {
    test('returns null when AI is disabled', () {
      const settings = AISettings(
        enabled: false,
        provider: AIProviderType.openai,
        openaiApiKey: 'test-key',
      );

      final provider = AIConfig.createProvider(settings);

      expect(provider, isNull);
    });

    test('returns null when OpenAI API key is missing', () {
      const settings = AISettings(
        enabled: true,
        provider: AIProviderType.openai,
        openaiApiKey: null,
      );

      final provider = AIConfig.createProvider(settings);

      expect(provider, isNull);
    });

    test('returns null when OpenAI API key is empty', () {
      const settings = AISettings(
        enabled: true,
        provider: AIProviderType.openai,
        openaiApiKey: '',
      );

      final provider = AIConfig.createProvider(settings);

      expect(provider, isNull);
    });

    test('creates OpenAI provider with valid settings', () {
      const settings = AISettings(
        enabled: true,
        provider: AIProviderType.openai,
        openaiApiKey: 'test-openai-key',
        openaiModel: 'gpt-4o',
      );

      final provider = AIConfig.createProvider(settings);

      expect(provider, isNotNull);
      expect(provider, isA<OpenAIProvider>());
    });

    test('creates OpenAI provider with default model', () {
      const settings = AISettings(
        enabled: true,
        provider: AIProviderType.openai,
        openaiApiKey: 'test-openai-key',
        // Using default openaiModel from AISettings
      );

      final provider = AIConfig.createProvider(settings);

      expect(provider, isNotNull);
      expect(provider, isA<OpenAIProvider>());
    });

    test('returns null when Anthropic API key is missing', () {
      const settings = AISettings(
        enabled: true,
        provider: AIProviderType.anthropic,
        anthropicApiKey: null,
      );

      final provider = AIConfig.createProvider(settings);

      expect(provider, isNull);
    });

    test('returns null when Anthropic API key is empty', () {
      const settings = AISettings(
        enabled: true,
        provider: AIProviderType.anthropic,
        anthropicApiKey: '',
      );

      final provider = AIConfig.createProvider(settings);

      expect(provider, isNull);
    });

    test('creates Anthropic provider with valid settings', () {
      const settings = AISettings(
        enabled: true,
        provider: AIProviderType.anthropic,
        anthropicApiKey: 'test-anthropic-key',
        anthropicModel: 'claude-3-5-sonnet-20241022',
      );

      final provider = AIConfig.createProvider(settings);

      expect(provider, isNotNull);
      expect(provider, isA<AnthropicProvider>());
    });

    test('creates Anthropic provider with default model', () {
      const settings = AISettings(
        enabled: true,
        provider: AIProviderType.anthropic,
        anthropicApiKey: 'test-anthropic-key',
        // Using default anthropicModel from AISettings
      );

      final provider = AIConfig.createProvider(settings);

      expect(provider, isNotNull);
      expect(provider, isA<AnthropicProvider>());
    });

    test('throws UnimplementedError for Ollama provider', () {
      const settings = AISettings(
        enabled: true,
        provider: AIProviderType.ollama,
      );

      expect(
        () => AIConfig.createProvider(settings),
        throwsA(isA<UnimplementedError>()),
      );
    });
  });
}
