import 'dart:io';
import 'package:process_run/shell.dart';
import 'service_installer.dart';

/// Windows service installer using startup shortcut
/// Note: For a proper Windows Service, additional native code would be needed
class WindowsServiceInstaller extends ServiceInstaller {
  WindowsServiceInstaller(super.config);

  String get serviceName => 'AltairDB';

  Future<String> get startupFolderPath async {
    final appData = Platform.environment['APPDATA'];
    if (appData == null) {
      throw StateError('APPDATA environment variable not set');
    }
    return '$appData\\Microsoft\\Windows\\Start Menu\\Programs\\Startup';
  }

  Future<String> get shortcutPath async {
    final startupFolder = await startupFolderPath;
    return '$startupFolder\\Altair Database.lnk';
  }

  Future<String> get batchFilePath async {
    final configDir = await config.getConfigDirectory();
    return '$configDir\\start-service.bat';
  }

  @override
  Future<bool> isInstalled() async {
    try {
      final path = await shortcutPath;
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
    final binaryPath = '$configDir\\surrealdb-windows.exe';

    // Get or generate credentials
    final credentials = await config.getOrGenerateCredentials();

    // Create batch file with environment variables for credentials
    // This prevents credentials from being visible in process listings
    final batchContent = '''@echo off
echo Starting Altair Database Service...
set SURREAL_USER=${credentials.username}
set SURREAL_PASS=${credentials.password}
"$binaryPath" start "file://$dataDir\\altair.db" --bind ${config.bindAddress}:${config.port} --auth
''';

    final batchPath = await batchFilePath;
    await File(batchPath).writeAsString(batchContent);

    // Set secure permissions on batch file (current user only)
    await Process.run('icacls', [
      batchPath,
      '/inheritance:r',
      '/grant:r',
      '${Platform.environment['USERNAME']}:(R,W)',
    ]);

    // Create shortcut using PowerShell
    final shortcut = await shortcutPath;
    final psCommand = '''
\$WshShell = New-Object -ComObject WScript.Shell
\$Shortcut = \$WshShell.CreateShortcut('$shortcut')
\$Shortcut.TargetPath = '$batchPath'
\$Shortcut.WorkingDirectory = '$configDir'
\$Shortcut.WindowStyle = 7
\$Shortcut.Save()
''';

    await shell.run('powershell -Command "$psCommand"');

    print('Service installed (will start on login)');
    print('Shortcut created at: $shortcut');
    print('Credentials stored securely');
  }

  @override
  Future<void> uninstall() async {
    // Stop the service first
    try {
      await stop();
    } catch (e) {
      // Service might not be running
    }

    // Remove shortcut
    final shortcut = await shortcutPath;
    final file = File(shortcut);
    if (await file.exists()) {
      await file.delete();
    }

    // Remove batch file
    final batchPath = await batchFilePath;
    final batchFile = File(batchPath);
    if (await batchFile.exists()) {
      await batchFile.delete();
    }

    print('Service uninstalled');
  }

  @override
  Future<void> start() async {
    final batchPath = await batchFilePath;
    if (!await File(batchPath).exists()) {
      throw StateError('Service not installed. Run install() first.');
    }

    // Start the batch file in the background
    await Process.start(
        'cmd',
        [
          '/c',
          'start',
          '/B',
          batchPath,
        ],
        mode: ProcessStartMode.detached);

    print('Service started');
  }

  @override
  Future<void> stop() async {
    final shell = Shell();

    // Kill any running SurrealDB processes
    try {
      await shell.run('taskkill /F /IM surrealdb-windows.exe');
      print('Service stopped');
    } catch (e) {
      // Process might not be running
    }
  }

  @override
  Future<void> enable() async {
    // On Windows, the shortcut in Startup folder automatically enables it
    print('Service is enabled (shortcut exists in Startup folder)');
  }

  @override
  Future<void> disable() async {
    // To disable, just remove the shortcut
    final shortcut = await shortcutPath;
    final file = File(shortcut);
    if (await file.exists()) {
      await file.delete();
    }
    print('Service disabled (removed from Startup folder)');
  }

  /// Get service status by checking if process is running
  Future<String> status() async {
    final shell = Shell();
    try {
      final result = await shell.run(
        'tasklist /FI "IMAGENAME eq surrealdb-windows.exe"',
      );
      final output = result.first.outText;
      if (output.contains('surrealdb-windows.exe')) {
        return 'Service is running';
      } else {
        return 'Service is not running';
      }
    } catch (e) {
      return 'Error checking service status: $e';
    }
  }
}
