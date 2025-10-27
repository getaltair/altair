/// Status of the Altair database service
enum ServiceStatus {
  /// Service is not installed
  notInstalled,

  /// Service is installed but not running
  stopped,

  /// Service is starting up
  starting,

  /// Service is running and healthy
  running,

  /// Service encountered an error
  error,

  /// Service status is unknown
  unknown,
}

/// Extended service information
class ServiceInfo {
  final ServiceStatus status;
  final String? version;
  final int? pid;
  final DateTime? startedAt;
  final String? errorMessage;
  final String? dataDirectory;
  final int? port;

  const ServiceInfo({
    required this.status,
    this.version,
    this.pid,
    this.startedAt,
    this.errorMessage,
    this.dataDirectory,
    this.port,
  });

  bool get isHealthy => status == ServiceStatus.running;
  bool get isInstalled =>
      status != ServiceStatus.notInstalled && status != ServiceStatus.unknown;

  @override
  String toString() {
    return 'ServiceInfo(status: $status, version: $version, pid: $pid, '
        'startedAt: $startedAt, port: $port)';
  }
}
