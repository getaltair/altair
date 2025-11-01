import 'package:altair_db_service/altair_db_service.dart';
import 'package:uuid/uuid.dart';

import '../models/tag.dart';

/// SurrealDB-based repository for managing tags
class TagRepository {
  final Uuid _uuid = const Uuid();
  late final AltairConnectionManager _connectionManager;
  bool _initialized = false;

  /// Initialize the repository with database connection
  Future<void> initialize() async {
    if (_initialized) return;

    _connectionManager = await AltairConnectionManager.getInstance();
    _initialized = true;
  }

  /// Create a new tag
  Future<Tag> create(Tag tag) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    // Generate ID if not provided
    final id = tag.id.isEmpty ? 'tag:${_uuid.v4()}' : tag.id;

    final tagToInsert = tag.copyWith(id: id, createdAt: tag.createdAt);

    await db.create('tag', _tagToMap(tagToInsert));

    return tagToInsert;
  }

  /// Get a tag by ID
  Future<Tag?> findById(String id) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    final result = await db.query(
      '''
      SELECT * FROM \$tagId;
    ''',
      {'tagId': id},
    );

    if (result == null || result is! List) return null;
    if (result.isEmpty) return null;
    if (result[0] is! Map) return null;

    final responseMap = result[0] as Map<String, dynamic>;
    final data = responseMap['result'];

    if (data is! List || data.isEmpty) return null;

    return _tagFromMap(data[0] as Map<String, dynamic>);
  }

  /// Get a tag by name
  Future<Tag?> findByName(String name) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    final result = await db.query(
      '''
      SELECT * FROM tag
      WHERE name = \$name
      LIMIT 1;
    ''',
      {'name': name},
    );

    if (result == null || result is! List) return null;
    if (result.isEmpty) return null;
    if (result[0] is! Map) return null;

    final responseMap = result[0] as Map<String, dynamic>;
    final data = responseMap['result'];

    if (data is! List || data.isEmpty) return null;

    return _tagFromMap(data[0] as Map<String, dynamic>);
  }

  /// Get all tags
  Future<List<Tag>> findAll({int? limit, int? offset, String? orderBy}) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    final orderByClause = orderBy ?? 'usage_count DESC, name ASC';
    final limitClause = limit != null ? 'LIMIT $limit' : '';
    final startClause = offset != null ? 'START $offset' : '';

    final query =
        '''
      SELECT * FROM tag
      ORDER BY $orderByClause
      $startClause
      $limitClause;
    ''';

    final result = await db.query(query);

    if (result == null || result is! List) return [];
    if (result.isEmpty) return [];
    if (result[0] is! Map) return [];

    final responseMap = result[0] as Map<String, dynamic>;
    final data = responseMap['result'];

    if (data is! List) return [];

    return data
        .map((item) => _tagFromMap(item as Map<String, dynamic>))
        .toList();
  }

  /// Update a tag
  Future<Tag> update(Tag tag) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    await db.update(tag.id, _tagToMap(tag));

    return tag;
  }

  /// Delete a tag
  Future<void> delete(String id) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    await db.delete(id);
  }

  /// Search tags by name or description
  Future<List<Tag>> search(String query) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    final result = await db.query(
      '''
      SELECT * FROM tag
      WHERE name @@ \$query OR description @@ \$query
      ORDER BY usage_count DESC, name ASC
      LIMIT 50;
    ''',
      {'query': query},
    );

    if (result == null || result is! List) return [];
    if (result.isEmpty) return [];
    if (result[0] is! Map) return [];

    final responseMap = result[0] as Map<String, dynamic>;
    final data = responseMap['result'];

    if (data is! List) return [];

    return data
        .map((item) => _tagFromMap(item as Map<String, dynamic>))
        .toList();
  }

  /// Get the most used tags
  Future<List<Tag>> findMostUsed({int limit = 10}) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    final result = await db.query('''
      SELECT * FROM tag
      WHERE usage_count > 0
      ORDER BY usage_count DESC
      LIMIT $limit;
    ''');

    if (result == null || result is! List) return [];
    if (result.isEmpty) return [];
    if (result[0] is! Map) return [];

    final responseMap = result[0] as Map<String, dynamic>;
    final data = responseMap['result'];

    if (data is! List) return [];

    return data
        .map((item) => _tagFromMap(item as Map<String, dynamic>))
        .toList();
  }

  /// Increment the usage count of a tag
  Future<void> incrementUsageCount(String tagId) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    await db.query(
      '''
      UPDATE \$tagId SET usage_count = usage_count + 1;
    ''',
      {'tagId': tagId},
    );
  }

  /// Decrement the usage count of a tag
  Future<void> decrementUsageCount(String tagId) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    await db.query(
      '''
      UPDATE \$tagId SET usage_count = math::max(usage_count - 1, 0);
    ''',
      {'tagId': tagId},
    );
  }

  /// Get tags by IDs
  Future<List<Tag>> findByIds(List<String> ids) async {
    if (ids.isEmpty) return [];

    await _ensureInitialized();
    final db = _connectionManager.client;

    final result = await db.query(
      '''
      SELECT * FROM tag
      WHERE id INSIDE \$ids;
    ''',
      {'ids': ids},
    );

    if (result == null || result is! List) return [];
    if (result.isEmpty) return [];
    if (result[0] is! Map) return [];

    final responseMap = result[0] as Map<String, dynamic>;
    final data = responseMap['result'];

    if (data is! List) return [];

    return data
        .map((item) => _tagFromMap(item as Map<String, dynamic>))
        .toList();
  }

  /// Get count of all tags
  Future<int> count() async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    final result = await db.query('''
      SELECT count() as count FROM tag GROUP ALL;
    ''');

    if (result == null || result is! List) return 0;
    if (result.isEmpty) return 0;
    if (result[0] is! Map) return 0;

    final responseMap = result[0] as Map<String, dynamic>;
    final data = responseMap['result'];

    if (data is! List || data.isEmpty) return 0;

    final countMap = data[0] as Map<String, dynamic>;
    return (countMap['count'] as num?)?.toInt() ?? 0;
  }

  /// Ensure the repository is initialized
  Future<void> _ensureInitialized() async {
    if (!_initialized) {
      await initialize();
    }
  }

  /// Convert Tag to Map for database storage
  Map<String, dynamic> _tagToMap(Tag tag) {
    return {
      'id': tag.id,
      'name': tag.name,
      'description': tag.description,
      'color': tag.color,
      'created_at': tag.createdAt.toIso8601String(),
      'usage_count': tag.usageCount,
    };
  }

  /// Convert Map from database to Tag
  Tag _tagFromMap(Map<String, dynamic> map) {
    return Tag(
      id: map['id'] as String,
      name: map['name'] as String,
      description: map['description'] as String?,
      color: map['color'] as String?,
      createdAt: DateTime.parse(map['created_at'] as String),
      usageCount: (map['usage_count'] as num?)?.toInt() ?? 0,
    );
  }
}
