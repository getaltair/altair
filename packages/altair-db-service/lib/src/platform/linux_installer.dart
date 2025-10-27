import 'dart:io';
import 'package:process_run/shell.dart';
import 'service_installer.dart';

/// Linux service installer using systemd (user service)
class LinuxServiceInstaller extends ServiceInstaller {
  LinuxServiceInstaller(super.config);

  String get serviceName => 'altair-db';
  String get serviceFileName => '$serviceName.service';

  Future<String> get serviceFilePath async {
    final home = Platform.environment['HOME'];
    if (home == null) {
      throw StateError('HOME environment variable not set');
    }
    return '$home/.config/systemd/user/$serviceFileName';
  }

  @override
  Future<bool> isInstalled() async {
    try {
      final path = await serviceFilePath;
      return await File(path).exists();
    } catch (e) {
      return false;
    }
  }

  @override
  Future<void> install() async {
    final shell = Shell();
    final dataDir = await config.getDataDirectory();
    final configDir = await config.getConfigDirectory();
    final binaryPath = '$configDir/surrealdb-linux';

    // Ensure systemd user directory exists
    final home = Platform.environment['HOME'];
    final systemdDir = Directory('$home/.config/systemd/user');
    if (!await systemdDir.exists()) {
      await systemdDir.create(recursive: true);
    }

    // Generate service file
    final serviceContent = '''
[Unit]
Description=Altair Database Service
After=network.target

[Service]
Type=simple
ExecStart=$binaryPath start file://$dataDir/altair.db --bind ${config.bindAddress}:${config.port} --user ${config.username} --pass ${config.password ?? 'altair-local-dev'}
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=default.target
''';

    // Write service file
    final servicePath = await serviceFilePath;
    await File(servicePath).writeAsString(serviceContent);

    // Reload systemd daemon
    await shell.run('systemctl --user daemon-reload');

    print('Service installed at $servicePath');
  }

  @override
  Future<void> uninstall() async {
    // Stop and disable first
    try {
      await stop();
      await disable();
    } catch (e) {
      // Service might not be running
    }

    // Remove service file
    final servicePath = await serviceFilePath;
    final file = File(servicePath);
    if (await file.exists()) {
      await file.delete();
    }

    // Reload daemon
    final shell = Shell();
    await shell.run('systemctl --user daemon-reload');

    print('Service uninstalled');
  }

  @override
  Future<void> start() async {
    final shell = Shell();
    await shell.run('systemctl --user start $serviceName');
    print('Service started');
  }

  @override
  Future<void> stop() async {
    final shell = Shell();
    await shell.run('systemctl --user stop $serviceName');
    print('Service stopped');
  }

  @override
  Future<void> enable() async {
    final shell = Shell();
    await shell.run('systemctl --user enable $serviceName');
    print('Service enabled (will start on boot)');
  }

  @override
  Future<void> disable() async {
    final shell = Shell();
    await shell.run('systemctl --user disable $serviceName');
    print('Service disabled (will not start on boot)');
  }

  /// Get service status
  Future<String> status() async {
    final shell = Shell();
    try {
      final result = await shell.run('systemctl --user status $serviceName');
      return result.first.outText;
    } catch (e) {
      return 'Service not found or error: $e';
    }
  }
}
