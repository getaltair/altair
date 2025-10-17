import 'dart:convert';

import 'package:sqflite/sqflite.dart';
import 'package:uuid/uuid.dart';

import '../database/database.dart';
import '../models/task.dart';

/// Repository for managing tasks in the database
class TaskRepository {
  final AltairDatabase _db = AltairDatabase();
  final Uuid _uuid = const Uuid();

  /// Create a new task
  Future<Task> create(Task task) async {
    final db = await _db.database;
    final now = DateTime.now();

    // Generate ID if not provided
    final id = task.id.isEmpty ? _uuid.v4() : task.id;

    final taskToInsert = task.copyWith(
      id: id,
      createdAt: task.createdAt,
      updatedAt: now,
    );

    await db.insert(
      'tasks',
      _taskToMap(taskToInsert),
      conflictAlgorithm: ConflictAlgorithm.replace,
    );

    return taskToInsert;
  }

  /// Get a task by ID
  Future<Task?> findById(String id) async {
    final db = await _db.database;

    final List<Map<String, dynamic>> results = await db.query(
      'tasks',
      where: 'id = ?',
      whereArgs: [id],
      limit: 1,
    );

    if (results.isEmpty) return null;

    return _taskFromMap(results.first);
  }

  /// Get all tasks
  Future<List<Task>> findAll({
    TaskStatus? status,
    String? projectId,
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

    if (projectId != null) {
      where.add('project_id = ?');
      whereArgs.add(projectId);
    }

    final results = await db.query(
      'tasks',
      where: where.isEmpty ? null : where.join(' AND '),
      whereArgs: whereArgs.isEmpty ? null : whereArgs,
      orderBy: 'created_at DESC',
      limit: limit,
      offset: offset,
    );

    return results.map(_taskFromMap).toList();
  }

  /// Update a task
  Future<Task> update(Task task) async {
    final db = await _db.database;

    final taskToUpdate = task.copyWith(
      updatedAt: DateTime.now(),
    );

    await db.update(
      'tasks',
      _taskToMap(taskToUpdate),
      where: 'id = ?',
      whereArgs: [task.id],
    );

    return taskToUpdate;
  }

  /// Delete a task
  Future<void> delete(String id) async {
    final db = await _db.database;

    await db.delete(
      'tasks',
      where: 'id = ?',
      whereArgs: [id],
    );
  }

  /// Search tasks by title or description
  Future<List<Task>> search(String query) async {
    final db = await _db.database;

    final results = await db.query(
      'tasks',
      where: 'title LIKE ? OR description LIKE ?',
      whereArgs: ['%$query%', '%$query%'],
      orderBy: 'created_at DESC',
    );

    return results.map(_taskFromMap).toList();
  }

  /// Get subtasks for a parent task
  Future<List<Task>> findSubtasks(String parentTaskId) async {
    final db = await _db.database;

    final results = await db.query(
      'tasks',
      where: 'parent_task_id = ?',
      whereArgs: [parentTaskId],
      orderBy: 'created_at ASC',
    );

    return results.map(_taskFromMap).toList();
  }

  /// Convert Task to Map for database storage
  Map<String, dynamic> _taskToMap(Task task) {
    return {
      'id': task.id,
      'title': task.title,
      'description': task.description,
      'status': task.status.name,
      'tags': jsonEncode(task.tags),
      'project_id': task.projectId,
      'parent_task_id': task.parentTaskId,
      'created_at': task.createdAt.millisecondsSinceEpoch,
      'updated_at': task.updatedAt.millisecondsSinceEpoch,
      'completed_at': task.completedAt?.millisecondsSinceEpoch,
      'estimated_minutes': task.estimatedMinutes,
      'actual_minutes': task.actualMinutes,
      'priority': task.priority,
      'metadata': task.metadata != null ? jsonEncode(task.metadata) : null,
    };
  }

  /// Convert Map from database to Task
  Task _taskFromMap(Map<String, dynamic> map) {
    return Task(
      id: map['id'] as String,
      title: map['title'] as String,
      description: map['description'] as String?,
      status: TaskStatus.values.byName(map['status'] as String),
      tags: (jsonDecode(map['tags'] as String) as List<dynamic>)
          .map((e) => e as String)
          .toList(),
      projectId: map['project_id'] as String?,
      parentTaskId: map['parent_task_id'] as String?,
      createdAt: DateTime.fromMillisecondsSinceEpoch(map['created_at'] as int),
      updatedAt: DateTime.fromMillisecondsSinceEpoch(map['updated_at'] as int),
      completedAt: map['completed_at'] != null
          ? DateTime.fromMillisecondsSinceEpoch(map['completed_at'] as int)
          : null,
      estimatedMinutes: map['estimated_minutes'] as int?,
      actualMinutes: map['actual_minutes'] as int?,
      priority: map['priority'] as int,
      metadata: map['metadata'] != null
          ? jsonDecode(map['metadata'] as String) as Map<String, dynamic>
          : null,
    );
  }
}
