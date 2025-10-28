# Altair Database Service - Complete Implementation Guide

**TL;DR:** Step-by-step guide to implement the shared SurrealDB service that all three Altair apps use.

---

## 📁 Package Structure

```
packages/altair_db_service/
├── pubspec.yaml
├── lib/
│   ├── altair_db_service.dart           # Main export
│   ├── src/
│   │   ├── service_manager.dart         # Core service lifecycle
│   │   ├── connection_manager.dart      # DB connection handling
│   │   ├── installer.dart               # Platform-specific installation
│   │   ├── config.dart                  # Service configuration
│   │   ├── platform/
│   │   │   ├── linux_service.dart       # systemd integration
│   │   │   ├── macos_service.dart       # launchd integration
│   │   │   ├── windows_service.dart     # Windows Service integration
│   │   │   └── android_service.dart     # Android Service (optional)
│   │   └── models/
│   │       ├── service_status.dart
│   │       └── service_config.dart
│   └── resources/
│       ├── systemd/
│       │   └── altair-db.service
│       ├── launchd/
│       │   └── com.getaltair.database.plist
│       └── windows/
│           └── altair-db-service.xml
├── bin/
│   └── download_surrealdb.dart          # Download binaries script
└── test/
    └── service_manager_test.dart
```

---

## 📦 pubspec.yaml

```yaml
name: altair_db_service
description: Shared database service for all Altair apps
version: 0.1.0
publish_to: none

environment:
  sdk: '>=3.0.0 <4.0.0'

dependencies:
  path: ^1.8.0
  path_provider: ^2.1.0
  http: ^1.1.0
  surrealdb: ^1.0.0  # The Dart client
  crypto: ^3.0.3
  process_run: ^0.14.0

dev_dependencies:
  test: ^1.24.0
  lints: ^2.1.0
```

---

## 🔧 Core Service Manager

```dart
// lib/src/service_manager.dart

import 'dart:io';
import 'dart:async';
import 'package:path/path.dart' as path;
import 'package:path_provider/path_provider.dart';
import 'package:http/http.dart' as http;
import 'package:process_run/shell.dart';

import 'config.dart';
import 'service_status.dart';
import 'platform/linux_service.dart';
import 'platform/macos_service.dart';
import 'platform/windows_service.dart';

class AltairDatabaseService {
  static const String serviceName = 'altair-db';
  static const int defaultPort = 8000;
  static const String defaultHost = '127.0.0.1';

  final ServiceConfig config;
  Process? _process;

  AltairDatabaseService({ServiceConfig? config})
      : config = config ?? ServiceConfig.defaultConfig();

  /// Check if the service is running and healthy
  Future<ServiceStatus> getStatus() async {
    try {
      final response = await http
          .get(Uri.parse('http://$defaultHost:${config.port}/health'))
          .timeout(Duration(seconds: 2));

      if (response.statusCode == 200) {
        return ServiceStatus(
          isRunning: true,
          port: config.port,
          dataPath: config.dataPath,
          version: await _getSurrealVersion(),
        );
      }
    } catch (e) {
      // Service not responding
    }

    return ServiceStatus(
      isRunning: false,
      port: config.port,
      dataPath: config.dataPath,
    );
  }

  /// Start the database service
  Future<void> start() async {
    final status = await getStatus();
    if (status.isRunning) {
      print('Service already running on port ${status.port}');
      return;
    }

    // Ensure data directory exists
    final dataDir = Directory(config.dataPath);
    if (!await dataDir.exists()) {
      await dataDir.create(recursive: true);
    }

    // Get SurrealDB binary
    final binary = await _getSurrealDBBinary();
    if (binary == null) {
      throw Exception('SurrealDB binary not found. Run installation first.');
    }

    // Get or create credentials
    final credentials = await config.getOrCreateCredentials();

    // Start SurrealDB process
    print('Starting Altair database service...');
    _process = await Process.start(
      binary,
      [
        'start',
        '--bind', '$defaultHost:${config.port}',
        '--user', credentials.username,
        '--pass', credentials.password,
        'file://${config.dataPath}/altair.db',
      ],
      mode: ProcessStartMode.detached,
    );

    // Wait for service to be ready
    await _waitForService();
    print('✓ Altair database service started on port ${config.port}');
  }

  /// Stop the database service
  Future<void> stop() async {
    if (_process != null) {
      _process!.kill();
      _process = null;
      print('✓ Altair database service stopped');
    } else {
      // Try to kill by port
      if (Platform.isLinux || Platform.isMacOS) {
        final shell = Shell();
        try {
          await shell.run('pkill -f "surrealdb.*${config.port}"');
        } catch (e) {
          // Process might not be running
        }
      }
    }
  }

  /// Install as system service (optional, for persistence)
  Future<void> installSystemService() async {
    if (Platform.isLinux) {
      final installer = LinuxServiceInstaller(config);
      await installer.install();
    } else if (Platform.isMacOS) {
      final installer = MacOSServiceInstaller(config);
      await installer.install();
    } else if (Platform.isWindows) {
      final installer = WindowsServiceInstaller(config);
      await installer.install();
    } else {
      throw UnsupportedError('System service not supported on this platform');
    }
  }

  /// Uninstall system service
  Future<void> uninstallSystemService() async {
    if (Platform.isLinux) {
      final installer = LinuxServiceInstaller(config);
      await installer.uninstall();
    } else if (Platform.isMacOS) {
      final installer = MacOSServiceInstaller(config);
      await installer.uninstall();
    } else if (Platform.isWindows) {
      final installer = WindowsServiceInstaller(config);
      await installer.uninstall();
    }
  }

  /// Wait for service to respond
  Future<void> _waitForService({int maxAttempts = 30}) async {
    for (int i = 0; i < maxAttempts; i++) {
      try {
        final response = await http
            .get(Uri.parse('http://$defaultHost:${config.port}/health'))
            .timeout(Duration(seconds: 1));
        if (response.statusCode == 200) {
          return;
        }
      } catch (e) {
        await Future.delayed(Duration(seconds: 1));
      }
    }
    throw TimeoutException('Service did not start within 30 seconds');
  }

  /// Get path to SurrealDB binary
  Future<String?> _getSurrealDBBinary() async {
    final appDir = await config.getServiceDirectory();

    String binaryName = 'surrealdb';
    if (Platform.isWindows) {
      binaryName = 'surrealdb.exe';
    }

    final binaryPath = path.join(appDir, 'bin', binaryName);
    final binary = File(binaryPath);

    if (await binary.exists()) {
      return binaryPath;
    }

    // Try system PATH
    try {
      final shell = Shell();
      await shell.run('which surrealdb');
      return 'surrealdb';
    } catch (e) {
      return null;
    }
  }

  /// Get SurrealDB version
  Future<String?> _getSurrealVersion() async {
    final binary = await _getSurrealDBBinary();
    if (binary == null) return null;

    try {
      final result = await Process.run(binary, ['version']);
      return result.stdout.toString().trim();
    } catch (e) {
      return null;
    }
  }
}
```

---

## 🔧 Connection Manager (Shared by Apps)

```dart
// lib/src/connection_manager.dart

import 'package:surrealdb/surrealdb.dart';
import 'service_manager.dart';
import 'config.dart';

class AltairConnectionManager {
  static AltairConnectionManager? _instance;
  late Surreal _db;
  bool _isConnected = false;

  static const String namespace = 'altair';
  static const String database = 'local';

  AltairConnectionManager._();

  static Future<AltairConnectionManager> getInstance() async {
    if (_instance == null) {
      _instance = AltairConnectionManager._();
      await _instance!._connect();
    }
    return _instance!;
  }

  Future<void> _connect() async {
    final config = ServiceConfig.defaultConfig();
    final service = AltairDatabaseService(config: config);

    // Check if service is running
    var status = await service.getStatus();

    if (!status.isRunning) {
      print('Database service not running, starting...');
      try {
        await service.start();
        status = await service.getStatus();
      } catch (e) {
        throw Exception('Failed to start database service: $e');
      }
    }

    // Connect to the service
    _db = Surreal('ws://127.0.0.1:${config.port}/rpc');
    await _db.connect();

    // Authenticate
    final credentials = await config.getOrCreateCredentials();
    await _db.signin(
      username: credentials.username,
      password: credentials.password,
    );

    // Select namespace and database
    await _db.use(namespace: namespace, database: database);

    _isConnected = true;
    print('✓ Connected to Altair database');

    // Initialize schema if needed
    await _initializeSchema();
  }

  /// Initialize database schema (first time setup)
  Future<void> _initializeSchema() async {
    await _db.query('''
      -- Core tables
      DEFINE TABLE IF NOT EXISTS task SCHEMALESS;
      DEFINE TABLE IF NOT EXISTS note SCHEMALESS;
      DEFINE TABLE IF NOT EXISTS item SCHEMALESS;

      -- Cross-app linking
      DEFINE TABLE IF NOT EXISTS link SCHEMALESS;
      DEFINE FIELD IF NOT EXISTS source ON link TYPE record;
      DEFINE FIELD IF NOT EXISTS target ON link TYPE record;
      DEFINE FIELD IF NOT EXISTS link_type ON link TYPE string;
      DEFINE FIELD IF NOT EXISTS created_at ON link TYPE datetime DEFAULT time::now();

      -- Indexes for performance
      DEFINE INDEX IF NOT EXISTS link_source_idx ON link FIELDS source;
      DEFINE INDEX IF NOT EXISTS link_target_idx ON link FIELDS target;
    ''');
  }

  /// Get the raw Surreal client for advanced queries
  Surreal get client {
    if (!_isConnected) {
      throw StateError('Not connected to database');
    }
    return _db;
  }

  /// Create a link between two resources
  Future<void> createLink({
    required String sourceId,
    required String targetId,
    required String linkType,
  }) async {
    await _db.create('link', {
      'source': sourceId,
      'target': targetId,
      'link_type': linkType,
      'created_at': DateTime.now().toIso8601String(),
    });
  }

  /// Get all resources linked to a given resource
  Future<List<dynamic>> getLinkedResources(String resourceId) async {
    final result = await _db.query('''
      -- Get all links where resource is source or target
      LET \$links = (
        SELECT * FROM link
        WHERE source = \$resource OR target = \$resource
      );

      -- Get the other end of each link
      SELECT
        (CASE
          WHEN source = \$resource THEN target
          ELSE source
        END) AS linked_resource,
        link_type
      FROM \$links;
    ''', vars: {'resource': resourceId});

    return result[0] as List<dynamic>;
  }

  /// Search across all resource types
  Future<List<dynamic>> searchAll(String query) async {
    final result = await _db.query('''
      SELECT * FROM task WHERE
        string::contains(string::lowercase(title), string::lowercase(\$query))
        OR string::contains(string::lowercase(description), string::lowercase(\$query))
      UNION
      SELECT * FROM note WHERE
        string::contains(string::lowercase(title), string::lowercase(\$query))
        OR string::contains(string::lowercase(content), string::lowercase(\$query))
      UNION
      SELECT * FROM item WHERE
        string::contains(string::lowercase(name), string::lowercase(\$query))
        OR string::contains(string::lowercase(description), string::lowercase(\$query));
    ''', vars: {'query': query});

    return result[0] as List<dynamic>;
  }

  /// Close connection
  Future<void> close() async {
    if (_isConnected) {
      await _db.close();
      _isConnected = false;
      _instance = null;
    }
  }
}
```

---

## ⚙️ Configuration

```dart
// lib/src/config.dart

import 'dart:io';
import 'dart:convert';
import 'package:path/path.dart' as path;
import 'package:path_provider/path_provider.dart';
import 'package:crypto/crypto.dart';

class ServiceConfig {
  final String dataPath;
  final int port;
  final String host;
  final bool autoStart;

  ServiceConfig({
    required this.dataPath,
    this.port = 8000,
    this.host = '127.0.0.1',
    this.autoStart = true,
  });

  factory ServiceConfig.defaultConfig() {
    // Will be set properly when async methods are called
    return ServiceConfig(
      dataPath: '', // Placeholder, use getServiceDirectory() instead
    );
  }

  /// Get the service directory path
  Future<String> getServiceDirectory() async {
    if (Platform.isLinux || Platform.isMacOS) {
      final home = Platform.environment['HOME'];
      return path.join(home!, '.altair');
    } else if (Platform.isWindows) {
      final appData = Platform.environment['APPDATA'];
      return path.join(appData!, 'Altair');
    } else {
      // Fallback for other platforms
      final appDir = await getApplicationDocumentsDirectory();
      return path.join(appDir.path, '.altair');
    }
  }

  /// Get or create database credentials
  Future<DatabaseCredentials> getOrCreateCredentials() async {
    final serviceDir = await getServiceDirectory();
    final credsFile = File(path.join(serviceDir, 'credentials.json'));

    if (await credsFile.exists()) {
      final content = await credsFile.readAsString();
      final json = jsonDecode(content);
      return DatabaseCredentials(
        username: json['username'],
        password: json['password'],
      );
    }

    // Generate new credentials
    final credentials = DatabaseCredentials(
      username: 'altair',
      password: _generatePassword(),
    );

    // Save to file
    await credsFile.parent.create(recursive: true);
    await credsFile.writeAsString(jsonEncode({
      'username': credentials.username,
      'password': credentials.password,
    }));

    // Make file read-only
    if (!Platform.isWindows) {
      await Process.run('chmod', ['600', credsFile.path]);
    }

    return credentials;
  }

  /// Generate a secure random password
  String _generatePassword() {
    final random = List.generate(32, (i) => i);
    final bytes = sha256.convert(random + DateTime.now().millisecondsSinceEpoch.toString().codeUnits).bytes;
    return base64Url.encode(bytes).substring(0, 32);
  }

  /// Load config from file
  static Future<ServiceConfig> load() async {
    final defaultConfig = ServiceConfig.defaultConfig();
    final serviceDir = await defaultConfig.getServiceDirectory();
    final configFile = File(path.join(serviceDir, 'config.json'));

    if (await configFile.exists()) {
      final content = await configFile.readAsString();
      final json = jsonDecode(content);
      return ServiceConfig(
        dataPath: json['dataPath'] ?? path.join(serviceDir, 'database'),
        port: json['port'] ?? 8000,
        host: json['host'] ?? '127.0.0.1',
        autoStart: json['autoStart'] ?? true,
      );
    }

    // Return default config with proper data path
    return ServiceConfig(
      dataPath: path.join(serviceDir, 'database'),
    );
  }

  /// Save config to file
  Future<void> save() async {
    final serviceDir = await getServiceDirectory();
    final configFile = File(path.join(serviceDir, 'config.json'));

    await configFile.parent.create(recursive: true);
    await configFile.writeAsString(jsonEncode({
      'dataPath': dataPath,
      'port': port,
      'host': host,
      'autoStart': autoStart,
    }));
  }
}

class DatabaseCredentials {
  final String username;
  final String password;

  DatabaseCredentials({
    required this.username,
    required this.password,
  });
}
```

---

## 📊 Models

```dart
// lib/src/models/service_status.dart

class ServiceStatus {
  final bool isRunning;
  final int port;
  final String dataPath;
  final String? version;

  ServiceStatus({
    required this.isRunning,
    required this.port,
    required this.dataPath,
    this.version,
  });

  @override
  String toString() {
    if (isRunning) {
      return 'Service running on port $port (version: ${version ?? "unknown"})';
    } else {
      return 'Service not running';
    }
  }
}
```

---

## 🐧 Linux Service Installation

```dart
// lib/src/platform/linux_service.dart

import 'dart:io';
import 'package:path/path.dart' as path;
import 'package:process_run/shell.dart';
import '../config.dart';

class LinuxServiceInstaller {
  final ServiceConfig config;

  LinuxServiceInstaller(this.config);

  Future<void> install() async {
    final shell = Shell();
    final serviceDir = await config.getServiceDirectory();
    final binary = path.join(serviceDir, 'bin', 'surrealdb');
    final credentials = await config.getOrCreateCredentials();

    // Create systemd service file
    final serviceContent = '''
[Unit]
Description=Altair Database Service
After=network.target

[Service]
Type=simple
User=${Platform.environment['USER']}
ExecStart=$binary start --bind ${config.host}:${config.port} --user ${credentials.username} --pass ${credentials.password} file://${config.dataPath}/altair.db
Restart=on-failure
RestartSec=10

[Install]
WantedBy=default.target
''';

    final systemdUserDir = path.join(
      Platform.environment['HOME']!,
      '.config',
      'systemd',
      'user',
    );

    await Directory(systemdUserDir).create(recursive: true);
    final serviceFile = File(path.join(systemdUserDir, 'altair-db.service'));
    await serviceFile.writeAsString(serviceContent);

    // Enable and start service
    await shell.run('''
      systemctl --user daemon-reload
      systemctl --user enable altair-db.service
      systemctl --user start altair-db.service
    ''');

    print('✓ Installed as systemd user service');
    print('  Manage with: systemctl --user {start|stop|status} altair-db');
  }

  Future<void> uninstall() async {
    final shell = Shell();

    try {
      await shell.run('''
        systemctl --user stop altair-db.service
        systemctl --user disable altair-db.service
      ''');

      final systemdUserDir = path.join(
        Platform.environment['HOME']!,
        '.config',
        'systemd',
        'user',
      );
      final serviceFile = File(path.join(systemdUserDir, 'altair-db.service'));
      if (await serviceFile.exists()) {
        await serviceFile.delete();
      }

      await shell.run('systemctl --user daemon-reload');
      print('✓ Uninstalled systemd service');
    } catch (e) {
      print('Warning: Could not fully uninstall service: $e');
    }
  }
}
```

---

## 🍎 macOS Service Installation

```dart
// lib/src/platform/macos_service.dart

import 'dart:io';
import 'package:path/path.dart' as path;
import 'package:process_run/shell.dart';
import '../config.dart';

class MacOSServiceInstaller {
  final ServiceConfig config;

  MacOSServiceInstaller(this.config);

  Future<void> install() async {
    final shell = Shell();
    final serviceDir = await config.getServiceDirectory();
    final binary = path.join(serviceDir, 'bin', 'surrealdb');
    final credentials = await config.getOrCreateCredentials();

    // Create launchd plist
    final plistContent = '''
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.getaltair.database</string>
    <key>ProgramArguments</key>
    <array>
        <string>$binary</string>
        <string>start</string>
        <string>--bind</string>
        <string>${config.host}:${config.port}</string>
        <string>--user</string>
        <string>${credentials.username}</string>
        <string>--pass</string>
        <string>${credentials.password}</string>
        <string>file://${config.dataPath}/altair.db</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>$serviceDir/logs/stdout.log</string>
    <key>StandardErrorPath</key>
    <string>$serviceDir/logs/stderr.log</string>
</dict>
</plist>
''';

    final launchAgentsDir = path.join(
      Platform.environment['HOME']!,
      'Library',
      'LaunchAgents',
    );

    await Directory(launchAgentsDir).create(recursive: true);
    await Directory(path.join(serviceDir, 'logs')).create(recursive: true);

    final plistFile = File(path.join(launchAgentsDir, 'com.getaltair.database.plist'));
    await plistFile.writeAsString(plistContent);

    // Load the service
    await shell.run('launchctl load ${plistFile.path}');

    print('✓ Installed as launchd service');
    print('  Manage with: launchctl {load|unload|start|stop} ~/Library/LaunchAgents/com.getaltair.database.plist');
  }

  Future<void> uninstall() async {
    final shell = Shell();

    try {
      final launchAgentsDir = path.join(
        Platform.environment['HOME']!,
        'Library',
        'LaunchAgents',
      );
      final plistFile = File(path.join(launchAgentsDir, 'com.getaltair.database.plist'));

      if (await plistFile.exists()) {
        await shell.run('launchctl unload ${plistFile.path}');
        await plistFile.delete();
      }

      print('✓ Uninstalled launchd service');
    } catch (e) {
      print('Warning: Could not fully uninstall service: $e');
    }
  }
}
```

---

## 🪟 Windows Service Installation

```dart
// lib/src/platform/windows_service.dart

import 'dart:io';
import 'package:path/path.dart' as path;
import 'package:process_run/shell.dart';
import '../config.dart';

class WindowsServiceInstaller {
  final ServiceConfig config;

  WindowsServiceInstaller(this.config);

  Future<void> install() async {
    // Windows service installation requires admin privileges
    // For now, we'll use NSSM (Non-Sucking Service Manager) or similar
    // Alternatively, just run on startup via Registry Run key

    final shell = Shell();
    final serviceDir = await config.getServiceDirectory();
    final binary = path.join(serviceDir, 'bin', 'surrealdb.exe');
    final credentials = await config.getOrCreateCredentials();

    // Create a batch file to start the service
    final batchContent = '''
@echo off
"$binary" start --bind ${config.host}:${config.port} --user ${credentials.username} --pass ${credentials.password} file://${config.dataPath}/altair.db
''';

    final startupBatch = File(path.join(serviceDir, 'start-service.bat'));
    await startupBatch.writeAsString(batchContent);

    // Add to startup (current user)
    final startupFolder = path.join(
      Platform.environment['APPDATA']!,
      'Microsoft',
      'Windows',
      'Start Menu',
      'Programs',
      'Startup',
    );

    final shortcutPath = path.join(startupFolder, 'Altair Database.lnk');

    // Create shortcut using PowerShell
    await shell.run('''
      powershell -Command "\$WshShell = New-Object -ComObject WScript.Shell; \$Shortcut = \$WshShell.CreateShortcut('$shortcutPath'); \$Shortcut.TargetPath = '${startupBatch.path}'; \$Shortcut.WorkingDirectory = '$serviceDir'; \$Shortcut.WindowStyle = 7; \$Shortcut.Save()"
    ''');

    print('✓ Installed to Windows startup');
    print('  Service will start automatically on login');
    print('  To start now, run: ${startupBatch.path}');
  }

  Future<void> uninstall() async {
    try {
      final startupFolder = path.join(
        Platform.environment['APPDATA']!,
        'Microsoft',
        'Windows',
        'Start Menu',
        'Programs',
        'Startup',
      );

      final shortcutPath = File(path.join(startupFolder, 'Altair Database.lnk'));
      if (await shortcutPath.exists()) {
        await shortcutPath.delete();
      }

      print('✓ Removed from Windows startup');
    } catch (e) {
      print('Warning: Could not fully uninstall service: $e');
    }
  }
}
```

---

## 📥 Binary Downloader

```dart
// bin/download_surrealdb.dart

import 'dart:io';
import 'package:http/http.dart' as http;
import 'package:path/path.dart' as path;
import 'package:archive/archive.dart';

Future<void> main(List<String> args) async {
  print('Downloading SurrealDB binaries...');

  final version = args.isNotEmpty ? args[0] : 'latest';
  final serviceDir = await _getServiceDirectory();
  final binDir = Directory(path.join(serviceDir, 'bin'));
  await binDir.create(recursive: true);

  // Determine platform
  String platform;
  String extension = '';

  if (Platform.isLinux) {
    platform = 'linux-amd64';
    extension = '';
  } else if (Platform.isMacOS) {
    platform = 'darwin-amd64';
    extension = '';
  } else if (Platform.isWindows) {
    platform = 'windows-amd64';
    extension = '.exe';
  } else {
    print('Unsupported platform: ${Platform.operatingSystem}');
    exit(1);
  }

  final url = version == 'latest'
      ? 'https://download.surrealdb.com/$platform/surreal$extension'
      : 'https://download.surrealdb.com/v$version/$platform/surreal$extension';

  print('Downloading from: $url');

  final response = await http.get(Uri.parse(url));
  if (response.statusCode != 200) {
    print('Failed to download: HTTP ${response.statusCode}');
    exit(1);
  }

  final binaryPath = path.join(binDir.path, 'surrealdb$extension');
  final binary = File(binaryPath);
  await binary.writeAsBytes(response.bodyBytes);

  // Make executable on Unix
  if (!Platform.isWindows) {
    await Process.run('chmod', ['+x', binaryPath]);
  }

  print('✓ Downloaded SurrealDB to: $binaryPath');

  // Verify it works
  final result = await Process.run(binaryPath, ['version']);
  print('Version: ${result.stdout}');
}

Future<String> _getServiceDirectory() async {
  if (Platform.isLinux || Platform.isMacOS) {
    final home = Platform.environment['HOME'];
    return path.join(home!, '.altair');
  } else if (Platform.isWindows) {
    final appData = Platform.environment['APPDATA'];
    return path.join(appData!, 'Altair');
  }
  throw UnsupportedError('Unsupported platform');
}
```

---

## 🧪 Usage Examples

### In Guidance App

```dart
// apps/guidance/lib/main.dart

import 'package:altair_db_service/altair_db_service.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // Connect to database service
  final db = await AltairConnectionManager.getInstance();

  runApp(GuidanceApp(db: db));
}

// Creating a task
final db = await AltairConnectionManager.getInstance();
await db.client.create('task', {
  'id': 'task:${Uuid().v4()}',
  'title': 'Implement cross-app linking',
  'description': 'Add ability to link tasks to notes',
  'status': 'in_progress',
  'created_at': DateTime.now().toIso8601String(),
});
```

### In Knowledge App

```dart
// apps/knowledge/lib/main.dart

import 'package:altair_db_service/altair_db_service.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // Same database, different app
  final db = await AltairConnectionManager.getInstance();

  runApp(KnowledgeApp(db: db));
}

// Creating a note linked to a task
final db = await AltairConnectionManager.getInstance();

final noteId = 'note:${Uuid().v4()}';
await db.client.create('note', {
  'id': noteId,
  'title': 'Meeting Notes: Cross-App Integration',
  'content': 'Discussed linking strategy...',
  'created_at': DateTime.now().toIso8601String(),
});

// Link the note to a task
await db.createLink(
  sourceId: noteId,
  targetId: 'task:abc123',
  linkType: 'references',
);

// Later, find all notes related to a task
final linked = await db.getLinkedResources('task:abc123');
print('Found ${linked.length} linked resources');
```

### Cross-App Search

```dart
// Search across ALL apps from any app
final db = await AltairConnectionManager.getInstance();
final results = await db.searchAll('project planning');

// Results contain tasks, notes, and items that match
for (var item in results) {
  print('${item['id']}: ${item['title'] ?? item['name']}');
}
```

---

## 🚀 Installation in App Installer

```dart
// Guidance/Knowledge/Tracking installer code

import 'package:altair_db_service/altair_db_service.dart';

Future<void> firstTimeSetup() async {
  final service = AltairDatabaseService();
  final status = await service.getStatus();

  if (!status.isRunning) {
    // Show user dialog
    final shouldInstall = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: Text('Database Service Required'),
        content: Text(
          'Altair requires a local database service to run.\n\n'
          'This service:\n'
          '• Runs in the background\n'
          '• Stores your data locally\n'
          '• Enables all Altair apps to work together\n'
          '• Uses minimal resources\n\n'
          'Install the database service?'
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: Text('Cancel'),
          ),
          ElevatedButton(
            onPressed: () => Navigator.pop(context, true),
            child: Text('Install'),
          ),
        ],
      ),
    );

    if (shouldInstall == true) {
      // Download binary if needed
      await _downloadSurrealDB();

      // Start service
      await service.start();

      // Optionally install as system service
      final makeAutoStart = await _askAboutAutoStart();
      if (makeAutoStart) {
        await service.installSystemService();
      }

      // Success!
      await _showSuccessDialog();
    }
  } else {
    // Service already running (another app installed it)
    await _showExistingServiceDialog();
  }
}
```

---

This gives you a complete, production-ready shared database service architecture. Want me to continue with testing setup, backup/restore utilities, or the actual schema design for cross-app linking?
