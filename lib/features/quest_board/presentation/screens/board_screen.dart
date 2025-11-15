import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../domain/entities/quest.dart';
import '../widgets/quest_board.dart';
import '../widgets/board_header.dart';
import '../providers/board_state_provider.dart';
import '../providers/keyboard_navigation_provider.dart';
import '../providers/drag_provider.dart';

/// Main board screen
class BoardScreen extends ConsumerStatefulWidget {
  const BoardScreen({super.key});

  @override
  ConsumerState<BoardScreen> createState() => _BoardScreenState();
}

class _BoardScreenState extends ConsumerState<BoardScreen> {
  final FocusNode _boardFocusNode = FocusNode();

  @override
  void initState() {
    super.initState();
    // Load initial quests
    WidgetsBinding.instance.addPostFrameCallback((_) {
      ref.read(questBoardProvider.notifier).loadQuests();
      // Focus the board for keyboard navigation
      _boardFocusNode.requestFocus();
    });
  }

  @override
  void dispose() {
    _boardFocusNode.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final boardState = ref.watch(questBoardProvider);

    return KeyboardListener(
      focusNode: _boardFocusNode,
      autofocus: true,
      onKeyEvent: (event) {
        if (event is KeyDownEvent) {
          _handleKeyboardShortcut(event);
        }
      },
      child: Scaffold(
        body: Column(
          children: [
            BoardHeader(
              onNewQuest: _showNewQuestDialog,
              onUndo: () => ref.read(questBoardProvider.notifier).undo(),
              onRedo: () => ref.read(questBoardProvider.notifier).redo(),
              canUndo: boardState.undoStack.isNotEmpty,
              canRedo: boardState.redoStack.isNotEmpty,
            ),
            Expanded(
              child: QuestBoard(),
            ),
          ],
        ),
        floatingActionButton: FloatingActionButton(
          onPressed: _showNewQuestDialog,
          child: const Icon(Icons.add),
          tooltip: 'New Quest (Ctrl+N)',
        ),
      ),
    );
  }

  void _handleKeyboardShortcut(KeyDownEvent event) {
    final isModifierPressed = HardwareKeyboard.instance.isControlPressed ||
        HardwareKeyboard.instance.isMetaPressed;
    final keyboardNavNotifier = ref.read(keyboardNavigationProvider.notifier);
    final boardNotifier = ref.read(questBoardProvider.notifier);
    final dragNotifier = ref.read(dragStateProvider.notifier);
    final keyboardNav = ref.read(keyboardNavigationProvider);

    // Number keys 1-6: Jump to column
    if (event.logicalKey == LogicalKeyboardKey.digit1 ||
        event.logicalKey == LogicalKeyboardKey.digit2 ||
        event.logicalKey == LogicalKeyboardKey.digit3 ||
        event.logicalKey == LogicalKeyboardKey.digit4 ||
        event.logicalKey == LogicalKeyboardKey.digit5 ||
        event.logicalKey == LogicalKeyboardKey.digit6) {
      final number = int.tryParse(event.logicalKey.keyLabel);
      if (number != null) {
        final column = keyboardNavNotifier.getColumnByNumber(number);
        if (column != null) {
          keyboardNavNotifier.focusColumn(column);
        }
      }
      return;
    }

    // Tab: Navigate to next column
    if (event.logicalKey == LogicalKeyboardKey.tab && !isModifierPressed) {
      final currentColumn = keyboardNav.focusedColumn ?? QuestColumn.ideaGreenhouse;
      final nextColumn = keyboardNavNotifier.getNextColumn(currentColumn);
      if (nextColumn != null) {
        keyboardNavNotifier.focusColumn(nextColumn);
      } else {
        // Wrap to first column
        keyboardNavNotifier.focusColumn(QuestColumn.ideaGreenhouse);
      }
      return;
    }

    // Shift+Tab: Navigate to previous column
    if (event.logicalKey == LogicalKeyboardKey.tab && 
        HardwareKeyboard.instance.isShiftPressed) {
      final currentColumn = keyboardNav.focusedColumn ?? QuestColumn.harvested;
      final prevColumn = keyboardNavNotifier.getPreviousColumn(currentColumn);
      if (prevColumn != null) {
        keyboardNavNotifier.focusColumn(prevColumn);
      } else {
        // Wrap to last column
        keyboardNavNotifier.focusColumn(QuestColumn.harvested);
      }
      return;
    }

    // Arrow keys: Navigate quests or columns
    if (event.logicalKey == LogicalKeyboardKey.arrowRight) {
      if (keyboardNav.isDraggingWithKeyboard && keyboardNav.focusedColumn != null) {
        // Move to next column while dragging
        final nextColumn = keyboardNavNotifier.getNextColumn(keyboardNav.focusedColumn!);
        if (nextColumn != null) {
          keyboardNavNotifier.focusColumn(nextColumn);
        }
      }
      return;
    }

    if (event.logicalKey == LogicalKeyboardKey.arrowLeft) {
      if (keyboardNav.isDraggingWithKeyboard && keyboardNav.focusedColumn != null) {
        // Move to previous column while dragging
        final prevColumn = keyboardNavNotifier.getPreviousColumn(keyboardNav.focusedColumn!);
        if (prevColumn != null) {
          keyboardNavNotifier.focusColumn(prevColumn);
        }
      }
      return;
    }

    // Space: Grab/drop quest
    if (event.logicalKey == LogicalKeyboardKey.space) {
      if (keyboardNav.isDraggingWithKeyboard) {
        // Drop the quest
        if (keyboardNav.focusedQuestId != null && keyboardNav.focusedColumn != null) {
          boardNotifier.moveQuest(keyboardNav.focusedQuestId!, keyboardNav.focusedColumn!);
          keyboardNavNotifier.stopKeyboardDrag();
          dragNotifier.clearDragState();
        }
      } else if (keyboardNav.focusedQuestId != null && keyboardNav.focusedColumn != null) {
        // Start dragging
        keyboardNavNotifier.startKeyboardDrag(
          keyboardNav.focusedQuestId!,
          keyboardNav.focusedColumn!,
        );
        dragNotifier.setDragState(
          DragState(
            questId: keyboardNav.focusedQuestId!,
            sourceColumn: keyboardNav.focusedColumn!,
          ),
        );
      }
      return;
    }

    // Escape: Cancel drag
    if (event.logicalKey == LogicalKeyboardKey.escape) {
      if (keyboardNav.isDraggingWithKeyboard) {
        keyboardNavNotifier.stopKeyboardDrag();
        dragNotifier.clearDragState();
      }
      return;
    }

    // Ctrl+N: New quest
    if (event.logicalKey == LogicalKeyboardKey.keyN && isModifierPressed) {
      _showNewQuestDialog();
      return;
    }

    // Ctrl+F: Focus filter (placeholder - would need filter bar focus)
    if (event.logicalKey == LogicalKeyboardKey.keyF && isModifierPressed) {
      // TODO: Focus filter bar when implemented
      return;
    }
  }

  void _showNewQuestDialog() {
    showDialog(
      context: context,
      builder: (context) => _NewQuestDialog(
        onSave: (title, energyPoints) {
          final quest = Quest(
            id: DateTime.now().millisecondsSinceEpoch.toString(),
            title: title,
            energyPoints: energyPoints,
            column: QuestColumn.ideaGreenhouse,
            createdAt: DateTime.now(),
          );
          ref.read(questBoardProvider.notifier).createQuest(quest);
        },
      ),
    );
  }
}

class _NewQuestDialog extends StatefulWidget {
  final Function(String title, int energyPoints) onSave;

  const _NewQuestDialog({required this.onSave});

  @override
  State<_NewQuestDialog> createState() => _NewQuestDialogState();
}

class _NewQuestDialogState extends State<_NewQuestDialog> {
  final _titleController = TextEditingController();
  int _energyPoints = 3;

  @override
  void dispose() {
    _titleController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Create New Quest'),
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
          child: const Text('Create'),
        ),
      ],
    );
  }
}

