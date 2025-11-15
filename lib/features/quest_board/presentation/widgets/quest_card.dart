import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../domain/entities/quest.dart';
import 'energy_indicator.dart';
import '../providers/drag_provider.dart';

/// Draggable quest card widget
class QuestCard extends ConsumerWidget {
  final Quest quest;
  final bool isDragging;
  final VoidCallback? onTap;
  final VoidCallback? onDoubleTap;

  const QuestCard({
    super.key,
    required this.quest,
    this.isDragging = false,
    this.onTap,
    this.onDoubleTap,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final isEpic = quest.epicId == null;
    final isSubquest = quest.subquests.isNotEmpty;

    final cardContent = Card(
      elevation: isDragging ? 8 : 4,
      margin: const EdgeInsets.only(bottom: 8),
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(8),
        side: const BorderSide(color: Colors.black, width: 2),
      ),
      child: InkWell(
        onTap: onTap,
        onDoubleTap: onDoubleTap,
        borderRadius: BorderRadius.circular(8),
        child: Padding(
          padding: const EdgeInsets.all(12),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              // Header with icon and title
              Row(
                children: [
                  Icon(
                    isEpic
                        ? Icons.landscape
                        : isSubquest
                            ? Icons.check_box_outline_blank
                            : Icons.flag,
                    size: 16,
                  ),
                  const SizedBox(width: 8),
                  Expanded(
                    child: Text(
                      quest.title,
                      style: const TextStyle(
                        fontSize: 14,
                        fontWeight: FontWeight.w500,
                      ),
                      maxLines: 2,
                      overflow: TextOverflow.ellipsis,
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 8),
              // Energy indicator
              EnergyIndicator(energyPoints: quest.energyPoints),
              // Tags if any
              if (quest.tags.isNotEmpty) ...[
                const SizedBox(height: 8),
                Wrap(
                  spacing: 4,
                  runSpacing: 4,
                  children: quest.tags.take(3).map((tag) {
                    return Chip(
                      label: Text(
                        tag,
                        style: const TextStyle(fontSize: 10),
                      ),
                      padding: EdgeInsets.zero,
                      materialTapTargetSize: MaterialTapTargetSize.shrinkWrap,
                    );
                  }).toList(),
                ),
              ],
              // Subquest count if any
              if (quest.subquests.isNotEmpty) ...[
                const SizedBox(height: 8),
                Text(
                  '${quest.subquests.length} subquest${quest.subquests.length > 1 ? 's' : ''}',
                  style: TextStyle(
                    fontSize: 12,
                    color: Colors.grey.shade600,
                  ),
                ),
              ],
            ],
          ),
        ),
      ),
    );

    // Make the card draggable using Flutter's built-in Draggable
    return Draggable<String>(
      data: quest.id,
      feedback: Material(
        elevation: 8,
        borderRadius: BorderRadius.circular(8),
        child: Opacity(
          opacity: 0.8,
          child: SizedBox(
            width: 260,
            child: cardContent,
          ),
        ),
      ),
      childWhenDragging: Opacity(
        opacity: 0.3,
        child: cardContent,
      ),
      onDragStarted: () {
        // Update drag state when drag starts
        ref.read(dragStateProvider.notifier).setDragState(
              DragState(
                questId: quest.id,
                sourceColumn: quest.column,
              ),
            );
      },
      onDragEnd: (details) {
        // Clear drag state when drag ends
        ref.read(dragStateProvider.notifier).clearDragState();
      },
      child: cardContent,
    );
  }
}
