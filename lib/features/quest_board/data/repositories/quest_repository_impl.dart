import '../../domain/entities/quest.dart';
import '../../domain/entities/epic.dart';
import '../../domain/entities/subquest.dart';
import '../../domain/repositories/quest_repository.dart';

/// In-memory implementation of QuestRepository (for MVP)
/// TODO: Replace with SurrealDB implementation
class QuestRepositoryImpl implements QuestRepository {
  final Map<String, Quest> _quests = {};
  final Map<String, Epic> _epics = {};
  final Map<String, List<Subquest>> _subquests = {};

  @override
  Future<List<Quest>> getAllQuests() async {
    return _quests.values.toList();
  }

  @override
  Future<Quest?> getQuestById(String id) async {
    return _quests[id];
  }

  @override
  Future<Quest> createQuest(Quest quest) async {
    _quests[quest.id] = quest;
    return quest;
  }

  @override
  Future<Quest> updateQuest(Quest quest) async {
    _quests[quest.id] = quest.copyWith(updatedAt: DateTime.now());
    return _quests[quest.id]!;
  }

  @override
  Future<void> deleteQuest(String id) async {
    _quests.remove(id);
    _subquests.remove(id);
  }

  @override
  Future<Quest> moveQuestToColumn(String questId, QuestColumn column) async {
    final quest = _quests[questId];
    if (quest == null) {
      throw Exception('Quest not found: $questId');
    }
    final updatedQuest =
        quest.copyWith(column: column, updatedAt: DateTime.now());
    _quests[questId] = updatedQuest;
    return updatedQuest;
  }

  @override
  Future<List<Epic>> getAllEpics() async {
    return _epics.values.toList();
  }

  @override
  Future<Epic?> getEpicById(String id) async {
    return _epics[id];
  }

  @override
  Future<Epic> createEpic(Epic epic) async {
    _epics[epic.id] = epic;
    return epic;
  }

  @override
  Future<Epic> updateEpic(Epic epic) async {
    _epics[epic.id] = epic.copyWith(updatedAt: DateTime.now());
    return _epics[epic.id]!;
  }

  @override
  Future<List<Subquest>> getSubquestsByQuestId(String questId) async {
    return _subquests[questId] ?? [];
  }

  @override
  Future<Subquest> createSubquest(Subquest subquest) async {
    _subquests.putIfAbsent(subquest.questId, () => []).add(subquest);
    return subquest;
  }

  @override
  Future<Subquest> updateSubquest(Subquest subquest) async {
    final subquests = _subquests[subquest.questId] ?? [];
    final index = subquests.indexWhere((s) => s.id == subquest.id);
    if (index != -1) {
      subquests[index] = subquest.copyWith(updatedAt: DateTime.now());
    }
    return subquest;
  }

  @override
  Future<List<Quest>> getQuestsByColumn(QuestColumn column) async {
    return _quests.values
        .where((q) => q.column == column && !q.isArchived)
        .toList();
  }

  @override
  Future<void> archiveOldQuests(int daysOld) async {
    final cutoffDate = DateTime.now().subtract(Duration(days: daysOld));
    for (final quest in _quests.values) {
      if (quest.completedAt != null &&
          quest.completedAt!.isBefore(cutoffDate) &&
          !quest.isArchived) {
        _quests[quest.id] = quest.copyWith(isArchived: true);
      }
    }
  }
}
