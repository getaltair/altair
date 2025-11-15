import 'package:flutter_riverpod/flutter_riverpod.dart';

/// Provider for bulk selection state
class BulkSelectionNotifier extends Notifier<Set<String>> {
  @override
  Set<String> build() {
    return {};
  }

  void toggleSelection(String questId) {
    final current = Set<String>.from(state);
    if (current.contains(questId)) {
      current.remove(questId);
    } else {
      current.add(questId);
    }
    state = current;
  }

  void selectAll(List<String> questIds) {
    state = Set<String>.from(questIds);
  }

  void clearSelection() {
    state = {};
  }

  bool isSelected(String questId) {
    return state.contains(questId);
  }

  int get selectedCount => state.length;
}

final bulkSelectionProvider =
    NotifierProvider<BulkSelectionNotifier, Set<String>>(
  BulkSelectionNotifier.new,
);

