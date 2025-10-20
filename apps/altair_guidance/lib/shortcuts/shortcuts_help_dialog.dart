/// Keyboard shortcuts help dialog.
library;

import 'package:altair_ui/altair_ui.dart';
import 'package:flutter/material.dart';

import 'shortcuts_config.dart';

/// Dialog showing all available keyboard shortcuts.
class ShortcutsHelpDialog extends StatelessWidget {
  /// Creates a shortcuts help dialog.
  const ShortcutsHelpDialog({super.key});

  @override
  Widget build(BuildContext context) {
    return Dialog(
      backgroundColor: Theme.of(context).scaffoldBackgroundColor,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.zero,
        side: BorderSide(
          color: Colors.black,
          width: AltairBorders.thick,
        ),
      ),
      child: Container(
        constraints: const BoxConstraints(maxWidth: 600, maxHeight: 700),
        padding: const EdgeInsets.all(AltairSpacing.lg),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          children: [
            // Header
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text(
                  'Keyboard Shortcuts',
                  style: Theme.of(context).textTheme.headlineMedium?.copyWith(
                        fontWeight: FontWeight.bold,
                      ),
                ),
                IconButton(
                  icon: const Icon(Icons.close),
                  onPressed: () => Navigator.of(context).pop(),
                ),
              ],
            ),
            const SizedBox(height: AltairSpacing.md),

            // Shortcuts list
            Expanded(
              child: ListView.separated(
                shrinkWrap: true,
                itemCount: ShortcutsConfig.shortcutDescriptions.length,
                separatorBuilder: (context, index) =>
                    const SizedBox(height: AltairSpacing.lg),
                itemBuilder: (context, index) {
                  final category = ShortcutsConfig.shortcutDescriptions[index];
                  return _ShortcutCategory(category: category);
                },
              ),
            ),
          ],
        ),
      ),
    );
  }
}

/// Widget displaying a category of shortcuts.
class _ShortcutCategory extends StatelessWidget {
  const _ShortcutCategory({required this.category});

  final ShortcutDescription category;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Category title
        Text(
          category.category,
          style: Theme.of(context).textTheme.titleLarge?.copyWith(
                fontWeight: FontWeight.bold,
                color: AltairColors.accentYellow,
              ),
        ),
        const SizedBox(height: AltairSpacing.sm),

        // Shortcuts in this category
        ...category.shortcuts.map(
          (shortcut) => Padding(
            padding: const EdgeInsets.symmetric(vertical: AltairSpacing.xs),
            child: Row(
              children: [
                // Key combination
                Expanded(
                  flex: 2,
                  child: Container(
                    padding: const EdgeInsets.symmetric(
                      horizontal: AltairSpacing.sm,
                      vertical: AltairSpacing.xs,
                    ),
                    decoration: BoxDecoration(
                      color: Theme.of(context).cardColor,
                      border: Border.all(
                        color: Colors.black,
                        width: AltairBorders.medium,
                      ),
                    ),
                    child: Text(
                      shortcut.$1,
                      style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                            fontFamily: 'monospace',
                            fontWeight: FontWeight.bold,
                          ),
                    ),
                  ),
                ),
                const SizedBox(width: AltairSpacing.md),

                // Description
                Expanded(
                  flex: 3,
                  child: Text(
                    shortcut.$2,
                    style: Theme.of(context).textTheme.bodyMedium,
                  ),
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }
}

/// Shows the keyboard shortcuts help dialog.
void showShortcutsHelp(BuildContext context) {
  showDialog<void>(
    context: context,
    builder: (context) => const ShortcutsHelpDialog(),
  );
}
