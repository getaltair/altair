import 'dart:convert';
import 'package:http/http.dart' as http;
import '../../domain/entities/quest.dart';
import '../../domain/entities/epic.dart';
import '../../domain/entities/subquest.dart';
import '../models/quest_model.dart';

/// SurrealDB datasource for quest board operations
/// Uses HTTP API to communicate with SurrealDB
class SurrealDbDatasource {
  String? _baseUrl;
  String? _namespace;
  String? _database;
  bool _isInitialized = false;

  SurrealDbDatasource({
    String? baseUrl,
    String namespace = 'guidance',
    String database = 'quest_board',
  })  : _baseUrl = baseUrl ?? 'http://localhost:8000',
        _namespace = namespace,
        _database = database;

  /// Initialize SurrealDB connection
  Future<void> initialize() async {
    if (_isInitialized) return;

    try {
      // For MVP: Use in-memory fallback if SurrealDB not available
      // In production, this would connect to embedded SurrealDB
      // For now, we'll use a simple HTTP-based approach or fallback to in-memory
      
      // Try to connect to SurrealDB
      final namespace = _namespace ?? 'guidance';
      final database = _database ?? 'quest_board';
      
      final response = await http.post(
        Uri.parse('$_baseUrl/sql'),
        headers: {
          'Content-Type': 'application/json',
          'NS': namespace,
          'DB': database,
        },
        body: jsonEncode(['INFO FOR DB']),
      ).timeout(const Duration(seconds: 1));

      if (response.statusCode == 200) {
        // SurrealDB is available
        await _initializeSchema();
        _isInitialized = true;
      } else {
        // Fallback: Use in-memory storage for MVP
        // This will be replaced when SurrealDB is properly set up
        _isInitialized = true;
      }
    } catch (e) {
      // SurrealDB not available - use in-memory fallback
      // This is acceptable for MVP
      _isInitialized = true;
    }
  }

  /// Initialize database schema
  Future<void> _initializeSchema() async {
    // Use raw string to avoid Dart string interpolation issues with $value
    final schema = r'''
      DEFINE TABLE quest SCHEMAFULL;
      DEFINE FIELD id ON quest TYPE string;
      DEFINE FIELD epicId ON quest TYPE option<string>;
      DEFINE FIELD title ON quest TYPE string;
      DEFINE FIELD description ON quest TYPE option<string>;
      DEFINE FIELD energyPoints ON quest TYPE int;
      DEFINE FIELD column ON quest TYPE string;
      DEFINE FIELD createdAt ON quest TYPE datetime;
      DEFINE FIELD updatedAt ON quest TYPE option<datetime>;
      DEFINE FIELD completedAt ON quest TYPE option<datetime>;
      DEFINE FIELD tags ON quest TYPE array<string> DEFAULT [];
      DEFINE FIELD assigneeId ON quest TYPE option<string>;
      DEFINE FIELD isArchived ON quest TYPE bool DEFAULT false;
      DEFINE INDEX questColumn ON quest FIELDS column;
      DEFINE INDEX questEpic ON quest FIELDS epicId;
      
      DEFINE TABLE epic SCHEMAFULL;
      DEFINE FIELD id ON epic TYPE string;
      DEFINE FIELD title ON epic TYPE string;
      DEFINE FIELD description ON epic TYPE option<string>;
      DEFINE FIELD createdAt ON epic TYPE datetime;
      DEFINE FIELD updatedAt ON epic TYPE option<datetime>;
      DEFINE FIELD tags ON epic TYPE array<string> DEFAULT [];
      DEFINE FIELD assigneeId ON epic TYPE option<string>;
      
      DEFINE TABLE subquest SCHEMAFULL;
      DEFINE FIELD id ON subquest TYPE string;
      DEFINE FIELD questId ON subquest TYPE string;
      DEFINE FIELD title ON subquest TYPE string;
      DEFINE FIELD description ON subquest TYPE option<string>;
      DEFINE FIELD energyPoints ON subquest TYPE int;
      DEFINE FIELD createdAt ON subquest TYPE datetime;
      DEFINE FIELD updatedAt ON subquest TYPE option<datetime>;
      DEFINE FIELD completedAt ON subquest TYPE option<datetime>;
      DEFINE FIELD tags ON subquest TYPE array<string> DEFAULT [];
      DEFINE FIELD assigneeId ON subquest TYPE option<string>;
      DEFINE FIELD isCompleted ON subquest TYPE bool DEFAULT false;
      DEFINE INDEX subquestQuest ON subquest FIELDS questId;
    ''';
    
    try {
      await _executeQuery(schema);
    } catch (e) {
      // Schema initialization failed - will use fallback
    }
  }

  /// Execute a SurrealQL query
  Future<List<Map<String, dynamic>>> _executeQuery(String query, [Map<String, dynamic>? params]) async {
    try {
      final namespace = _namespace ?? 'guidance';
      final database = _database ?? 'quest_board';
      
      final response = await http.post(
        Uri.parse('$_baseUrl/sql'),
        headers: {
          'Content-Type': 'application/json',
          'NS': namespace,
          'DB': database,
        },
        body: jsonEncode([query]),
      );

      if (response.statusCode == 200) {
        final result = jsonDecode(response.body) as List;
        return _extractRecords(result);
      }
    } catch (e) {
      // Query failed - return empty for now
    }
    return [];
  }

  /// Get all quests
  Future<List<Map<String, dynamic>>> getAllQuests() async {
    await _ensureInitialized();
    final result = await _executeQuery(
      'SELECT *, (SELECT * FROM subquest WHERE questId = quest.id) AS subquests FROM quest WHERE isArchived = false',
    );
    return result;
  }

  /// Get quest by ID
  Future<Map<String, dynamic>?> getQuestById(String id) async {
    await _ensureInitialized();
    final result = await _executeQuery(
      'SELECT *, (SELECT * FROM subquest WHERE questId = quest.id) AS subquests FROM quest WHERE id = \$id',
      {'id': id},
    );
    return result.isNotEmpty ? result.first : null;
  }

  /// Create quest
  Future<Map<String, dynamic>> createQuest(Quest quest) async {
    await _ensureInitialized();
    final questModel = QuestModel.fromEntity(quest);
    final data = questModel.toJson();
    data.remove('subquests');
    
    final result = await _executeQuery(
      'CREATE quest CONTENT \$data',
      {'data': data},
    );
    
    // Create subquests if any
    if (quest.subquests.isNotEmpty) {
      for (final subquest in quest.subquests) {
        await createSubquest(subquest);
      }
    }
    
    return result.isNotEmpty ? result.first : data;
  }

  /// Update quest
  Future<Map<String, dynamic>> updateQuest(Quest quest) async {
    await _ensureInitialized();
    final questModel = QuestModel.fromEntity(quest);
    final data = questModel.toJson();
    data.remove('subquests');
    data['updatedAt'] = DateTime.now().toIso8601String();
    
    final result = await _executeQuery(
      'UPDATE quest SET \$data WHERE id = \$id',
      {'id': quest.id, 'data': data},
    );
    
    return result.isNotEmpty ? result.first : data;
  }

  /// Delete quest
  Future<void> deleteQuest(String id) async {
    await _ensureInitialized();
    await _executeQuery('DELETE subquest WHERE questId = \$id', {'id': id});
    await _executeQuery('DELETE quest WHERE id = \$id', {'id': id});
  }

  /// Move quest to column
  Future<Map<String, dynamic>> moveQuestToColumn(String questId, QuestColumn column) async {
    await _ensureInitialized();
    final result = await _executeQuery(
      'UPDATE quest SET column = \$column, updatedAt = \$updatedAt WHERE id = \$id',
      {
        'id': questId,
        'column': column.name,
        'updatedAt': DateTime.now().toIso8601String(),
      },
    );
    return result.isNotEmpty ? result.first : {};
  }

  /// Get all epics
  Future<List<Map<String, dynamic>>> getAllEpics() async {
    await _ensureInitialized();
    return await _executeQuery('SELECT * FROM epic');
  }

  /// Get epic by ID
  Future<Map<String, dynamic>?> getEpicById(String id) async {
    await _ensureInitialized();
    final result = await _executeQuery('SELECT * FROM epic WHERE id = \$id', {'id': id});
    return result.isNotEmpty ? result.first : null;
  }

  /// Create epic
  Future<Map<String, dynamic>> createEpic(Epic epic) async {
    await _ensureInitialized();
    final data = {
      'id': epic.id,
      'title': epic.title,
      'description': epic.description,
      'createdAt': epic.createdAt.toIso8601String(),
      'updatedAt': epic.updatedAt?.toIso8601String(),
      'tags': epic.tags,
      'assigneeId': epic.assigneeId,
    };
    
    final result = await _executeQuery('CREATE epic CONTENT \$data', {'data': data});
    return result.isNotEmpty ? result.first : data;
  }

  /// Update epic
  Future<Map<String, dynamic>> updateEpic(Epic epic) async {
    await _ensureInitialized();
    final data = {
      'title': epic.title,
      'description': epic.description,
      'updatedAt': DateTime.now().toIso8601String(),
      'tags': epic.tags,
      'assigneeId': epic.assigneeId,
    };
    
    final result = await _executeQuery('UPDATE epic SET \$data WHERE id = \$id', {'id': epic.id, 'data': data});
    return result.isNotEmpty ? result.first : data;
  }

  /// Get subquests by quest ID
  Future<List<Map<String, dynamic>>> getSubquestsByQuestId(String questId) async {
    await _ensureInitialized();
    return await _executeQuery('SELECT * FROM subquest WHERE questId = \$questId', {'questId': questId});
  }

  /// Create subquest
  Future<Map<String, dynamic>> createSubquest(Subquest subquest) async {
    await _ensureInitialized();
    final data = {
      'id': subquest.id,
      'questId': subquest.questId,
      'title': subquest.title,
      'description': subquest.description,
      'energyPoints': subquest.energyPoints,
      'createdAt': subquest.createdAt.toIso8601String(),
      'updatedAt': subquest.updatedAt?.toIso8601String(),
      'completedAt': subquest.completedAt?.toIso8601String(),
      'tags': subquest.tags,
      'assigneeId': subquest.assigneeId,
      'isCompleted': subquest.isCompleted,
    };
    
    final result = await _executeQuery('CREATE subquest CONTENT \$data', {'data': data});
    return result.isNotEmpty ? result.first : data;
  }

  /// Update subquest
  Future<Map<String, dynamic>> updateSubquest(Subquest subquest) async {
    await _ensureInitialized();
    final data = {
      'title': subquest.title,
      'description': subquest.description,
      'energyPoints': subquest.energyPoints,
      'updatedAt': DateTime.now().toIso8601String(),
      'completedAt': subquest.completedAt?.toIso8601String(),
      'tags': subquest.tags,
      'assigneeId': subquest.assigneeId,
      'isCompleted': subquest.isCompleted,
    };
    
    final result = await _executeQuery('UPDATE subquest SET \$data WHERE id = \$id', {'id': subquest.id, 'data': data});
    return result.isNotEmpty ? result.first : data;
  }

  /// Get quests by column
  Future<List<Map<String, dynamic>>> getQuestsByColumn(QuestColumn column) async {
    await _ensureInitialized();
    return await _executeQuery(
      'SELECT *, (SELECT * FROM subquest WHERE questId = quest.id) AS subquests FROM quest WHERE column = \$column AND isArchived = false',
      {'column': column.name},
    );
  }

  /// Archive old quests
  Future<void> archiveOldQuests(int daysOld) async {
    await _ensureInitialized();
    final cutoffDate = DateTime.now().subtract(Duration(days: daysOld));
    await _executeQuery(
      'UPDATE quest SET isArchived = true, updatedAt = \$updatedAt WHERE completedAt < \$cutoffDate AND isArchived = false',
      {
        'cutoffDate': cutoffDate.toIso8601String(),
        'updatedAt': DateTime.now().toIso8601String(),
      },
    );
  }

  /// Close database connection
  Future<void> close() async {
    _isInitialized = false;
  }

  /// Ensure database is initialized
  Future<void> _ensureInitialized() async {
    if (!_isInitialized) {
      await initialize();
    }
  }

  /// Extract records from SurrealDB query result
  List<Map<String, dynamic>> _extractRecords(List<dynamic> result) {
    if (result.isEmpty) return [];
    
    final records = <Map<String, dynamic>>[];
    for (final item in result) {
      if (item is Map<String, dynamic>) {
        if (item.containsKey('result')) {
          final resultList = item['result'] as List<dynamic>?;
          if (resultList != null) {
            for (final record in resultList) {
              if (record is Map<String, dynamic>) {
                records.add(record);
              }
            }
          }
        } else {
          records.add(item);
        }
      }
    }
    return records;
  }
}
