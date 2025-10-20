/// Project repository for CRUD operations and queries.
///
/// Handles all database interactions for [Project] entities including:
/// - Creating new projects
/// - Retrieving projects by ID or filters
/// - Updating existing projects
/// - Deleting projects
/// - Searching projects by name/description
/// - Counting tasks in projects
library;

import 'dart:convert';

import 'package:sqflite/sqflite.dart';
import 'package:uuid/uuid.dart';

import '../database/database.dart';
import '../models/project.dart';

/// Repository for managing projects in the database.
///
/// Provides CRUD operations and query methods for [Project] entities.
/// All operations are async and interact with the SQLite database.
class ProjectRepository {
  final AltairDatabase _db = AltairDatabase();
  final Uuid _uuid = const Uuid();

  /// Creates a new project in the database.
  ///
  /// Generates a UUID for the project if [project.id] is empty.
  /// Updates the [updatedAt] timestamp to the current time.
  ///
  /// Returns the created [Project] with its generated ID.
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

  /// Retrieves a project by its unique identifier.
  ///
  /// Returns the [Project] if found, or `null` if no project exists with the given [id].
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

  /// Retrieves all projects, optionally filtered by criteria.
  ///
  /// Optional filters:
  /// - [status]: Filter by project status (active, onHold, completed, cancelled)
  /// - [tags]: Filter by tags (currently not implemented in query)
  /// - [limit]: Maximum number of results to return
  /// - [offset]: Number of results to skip (for pagination)
  ///
  /// Results are ordered by creation date (newest first).
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

  /// Updates an existing project in the database.
  ///
  /// Automatically updates the [updatedAt] timestamp to the current time.
  ///
  /// Returns the updated [Project] with the new timestamp.
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

  /// Deletes a project from the database.
  ///
  /// **Warning**: This will cascade delete all tasks associated with this project
  /// due to the foreign key constraint in the database schema.
  ///
  /// The [id] parameter specifies which project to delete.
  Future<void> delete(String id) async {
    final db = await _db.database;

    await db.delete(
      'projects',
      where: 'id = ?',
      whereArgs: [id],
    );
  }

  /// Searches for projects by name or description.
  ///
  /// Performs a case-insensitive substring search across both project name
  /// and description fields. Results are ordered by creation date (newest first).
  ///
  /// The [query] parameter is the search term to match against.
  /// Returns a list of matching projects, or an empty list if no matches found.
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

  /// Retrieves the count of tasks associated with a project.
  ///
  /// Counts all tasks where `project_id` matches the given [projectId].
  ///
  /// Returns the number of tasks in the project, or 0 if the project has no tasks.
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
