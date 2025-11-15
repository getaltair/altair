import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../domain/entities/quest.dart';
import '../../domain/repositories/quest_repository.dart';
import '../../data/repositories/quest_repository_impl.dart';
import 'filter_provider.dart';
import 'dart:collection';

/// State for the quest board
class QuestBoardState {
  final List<Quest> quests;
  final Map<QuestColumn, List<Quest>> questsByColumn;
  final bool isLoading;
  final String? error;
  final Queue<QuestBoardAction> undoStack;
  final Queue<QuestBoardAction> redoStack;

  QuestBoardState({
    this.quests = const [],
    Map<QuestColumn, List<Quest>>? questsByColumn,
    this.isLoading = false,
    this.error,
    Queue<QuestBoardAction>? undoStack,
    Queue<QuestBoardAction>? redoStack,
  })  : questsByColumn = questsByColumn ??
            Map.fromIterable(
              QuestColumn.values,
              key: (col) => col,
              value: (_) => <Quest>[],
            ),
        undoStack = undoStack ?? Queue(),
        redoStack = redoStack ?? Queue();

  QuestBoardState copyWith({
    List<Quest>? quests,
    Map<QuestColumn, List<Quest>>? questsByColumn,
    bool? isLoading,
    String? error,
    Queue<QuestBoardAction>? undoStack,
    Queue<QuestBoardAction>? redoStack,
  }) {
    return QuestBoardState(
      quests: quests ?? this.quests,
      questsByColumn: questsByColumn ?? this.questsByColumn,
      isLoading: isLoading ?? this.isLoading,
      error: error ?? this.error,
      undoStack: undoStack ?? this.undoStack,
      redoStack: redoStack ?? this.redoStack,
    );
  }

  /// Get filtered quests for a column
  List<Quest> getFilteredQuestsForColumn(
      QuestColumn column, BoardFilters filters) {
    final columnQuests = questsByColumn[column] ?? [];
    return columnQuests.where((quest) => filters.matches(quest)).toList();
  }

  /// Check if In-Progress column has WIP violation
  bool hasWipViolation() {
    final inProgressQuests = questsByColumn[QuestColumn.inProgress] ?? [];
    return inProgressQuests.length > 1;
  }
}

/// Action for undo/redo
class QuestBoardAction {
  final String questId;
  final QuestColumn fromColumn;
  final QuestColumn toColumn;
  final DateTime timestamp;

  QuestBoardAction({
    required this.questId,
    required this.fromColumn,
    required this.toColumn,
    required this.timestamp,
  });
}

/// Repository provider
final questRepositoryProvider = Provider<QuestRepository>((ref) {
  return QuestRepositoryImpl();
});

/// Board state notifier
class QuestBoardNotifier extends Notifier<QuestBoardState> {
  QuestRepository get _repository => ref.read(questRepositoryProvider);

  @override
  QuestBoardState build() {
    // Load quests after initial build
    Future.microtask(() {
      loadQuests();
      // Start auto-archive timer
      _startAutoArchiveTimer();
    });
    return QuestBoardState();
  }

  void _startAutoArchiveTimer() {
    // Check for old quests to archive every hour
    Future.delayed(const Duration(hours: 1), () {
      _archiveOldQuests();
      _startAutoArchiveTimer(); // Schedule next check
    });
  }

  Future<void> _archiveOldQuests() async {
    try {
      // Archive quests completed more than 24 hours ago (configurable)
      const daysOld = 1; // Default: 24 hours
      await _repository.archiveOldQuests(daysOld);
      // Reload quests to reflect archived state
      await loadQuests();
    } catch (e) {
      // Silently fail - don't disrupt user experience
      print('Auto-archive error: $e');
    }
  }

  Future<void> loadQuests() async {
    state = state.copyWith(isLoading: true, error: null);
    try {
      final quests = await _repository.getAllQuests();
      final questsByColumn = <QuestColumn, List<Quest>>{};
      for (final column in QuestColumn.values) {
        questsByColumn[column] =
            quests.where((q) => q.column == column && !q.isArchived).toList();
      }
      state = state.copyWith(
        quests: quests,
        questsByColumn: questsByColumn,
        isLoading: false,
      );
    } catch (e) {
      state = state.copyWith(
        isLoading: false,
        error: e.toString(),
      );
    }
  }

  Future<void> moveQuest(String questId, QuestColumn targetColumn) async {
    try {
      final quest = await _repository.getQuestById(questId);
      if (quest == null) return;

      // WIP=1 enforcement for In-Progress column
      if (targetColumn == QuestColumn.inProgress) {
        final currentInProgress =
            state.questsByColumn[QuestColumn.inProgress] ?? [];
        if (currentInProgress.isNotEmpty &&
            currentInProgress.first.id != questId) {
          // Move current item out first
          final currentQuest = currentInProgress.first;
          await _repository.moveQuestToColumn(
            currentQuest.id,
            QuestColumn.nextUp,
          );
        }
      }

      final sourceColumn = quest.column;
      final updatedQuest =
          await _repository.moveQuestToColumn(questId, targetColumn);

      // Add to undo stack (max 10)
      final action = QuestBoardAction(
        questId: questId,
        fromColumn: sourceColumn,
        toColumn: targetColumn,
        timestamp: DateTime.now(),
      );
      final newUndoStack = Queue<QuestBoardAction>.from(state.undoStack);
      newUndoStack.add(action);
      if (newUndoStack.length > 10) {
        newUndoStack.removeFirst();
      }

      // Update state
      final newQuests = List<Quest>.from(state.quests);
      final index = newQuests.indexWhere((q) => q.id == questId);
      if (index != -1) {
        newQuests[index] = updatedQuest;
      }

      final newQuestsByColumn =
          Map<QuestColumn, List<Quest>>.from(state.questsByColumn);
      newQuestsByColumn[sourceColumn] = newQuestsByColumn[sourceColumn]!
          .where((q) => q.id != questId)
          .toList();
      newQuestsByColumn[targetColumn] = [
        ...newQuestsByColumn[targetColumn]!,
        updatedQuest,
      ];

      state = state.copyWith(
        quests: newQuests,
        questsByColumn: newQuestsByColumn,
        undoStack: newUndoStack,
        redoStack: Queue(), // Clear redo on new action
      );
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }

  Future<void> createQuest(Quest quest) async {
    try {
      final createdQuest = await _repository.createQuest(quest);
      final newQuests = [...state.quests, createdQuest];
      final newQuestsByColumn =
          Map<QuestColumn, List<Quest>>.from(state.questsByColumn);
      newQuestsByColumn[quest.column] = [
        ...newQuestsByColumn[quest.column]!,
        createdQuest,
      ];
      state = state.copyWith(
        quests: newQuests,
        questsByColumn: newQuestsByColumn,
      );
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }

  Future<void> undo() async {
    if (state.undoStack.isEmpty) return;

    final action = state.undoStack.last;
    final newUndoStack = Queue<QuestBoardAction>.from(state.undoStack);
    newUndoStack.removeLast();

    final newRedoStack = Queue<QuestBoardAction>.from(state.redoStack);
    newRedoStack.add(action);

    await moveQuest(action.questId, action.fromColumn);

    state = state.copyWith(
      undoStack: newUndoStack,
      redoStack: newRedoStack,
    );
  }

  Future<void> redo() async {
    if (state.redoStack.isEmpty) return;

    final action = state.redoStack.last;
    final newRedoStack = Queue<QuestBoardAction>.from(state.redoStack);
    newRedoStack.removeLast();

    final newUndoStack = Queue<QuestBoardAction>.from(state.undoStack);
    newUndoStack.add(action);

    await moveQuest(action.questId, action.toColumn);

    state = state.copyWith(
      undoStack: newUndoStack,
      redoStack: newRedoStack,
    );
  }

  Future<void> deleteQuest(String questId) async {
    try {
      await _repository.deleteQuest(questId);
      final newQuests = state.quests.where((q) => q.id != questId).toList();
      final newQuestsByColumn = <QuestColumn, List<Quest>>{};
      for (final column in QuestColumn.values) {
        newQuestsByColumn[column] = newQuests
            .where((q) => q.column == column && !q.isArchived)
            .toList();
      }
      state = state.copyWith(
        quests: newQuests,
        questsByColumn: newQuestsByColumn,
      );
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }

  Future<void> duplicateQuest(String questId) async {
    try {
      final originalQuest = await _repository.getQuestById(questId);
      if (originalQuest == null) return;

      final duplicatedQuest = originalQuest.copyWith(
        id: DateTime.now().millisecondsSinceEpoch.toString(),
        title: '${originalQuest.title} (Copy)',
        createdAt: DateTime.now(),
        updatedAt: null,
        completedAt: null,
      );

      await createQuest(duplicatedQuest);
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }

  Future<void> archiveQuest(String questId) async {
    try {
      final quest = await _repository.getQuestById(questId);
      if (quest == null) return;

      final archivedQuest = quest.copyWith(
        isArchived: true,
        updatedAt: DateTime.now(),
      );

      await _repository.updateQuest(archivedQuest);

      final newQuests = List<Quest>.from(state.quests);
      final index = newQuests.indexWhere((q) => q.id == questId);
      if (index != -1) {
        newQuests[index] = archivedQuest;
      }

      final newQuestsByColumn = Map<QuestColumn, List<Quest>>.from(state.questsByColumn);
      newQuestsByColumn[quest.column] = newQuestsByColumn[quest.column]!
          .where((q) => q.id != questId)
          .toList();

      state = state.copyWith(
        quests: newQuests,
        questsByColumn: newQuestsByColumn,
      );
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }

  Future<void> updateQuest(Quest quest) async {
    try {
      final updatedQuest = await _repository.updateQuest(quest);
      final newQuests = List<Quest>.from(state.quests);
      final index = newQuests.indexWhere((q) => q.id == quest.id);
      if (index != -1) {
        newQuests[index] = updatedQuest;
      } else {
        newQuests.add(updatedQuest);
      }

      final newQuestsByColumn = <QuestColumn, List<Quest>>{};
      for (final column in QuestColumn.values) {
        newQuestsByColumn[column] = newQuests
            .where((q) => q.column == column && !q.isArchived)
            .toList();
      }

      state = state.copyWith(
        quests: newQuests,
        questsByColumn: newQuestsByColumn,
      );
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }
}

/// Board state provider
final questBoardProvider = NotifierProvider<QuestBoardNotifier, QuestBoardState>(
  QuestBoardNotifier.new,
);
