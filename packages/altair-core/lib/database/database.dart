import 'dart:io';

import 'package:path/path.dart';
import 'package:path_provider/path_provider.dart';
import 'package:sqflite_common_ffi/sqflite_ffi.dart';

import 'schema.dart';

/// Database helper for Altair local storage
///
/// This class manages the SQLite database instance and provides
/// methods for initializing, upgrading, and accessing the database.
class AltairDatabase {
  static AltairDatabase? _instance;
  static Database? _database;
  static bool _isTestMode = false;

  AltairDatabase._internal();

  /// Get the singleton instance of AltairDatabase
  factory AltairDatabase() {
    _instance ??= AltairDatabase._internal();
    return _instance!;
  }

  /// Enable test mode (uses in-memory database)
  /// This must be called before any database operations in tests
  static void enableTestMode() {
    _isTestMode = true;
    _database = null;
    _instance = null;
  }

  /// Reset the database instance (useful for testing)
  static void reset() {
    _database = null;
    _instance = null;
    _isTestMode = false;
  }

  /// Get the database instance
  Future<Database> get database async {
    if (_database != null) return _database!;

    _database = await _initDatabase();
    return _database!;
  }

  /// Initialize the database
  Future<Database> _initDatabase() async {
    // Initialize FFI for desktop platforms or test mode
    if (_isTestMode || Platform.isLinux || Platform.isWindows || Platform.isMacOS) {
      sqfliteFfiInit();
      databaseFactory = databaseFactoryFfi;
    }

    // Use in-memory database for tests
    if (_isTestMode) {
      return await openDatabase(
        inMemoryDatabasePath,
        version: schemaVersion,
        onCreate: _onCreate,
        onUpgrade: _onUpgrade,
        onConfigure: _onConfigure,
      );
    }

    // Get the database path for production
    final Directory appDocDir = await getApplicationDocumentsDirectory();
    final String dbPath = join(appDocDir.path, 'altair', 'guidance.db');

    // Ensure directory exists
    await Directory(dirname(dbPath)).create(recursive: true);

    // Open the database
    return await openDatabase(
      dbPath,
      version: schemaVersion,
      onCreate: _onCreate,
      onUpgrade: _onUpgrade,
      onConfigure: _onConfigure,
    );
  }

  /// Configure database settings
  Future<void> _onConfigure(Database db) async {
    // Enable foreign keys
    await db.execute('PRAGMA foreign_keys = ON');
  }

  /// Create database tables
  Future<void> _onCreate(Database db, int version) async {
    // Execute all create table statements
    for (final statement in createTableStatements) {
      await db.execute(statement);
    }
  }

  /// Upgrade database schema
  Future<void> _onUpgrade(Database db, int oldVersion, int newVersion) async {
    // Apply migrations sequentially
    for (int version = oldVersion + 1; version <= newVersion; version++) {
      final migrationStatements = migrations[version];
      if (migrationStatements != null) {
        for (final statement in migrationStatements) {
          await db.execute(statement);
        }
      }
    }
  }

  /// Close the database
  Future<void> close() async {
    final db = await database;
    await db.close();
    _database = null;
  }

  /// Delete the database (useful for testing)
  Future<void> deleteDatabase() async {
    final Directory appDocDir = await getApplicationDocumentsDirectory();
    final String dbPath = join(appDocDir.path, 'altair', 'guidance.db');
    await File(dbPath).delete();
    _database = null;
  }
}
