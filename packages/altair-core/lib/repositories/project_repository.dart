import 'package:altair_db_service/altair_db_service.dart';
import 'package:uuid/uuid.dart';

import '../models/project.dart';

/// SurrealDB-based repository for managing projects
class ProjectRepository {
  final Uuid _uuid = const Uuid();
  late final AltairConnectionManager _connectionManager;
  bool _initialized = false;

  /// Initialize the repository with database connection
  Future<void> initialize() async {
    if (_initialized) return;

    _connectionManager = await AltairConnectionManager.getInstance();
    _initialized = true;
  }

  /// Create a new project
  Future<Project> create(Project project) async {
    await _ensureInitialized();
    final db = _connectionManager.client;
    final now = DateTime.now();

    // Generate ID if not provided
    final id = project.id.isEmpty ? 'project:${_uuid.v4()}' : project.id;

    final projectToInsert = project.copyWith(
      id: id,
      createdAt: project.createdAt,
      updatedAt: now,
    );

    await db.create('project', _projectToMap(projectToInsert));

    return projectToInsert;
  }

  /// Get a project by ID
  Future<Project?> findById(String id) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    final result = await db.query('''
      SELECT * FROM \$projectId;
    ''', {'projectId': id});

    if (result == null || result is! List) return null;
    if (result.isEmpty) return null;
    if (result[0] is! Map) return null;

    final responseMap = result[0] as Map<String, dynamic>;
    final data = responseMap['result'];

    if (data is! List || data.isEmpty) return null;

    return _projectFromMap(data[0] as Map<String, dynamic>);
  }

  /// Get all projects
  Future<List<Project>> findAll({
    ProjectStatus? status,
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

    if (tags != null && tags.isNotEmpty) {
      conditions.add('\$tags ALLINSIDE tags');
      params['tags'] = tags;
    }

    final whereClause =
        conditions.isEmpty ? '' : 'WHERE ${conditions.join(' AND ')}';
    final limitClause = limit != null ? 'LIMIT $limit' : '';
    final startClause = offset != null ? 'START $offset' : '';

    final query = '''
      SELECT * FROM project
      $whereClause
      ORDER BY created_at DESC
      $startClause
      $limitClause;
    ''';

    final result = await db.query(query, params);

    if (result == null || result is! List) return [];
    if (result.isEmpty) return [];
    if (result[0] is! Map) return [];

    final responseMap = result[0] as Map<String, dynamic>;
    final data = responseMap['result'];

    if (data is! List) return [];

    return data
        .map((item) => _projectFromMap(item as Map<String, dynamic>))
        .toList();
  }

  /// Update a project
  Future<Project> update(Project project) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    final projectToUpdate = project.copyWith(
      updatedAt: DateTime.now(),
    );

    await db.update(project.id, _projectToMap(projectToUpdate));

    return projectToUpdate;
  }

  /// Delete a project
  Future<void> delete(String id) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    await db.delete(id);
  }

  /// Search projects by name or description
  Future<List<Project>> search(String query) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    final result = await db.query('''
      SELECT * FROM project
      WHERE name @@ \$query OR description @@ \$query
      ORDER BY created_at DESC
      LIMIT 50;
    ''', {'query': query});

    if (result == null || result is! List) return [];
    if (result.isEmpty) return [];
    if (result[0] is! Map) return [];

    final responseMap = result[0] as Map<String, dynamic>;
    final data = responseMap['result'];

    if (data is! List) return [];

    return data
        .map((item) => _projectFromMap(item as Map<String, dynamic>))
        .toList();
  }

  /// Get task count for a project
  Future<int> getTaskCount(String projectId) async {
    await _ensureInitialized();
    final db = _connectionManager.client;

    final result = await db.query('''
      SELECT count() as count FROM task
      WHERE project_id = \$projectId
      GROUP ALL;
    ''', {'projectId': projectId});

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

  /// Convert Project to Map for database storage
  Map<String, dynamic> _projectToMap(Project project) {
    return {
      'id': project.id,
      'name': project.name,
      'description': project.description,
      'status': project.status.name,
      'tags': project.tags,
      'color': project.color,
      'created_at': project.createdAt.toIso8601String(),
      'updated_at': project.updatedAt.toIso8601String(),
      'target_date': project.targetDate?.toIso8601String(),
      'completed_at': project.completedAt?.toIso8601String(),
      'metadata': project.metadata,
    };
  }

  /// Convert Map from database to Project
  Project _projectFromMap(Map<String, dynamic> map) {
    return Project(
      id: map['id'] as String,
      name: map['name'] as String,
      description: map['description'] as String?,
      status: ProjectStatus.values.byName(map['status'] as String),
      tags: (map['tags'] as List<dynamic>?)?.map((e) => e as String).toList() ??
          [],
      color: map['color'] as String?,
      createdAt: DateTime.parse(map['created_at'] as String),
      updatedAt: DateTime.parse(map['updated_at'] as String),
      targetDate: map['target_date'] != null
          ? DateTime.parse(map['target_date'] as String)
          : null,
      completedAt: map['completed_at'] != null
          ? DateTime.parse(map['completed_at'] as String)
          : null,
      metadata: map['metadata'] as Map<String, dynamic>?,
    );
  }
}
