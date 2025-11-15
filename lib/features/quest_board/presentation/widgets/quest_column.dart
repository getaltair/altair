import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../domain/entities/quest.dart' as domain;
import 'quest_card.dart';
import 'wip_limit_badge.dart';
import 'quest_detail_view.dart';
import '../providers/board_state_provider.dart';
import '../providers/filter_provider.dart';
import '../providers/drag_provider.dart';
import '../providers/keyboard_navigation_provider.dart';

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
    final keyboardNav = ref.watch(keyboardNavigationProvider);

    final quests = boardState.getFilteredQuestsForColumn(column, filters);
    final currentCount = quests.length;
    final isWipViolation = wipLimit != null && currentCount > wipLimit!;
    final isInProgress = column == domain.QuestColumn.inProgress;
    final isKeyboardFocused = keyboardNav.focusedColumn == column;

    final columnContent = Container(
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
                      final isKeyboardFocusedQuest =
                          keyboardNav.focusedQuestId == quest.id;
                      return QuestCard(
                        quest: quest,
                        isDragging: isDragging,
                        isKeyboardFocused: isKeyboardFocusedQuest,
                        onTap: () {
                          // Focus quest for keyboard navigation
                          ref
                              .read(keyboardNavigationProvider.notifier)
                              .focusQuest(quest.id, column);
                        },
                        onDoubleTap: () {
                          // Open quest detail view
                          showDialog(
                            context: context,
                            builder: (context) => QuestDetailView(quest: quest),
                          );
                        },
                      );
                    },
                  ),
          ),
        ],
      ),
    );

    // Make the column a drop target using Flutter's built-in DragTarget
    return DragTarget<String>(
      onWillAccept: (data) {
        // Update drag state to show target column
        final currentDragState = dragState;
        if (currentDragState != null && data == currentDragState.questId) {
          ref.read(dragStateProvider.notifier).setDragState(
                currentDragState.copyWith(targetColumn: column),
              );
        }
        return true;
      },
      onLeave: (data) {
        // Clear target column when drag leaves
        final currentDragState = dragState;
        if (currentDragState != null) {
          ref.read(dragStateProvider.notifier).setDragState(
                currentDragState.copyWith(targetColumn: null),
              );
        }
      },
      onAccept: (questId) async {
        // Handle the drop
        // Validate drop (WIP=1 enforcement)
        if (column == domain.QuestColumn.inProgress && wipLimit != null) {
          final currentInProgress =
              boardState.questsByColumn[domain.QuestColumn.inProgress] ?? [];
          if (currentInProgress.isNotEmpty &&
              currentInProgress.first.id != questId) {
            // Show error toast
            ScaffoldMessenger.of(context).showSnackBar(
              const SnackBar(
                content: Text('WIP limit reached! Move current item first.'),
                backgroundColor: Colors.red,
              ),
            );
            ref.read(dragStateProvider.notifier).clearDragState();
            return;
          }
        }

        // Move the quest to the target column
        await ref.read(questBoardProvider.notifier).moveQuest(questId, column);

        // Clear drag state
        ref.read(dragStateProvider.notifier).clearDragState();
      },
      builder: (context, candidateData, rejectedData) {
        final dragState = ref.watch(dragStateProvider);
        final isDropTarget =
            dragState?.targetColumn == column && candidateData.isNotEmpty;

        return AnimatedContainer(
          duration: const Duration(milliseconds: 150),
          decoration: BoxDecoration(
            color: backgroundColor,
            borderRadius: BorderRadius.circular(8),
            border: isDropTarget
                ? Border.all(color: Colors.blue, width: 3)
                : isKeyboardFocused
                    ? Border.all(color: Colors.orange, width: 3)
                    : isInProgress && isWipViolation
                        ? Border.all(color: Colors.red, width: 4)
                        : isInProgress
                            ? Border.all(
                                color: Colors.purple.shade300, width: 4)
                            : null,
          ),
          child: columnContent,
        );
      },
    );
  }
}
