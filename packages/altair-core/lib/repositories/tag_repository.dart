import 'package:sqflite/sqflite.dart';
import 'package:uuid/uuid.dart';

import '../database/database.dart';
import '../models/tag.dart';

/// Repository for managing tags in the database
class TagRepository {
  final AltairDatabase _db = AltairDatabase();
  final Uuid _uuid = const Uuid();

  /// Create a new tag
  Future<Tag> create(Tag tag) async {
    final db = await _db.database;
    final now = DateTime.now();

    // Generate ID if not provided
    final id = tag.id.isEmpty ? _uuid.v4() : tag.id;

    final tagToInsert = tag.copyWith(
      id: id,
      createdAt: tag.createdAt,
    );

    await db.insert(
      'tags',
      _tagToMap(tagToInsert),
      conflictAlgorithm: ConflictAlgorithm.replace,
    );

    return tagToInsert;
  }

  /// Get a tag by ID
  Future<Tag?> findById(String id) async {
    final db = await _db.database;

    final List<Map<String, dynamic>> results = await db.query(
      'tags',
      where: 'id = ?',
      whereArgs: [id],
      limit: 1,
    );

    if (results.isEmpty) return null;

    return _tagFromMap(results.first);
  }

  /// Get a tag by name
  Future<Tag?> findByName(String name) async {
    final db = await _db.database;

    final List<Map<String, dynamic>> results = await db.query(
      'tags',
      where: 'name = ?',
      whereArgs: [name],
      limit: 1,
    );

    if (results.isEmpty) return null;

    return _tagFromMap(results.first);
  }

  /// Get all tags
  Future<List<Tag>> findAll({
    int? limit,
    int? offset,
    String? orderBy,
  }) async {
    final db = await _db.database;

    final results = await db.query(
      'tags',
      orderBy: orderBy ?? 'usage_count DESC, name ASC',
      limit: limit,
      offset: offset,
    );

    return results.map(_tagFromMap).toList();
  }

  /// Update a tag
  Future<Tag> update(Tag tag) async {
    final db = await _db.database;

    await db.update(
      'tags',
      _tagToMap(tag),
      where: 'id = ?',
      whereArgs: [tag.id],
    );

    return tag;
  }

  /// Delete a tag
  Future<void> delete(String id) async {
    final db = await _db.database;

    await db.delete(
      'tags',
      where: 'id = ?',
      whereArgs: [id],
    );
  }

  /// Search tags by name or description
  Future<List<Tag>> search(String query) async {
    final db = await _db.database;

    final results = await db.query(
      'tags',
      where: 'name LIKE ? OR description LIKE ?',
      whereArgs: ['%$query%', '%$query%'],
      orderBy: 'usage_count DESC, name ASC',
    );

    return results.map(_tagFromMap).toList();
  }

  /// Get the most used tags
  Future<List<Tag>> findMostUsed({int limit = 10}) async {
    final db = await _db.database;

    final results = await db.query(
      'tags',
      where: 'usage_count > 0',
      orderBy: 'usage_count DESC',
      limit: limit,
    );

    return results.map(_tagFromMap).toList();
  }

  /// Increment the usage count of a tag
  Future<void> incrementUsageCount(String tagId) async {
    final db = await _db.database;

    await db.rawUpdate(
      'UPDATE tags SET usage_count = usage_count + 1 WHERE id = ?',
      [tagId],
    );
  }

  /// Decrement the usage count of a tag
  Future<void> decrementUsageCount(String tagId) async {
    final db = await _db.database;

    await db.rawUpdate(
      'UPDATE tags SET usage_count = CASE WHEN usage_count > 0 THEN usage_count - 1 ELSE 0 END WHERE id = ?',
      [tagId],
    );
  }

  /// Get tags by IDs
  Future<List<Tag>> findByIds(List<String> ids) async {
    if (ids.isEmpty) return [];

    final db = await _db.database;
    final placeholders = List.filled(ids.length, '?').join(', ');

    final results = await db.query(
      'tags',
      where: 'id IN ($placeholders)',
      whereArgs: ids,
    );

    return results.map(_tagFromMap).toList();
  }

  /// Get count of all tags
  Future<int> count() async {
    final db = await _db.database;
    final result = await db.rawQuery('SELECT COUNT(*) as count FROM tags');
    return (result.first['count'] as int?) ?? 0;
  }

  /// Convert Tag to Map for database storage
  Map<String, dynamic> _tagToMap(Tag tag) {
    return {
      'id': tag.id,
      'name': tag.name,
      'description': tag.description,
      'color': tag.color,
      'created_at': tag.createdAt.millisecondsSinceEpoch,
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
      createdAt: DateTime.fromMillisecondsSinceEpoch(map['created_at'] as int),
      usageCount: map['usage_count'] as int,
    );
  }
}
