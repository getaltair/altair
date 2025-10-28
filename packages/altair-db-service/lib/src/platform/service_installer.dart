import '../config.dart';

/// Abstract interface for platform-specific service installation
abstract class ServiceInstaller {
  final AltairDatabaseConfig config;

  ServiceInstaller(this.config);

  /// Check if the service is installed
  Future<bool> isInstalled();

  /// Install the service
  Future<void> install();

  /// Uninstall the service
  Future<void> uninstall();

  /// Start the service
  Future<void> start();

  /// Stop the service
  Future<void> stop();

  /// Restart the service
  Future<void> restart() async {
    await stop();
    await Future.delayed(const Duration(seconds: 2));
    await start();
  }

  /// Enable service to start on boot
  Future<void> enable();

  /// Disable service from starting on boot
  Future<void> disable();
}
