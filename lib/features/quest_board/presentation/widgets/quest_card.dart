import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../domain/entities/quest.dart';
import 'energy_indicator.dart';
import '../providers/drag_provider.dart';
import '../providers/board_state_provider.dart';

/// Draggable quest card widget
class QuestCard extends ConsumerWidget {
  final Quest quest;
  final bool isDragging;
  final bool isKeyboardFocused;
  final VoidCallback? onTap;
  final VoidCallback? onDoubleTap;

  const QuestCard({
    super.key,
    required this.quest,
    this.isDragging = false,
    this.isKeyboardFocused = false,
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
        side: BorderSide(
          color: isKeyboardFocused ? Colors.orange : Colors.black,
          width: isKeyboardFocused ? 3 : 2,
        ),
      ),
      child: GestureDetector(
        onTap: onTap,
        onDoubleTap: onDoubleTap,
        onSecondaryTap: () {
          // Right-click context menu
          _showContextMenu(context, ref);
        },
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

  void _showContextMenu(BuildContext context, WidgetRef ref) {
    final renderBox = context.findRenderObject() as RenderBox?;
    if (renderBox == null) return;

    final offset = renderBox.localToGlobal(Offset.zero);
    
    showMenu<String>(
      context: context,
      position: RelativeRect.fromLTRB(
        offset.dx,
        offset.dy,
        offset.dx + renderBox.size.width,
        offset.dy + renderBox.size.height,
      ),
      items: [
        const PopupMenuItem<String>(
          value: 'edit',
          child: Row(
            children: [
              Icon(Icons.edit, size: 20),
              SizedBox(width: 8),
              Text('Edit Quest'),
            ],
          ),
        ),
        const PopupMenuItem<String>(
          value: 'delete',
          child: Row(
            children: [
              Icon(Icons.delete, size: 20),
              SizedBox(width: 8),
              Text('Delete Quest'),
            ],
          ),
        ),
        const PopupMenuItem<String>(
          value: 'duplicate',
          child: Row(
            children: [
              Icon(Icons.copy, size: 20),
              SizedBox(width: 8),
              Text('Duplicate Quest'),
            ],
          ),
        ),
        const PopupMenuDivider(),
        const PopupMenuItem<String>(
          value: 'archive',
          child: Row(
            children: [
              Icon(Icons.archive, size: 20),
              SizedBox(width: 8),
              Text('Archive Quest'),
            ],
          ),
        ),
      ],
    ).then((value) {
      if (value != null) {
        switch (value) {
          case 'edit':
            _showEditDialog(context, ref);
            break;
          case 'delete':
            _showDeleteConfirmation(context, ref);
            break;
          case 'duplicate':
            ref.read(questBoardProvider.notifier).duplicateQuest(quest.id);
            break;
          case 'archive':
            ref.read(questBoardProvider.notifier).archiveQuest(quest.id);
            break;
        }
      }
    });
  }

  void _showEditDialog(BuildContext context, WidgetRef ref) {
    showDialog(
      context: context,
      builder: (context) => _EditQuestDialog(
        quest: quest,
        onSave: (title, energyPoints) {
          final updatedQuest = quest.copyWith(
            title: title,
            energyPoints: energyPoints,
            updatedAt: DateTime.now(),
          );
          ref.read(questBoardProvider.notifier).updateQuest(updatedQuest);
        },
      ),
    );
  }

  void _showDeleteConfirmation(BuildContext context, WidgetRef ref) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Delete Quest'),
        content: Text('Are you sure you want to delete "${quest.title}"? This action cannot be undone.'),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              ref.read(questBoardProvider.notifier).deleteQuest(quest.id);
              Navigator.pop(context);
            },
            style: TextButton.styleFrom(foregroundColor: Colors.red),
            child: const Text('Delete'),
          ),
        ],
      ),
    );
  }
}

class _EditQuestDialog extends StatefulWidget {
  final Quest quest;
  final Function(String title, int energyPoints) onSave;

  const _EditQuestDialog({
    required this.quest,
    required this.onSave,
  });

  @override
  State<_EditQuestDialog> createState() => _EditQuestDialogState();
}

class _EditQuestDialogState extends State<_EditQuestDialog> {
  late final TextEditingController _titleController;
  late int _energyPoints;

  @override
  void initState() {
    super.initState();
    _titleController = TextEditingController(text: widget.quest.title);
    _energyPoints = widget.quest.energyPoints;
  }

  @override
  void dispose() {
    _titleController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Edit Quest'),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          TextField(
            controller: _titleController,
            decoration: const InputDecoration(
              labelText: 'Quest Title',
              border: OutlineInputBorder(),
            ),
            autofocus: true,
            onSubmitted: (value) {
              if (value.isNotEmpty) {
                widget.onSave(value, _energyPoints);
                Navigator.pop(context);
              }
            },
          ),
          const SizedBox(height: 16),
          const Text('Energy Points:'),
          Row(
            children: List.generate(5, (index) {
              final level = index + 1;
              return Expanded(
                child: RadioListTile<int>(
                  title: Text('$level'),
                  value: level,
                  groupValue: _energyPoints,
                  onChanged: (value) {
                    setState(() {
                      _energyPoints = value!;
                    });
                  },
                ),
              );
            }),
          ),
        ],
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: () {
            if (_titleController.text.isNotEmpty) {
              widget.onSave(_titleController.text, _energyPoints);
              Navigator.pop(context);
            }
          },
          child: const Text('Save'),
        ),
      ],
    );
  }
}
