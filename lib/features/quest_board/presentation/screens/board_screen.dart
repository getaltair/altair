import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../domain/entities/quest.dart';
import '../widgets/quest_board.dart';
import '../widgets/board_header.dart';
import '../providers/board_state_provider.dart';

/// Main board screen
class BoardScreen extends ConsumerStatefulWidget {
  const BoardScreen({super.key});

  @override
  ConsumerState<BoardScreen> createState() => _BoardScreenState();
}

class _BoardScreenState extends ConsumerState<BoardScreen> {
  @override
  void initState() {
    super.initState();
    // Load initial quests
    WidgetsBinding.instance.addPostFrameCallback((_) {
      ref.read(questBoardProvider.notifier).loadQuests();
    });
  }

  @override
  Widget build(BuildContext context) {
    final boardState = ref.watch(questBoardProvider);

    return KeyboardListener(
      focusNode: FocusNode(),
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
    
    if (event.logicalKey == LogicalKeyboardKey.keyN && isModifierPressed) {
      _showNewQuestDialog();
    } else if (event.logicalKey == LogicalKeyboardKey.keyF && isModifierPressed) {
      // TODO: Focus filter bar
    } else if (event.logicalKey == LogicalKeyboardKey.escape) {
      // TODO: Cancel drag
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

