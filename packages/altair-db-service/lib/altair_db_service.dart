/// Altair Database Service
///
/// Manages a shared local SurrealDB instance for all Altair applications.
/// Handles service lifecycle, connections, and cross-app data integration.
library altair_db_service;

export 'src/service_manager.dart';
export 'src/connection_manager.dart';
export 'src/config.dart';
export 'src/models/service_status.dart';
export 'src/platform/service_installer.dart';
export 'src/platform/linux_installer.dart';
export 'src/platform/macos_installer.dart';
export 'src/platform/windows_installer.dart';
export 'src/queries.dart';
export 'src/security/credential_manager.dart';
