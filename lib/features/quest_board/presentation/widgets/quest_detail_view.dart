import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../domain/entities/quest.dart';
import '../../domain/entities/subquest.dart';
import 'energy_indicator.dart';

/// Quest detail view shown on double-click
class QuestDetailView extends ConsumerWidget {
  final Quest quest;

  const QuestDetailView({
    super.key,
    required this.quest,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Dialog(
      child: Container(
        width: 600,
        constraints: const BoxConstraints(maxHeight: 700),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            // Header
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Theme.of(context).primaryColor.withOpacity(0.1),
                border: Border(
                  bottom: BorderSide(color: Colors.grey.shade300),
                ),
              ),
              child: Row(
                children: [
                  Expanded(
                    child: Text(
                      quest.title,
                      style: const TextStyle(
                        fontSize: 20,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ),
                  IconButton(
                    icon: const Icon(Icons.close),
                    onPressed: () => Navigator.pop(context),
                  ),
                ],
              ),
            ),
            // Content
            Flexible(
              child: SingleChildScrollView(
                padding: const EdgeInsets.all(16),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    // Energy points
                    Row(
                      children: [
                        const Text(
                          'Energy Points: ',
                          style: TextStyle(fontWeight: FontWeight.w600),
                        ),
                        EnergyIndicator(energyPoints: quest.energyPoints),
                      ],
                    ),
                    const SizedBox(height: 16),
                    // Description
                    if (quest.description != null && quest.description!.isNotEmpty) ...[
                      const Text(
                        'Description:',
                        style: TextStyle(fontWeight: FontWeight.w600),
                      ),
                      const SizedBox(height: 8),
                      Text(quest.description!),
                      const SizedBox(height: 16),
                    ],
                    // Column
                    Row(
                      children: [
                        const Text(
                          'Column: ',
                          style: TextStyle(fontWeight: FontWeight.w600),
                        ),
                        Chip(
                          label: Text(_getColumnName(quest.column)),
                        ),
                      ],
                    ),
                    const SizedBox(height: 16),
                    // Tags
                    if (quest.tags.isNotEmpty) ...[
                      const Text(
                        'Tags:',
                        style: TextStyle(fontWeight: FontWeight.w600),
                      ),
                      const SizedBox(height: 8),
                      Wrap(
                        spacing: 8,
                        runSpacing: 8,
                        children: quest.tags.map((tag) {
                          return Chip(label: Text(tag));
                        }).toList(),
                      ),
                      const SizedBox(height: 16),
                    ],
                    // Subquests
                    if (quest.subquests.isNotEmpty) ...[
                      const Text(
                        'Subquests:',
                        style: TextStyle(fontWeight: FontWeight.w600),
                      ),
                      const SizedBox(height: 8),
                      ...quest.subquests.map((subquest) {
                        return _SubquestItem(subquest: subquest);
                      }),
                      const SizedBox(height: 16),
                    ],
                    // Metadata
                    const Divider(),
                    const SizedBox(height: 8),
                    Text(
                      'Created: ${_formatDate(quest.createdAt)}',
                      style: TextStyle(
                        fontSize: 12,
                        color: Colors.grey.shade600,
                      ),
                    ),
                    if (quest.updatedAt != null)
                      Text(
                        'Updated: ${_formatDate(quest.updatedAt!)}',
                        style: TextStyle(
                          fontSize: 12,
                          color: Colors.grey.shade600,
                        ),
                      ),
                    if (quest.completedAt != null)
                      Text(
                        'Completed: ${_formatDate(quest.completedAt!)}',
                        style: TextStyle(
                          fontSize: 12,
                          color: Colors.grey.shade600,
                        ),
                      ),
                  ],
                ),
              ),
            ),
            // Actions
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                border: Border(
                  top: BorderSide(color: Colors.grey.shade300),
                ),
              ),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.end,
                children: [
                  ElevatedButton(
                    onPressed: () => Navigator.pop(context),
                    child: const Text('Close'),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  String _getColumnName(QuestColumn column) {
    switch (column) {
      case QuestColumn.ideaGreenhouse:
        return 'Idea Greenhouse';
      case QuestColumn.questLog:
        return 'Quest Log';
      case QuestColumn.thisCycle:
        return "This Cycle's Quest";
      case QuestColumn.nextUp:
        return 'Next Up';
      case QuestColumn.inProgress:
        return 'In-Progress';
      case QuestColumn.harvested:
        return 'Harvested';
    }
  }

  String _formatDate(DateTime date) {
    return '${date.year}-${date.month.toString().padLeft(2, '0')}-${date.day.toString().padLeft(2, '0')} ${date.hour.toString().padLeft(2, '0')}:${date.minute.toString().padLeft(2, '0')}';
  }
}

class _SubquestItem extends StatelessWidget {
  final Subquest subquest;

  const _SubquestItem({required this.subquest});

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(bottom: 8),
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: Colors.grey.shade100,
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: Colors.grey.shade300),
      ),
      child: Row(
        children: [
          Checkbox(
            value: subquest.isCompleted,
            onChanged: null, // Read-only in detail view
          ),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  subquest.title,
                  style: TextStyle(
                    decoration: subquest.isCompleted
                        ? TextDecoration.lineThrough
                        : null,
                  ),
                ),
                if (subquest.description != null && subquest.description!.isNotEmpty)
                  Padding(
                    padding: const EdgeInsets.only(top: 4),
                    child: Text(
                      subquest.description!,
                      style: TextStyle(
                        fontSize: 12,
                        color: Colors.grey.shade600,
                      ),
                    ),
                  ),
              ],
            ),
          ),
          EnergyIndicator(energyPoints: subquest.energyPoints, size: 12),
        ],
      ),
    );
  }
}

