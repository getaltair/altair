# Keyboard Shortcuts Reference

> **TL;DR:** Power-user keyboard shortcuts for Altair Guidance. Press `Shift + ?` in the app to see this list.

## Quick Reference Card

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| **Create new task** | `Ctrl + N` | `Cmd + N` |
| **Focus quick capture** | `Ctrl + K` | `Cmd + K` |
| **Search tasks** | `Ctrl + F` | `Cmd + F` |
| **Toggle focus mode** | `Ctrl + D` | `Cmd + D` |
| **Mark task complete** | `Ctrl + Enter` | `Cmd + Enter` |
| **Delete task** | `Delete` | `Delete` (or `Cmd + Backspace`) |
| **Go to Tasks** | `Ctrl + 1` | `Cmd + 1` |
| **Go to Projects** | `Ctrl + 2` | `Cmd + 2` |
| **Refresh view** | `Ctrl + R` | `Cmd + R` |
| **Show shortcuts help** | `Shift + ?` | `Shift + ?` |

---

## Task Management

### Create New Task

- **Windows/Linux:** `Ctrl + N`
- **macOS:** `Cmd + N`
- **Description:** Opens the task creation dialog with full editing capabilities
- **Context:** Works from main task list view

### Focus Quick Capture

- **Windows/Linux:** `Ctrl + K`
- **macOS:** `Cmd + K`
- **Description:** Instantly focuses the quick capture input field for rapid task entry
- **ADHD-friendly:** Designed for < 3 second thought-to-capture workflow
- **Context:** Always available when quick capture is visible

### Mark Task as Complete

- **Windows/Linux:** `Ctrl + Enter`
- **macOS:** `Cmd + Enter`
- **Description:** Marks the currently selected/focused task as complete
- **Context:** When a task is selected or focused

### Delete Task

- **Windows/Linux:** `Delete` or `Ctrl + Backspace`
- **macOS:** `Delete` or `Cmd + Backspace`
- **Description:** Deletes the currently selected task
- **Context:** When a task is selected or focused
- **Note:** Permanent deletion, no undo currently available

---

## Navigation

### Go to Tasks View

- **Windows/Linux:** `Ctrl + 1`
- **macOS:** `Cmd + 1`
- **Description:** Navigate to the main tasks list view
- **Context:** Works from anywhere in the app

### Go to Projects View

- **Windows/Linux:** `Ctrl + 2`
- **macOS:** `Cmd + 2`
- **Description:** Navigate to the projects management view
- **Context:** Works from anywhere in the app

### Refresh Current View

- **Windows/Linux:** `Ctrl + R`
- **macOS:** `Cmd + R`
- **Description:** Reloads tasks/projects from the database
- **Context:** Works in any list view

---

## Search & Focus

### Search Tasks

- **Windows/Linux:** `Ctrl + F`
- **macOS:** `Cmd + F`
- **Description:** Opens the search/filter interface for tasks
- **Context:** Available in task list views
- **Note:** Feature coming soon

### Toggle Focus Mode

- **Windows/Linux:** `Ctrl + D`
- **macOS:** `Cmd + D`
- **Description:** Toggles ADHD-friendly focus mode on/off
- **ADHD-friendly:** Minimizes distractions by hiding UI elements
- **What it hides:**
  - Navigation drawer/sidebar
  - Filter buttons
  - Keyboard shortcuts button
  - Floating action button
- **What remains visible:**
  - Task list
  - Quick capture field
  - App bar with focus mode toggle

---

## Help

### Show Keyboard Shortcuts Help

- **Shortcut:** `Shift + ?`
- **Alternative:** `Ctrl + /` (Windows/Linux) or `Cmd + /` (macOS)
- **Description:** Displays the in-app keyboard shortcuts help dialog
- **Context:** Available from anywhere in the app
- **Note:** Can also access via keyboard icon in app bar

---

## ADHD-Friendly Design Principles

The keyboard shortcuts system is designed with ADHD users in mind:

### 1. **Muscle Memory**

- Common shortcuts use standard patterns (Ctrl+N for new, Ctrl+F for find)
- Consistent across Windows/Linux/macOS (only modifier key changes)

### 2. **Discoverability**

- `Shift + ?` shows all shortcuts (universal help pattern)
- Keyboard icon in app bar for visual reminder
- Tooltips show shortcuts (e.g., "Focus quick capture (Ctrl/Cmd + K)")

### 3. **Speed**

- Quick capture focus (`Ctrl + K`) is one keystroke away
- No nested menus or complex key combinations
- Single-key actions where safe (Delete for delete)

### 4. **Focus Mode Integration**

- `Ctrl + D` instantly removes distractions
- Toggle on/off without losing context
- Visual indicator when active (yellow icon)

### 5. **Forgiveness**

- Undo/redo system (coming soon)
- Confirmation for destructive actions
- Clear visual feedback for all actions

---

## Tips for Power Users

### Quick Capture Workflow

1. `Ctrl + K` - Focus quick capture
2. Type your task
3. `Enter` - Submit task
4. Repeat (focus automatically returns)

**Result:** < 3 second capture time, perfect for catching fleeting thoughts

### Focus Mode for Deep Work

1. `Ctrl + D` - Enter focus mode
2. Work on your tasks without distractions
3. `Ctrl + D` - Exit when done

**Result:** Minimal UI, maximum concentration

### Navigation Speed

- `Ctrl + 1` - Tasks
- `Ctrl + 2` - Projects
- `Ctrl + R` - Refresh if needed

**Result:** Navigate entire app without touching mouse

### Keyboard-Only Task Management

1. `Ctrl + K` - Quick capture new task
2. `Tab` to navigate to task list
3. `Arrow keys` to select task
4. `Enter` to edit (or `Ctrl + Enter` to complete)
5. `Delete` to remove

**Result:** Complete task management without mouse

---

## Customization

**Note:** Custom keyboard shortcuts are not currently supported but are planned for a future release.

**Roadmap:**

- User-configurable shortcuts
- Import/export shortcut profiles
- Conflict detection
- Platform-specific defaults

---

## Accessibility

All keyboard shortcuts are designed to be:

- **Screen reader friendly:** All actions have proper labels
- **Motor-friendly:** No complex multi-key combinations required
- **Discoverable:** Multiple ways to learn about shortcuts
- **Consistent:** Standard patterns used throughout

---

## Troubleshooting

### Shortcuts Not Working

**Problem:** Keyboard shortcuts don't respond

**Solutions:**

1. Ensure app window has focus
2. Check if focus is in a text input field (some shortcuts disabled during typing)
3. Verify keyboard layout (US QWERTY expected)
4. Restart the application

### Conflicts with System Shortcuts

**Problem:** Shortcut triggers system action instead of app action

**Solutions:**

- **Windows:** Some Ctrl shortcuts may conflict with Windows features
- **macOS:** Some Cmd shortcuts may conflict with system/other apps
- **Linux:** Depends on desktop environment (GNOME, KDE, etc.)

**Workaround:** Use alternative shortcuts where provided (e.g., `Shift + ?` vs `Ctrl + /`)

### Focus Mode Stuck

**Problem:** Can't exit focus mode or UI elements missing

**Solutions:**

1. Press `Ctrl + D` to toggle focus mode off
2. Click the eye icon in app bar
3. Restart the application if needed

---

## Development

For developers working on the shortcuts system:

**File Structure:**

```
lib/shortcuts/
├── intents.dart              # Intent definitions
├── shortcuts_config.dart     # Keyboard mappings
└── shortcuts_help_dialog.dart # Help UI
```

**Adding New Shortcuts:**

1. Define Intent class in `intents.dart`
2. Add mapping in `ShortcutsConfig.defaultShortcuts`
3. Add description in `ShortcutsConfig.shortcutDescriptions`
4. Implement action handler in the relevant widget
5. Update this documentation

**Testing:**

- All shortcuts should be tested on Windows, macOS, and Linux
- Verify no conflicts with common system shortcuts
- Test with screen readers
- Validate keyboard-only navigation works

---

## See Also

- [Architecture Overview](./ARCHITECTURE-OVERVIEW.md) - System design
- [Development Roadmap](./DEVELOPMENT-ROADMAP.md) - Feature timeline
- [Testing Strategy](./TESTING-STRATEGY.md) - Testing best practices

---

**Last updated:** 2025-10-20
**Version:** Week 8 (Month 2, Phase 1)
**Maintainers:** Altair Development Team
