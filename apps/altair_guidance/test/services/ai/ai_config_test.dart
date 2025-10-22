import 'package:altair_guidance/services/ai/ai_config.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('AIConfig', () {
    test('creates config with required parameters', () {
      const config = AIConfig(
        baseUrl: 'http://localhost:8001/api',
      );

      expect(config.baseUrl, 'http://localhost:8001/api');
      expect(config.apiKey, isNull);
      expect(config.enableSSL, true);
      expect(config.breakdownTimeout, const Duration(seconds: 60));
      expect(config.prioritizationTimeout, const Duration(seconds: 45));
      expect(config.estimateTimeout, const Duration(seconds: 30));
      expect(config.suggestionsTimeout, const Duration(seconds: 30));
      expect(config.healthCheckTimeout, const Duration(seconds: 5));
    });

    test('creates config with all parameters', () {
      const config = AIConfig(
        baseUrl: 'https://api.example.com',
        apiKey: 'test-key-123',
        enableSSL: false,
        breakdownTimeout: Duration(seconds: 120),
        prioritizationTimeout: Duration(seconds: 90),
        estimateTimeout: Duration(seconds: 60),
        suggestionsTimeout: Duration(seconds: 45),
        healthCheckTimeout: Duration(seconds: 10),
      );

      expect(config.baseUrl, 'https://api.example.com');
      expect(config.apiKey, 'test-key-123');
      expect(config.enableSSL, false);
      expect(config.breakdownTimeout, const Duration(seconds: 120));
      expect(config.prioritizationTimeout, const Duration(seconds: 90));
      expect(config.estimateTimeout, const Duration(seconds: 60));
      expect(config.suggestionsTimeout, const Duration(seconds: 45));
      expect(config.healthCheckTimeout, const Duration(seconds: 10));
    });

    test('factory development() creates correct config', () {
      final config = AIConfig.development();

      expect(config.baseUrl, 'http://localhost:8001/api');
      expect(config.apiKey, isNull);
      expect(config.enableSSL, false);
    });

    test('factory production() accepts HTTP (relaxed for v0.1.0)', () {
      final config = AIConfig.production(
        baseUrl: 'http://api.example.com',
        apiKey: 'test-key',
      );

      expect(config.baseUrl, 'http://api.example.com');
      expect(config.apiKey, 'test-key');
      expect(config.enableSSL, false);
    });

    test('factory production() accepts empty API key (relaxed for v0.1.0)', () {
      final config = AIConfig.production(
        baseUrl: 'https://api.example.com',
        apiKey: '',
      );

      expect(config.baseUrl, 'https://api.example.com');
      expect(config.apiKey, isNull);
      expect(config.enableSSL, true);
    });

    test('factory production() creates correct config with HTTPS', () {
      final config = AIConfig.production(
        baseUrl: 'https://api.example.com',
        apiKey: 'production-key',
      );

      expect(config.baseUrl, 'https://api.example.com');
      expect(config.apiKey, 'production-key');
      expect(config.enableSSL, true);
    });

    group('validate()', () {
      test('passes for valid HTTP URL in development', () {
        const config = AIConfig(
          baseUrl: 'http://localhost:8001/api',
          enableSSL: false,
        );

        expect(() => config.validate(), returnsNormally);
      });

      test('passes for valid HTTPS URL', () {
        const config = AIConfig(
          baseUrl: 'https://api.example.com',
        );

        expect(() => config.validate(), returnsNormally);
      });

      test('throws for URL without valid scheme', () {
        const config = AIConfig(
          baseUrl: 'not-a-url',
        );

        expect(
          () => config.validate(),
          throwsA(
            isA<StateError>().having(
              (e) => e.message,
              'message',
              contains('must use http:// or https://'),
            ),
          ),
        );
      });

      test('throws for URL without scheme', () {
        const config = AIConfig(
          baseUrl: 'api.example.com',
        );

        expect(
          () => config.validate(),
          throwsA(
            isA<StateError>().having(
              (e) => e.message,
              'message',
              contains('must use http:// or https://'),
            ),
          ),
        );
      });

      test('throws for URL with invalid scheme', () {
        const config = AIConfig(
          baseUrl: 'ftp://api.example.com',
        );

        expect(
          () => config.validate(),
          throwsA(
            isA<StateError>().having(
              (e) => e.message,
              'message',
              contains('must use http:// or https://'),
            ),
          ),
        );
      });

      test('passes when SSL enabled with HTTP (relaxed for v0.1.0)', () {
        const config = AIConfig(
          baseUrl: 'http://api.example.com',
          enableSSL: true,
        );

        // Validation relaxed for v0.1.0 - no longer enforces HTTPS when SSL is enabled
        expect(() => config.validate(), returnsNormally);
      });
    });

    group('authHeaders', () {
      test('returns empty map when no API key', () {
        const config = AIConfig(
          baseUrl: 'http://localhost:8001/api',
        );

        expect(config.authHeaders, isEmpty);
      });

      test('returns Bearer token when API key provided', () {
        const config = AIConfig(
          baseUrl: 'http://localhost:8001/api',
          apiKey: 'test-key-123',
        );

        expect(config.authHeaders, {
          'Authorization': 'Bearer test-key-123',
        });
      });
    });

    group('headers', () {
      test('includes Content-Type', () {
        const config = AIConfig(
          baseUrl: 'http://localhost:8001/api',
        );

        expect(config.headers['Content-Type'], 'application/json');
      });

      test('includes auth headers when API key provided', () {
        const config = AIConfig(
          baseUrl: 'http://localhost:8001/api',
          apiKey: 'test-key',
        );

        expect(config.headers, {
          'Content-Type': 'application/json',
          'Authorization': 'Bearer test-key',
        });
      });

      test('only includes Content-Type when no API key', () {
        const config = AIConfig(
          baseUrl: 'http://localhost:8001/api',
        );

        expect(config.headers, {
          'Content-Type': 'application/json',
        });
      });
    });

    test('toString() includes baseUrl and auth status', () {
      const configWithKey = AIConfig(
        baseUrl: 'http://localhost:8001/api',
        apiKey: 'secret',
        enableSSL: false,
      );
      const configWithoutKey = AIConfig(
        baseUrl: 'http://localhost:8001/api',
      );

      expect(
        configWithKey.toString(),
        'AIConfig(baseUrl: http://localhost:8001/api, hasApiKey: true, enableSSL: false)',
      );
      expect(
        configWithoutKey.toString(),
        'AIConfig(baseUrl: http://localhost:8001/api, hasApiKey: false, enableSSL: true)',
      );
    });
  });
}
