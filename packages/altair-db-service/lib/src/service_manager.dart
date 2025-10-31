import 'dart:io';
import 'dart:async';
import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:process_run/shell.dart';
import 'package:surrealdb/surrealdb.dart';

import 'config.dart';
import 'models/service_status.dart';
import 'platform/service_installer.dart';
import 'platform/linux_installer.dart';
import 'platform/macos_installer.dart';
import 'platform/windows_installer.dart';

/// Manages the Altair database service lifecycle
class AltairDatabaseService {
  final AltairDatabaseConfig config;
  ServiceInstaller? _installer;
  Process? _process;
  SurrealDB? _connection;

  AltairDatabaseService({
    this.config = AltairDatabaseConfig.defaultConfig,
  }) {
    _initializeInstaller();
  }

  void _initializeInstaller() {
    if (Platform.isLinux) {
      _installer = LinuxServiceInstaller(config);
    } else if (Platform.isMacOS) {
      _installer = MacOSServiceInstaller(config);
    } else if (Platform.isWindows) {
      _installer = WindowsServiceInstaller(config);
    }
    // Android and iOS will use fallback mechanisms
  }

  /// Check if the service is currently running
  Future<bool> isRunning() async {
    try {
      final response = await http
          .get(Uri.parse(config.healthCheckUrl))
          .timeout(const Duration(seconds: 2));
      return response.statusCode == 200;
    } catch (e) {
      return false;
    }
  }

  /// Get detailed service information
  Future<ServiceInfo> getStatus() async {
    try {
      if (await isRunning()) {
        return ServiceInfo(
          status: ServiceStatus.running,
          port: config.port,
          dataDirectory: await config.getDataDirectory(),
        );
      } else if (_installer != null && await _installer!.isInstalled()) {
        return const ServiceInfo(status: ServiceStatus.stopped);
      } else {
        return const ServiceInfo(status: ServiceStatus.notInstalled);
      }
    } catch (e) {
      return ServiceInfo(
        status: ServiceStatus.error,
        errorMessage: e.toString(),
      );
    }
  }

  /// Start the database service
  Future<void> start() async {
    // Check if already running
    if (await isRunning()) {
      print('Service is already running');
      return;
    }

    // Try to start via system service first
    if (_installer != null && await _installer!.isInstalled()) {
      await _installer!.start();
      await _waitForService();
      return;
    }

    // Fallback: Start process directly
    await _startProcess();
  }

  /// Start service as a direct process (not as system service)
  Future<void> _startProcess() async {
    final dataDir = await config.getDataDirectory();
    final binaryPath = await _getSurrealDBBinary();
    final credentials = await config.getOrGenerateCredentials();

    final args = [
      'start',
      'file://$dataDir/altair.db',
      '--bind',
      '${config.bindAddress}:${config.port}',
      '--user',
      credentials.username,
      '--pass',
      credentials.password,
    ];

    print('Starting SurrealDB: $binaryPath ${args.join(' ')}');

    _process = await Process.start(binaryPath, args);

    // Log output for debugging
    _process!.stdout.transform(utf8.decoder).listen((data) {
      print('[SurrealDB] $data');
    });

    _process!.stderr.transform(utf8.decoder).listen((data) {
      print('[SurrealDB Error] $data');
    });

    await _waitForService();
  }

  /// Stop the database service
  Future<void> stop() async {
    if (_installer != null && await _installer!.isInstalled()) {
      await _installer!.stop();
    } else if (_process != null) {
      _process!.kill();
      _process = null;
    }

    _connection = null;
  }

  /// Restart the database service
  Future<void> restart() async {
    await stop();
    await Future.delayed(const Duration(seconds: 2));
    await start();
  }

  /// Install the service as a system service
  Future<void> install() async {
    if (_installer == null) {
      throw UnsupportedError(
          'System service installation not supported on ${Platform.operatingSystem}');
    }

    await _installer!.install();
  }

  /// Uninstall the system service
  Future<void> uninstall() async {
    if (_installer == null) {
      throw UnsupportedError(
          'System service installation not supported on ${Platform.operatingSystem}');
    }

    await _installer!.uninstall();
  }

  /// Get a connection to the database
  Future<SurrealDB> getConnection() async {
    if (_connection != null) {
      return _connection!;
    }

    if (!await isRunning()) {
      await start();
    }

    _connection = SurrealDB(config.connectionUri);
    _connection!.connect();

    // Get credentials
    final credentials = await config.getOrGenerateCredentials();

    // Sign in
    await _connection!.signin(
      user: credentials.username,
      pass: credentials.password,
    );

    // Use namespace and database
    await _connection!.use(config.namespace, config.database);

    return _connection!;
  }

  /// Wait for service to be ready (health check passes)
  Future<void> _waitForService({int maxAttempts = 30}) async {
    for (var i = 0; i < maxAttempts; i++) {
      if (await isRunning()) {
        print('Service is ready');
        return;
      }
      await Future.delayed(const Duration(seconds: 1));
    }
    throw TimeoutException(
        'Service failed to start within $maxAttempts seconds');
  }

  /// Get path to SurrealDB binary for current platform
  Future<String> _getSurrealDBBinary() async {
    // First, check if surrealdb is in PATH
    try {
      final shell = Shell();
      final result = await shell.run('which surrealdb || where surrealdb');
      if (result.isNotEmpty && result.first.exitCode == 0) {
        final path = result.first.outText.trim();
        if (path.isNotEmpty) {
          return path;
        }
      }
    } catch (e) {
      // Continue to bundled binary
    }

    // Use bundled binary
    final configDir = await config.getConfigDirectory();
    String binaryName;

    if (Platform.isLinux) {
      binaryName = 'surrealdb-linux';
    } else if (Platform.isMacOS) {
      binaryName = 'surrealdb-macos';
    } else if (Platform.isWindows) {
      binaryName = 'surrealdb-windows.exe';
    } else if (Platform.isAndroid) {
      binaryName = 'surrealdb-android';
    } else {
      throw UnsupportedError(
          'Platform ${Platform.operatingSystem} is not supported');
    }

    final binaryPath = '$configDir/$binaryName';

    // Check if binary exists
    final file = File(binaryPath);
    if (!await file.exists()) {
      throw FileSystemException(
          'SurrealDB binary not found at $binaryPath. Please install the database service.');
    }

    // Make executable on Unix-like systems
    if (Platform.isLinux || Platform.isMacOS) {
      await Process.run('chmod', ['+x', binaryPath]);
    }

    return binaryPath;
  }

  /// Dispose and clean up resources
  void dispose() {
    _connection = null;
    _process?.kill();
    _process = null;
  }
}
