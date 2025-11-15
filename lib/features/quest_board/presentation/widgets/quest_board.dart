import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../domain/entities/quest.dart' as domain;
import 'quest_column.dart';
import 'filter_bar.dart';
import '../providers/board_state_provider.dart';

/// Main board container widget with 6 columns
class QuestBoard extends ConsumerStatefulWidget {
  const QuestBoard({super.key});

  @override
  ConsumerState<QuestBoard> createState() => _QuestBoardState();
}

class _QuestBoardState extends ConsumerState<QuestBoard> {
  @override
  Widget build(BuildContext context) {
    final boardState = ref.watch(questBoardProvider);

    if (boardState.isLoading) {
      return const Center(child: CircularProgressIndicator());
    }

    if (boardState.error != null) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text('Error: ${boardState.error}'),
            ElevatedButton(
              onPressed: () {
                ref.read(questBoardProvider.notifier).loadQuests();
              },
              child: const Text('Retry'),
            ),
          ],
        ),
      );
    }

    return Column(
      children: [
        const FilterBar(),
        Expanded(
          child: SingleChildScrollView(
            scrollDirection: Axis.horizontal,
            padding: const EdgeInsets.all(16),
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                // Idea Greenhouse
                QuestColumnWidget(
                  column: domain.QuestColumn.ideaGreenhouse,
                  title: 'Idea Greenhouse',
                  backgroundColor: const Color(0xFFE8F5E9),
                ),
                // Quest Log
                QuestColumnWidget(
                  column: domain.QuestColumn.questLog,
                  title: 'Quest Log',
                  backgroundColor: const Color(0xFFE3F2FD),
                ),
                // This Cycle's Quest
                QuestColumnWidget(
                  column: domain.QuestColumn.thisCycle,
                  title: "This Cycle's Quest",
                  backgroundColor: const Color(0xFFFFF3E0),
                ),
                // Next Up
                QuestColumnWidget(
                  column: domain.QuestColumn.nextUp,
                  title: 'Next Up',
                  backgroundColor: const Color(0xFFFCE4EC),
                ),
                // In-Progress (WIP=1)
                QuestColumnWidget(
                  column: domain.QuestColumn.inProgress,
                  title: 'In-Progress',
                  backgroundColor: const Color(0xFFF3E5F5),
                  wipLimit: 1,
                ),
                // Harvested
                QuestColumnWidget(
                  column: domain.QuestColumn.harvested,
                  title: 'Harvested',
                  backgroundColor: const Color(0xFFF5F5F5).withOpacity(0.5),
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }
}

