/// Keyboard shortcut intents for Altair Guidance.
library;

import 'package:flutter/widgets.dart';

/// Intent to create a new task.
class NewTaskIntent extends Intent {
  /// Creates a new task intent.
  const NewTaskIntent();
}

/// Intent to focus on the quick capture field.
class FocusQuickCaptureIntent extends Intent {
  /// Creates a focus quick capture intent.
  const FocusQuickCaptureIntent();
}

/// Intent to search tasks.
class SearchTasksIntent extends Intent {
  /// Creates a search tasks intent.
  const SearchTasksIntent();
}

/// Intent to toggle focus mode.
class ToggleFocusModeIntent extends Intent {
  /// Creates a toggle focus mode intent.
  const ToggleFocusModeIntent();
}

/// Intent to show keyboard shortcuts help.
class ShowShortcutsHelpIntent extends Intent {
  /// Creates a show shortcuts help intent.
  const ShowShortcutsHelpIntent();
}

/// Intent to navigate to projects page.
class NavigateToProjectsIntent extends Intent {
  /// Creates a navigate to projects intent.
  const NavigateToProjectsIntent();
}

/// Intent to navigate to tasks page.
class NavigateToTasksIntent extends Intent {
  /// Creates a navigate to tasks intent.
  const NavigateToTasksIntent();
}

/// Intent to refresh the current view.
class RefreshIntent extends Intent {
  /// Creates a refresh intent.
  const RefreshIntent();
}

/// Intent to delete the selected task.
class DeleteSelectedTaskIntent extends Intent {
  /// Creates a delete selected task intent.
  const DeleteSelectedTaskIntent();
}

/// Intent to mark task as complete.
class CompleteTaskIntent extends Intent {
  /// Creates a complete task intent.
  const CompleteTaskIntent();
}
