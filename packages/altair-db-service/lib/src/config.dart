import 'dart:io';
import 'package:path_provider/path_provider.dart';
import 'security/credential_manager.dart';

/// Configuration for Altair database service
class AltairDatabaseConfig {
  /// Port the service binds to
  final int port;

  /// Bind address (default: localhost only)
  final String bindAddress;

  /// Whether to auto-start service on boot
  final bool autoStart;

  /// SurrealDB namespace
  final String namespace;

  /// SurrealDB database name
  final String database;

  /// Directory where database files are stored
  final String? dataDirectory;

  /// Username for database authentication
  final String username;

  /// Whether cloud sync is enabled
  final bool syncEnabled;

  /// Cloud sync URL (if enabled)
  final String? cloudSyncUrl;

  /// Sync interval in seconds
  final int syncInterval;

  const AltairDatabaseConfig({
    this.port = 8000,
    this.bindAddress = '127.0.0.1',
    this.autoStart = true,
    this.namespace = 'altair',
    this.database = 'local',
    this.dataDirectory,
    this.username = 'altair',
    this.syncEnabled = false,
    this.cloudSyncUrl,
    this.syncInterval = 300,
  });

  /// Default configuration
  static const AltairDatabaseConfig defaultConfig = AltairDatabaseConfig();

  /// Get the data directory, creating it if necessary
  Future<String> getDataDirectory() async {
    if (dataDirectory != null) {
      final dir = Directory(dataDirectory!);
      if (!await dir.exists()) {
        await dir.create(recursive: true);
      }
      return dataDirectory!;
    }

    // Platform-specific default locations
    if (Platform.isLinux || Platform.isMacOS) {
      final appSupport = await getApplicationSupportDirectory();
      final altairDir = Directory('${appSupport.path}/altair/database');
      if (!await altairDir.exists()) {
        await altairDir.create(recursive: true);
      }
      return altairDir.path;
    } else if (Platform.isWindows) {
      final appData = Platform.environment['APPDATA'];
      if (appData != null) {
        final altairDir = Directory('$appData\\altair\\database');
        if (!await altairDir.exists()) {
          await altairDir.create(recursive: true);
        }
        return altairDir.path;
      }
    } else if (Platform.isAndroid) {
      final appSupport = await getApplicationSupportDirectory();
      return '${appSupport.path}/database';
    }

    // Fallback
    final temp = await getTemporaryDirectory();
    return '${temp.path}/altair/database';
  }

  /// Get config directory
  Future<String> getConfigDirectory() async {
    if (Platform.isLinux || Platform.isMacOS) {
      final home = Platform.environment['HOME'];
      if (home != null) {
        final configDir = Directory('$home/.altair');
        if (!await configDir.exists()) {
          await configDir.create(recursive: true);
        }
        return configDir.path;
      }
    } else if (Platform.isWindows) {
      final appData = Platform.environment['APPDATA'];
      if (appData != null) {
        final configDir = Directory('$appData\\altair');
        if (!await configDir.exists()) {
          await configDir.create(recursive: true);
        }
        return configDir.path;
      }
    }

    final appSupport = await getApplicationSupportDirectory();
    return '${appSupport.path}/altair';
  }

  /// Connection URI for the database
  String get connectionUri => 'ws://$bindAddress:$port/rpc';

  /// HTTP health check URL
  String get healthCheckUrl => 'http://$bindAddress:$port/health';

  /// Get or generate credentials using CredentialManager
  Future<Credentials> getOrGenerateCredentials() async {
    final configDir = await getConfigDirectory();
    final credManager = CredentialManager(configDir);

    var credentials = await credManager.getCredentials();
    if (credentials == null) {
      // Generate new secure credentials
      credentials = await credManager.generateAndStoreCredentials(
        username: username,
      );
    }

    return credentials;
  }

  /// Copy with changes
  AltairDatabaseConfig copyWith({
    int? port,
    String? bindAddress,
    bool? autoStart,
    String? namespace,
    String? database,
    String? dataDirectory,
    String? username,
    bool? syncEnabled,
    String? cloudSyncUrl,
    int? syncInterval,
  }) {
    return AltairDatabaseConfig(
      port: port ?? this.port,
      bindAddress: bindAddress ?? this.bindAddress,
      autoStart: autoStart ?? this.autoStart,
      namespace: namespace ?? this.namespace,
      database: database ?? this.database,
      dataDirectory: dataDirectory ?? this.dataDirectory,
      username: username ?? this.username,
      syncEnabled: syncEnabled ?? this.syncEnabled,
      cloudSyncUrl: cloudSyncUrl ?? this.cloudSyncUrl,
      syncInterval: syncInterval ?? this.syncInterval,
    );
  }

  @override
  String toString() {
    return 'AltairDatabaseConfig(port: $port, bindAddress: $bindAddress, '
        'namespace: $namespace, database: $database)';
  }
}
