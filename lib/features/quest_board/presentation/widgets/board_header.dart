import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../providers/bulk_operations_provider.dart';

/// Board header with title and view switcher
class BoardHeader extends ConsumerWidget {
  final VoidCallback? onNewQuest;
  final VoidCallback? onUndo;
  final VoidCallback? onRedo;
  final bool canUndo;
  final bool canRedo;
  final VoidCallback? onBulkMove;
  final VoidCallback? onBulkDelete;
  final VoidCallback? onBulkArchive;

  const BoardHeader({
    super.key,
    this.onNewQuest,
    this.onUndo,
    this.onRedo,
    this.canUndo = false,
    this.canRedo = false,
    this.onBulkMove,
    this.onBulkDelete,
    this.onBulkArchive,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final selectedCount = ref.watch(bulkSelectionProvider).length;
    final hasSelection = selectedCount > 0;

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
          if (hasSelection) ...[
            const SizedBox(width: 16),
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
              decoration: BoxDecoration(
                color: Colors.blue.shade100,
                borderRadius: BorderRadius.circular(16),
              ),
              child: Text(
                '$selectedCount selected',
                style: TextStyle(
                  color: Colors.blue.shade900,
                  fontWeight: FontWeight.w600,
                ),
              ),
            ),
            const SizedBox(width: 8),
            TextButton.icon(
              onPressed: onBulkMove,
              icon: const Icon(Icons.arrow_forward, size: 18),
              label: const Text('Move'),
            ),
            TextButton.icon(
              onPressed: onBulkDelete,
              icon: const Icon(Icons.delete, size: 18),
              label: const Text('Delete'),
            ),
            TextButton.icon(
              onPressed: onBulkArchive,
              icon: const Icon(Icons.archive, size: 18),
              label: const Text('Archive'),
            ),
            const SizedBox(width: 8),
            TextButton(
              onPressed: () {
                ref.read(bulkSelectionProvider.notifier).clearSelection();
              },
              child: const Text('Clear'),
            ),
          ],
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

