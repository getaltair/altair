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

    final screenWidth = MediaQuery.of(context).size.width;
    
    // Responsive breakpoints
    final isTablet = screenWidth >= 768 && screenWidth <= 1199;
    final isMobile = screenWidth < 768;

    return Column(
      children: [
        const FilterBar(),
        Expanded(
          child: LayoutBuilder(
            builder: (context, constraints) {
              if (isMobile) {
                // Mobile: Single column view with column switcher
                return _MobileBoardView();
              } else if (isTablet) {
                // Tablet: 3 columns visible, horizontal scroll
                return SingleChildScrollView(
                  scrollDirection: Axis.horizontal,
                  padding: const EdgeInsets.all(16),
                  child: Row(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: _buildAllColumns(),
                  ),
                );
              } else {
                // Desktop: All 6 columns visible
                return SingleChildScrollView(
                  scrollDirection: Axis.horizontal,
                  padding: const EdgeInsets.all(16),
                  child: Row(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: _buildAllColumns(),
                  ),
                );
              }
            },
          ),
        ),
      ],
    );
  }

  List<Widget> _buildAllColumns() {
    return [
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
    ];
  }
}

/// Mobile view with column switcher
class _MobileBoardView extends ConsumerStatefulWidget {
  @override
  ConsumerState<_MobileBoardView> createState() => _MobileBoardViewState();
}

class _MobileBoardViewState extends ConsumerState<_MobileBoardView> {
  domain.QuestColumn _selectedColumn = domain.QuestColumn.ideaGreenhouse;

  final List<Map<domain.QuestColumn, String>> _columns = [
    {domain.QuestColumn.ideaGreenhouse: 'Idea Greenhouse'},
    {domain.QuestColumn.questLog: 'Quest Log'},
    {domain.QuestColumn.thisCycle: "This Cycle's Quest"},
    {domain.QuestColumn.nextUp: 'Next Up'},
    {domain.QuestColumn.inProgress: 'In-Progress'},
    {domain.QuestColumn.harvested: 'Harvested'},
  ];

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        // Column switcher
        Container(
          height: 50,
          padding: const EdgeInsets.symmetric(horizontal: 8),
          child: ListView.builder(
            scrollDirection: Axis.horizontal,
            itemCount: _columns.length,
            itemBuilder: (context, index) {
              final columnEntry = _columns[index].entries.first;
              final column = columnEntry.key;
              final title = columnEntry.value;
              final isSelected = column == _selectedColumn;
              
              return Padding(
                padding: const EdgeInsets.symmetric(horizontal: 4),
                child: ChoiceChip(
                  label: Text(title),
                  selected: isSelected,
                  onSelected: (selected) {
                    if (selected) {
                      setState(() {
                        _selectedColumn = column;
                      });
                    }
                  },
                ),
              );
            },
          ),
        ),
        // Selected column view
        Expanded(
          child: Padding(
            padding: const EdgeInsets.all(8),
            child: QuestColumnWidget(
              column: _selectedColumn,
              title: _columns.firstWhere((c) => c.keys.first == _selectedColumn).values.first,
              backgroundColor: _getColumnColor(_selectedColumn),
              wipLimit: _selectedColumn == domain.QuestColumn.inProgress ? 1 : null,
            ),
          ),
        ),
      ],
    );
  }

  Color _getColumnColor(domain.QuestColumn column) {
    switch (column) {
      case domain.QuestColumn.ideaGreenhouse:
        return const Color(0xFFE8F5E9);
      case domain.QuestColumn.questLog:
        return const Color(0xFFE3F2FD);
      case domain.QuestColumn.thisCycle:
        return const Color(0xFFFFF3E0);
      case domain.QuestColumn.nextUp:
        return const Color(0xFFFCE4EC);
      case domain.QuestColumn.inProgress:
        return const Color(0xFFF3E5F5);
      case domain.QuestColumn.harvested:
        return const Color(0xFFF5F5F5).withOpacity(0.5);
    }
  }
}

