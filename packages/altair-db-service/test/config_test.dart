import 'package:altair_db_service/altair_db_service.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('AltairDatabaseConfig', () {
    test('default config has correct values', () {
      const config = AltairDatabaseConfig.defaultConfig;

      expect(config.port, equals(8000));
      expect(config.bindAddress, equals('127.0.0.1'));
      expect(config.namespace, equals('altair'));
      expect(config.database, equals('local'));
      expect(config.username, equals('altair'));
    });

    test('health check URL is constructed correctly', () {
      const config = AltairDatabaseConfig.defaultConfig;

      expect(config.healthCheckUrl, equals('http://127.0.0.1:8000/health'));
    });

    test('connection URI is constructed correctly', () {
      const config = AltairDatabaseConfig.defaultConfig;

      expect(config.connectionUri, equals('ws://127.0.0.1:8000/rpc'));
    });

    test('custom config preserves values', () {
      const config = AltairDatabaseConfig(
        port: 9000,
        bindAddress: '0.0.0.0',
        namespace: 'test',
        database: 'testdb',
        username: 'testuser',
      );

      expect(config.port, equals(9000));
      expect(config.bindAddress, equals('0.0.0.0'));
      expect(config.namespace, equals('test'));
      expect(config.database, equals('testdb'));
      expect(config.username, equals('testuser'));
    });

    test('custom config generates correct URLs', () {
      const config = AltairDatabaseConfig(port: 9000, bindAddress: 'localhost');

      expect(config.healthCheckUrl, equals('http://localhost:9000/health'));
      expect(config.connectionUri, equals('ws://localhost:9000/rpc'));
    });

    // Skip platform-specific directory tests in unit tests
    // These require Flutter test environment with bindings
    test(
      'getDataDirectory returns platform-specific path',
      () async {
        const config = AltairDatabaseConfig.defaultConfig;

        // This test would require proper Flutter test environment
        expect(() => config.getDataDirectory(), returnsNormally);
      },
      skip: 'Requires Flutter test environment',
    );

    test(
      'getConfigDirectory returns platform-specific path',
      () async {
        const config = AltairDatabaseConfig.defaultConfig;

        // This test would require proper Flutter test environment
        expect(() => config.getConfigDirectory(), returnsNormally);
      },
      skip: 'Requires Flutter test environment',
    );
  });
}
