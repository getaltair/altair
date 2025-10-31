import 'package:altair_db_service/altair_db_service.dart';
import 'package:uuid/uuid.dart';

import '../models/task.dart';

/// SurrealDB-based repository for managing tasks
class TaskRepository {
  final Uuid _uuid = const Uuid();
  late final AltairConnectionManager _connectionManager;
  bool _initialized = false;

  /// Initialize the repository with database connection
  Future<void> initialize() async {
    if (_initialized) return;

    _connectionManager = await AltairConnectionManager.getInstance();
    _initialized = true;
  }

  /// Create a new task
  Future<Task> create(Task task) async {
    await _ensureInitialized();
    final db = _connectionManager.client;
    final now = DateTime.now();

    // Generate ID if not provided
    final id = task.id.isEmpty ? 'task:${_uuid.v4()}' : task.id;

    final taskToInsert = task.copyWith(
      id: id,
      createdAt: task.createdAt,
      updatedAt: now,
    );

    await db.create('task', _taskToMap(taskToInsert));

    return taskToInsert;
  }

  /// Get a task by ID
  Future<Task?> findById(String id) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    final result = await db.query(
      '''
      SELECT * FROM \$taskId;
    ''',
      {'taskId': id},
    );

    if (result == null || result is! List) return null;
    if (result.isEmpty) return null;
    if (result[0] is! Map) return null;

    final responseMap = result[0] as Map<String, dynamic>;
    final data = responseMap['result'];

    if (data is! List || data.isEmpty) return null;

    return _taskFromMap(data[0] as Map<String, dynamic>);
  }

  /// Get all tasks
  Future<List<Task>> findAll({
    TaskStatus? status,
    String? projectId,
    List<String>? tags,
    int? limit,
    int? offset,
  }) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    // Build query with filters
    final conditions = <String>[];
    final params = <String, dynamic>{};

    if (status != null) {
      conditions.add('status = \$status');
      params['status'] = status.name;
    }

    if (projectId != null) {
      conditions.add('project_id = \$projectId');
      params['projectId'] = projectId;
    }

    if (tags != null && tags.isNotEmpty) {
      conditions.add('\$tags ALLINSIDE tags');
      params['tags'] = tags;
    }

    // Validate limit and offset to prevent DoS and ensure valid queries
    if (limit != null && limit <= 0) {
      throw ArgumentError('limit must be positive, got: $limit');
    }
    if (limit != null && limit > 10000) {
      throw ArgumentError('limit too large (max 10000), got: $limit');
    }
    if (offset != null && offset < 0) {
      throw ArgumentError('offset must be non-negative, got: $offset');
    }

    final whereClause =
        conditions.isEmpty ? '' : 'WHERE ${conditions.join(' AND ')}';
    final limitClause = limit != null ? 'LIMIT $limit' : '';
    final startClause = offset != null ? 'START $offset' : '';

    final query = '''
      SELECT * FROM task
      $whereClause
      ORDER BY created_at DESC
      $startClause
      $limitClause;
    ''';

    final result = await db.query(query, params);

    if (result == null) return [];

    // Handle error responses (SurrealDB returns Map for errors)
    if (result is! List) {
      print('❌ SurrealDB query error (not a List): $result');
      return [];
    }

    if (result.isEmpty) return [];

    // SurrealDB returns results as: [{result: [...data...], status: "OK", time: "..."}]
    // Extract the actual data from result[0]['result']
    if (result[0] is! Map) {
      print('❌ SurrealDB unexpected response format: ${result[0]}');
      return [];
    }

    final responseMap = result[0] as Map<String, dynamic>;
    final data = responseMap['result'];

    if (data is! List) return [];

    return data
        .map((item) => _taskFromMap(item as Map<String, dynamic>))
        .toList();
  }

  /// Update a task
  Future<Task> update(Task task) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    final taskToUpdate = task.copyWith(updatedAt: DateTime.now());

    await db.update(task.id, _taskToMap(taskToUpdate));

    return taskToUpdate;
  }

  /// Delete a task and all its subtasks (cascade delete)
  ///
  /// This operation:
  /// - Detects circular references and throws an exception
  /// - Enforces maximum recursion depth (100 levels)
  /// - Uses atomic transactions to prevent partial deletes
  /// - Prevents orphaned subtasks
  ///
  /// Throws:
  /// - [StateError] if circular reference is detected
  /// - [StateError] if max depth exceeded
  /// - [Exception] if transaction fails
  Future<void> delete(String id) async {
    await _ensureInitialized();

    try {
      final visited = <String>{};
      const maxDepth = 100;

      // Phase 1: Collect all task IDs in hierarchy (with safety checks)
      final idsToDelete = await _collectTaskHierarchyIds(
        id,
        visited: visited,
        depth: 0,
        maxDepth: maxDepth,
      );

      // Phase 2: Delete all tasks atomically within a transaction
      // This ensures either all tasks are deleted, or none are (atomicity)
      await _deleteTasksInTransaction(idsToDelete);
    } catch (e) {
      // Rethrow with context
      if (e is StateError) {
        rethrow; // Circular reference or max depth errors
      }
      throw Exception('Cascade delete failed: $e');
    }
  }

  /// Delete multiple tasks atomically within a SurrealDB transaction
  ///
  /// This ensures that either ALL tasks are deleted successfully,
  /// or NONE are deleted if any error occurs (atomic operation).
  ///
  /// Uses SurrealDB's BEGIN TRANSACTION / COMMIT TRANSACTION syntax.
  ///
  /// Throws:
  /// - [Exception] if transaction fails
  Future<void> _deleteTasksInTransaction(List<String> taskIds) async {
    if (taskIds.isEmpty) return;

    final db = _connectionManager.client;

    // Build DELETE statements for all tasks
    final deleteStatements = taskIds
        .map((taskId) => 'DELETE \$taskId_${taskIds.indexOf(taskId)};')
        .join('\n      ');

    // Build parameter map for task IDs
    final params = <String, dynamic>{};
    for (var i = 0; i < taskIds.length; i++) {
      params['taskId_$i'] = taskIds[i];
    }

    // Execute all deletes within a single transaction
    final query = '''
      BEGIN TRANSACTION;
      $deleteStatements
      COMMIT TRANSACTION;
    ''';

    try {
      await db.query(query, params);
    } catch (e) {
      // Transaction automatically rolls back on error in SurrealDB
      throw Exception(
        'Transaction failed while deleting ${taskIds.length} tasks: $e',
      );
    }
  }

  /// Collect all task IDs in the hierarchy for batch deletion
  ///
  /// This prevents N+1 queries by collecting IDs first, then deleting in batch.
  /// Also provides circular reference detection and depth protection.
  ///
  /// Parameters:
  /// - [taskId]: The root task ID to start from
  /// - [visited]: Set of already visited task IDs (for cycle detection)
  /// - [depth]: Current recursion depth
  /// - [maxDepth]: Maximum allowed recursion depth
  ///
  /// Returns: List of all task IDs to delete (including root)
  ///
  /// Throws:
  /// - [StateError] if circular reference detected
  /// - [StateError] if max depth exceeded
  Future<List<String>> _collectTaskHierarchyIds(
    String taskId, {
    required Set<String> visited,
    required int depth,
    required int maxDepth,
  }) async {
    // Check for circular reference
    if (visited.contains(taskId)) {
      throw StateError(
        'Circular reference detected in task hierarchy at task: $taskId. '
        'This indicates corrupted data where tasks form a cycle.',
      );
    }

    // Check for maximum depth exceeded
    if (depth > maxDepth) {
      throw StateError(
        'Maximum task hierarchy depth ($maxDepth) exceeded at task: $taskId. '
        'This may indicate a circular reference or extremely deep nesting.',
      );
    }

    // Mark as visited
    visited.add(taskId);

    // Find all direct subtasks
    final subtasks = await findSubtasks(taskId);

    // Collect IDs from this task and all descendants
    final allIds = <String>[taskId];

    // Recursively collect IDs from subtasks
    for (final subtask in subtasks) {
      final subtaskIds = await _collectTaskHierarchyIds(
        subtask.id,
        visited: visited,
        depth: depth + 1,
        maxDepth: maxDepth,
      );
      allIds.addAll(subtaskIds);
    }

    return allIds;
  }

  /// Search tasks by title or description
  Future<List<Task>> search(String query) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    final result = await db.query(
      '''
      SELECT * FROM task
      WHERE title @@ \$query OR description @@ \$query
      ORDER BY created_at DESC
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
        .map((item) => _taskFromMap(item as Map<String, dynamic>))
        .toList();
  }

  /// Get subtasks for a parent task
  Future<List<Task>> findSubtasks(String parentTaskId) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    final result = await db.query(
      '''
      SELECT * FROM task
      WHERE parent_task_id = \$parentTaskId
      ORDER BY created_at ASC;
    ''',
      {'parentTaskId': parentTaskId},
    );

    if (result == null || result is! List) return [];
    if (result.isEmpty) return [];
    if (result[0] is! Map) return [];

    final responseMap = result[0] as Map<String, dynamic>;
    final data = responseMap['result'];

    if (data is! List) return [];

    return data
        .map((item) => _taskFromMap(item as Map<String, dynamic>))
        .toList();
  }

  /// Ensure the repository is initialized
  Future<void> _ensureInitialized() async {
    if (!_initialized) {
      await initialize();
    }
  }

  /// Convert Task to Map for database storage
  Map<String, dynamic> _taskToMap(Task task) {
    return {
      'id': task.id,
      'title': task.title,
      'description': task.description,
      'status': task.status.name,
      'tags': task.tags,
      'project_id': task.projectId,
      'parent_task_id': task.parentTaskId,
      'created_at': task.createdAt.toIso8601String(),
      'updated_at': task.updatedAt.toIso8601String(),
      'completed_at': task.completedAt?.toIso8601String(),
      'estimated_minutes': task.estimatedMinutes,
      'actual_minutes': task.actualMinutes,
      'priority': task.priority,
      'metadata': task.metadata,
    };
  }

  /// Convert Map from database to Task
  Task _taskFromMap(Map<String, dynamic> map) {
    return Task(
      id: map['id'] as String,
      title: map['title'] as String,
      description: map['description'] as String?,
      status: TaskStatus.values.byName(map['status'] as String),
      tags: (map['tags'] as List<dynamic>?)?.map((e) => e as String).toList() ??
          [],
      projectId: map['project_id'] as String?,
      parentTaskId: map['parent_task_id'] as String?,
      createdAt: DateTime.parse(map['created_at'] as String),
      updatedAt: DateTime.parse(map['updated_at'] as String),
      completedAt: map['completed_at'] != null
          ? DateTime.parse(map['completed_at'] as String)
          : null,
      estimatedMinutes: map['estimated_minutes'] as int?,
      actualMinutes: map['actual_minutes'] as int?,
      priority: map['priority'] as int,
      metadata: map['metadata'] as Map<String, dynamic>?,
    );
  }
}
