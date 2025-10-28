import 'package:altair_db_service/altair_db_service.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('ServiceStatus', () {
    test('enum has all expected values', () {
      expect(ServiceStatus.values, hasLength(6));
      expect(ServiceStatus.values, contains(ServiceStatus.notInstalled));
      expect(ServiceStatus.values, contains(ServiceStatus.stopped));
      expect(ServiceStatus.values, contains(ServiceStatus.starting));
      expect(ServiceStatus.values, contains(ServiceStatus.running));
      expect(ServiceStatus.values, contains(ServiceStatus.error));
      expect(ServiceStatus.values, contains(ServiceStatus.unknown));
    });
  });

  group('ServiceInfo', () {
    test('creates info with running status', () {
      final info = ServiceInfo(
        status: ServiceStatus.running,
        port: 8000,
        dataDirectory: '/path/to/data',
      );

      expect(info.status, equals(ServiceStatus.running));
      expect(info.port, equals(8000));
      expect(info.dataDirectory, equals('/path/to/data'));
      expect(info.errorMessage, isNull);
    });

    test('creates info with error status and message', () {
      const info = ServiceInfo(
        status: ServiceStatus.error,
        errorMessage: 'Connection failed',
      );

      expect(info.status, equals(ServiceStatus.error));
      expect(info.errorMessage, equals('Connection failed'));
      expect(info.port, isNull);
      expect(info.dataDirectory, isNull);
    });

    test('creates info with stopped status', () {
      const info = ServiceInfo(status: ServiceStatus.stopped);

      expect(info.status, equals(ServiceStatus.stopped));
      expect(info.port, isNull);
      expect(info.dataDirectory, isNull);
      expect(info.errorMessage, isNull);
    });

    test('creates info with notInstalled status', () {
      const info = ServiceInfo(status: ServiceStatus.notInstalled);

      expect(info.status, equals(ServiceStatus.notInstalled));
      expect(info.port, isNull);
      expect(info.dataDirectory, isNull);
      expect(info.errorMessage, isNull);
    });
  });
}
