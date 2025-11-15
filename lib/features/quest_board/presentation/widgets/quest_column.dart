import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../domain/entities/quest.dart' as domain;
import 'quest_card.dart';
import 'wip_limit_badge.dart';
import '../providers/board_state_provider.dart';
import '../providers/filter_provider.dart';
import '../providers/drag_provider.dart';

/// Individual column component for the quest board
class QuestColumnWidget extends ConsumerWidget {
  final domain.QuestColumn column;
  final String title;
  final Color backgroundColor;
  final int? wipLimit;

  const QuestColumnWidget({
    super.key,
    required this.column,
    required this.title,
    required this.backgroundColor,
    this.wipLimit,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final boardState = ref.watch(questBoardProvider);
    final filters = ref.watch(activeFiltersProvider);
    final dragState = ref.watch(dragStateProvider);

    final quests = boardState.getFilteredQuestsForColumn(column, filters);
    final currentCount = quests.length;
    final isWipViolation = wipLimit != null && currentCount > wipLimit!;
    final isInProgress = column == domain.QuestColumn.inProgress;

    return Container(
      width: 280,
      margin: const EdgeInsets.symmetric(horizontal: 8),
      decoration: BoxDecoration(
        color: backgroundColor,
        borderRadius: BorderRadius.circular(8),
        border: isInProgress && isWipViolation
            ? Border.all(color: Colors.red, width: 4)
            : isInProgress
                ? Border.all(color: Colors.purple.shade300, width: 4)
                : null,
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // Column header
          Container(
            padding: const EdgeInsets.all(12),
            decoration: BoxDecoration(
              color: backgroundColor.withOpacity(0.7),
              borderRadius: const BorderRadius.only(
                topLeft: Radius.circular(8),
                topRight: Radius.circular(8),
              ),
            ),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Expanded(
                  child: Text(
                    title,
                    style: const TextStyle(
                      fontSize: 16,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                ),
                if (wipLimit != null)
                  WipLimitBadge(
                    currentCount: currentCount,
                    limit: wipLimit!,
                    isViolation: isWipViolation,
                  ),
              ],
            ),
          ),
          // Quest cards
          Expanded(
            child: quests.isEmpty
                ? Center(
                    child: Text(
                      'No quests',
                      style: TextStyle(
                        color: Colors.grey.shade600,
                        fontSize: 12,
                      ),
                    ),
                  )
                : ListView.builder(
                    padding: const EdgeInsets.all(8),
                    itemCount: quests.length,
                    itemBuilder: (context, index) {
                      final quest = quests[index];
                      final isDragging = dragState?.questId == quest.id;
                      return QuestCard(
                        quest: quest,
                        isDragging: isDragging,
                        onTap: () {
                          // TODO: Open quest detail view
                        },
                        onDoubleTap: () {
                          // TODO: Open quest detail view
                        },
                      );
                    },
                  ),
          ),
        ],
      ),
    );
  }
}
