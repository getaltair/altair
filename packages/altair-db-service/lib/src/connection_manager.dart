import 'package:surrealdb/surrealdb.dart';
import 'service_manager.dart';
import 'config.dart';

/// Manages database connections for Altair applications
///
/// This is a singleton that ensures all apps connect to the same database
/// instance and share data seamlessly.
class AltairConnectionManager {
  static AltairConnectionManager? _instance;
  late SurrealDB _db;
  bool _isConnected = false;

  static const String namespace = 'altair';
  static const String database = 'local';

  AltairConnectionManager._();

  /// Get the singleton instance
  static Future<AltairConnectionManager> getInstance({
    AltairDatabaseConfig? config,
  }) async {
    if (_instance == null) {
      _instance = AltairConnectionManager._();
      await _instance!._connect(config ?? AltairDatabaseConfig.defaultConfig);
    }
    return _instance!;
  }

  /// Internal connection setup
  Future<void> _connect(AltairDatabaseConfig config) async {
    final service = AltairDatabaseService(config: config);

    // Check if service is running, start if needed
    if (!await service.isRunning()) {
      print('Database service not running, starting...');
      try {
        await service.start();
      } catch (e) {
        throw Exception('Failed to start database service: $e');
      }
    }

    // Connect to the service
    _db = await service.getConnection();
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
      DEFINE TABLE IF NOT EXISTS project SCHEMALESS;
      DEFINE TABLE IF NOT EXISTS note SCHEMALESS;
      DEFINE TABLE IF NOT EXISTS item SCHEMALESS;
      DEFINE TABLE IF NOT EXISTS location SCHEMALESS;

      -- Cross-app linking
      DEFINE TABLE IF NOT EXISTS link SCHEMALESS;
      DEFINE FIELD IF NOT EXISTS source ON link TYPE record;
      DEFINE FIELD IF NOT EXISTS target ON link TYPE record;
      DEFINE FIELD IF NOT EXISTS link_type ON link TYPE string;
      DEFINE FIELD IF NOT EXISTS metadata ON link TYPE option<object>;
      DEFINE FIELD IF NOT EXISTS created_at ON link TYPE datetime DEFAULT time::now();
      DEFINE FIELD IF NOT EXISTS created_by ON link TYPE option<string>;

      -- Indexes for performance
      DEFINE INDEX IF NOT EXISTS link_source_idx ON link FIELDS source;
      DEFINE INDEX IF NOT EXISTS link_target_idx ON link FIELDS target;
      DEFINE INDEX IF NOT EXISTS link_type_idx ON link FIELDS link_type;

      -- Prevent duplicate links
      DEFINE INDEX IF NOT EXISTS link_unique ON link FIELDS source, target, link_type UNIQUE;

      -- Full-text search
      DEFINE ANALYZER IF NOT EXISTS altair_text TOKENIZERS blank, class FILTERS lowercase, ascii;
      DEFINE INDEX IF NOT EXISTS task_search ON task FIELDS title, description SEARCH ANALYZER altair_text BM25;
      DEFINE INDEX IF NOT EXISTS note_search ON note FIELDS title, content SEARCH ANALYZER altair_text BM25;
      DEFINE INDEX IF NOT EXISTS item_search ON item FIELDS name, description SEARCH ANALYZER altair_text BM25;
    ''');
  }

  /// Get the raw SurrealDB client for advanced queries
  SurrealDB get client {
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
    Map<String, dynamic>? metadata,
    String? createdBy,
  }) async {
    await _db.create('link', {
      'source': sourceId,
      'target': targetId,
      'link_type': linkType,
      'metadata': metadata,
      'created_at': DateTime.now().toIso8601String(),
      'created_by': createdBy,
    });
  }

  /// Remove a link between two resources
  Future<void> removeLink({
    required String sourceId,
    required String targetId,
    required String linkType,
  }) async {
    await _db.query('''
      DELETE FROM link
      WHERE source = \$source
        AND target = \$target
        AND link_type = \$linkType
    ''', {
      'source': sourceId,
      'target': targetId,
      'linkType': linkType,
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
        link_type,
        metadata,
        created_at
      FROM \$links;
    ''', {'resource': resourceId});

    if (result == null) return [];
    return (result as List).isNotEmpty ? (result[0] as List) : [];
  }

  /// Search across all resource types
  Future<List<dynamic>> searchAll(String query) async {
    final result = await _db.query('''
      SELECT
        id,
        title,
        description,
        'task' AS resource_type,
        created_at
      FROM task
      WHERE title @@ \$query OR description @@ \$query

      UNION

      SELECT
        id,
        title,
        content AS description,
        'note' AS resource_type,
        created_at
      FROM note
      WHERE title @@ \$query OR content @@ \$query

      UNION

      SELECT
        id,
        name AS title,
        description,
        'item' AS resource_type,
        created_at
      FROM item
      WHERE name @@ \$query OR description @@ \$query

      ORDER BY created_at DESC
      LIMIT 50;
    ''', {'query': query});

    if (result == null) return [];
    return (result as List).isNotEmpty ? (result[0] as List) : [];
  }

  /// Close connection
  Future<void> close() async {
    if (_isConnected) {
      _db.close();
      _isConnected = false;
      _instance = null;
    }
  }
}
