import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../domain/entities/quest.dart';

/// State for keyboard navigation
class KeyboardNavigationState {
  final QuestColumn? focusedColumn;
  final String? focusedQuestId;
  final bool isDraggingWithKeyboard;

  KeyboardNavigationState({
    this.focusedColumn,
    this.focusedQuestId,
    this.isDraggingWithKeyboard = false,
  });

  KeyboardNavigationState copyWith({
    QuestColumn? focusedColumn,
    String? focusedQuestId,
    bool? isDraggingWithKeyboard,
  }) {
    return KeyboardNavigationState(
      focusedColumn: focusedColumn ?? this.focusedColumn,
      focusedQuestId: focusedQuestId ?? this.focusedQuestId,
      isDraggingWithKeyboard: isDraggingWithKeyboard ?? this.isDraggingWithKeyboard,
    );
  }
}

/// Provider for keyboard navigation state
class KeyboardNavigationNotifier extends Notifier<KeyboardNavigationState> {
  @override
  KeyboardNavigationState build() {
    return KeyboardNavigationState();
  }

  void focusColumn(QuestColumn column) {
    state = state.copyWith(
      focusedColumn: column,
      focusedQuestId: null,
    );
  }

  void focusQuest(String questId, QuestColumn column) {
    state = state.copyWith(
      focusedQuestId: questId,
      focusedColumn: column,
    );
  }

  void startKeyboardDrag(String questId, QuestColumn column) {
    state = state.copyWith(
      focusedQuestId: questId,
      focusedColumn: column,
      isDraggingWithKeyboard: true,
    );
  }

  void stopKeyboardDrag() {
    state = state.copyWith(isDraggingWithKeyboard: false);
  }

  void clearFocus() {
    state = KeyboardNavigationState();
  }

  QuestColumn? getNextColumn(QuestColumn current) {
    final columns = QuestColumn.values;
    final currentIndex = columns.indexOf(current);
    if (currentIndex < columns.length - 1) {
      return columns[currentIndex + 1];
    }
    return null;
  }

  QuestColumn? getPreviousColumn(QuestColumn current) {
    final columns = QuestColumn.values;
    final currentIndex = columns.indexOf(current);
    if (currentIndex > 0) {
      return columns[currentIndex - 1];
    }
    return null;
  }

  QuestColumn? getColumnByNumber(int number) {
    if (number >= 1 && number <= QuestColumn.values.length) {
      return QuestColumn.values[number - 1];
    }
    return null;
  }
}

final keyboardNavigationProvider =
    NotifierProvider<KeyboardNavigationNotifier, KeyboardNavigationState>(
  KeyboardNavigationNotifier.new,
);

