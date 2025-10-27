import 'dart:io';
import 'package:process_run/shell.dart';
import 'service_installer.dart';

/// macOS service installer using launchd
class MacOSServiceInstaller extends ServiceInstaller {
  MacOSServiceInstaller(super.config);

  String get serviceName => 'com.getaltair.database';
  String get plistFileName => '$serviceName.plist';

  Future<String> get plistFilePath async {
    final home = Platform.environment['HOME'];
    if (home == null) {
      throw StateError('HOME environment variable not set');
    }
    return '$home/Library/LaunchAgents/$plistFileName';
  }

  @override
  Future<bool> isInstalled() async {
    try {
      final path = await plistFilePath;
      return await File(path).exists();
    } catch (e) {
      return false;
    }
  }

  @override
  Future<void> install() async {
    final dataDir = await config.getDataDirectory();
    final configDir = await config.getConfigDirectory();
    final binaryPath = '$configDir/surrealdb-macos';
    final home = Platform.environment['HOME'];

    // Ensure LaunchAgents directory exists
    final launchAgentsDir = Directory('$home/Library/LaunchAgents');
    if (!await launchAgentsDir.exists()) {
      await launchAgentsDir.create(recursive: true);
    }

    // Create logs directory
    final logsDir = Directory('$configDir/logs');
    if (!await logsDir.exists()) {
      await logsDir.create(recursive: true);
    }

    // Generate plist file
    final plistContent = '''<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>$serviceName</string>
    <key>ProgramArguments</key>
    <array>
        <string>$binaryPath</string>
        <string>start</string>
        <string>file://$dataDir/altair.db</string>
        <string>--bind</string>
        <string>${config.bindAddress}:${config.port}</string>
        <string>--user</string>
        <string>${config.username}</string>
        <string>--pass</string>
        <string>${config.password ?? 'altair-local-dev'}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>$configDir/logs/stdout.log</string>
    <key>StandardErrorPath</key>
    <string>$configDir/logs/stderr.log</string>
</dict>
</plist>
''';

    // Write plist file
    final plistPath = await plistFilePath;
    await File(plistPath).writeAsString(plistContent);

    print('Service installed at $plistPath');
  }

  @override
  Future<void> uninstall() async {
    // Stop and unload first
    try {
      await stop();
    } catch (e) {
      // Service might not be running
    }

    // Remove plist file
    final plistPath = await plistFilePath;
    final file = File(plistPath);
    if (await file.exists()) {
      await file.delete();
    }

    print('Service uninstalled');
  }

  @override
  Future<void> start() async {
    final shell = Shell();
    final plistPath = await plistFilePath;
    await shell.run('launchctl load $plistPath');
    print('Service started');
  }

  @override
  Future<void> stop() async {
    final shell = Shell();
    final plistPath = await plistFilePath;
    try {
      await shell.run('launchctl unload $plistPath');
      print('Service stopped');
    } catch (e) {
      // Service might not be loaded
    }
  }

  @override
  Future<void> enable() async {
    // On macOS, installing to LaunchAgents with RunAtLoad=true automatically enables it
    print('Service is enabled (RunAtLoad is set to true in plist)');
  }

  @override
  Future<void> disable() async {
    // To disable, we need to modify the plist and remove RunAtLoad
    final plistPath = await plistFilePath;
    final file = File(plistPath);

    if (await file.exists()) {
      var content = await file.readAsString();
      content = content.replaceAll(
        '<key>RunAtLoad</key>\n    <true/>',
        '<key>RunAtLoad</key>\n    <false/>',
      );
      await file.writeAsString(content);

      // Reload the service
      try {
        await restart();
      } catch (e) {
        // Service might not be running
      }

      print('Service disabled (will not start on boot)');
    }
  }

  /// Get service status
  Future<String> status() async {
    final shell = Shell();
    try {
      final result = await shell.run('launchctl list | grep $serviceName');
      return result.first.outText;
    } catch (e) {
      return 'Service not found or not running';
    }
  }
}
