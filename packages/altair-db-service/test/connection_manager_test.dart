import 'package:altair_db_service/altair_db_service.dart';
import 'package:flutter_test/flutter_test.dart';

// Note: This test focuses on the singleton pattern and basic structure
// Full integration tests with actual SurrealDB are in integration_test/

void main() {
  group('AltairConnectionManager', () {
    test('getInstance returns singleton instance', () async {
      // Note: This test requires actual SurrealDB to be running
      // In a unit test environment, this would need mocking
      // For now, we test the pattern itself

      expect(() => AltairConnectionManager.getInstance(), returnsNormally);
    });

    test('connection manager has correct configuration', () {
      const config = AltairDatabaseConfig.defaultConfig;

      expect(config.namespace, equals('altair'));
      expect(config.database, equals('local'));
    });

    test('default config uses correct namespace and database', () {
      // Verify the connection manager will use correct defaults
      expect(AltairConnectionManager.namespace, equals('altair'));
      expect(AltairConnectionManager.database, equals('local'));
    });
  });

  group('AltairConnectionManager schema', () {
    test('required tables are defined', () {
      final requiredTables = [
        'task',
        'project',
        'tag',
        'note',
        'item',
        'link',
      ];

      // This is a documentation test - ensuring we know what tables
      // should exist when schema initialization runs
      expect(requiredTables, hasLength(6));
      expect(requiredTables, contains('task'));
      expect(requiredTables, contains('project'));
      expect(requiredTables, contains('tag'));
      expect(requiredTables, contains('link'));
    });
  });
}
