import '../../domain/entities/quest.dart';
import '../../domain/entities/epic.dart';
import '../../domain/entities/subquest.dart';
import '../../domain/repositories/quest_repository.dart';
import '../datasources/surrealdb_datasource.dart';

/// SurrealDB implementation of QuestRepository
class QuestRepositoryImpl implements QuestRepository {
  final SurrealDbDatasource _datasource;

  QuestRepositoryImpl(this._datasource);

  @override
  Future<List<Quest>> getAllQuests() async {
    final records = await _datasource.getAllQuests();
    return records.map((record) => _questFromRecord(record)).toList();
  }

  @override
  Future<Quest?> getQuestById(String id) async {
    final record = await _datasource.getQuestById(id);
    if (record == null) return null;
    return _questFromRecord(record);
  }

  @override
  Future<Quest> createQuest(Quest quest) async {
    final record = await _datasource.createQuest(quest);
    return _questFromRecord(record);
  }

  @override
  Future<Quest> updateQuest(Quest quest) async {
    final record = await _datasource.updateQuest(quest);
    return _questFromRecord(record);
  }

  @override
  Future<void> deleteQuest(String id) async {
    await _datasource.deleteQuest(id);
  }

  @override
  Future<Quest> moveQuestToColumn(String questId, QuestColumn column) async {
    final record = await _datasource.moveQuestToColumn(questId, column);
    return _questFromRecord(record);
  }

  @override
  Future<List<Epic>> getAllEpics() async {
    final records = await _datasource.getAllEpics();
    return records.map((record) => _epicFromRecord(record)).toList();
  }

  @override
  Future<Epic?> getEpicById(String id) async {
    final record = await _datasource.getEpicById(id);
    if (record == null) return null;
    return _epicFromRecord(record);
  }

  @override
  Future<Epic> createEpic(Epic epic) async {
    final record = await _datasource.createEpic(epic);
    return _epicFromRecord(record);
  }

  @override
  Future<Epic> updateEpic(Epic epic) async {
    final record = await _datasource.updateEpic(epic);
    return _epicFromRecord(record);
  }

  @override
  Future<List<Subquest>> getSubquestsByQuestId(String questId) async {
    final records = await _datasource.getSubquestsByQuestId(questId);
    return records.map((record) => _subquestFromRecord(record)).toList();
  }

  @override
  Future<Subquest> createSubquest(Subquest subquest) async {
    final record = await _datasource.createSubquest(subquest);
    return _subquestFromRecord(record);
  }

  @override
  Future<Subquest> updateSubquest(Subquest subquest) async {
    final record = await _datasource.updateSubquest(subquest);
    return _subquestFromRecord(record);
  }

  @override
  Future<List<Quest>> getQuestsByColumn(QuestColumn column) async {
    final records = await _datasource.getQuestsByColumn(column);
    return records.map((record) => _questFromRecord(record)).toList();
  }

  @override
  Future<void> archiveOldQuests(int daysOld) async {
    await _datasource.archiveOldQuests(daysOld);
  }

  /// Convert SurrealDB record to Quest entity
  Quest _questFromRecord(Map<String, dynamic> record) {
    // Handle nested subquests if present
    List<Subquest> subquests = [];
    if (record['subquests'] != null) {
      final subquestList = record['subquests'] as List<dynamic>?;
      if (subquestList != null) {
        subquests = subquestList
            .whereType<Map<String, dynamic>>()
            .map((s) => _subquestFromRecord(s))
            .toList();
      }
    }

    return Quest(
      id: record['id'] as String,
      epicId: record['epicId'] as String?,
      title: record['title'] as String,
      description: record['description'] as String?,
      energyPoints: record['energyPoints'] as int,
      column: QuestColumn.values.firstWhere(
        (c) => c.name == (record['column'] as String),
        orElse: () => QuestColumn.ideaGreenhouse,
      ),
      createdAt: DateTime.parse(record['createdAt'] as String),
      updatedAt: record['updatedAt'] != null
          ? DateTime.parse(record['updatedAt'] as String)
          : null,
      completedAt: record['completedAt'] != null
          ? DateTime.parse(record['completedAt'] as String)
          : null,
      tags: (record['tags'] as List<dynamic>?)?.cast<String>() ?? [],
      assigneeId: record['assigneeId'] as String?,
      subquests: subquests,
      isArchived: record['isArchived'] as bool? ?? false,
    );
  }

  /// Convert SurrealDB record to Epic entity
  Epic _epicFromRecord(Map<String, dynamic> record) {
    return Epic(
      id: record['id'] as String,
      title: record['title'] as String,
      description: record['description'] as String?,
      createdAt: DateTime.parse(record['createdAt'] as String),
      updatedAt: record['updatedAt'] != null
          ? DateTime.parse(record['updatedAt'] as String)
          : null,
      tags: (record['tags'] as List<dynamic>?)?.cast<String>() ?? [],
      assigneeId: record['assigneeId'] as String?,
    );
  }

  /// Convert SurrealDB record to Subquest entity
  Subquest _subquestFromRecord(Map<String, dynamic> record) {
    return Subquest(
      id: record['id'] as String,
      questId: record['questId'] as String,
      title: record['title'] as String,
      description: record['description'] as String?,
      energyPoints: record['energyPoints'] as int,
      createdAt: DateTime.parse(record['createdAt'] as String),
      updatedAt: record['updatedAt'] != null
          ? DateTime.parse(record['updatedAt'] as String)
          : null,
      completedAt: record['completedAt'] != null
          ? DateTime.parse(record['completedAt'] as String)
          : null,
      tags: (record['tags'] as List<dynamic>?)?.cast<String>() ?? [],
      assigneeId: record['assigneeId'] as String?,
      isCompleted: record['isCompleted'] as bool? ?? false,
    );
  }
}
