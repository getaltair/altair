/// Keyboard shortcuts configuration for Altair Guidance.
library;

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import 'intents.dart';

/// Configuration for keyboard shortcuts.
class ShortcutsConfig {
  /// Default keyboard shortcuts map.
  static Map<ShortcutActivator, Intent> get defaultShortcuts => {
        // New task
        const SingleActivator(LogicalKeyboardKey.keyN, control: true):
            const NewTaskIntent(),
        const SingleActivator(LogicalKeyboardKey.keyN, meta: true):
            const NewTaskIntent(),

        // Focus quick capture
        const SingleActivator(LogicalKeyboardKey.keyK, control: true):
            const FocusQuickCaptureIntent(),
        const SingleActivator(LogicalKeyboardKey.keyK, meta: true):
            const FocusQuickCaptureIntent(),

        // Search
        const SingleActivator(LogicalKeyboardKey.keyF, control: true):
            const SearchTasksIntent(),
        const SingleActivator(LogicalKeyboardKey.keyF, meta: true):
            const SearchTasksIntent(),

        // Toggle focus mode
        const SingleActivator(LogicalKeyboardKey.keyD, control: true):
            const ToggleFocusModeIntent(),
        const SingleActivator(LogicalKeyboardKey.keyD, meta: true):
            const ToggleFocusModeIntent(),

        // Show shortcuts help
        const SingleActivator(LogicalKeyboardKey.slash, shift: true):
            const ShowShortcutsHelpIntent(),
        const SingleActivator(LogicalKeyboardKey.slash, control: true):
            const ShowShortcutsHelpIntent(),

        // Navigation
        const SingleActivator(LogicalKeyboardKey.digit1, control: true):
            const NavigateToTasksIntent(),
        const SingleActivator(LogicalKeyboardKey.digit1, meta: true):
            const NavigateToTasksIntent(),
        const SingleActivator(LogicalKeyboardKey.digit2, control: true):
            const NavigateToProjectsIntent(),
        const SingleActivator(LogicalKeyboardKey.digit2, meta: true):
            const NavigateToProjectsIntent(),

        // Refresh
        const SingleActivator(LogicalKeyboardKey.keyR, control: true):
            const RefreshIntent(),
        const SingleActivator(LogicalKeyboardKey.keyR, meta: true):
            const RefreshIntent(),

        // Delete
        const SingleActivator(LogicalKeyboardKey.delete):
            const DeleteSelectedTaskIntent(),
        const SingleActivator(LogicalKeyboardKey.backspace, meta: true):
            const DeleteSelectedTaskIntent(),

        // Complete task
        const SingleActivator(LogicalKeyboardKey.enter, control: true):
            const CompleteTaskIntent(),
        const SingleActivator(LogicalKeyboardKey.enter, meta: true):
            const CompleteTaskIntent(),
      };

  /// Shortcuts descriptions for help dialog.
  static List<ShortcutDescription> get shortcutDescriptions => [
        ShortcutDescription(
          category: 'Task Management',
          shortcuts: [
            ('Ctrl/Cmd + N', 'Create new task'),
            ('Ctrl/Cmd + K', 'Focus quick capture'),
            ('Ctrl/Cmd + Enter', 'Mark task as complete'),
            ('Delete', 'Delete selected task'),
          ],
        ),
        ShortcutDescription(
          category: 'Navigation',
          shortcuts: [
            ('Ctrl/Cmd + 1', 'Go to Tasks'),
            ('Ctrl/Cmd + 2', 'Go to Projects'),
            ('Ctrl/Cmd + R', 'Refresh view'),
          ],
        ),
        ShortcutDescription(
          category: 'Search & Focus',
          shortcuts: [
            ('Ctrl/Cmd + F', 'Search tasks'),
            ('Ctrl/Cmd + D', 'Toggle focus mode'),
          ],
        ),
        ShortcutDescription(
          category: 'Help',
          shortcuts: [
            ('Shift + ?', 'Show keyboard shortcuts'),
          ],
        ),
      ];
}

/// Description of a keyboard shortcut category.
class ShortcutDescription {
  /// Creates a shortcut description.
  const ShortcutDescription({
    required this.category,
    required this.shortcuts,
  });

  /// Category name.
  final String category;

  /// List of (key combination, description) pairs.
  final List<(String, String)> shortcuts;
}
