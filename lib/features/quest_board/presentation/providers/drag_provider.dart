import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../domain/entities/quest.dart';

/// State for drag operations
class DragState {
  final String questId;
  final QuestColumn sourceColumn;
  final QuestColumn? targetColumn;

  DragState({
    required this.questId,
    required this.sourceColumn,
    this.targetColumn,
  });

  DragState copyWith({
    String? questId,
    QuestColumn? sourceColumn,
    QuestColumn? targetColumn,
  }) {
    return DragState(
      questId: questId ?? this.questId,
      sourceColumn: sourceColumn ?? this.sourceColumn,
      targetColumn: targetColumn ?? this.targetColumn,
    );
  }
}

/// Provider for drag state
class DragStateNotifier extends Notifier<DragState?> {
  @override
  DragState? build() {
    return null;
  }

  void setDragState(DragState? state) {
    this.state = state;
  }

  void clearDragState() {
    state = null;
  }
}

final dragStateProvider = NotifierProvider<DragStateNotifier, DragState?>(
  DragStateNotifier.new,
);
