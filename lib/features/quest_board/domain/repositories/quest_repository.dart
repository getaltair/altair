import '../entities/quest.dart';
import '../entities/epic.dart';
import '../entities/subquest.dart';

/// Repository interface for quest-related operations
abstract class QuestRepository {
  /// Get all quests
  Future<List<Quest>> getAllQuests();

  /// Get quest by ID
  Future<Quest?> getQuestById(String id);

  /// Create a new quest
  Future<Quest> createQuest(Quest quest);

  /// Update an existing quest
  Future<Quest> updateQuest(Quest quest);

  /// Delete a quest
  Future<void> deleteQuest(String id);

  /// Move quest to a different column
  Future<Quest> moveQuestToColumn(String questId, QuestColumn column);

  /// Get all epics
  Future<List<Epic>> getAllEpics();

  /// Get epic by ID
  Future<Epic?> getEpicById(String id);

  /// Create a new epic
  Future<Epic> createEpic(Epic epic);

  /// Update an existing epic
  Future<Epic> updateEpic(Epic epic);

  /// Get subquests for a quest
  Future<List<Subquest>> getSubquestsByQuestId(String questId);

  /// Create a new subquest
  Future<Subquest> createSubquest(Subquest subquest);

  /// Update an existing subquest
  Future<Subquest> updateSubquest(Subquest subquest);

  /// Get quests by column
  Future<List<Quest>> getQuestsByColumn(QuestColumn column);

  /// Archive quests older than specified days
  Future<void> archiveOldQuests(int daysOld);
}
