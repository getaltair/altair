import 'package:flutter/material.dart';

/// Board header with title and view switcher
class BoardHeader extends StatelessWidget {
  final VoidCallback? onNewQuest;
  final VoidCallback? onUndo;
  final VoidCallback? onRedo;
  final bool canUndo;
  final bool canRedo;

  const BoardHeader({
    super.key,
    this.onNewQuest,
    this.onUndo,
    this.onRedo,
    this.canUndo = false,
    this.canRedo = false,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
      decoration: BoxDecoration(
        color: Colors.white,
        border: Border(
          bottom: BorderSide(color: Colors.grey.shade300),
        ),
      ),
      child: Row(
        children: [
          const Text(
            'Quest Board',
            style: TextStyle(
              fontSize: 24,
              fontWeight: FontWeight.bold,
            ),
          ),
          const Spacer(),
          // Undo/Redo buttons
          IconButton(
            icon: const Icon(Icons.undo),
            onPressed: canUndo ? onUndo : null,
            tooltip: 'Undo',
          ),
          IconButton(
            icon: const Icon(Icons.redo),
            onPressed: canRedo ? onRedo : null,
            tooltip: 'Redo',
          ),
          const SizedBox(width: 8),
          // New Quest button
          ElevatedButton.icon(
            onPressed: onNewQuest,
            icon: const Icon(Icons.add),
            label: const Text('New Quest'),
          ),
        ],
      ),
    );
  }
}

