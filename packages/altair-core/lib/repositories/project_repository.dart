import 'dart:convert';

import 'package:sqflite/sqflite.dart';
import 'package:uuid/uuid.dart';

import '../database/database.dart';
import '../models/project.dart';

/// Repository for managing projects in the database
class ProjectRepository {
  final AltairDatabase _db = AltairDatabase();
  final Uuid _uuid = const Uuid();

  /// Create a new project
  Future<Project> create(Project project) async {
    final db = await _db.database;
    final now = DateTime.now();

    // Generate ID if not provided
    final id = project.id.isEmpty ? _uuid.v4() : project.id;

    final projectToInsert = project.copyWith(
      id: id,
      createdAt: project.createdAt,
      updatedAt: now,
    );

    await db.insert(
      'projects',
      _projectToMap(projectToInsert),
      conflictAlgorithm: ConflictAlgorithm.replace,
    );

    return projectToInsert;
  }

  /// Get a project by ID
  Future<Project?> findById(String id) async {
    final db = await _db.database;

    final List<Map<String, dynamic>> results = await db.query(
      'projects',
      where: 'id = ?',
      whereArgs: [id],
      limit: 1,
    );

    if (results.isEmpty) return null;

    return _projectFromMap(results.first);
  }

  /// Get all projects
  Future<List<Project>> findAll({
    ProjectStatus? status,
    List<String>? tags,
    int? limit,
    int? offset,
  }) async {
    final db = await _db.database;

    // Build query
    final where = <String>[];
    final whereArgs = <dynamic>[];

    if (status != null) {
      where.add('status = ?');
      whereArgs.add(status.name);
    }

    final results = await db.query(
      'projects',
      where: where.isEmpty ? null : where.join(' AND '),
      whereArgs: whereArgs.isEmpty ? null : whereArgs,
      orderBy: 'created_at DESC',
      limit: limit,
      offset: offset,
    );

    return results.map(_projectFromMap).toList();
  }

  /// Update a project
  Future<Project> update(Project project) async {
    final db = await _db.database;

    final projectToUpdate = project.copyWith(
      updatedAt: DateTime.now(),
    );

    await db.update(
      'projects',
      _projectToMap(projectToUpdate),
      where: 'id = ?',
      whereArgs: [project.id],
    );

    return projectToUpdate;
  }

  /// Delete a project
  /// Note: This will cascade delete all tasks in the project
  Future<void> delete(String id) async {
    final db = await _db.database;

    await db.delete(
      'projects',
      where: 'id = ?',
      whereArgs: [id],
    );
  }

  /// Search projects by name or description
  Future<List<Project>> search(String query) async {
    final db = await _db.database;

    final results = await db.query(
      'projects',
      where: 'name LIKE ? OR description LIKE ?',
      whereArgs: ['%$query%', '%$query%'],
      orderBy: 'created_at DESC',
    );

    return results.map(_projectFromMap).toList();
  }

  /// Get count of tasks in a project
  Future<int> getTaskCount(String projectId) async {
    final db = await _db.database;

    final result = await db.rawQuery(
      'SELECT COUNT(*) as count FROM tasks WHERE project_id = ?',
      [projectId],
    );

    return (result.first['count'] as int?) ?? 0;
  }

  /// Convert Project to Map for database storage
  Map<String, dynamic> _projectToMap(Project project) {
    return {
      'id': project.id,
      'name': project.name,
      'description': project.description,
      'status': project.status.name,
      'tags': jsonEncode(project.tags),
      'color': project.color,
      'created_at': project.createdAt.millisecondsSinceEpoch,
      'updated_at': project.updatedAt.millisecondsSinceEpoch,
      'target_date': project.targetDate?.millisecondsSinceEpoch,
      'completed_at': project.completedAt?.millisecondsSinceEpoch,
      'metadata': project.metadata != null ? jsonEncode(project.metadata) : null,
    };
  }

  /// Convert Map from database to Project
  Project _projectFromMap(Map<String, dynamic> map) {
    return Project(
      id: map['id'] as String,
      name: map['name'] as String,
      description: map['description'] as String?,
      status: ProjectStatus.values.byName(map['status'] as String),
      tags: (jsonDecode(map['tags'] as String) as List<dynamic>)
          .map((e) => e as String)
          .toList(),
      color: map['color'] as String?,
      createdAt: DateTime.fromMillisecondsSinceEpoch(map['created_at'] as int),
      updatedAt: DateTime.fromMillisecondsSinceEpoch(map['updated_at'] as int),
      targetDate: map['target_date'] != null
          ? DateTime.fromMillisecondsSinceEpoch(map['target_date'] as int)
          : null,
      completedAt: map['completed_at'] != null
          ? DateTime.fromMillisecondsSinceEpoch(map['completed_at'] as int)
          : null,
      metadata: map['metadata'] != null
          ? jsonDecode(map['metadata'] as String) as Map<String, dynamic>
          : null,
    );
  }
}
