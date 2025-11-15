import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../domain/entities/quest.dart';

/// Filter state for the board
class BoardFilters {
  final Set<int>? energyLevels; // null means all
  final Set<String>? tags; // null means all
  final String? epicId; // null means all
  final String? assigneeId; // null means all
  final bool showArchived;

  BoardFilters({
    this.energyLevels,
    this.tags,
    this.epicId,
    this.assigneeId,
    this.showArchived = false,
  });

  BoardFilters copyWith({
    Set<int>? energyLevels,
    Set<String>? tags,
    String? epicId,
    String? assigneeId,
    bool? showArchived,
  }) {
    return BoardFilters(
      energyLevels: energyLevels ?? this.energyLevels,
      tags: tags ?? this.tags,
      epicId: epicId ?? this.epicId,
      assigneeId: assigneeId ?? this.assigneeId,
      showArchived: showArchived ?? this.showArchived,
    );
  }

  /// Check if a quest matches the filters
  bool matches(Quest quest) {
    if (!showArchived && quest.isArchived) return false;
    if (energyLevels != null && !energyLevels!.contains(quest.energyPoints)) {
      return false;
    }
    if (tags != null && !tags!.any((tag) => quest.tags.contains(tag))) {
      return false;
    }
    if (epicId != null && quest.epicId != epicId) {
      return false;
    }
    if (assigneeId != null && quest.assigneeId != assigneeId) {
      return false;
    }
    return true;
  }

  bool get hasActiveFilters {
    return energyLevels != null ||
        tags != null ||
        epicId != null ||
        assigneeId != null ||
        showArchived;
  }
}

/// Provider for active filters
class ActiveFiltersNotifier extends Notifier<BoardFilters> {
  @override
  BoardFilters build() {
    return BoardFilters();
  }

  void updateFilters(BoardFilters filters) {
    state = filters;
  }

  void clearFilters() {
    state = BoardFilters();
  }
}

final activeFiltersProvider = NotifierProvider<ActiveFiltersNotifier, BoardFilters>(
  ActiveFiltersNotifier.new,
);
