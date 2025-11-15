# Feature AG-006: WIP=1 Enforcement and Focus Mode

## What it does

Enforces Work-In-Progress limit of 1 task in the active column, provides focus mode that hides all other tasks, and includes gentle barriers to prevent impulsive task switching common in ADHD.

## User Journey

GIVEN user has ADHD tendencies for task switching
WHEN user attempts to move multiple tasks to In-Progress
THEN system enforces WIP=1 limit with gentle guidance and offers focus mode to maintain attention on single task

## Functional Requirements

- Hard enforcement of single task in In-Progress
- Focus mode hiding all but current task
- Task switching friction (confirmation dialogs)
- Context preservation on task switch
- Emergency task override with reason
- Focus session tracking
- Distraction log for self-awareness
- Return-to-task reminders
- Task commitment contracts
- Progress preservation
- Anti-rabbit-hole guards
- Bathroom break mode (timer pause)

## UI/UX Requirements

### Components

- `WIPEnforcer` - Prevents multiple active tasks
- `FocusModeOverlay` - Hides other tasks
- `TaskSwitchDialog` - Friction for switching
- `CommitmentContract` - Task dedication UI
- `DistractionLogger` - Track switch attempts
- `ContextSaver` - Preserve work state
- `ReturnReminder` - Nudge back to task
- `EmergencyOverride` - Critical task handling
- `BreakModeToggle` - Pause without switching
- `FocusStatistics` - Session analytics

### Visual Design

- **Layout:**
  - Focus mode: Full screen task card
  - WIP warning: Modal dialog, 400px
  - Distraction log: Slide-out panel
  - Commitment: Centered card, 500px
- **Colors:**
  - Active task: `#4CAF50` border
  - Blocked tasks: `#9E9E9E` grayed out
  - Warning: `#FF9800` for switches
  - Focus mode: `#1A237E` dark theme
  - Emergency: `#F44336` red override
- **Typography:**
  - Current task: 20px bold
  - Blocked reason: 14px italic
  - Commitment: 18px medium
  - Statistics: 12px regular
- **Iconography:**
  - Lock icon for WIP limit
  - Eye icon for focus mode
  - Shield for commitment
  - Warning triangle for switches
  - Pause icon for breaks
- **Borders/Shadows:**
  - Active task: 4px solid border
  - Blocked tasks: Dashed border
  - Focus mode: Vignette effect

### User Interactions

- **Input Methods:**
  - Click blocked task shows dialog
  - Double-click for emergency override
  - Type reason for switching
  - Voice note for context
- **Keyboard Shortcuts:**
  - `F`: Toggle focus mode
  - `Escape`: Exit focus mode
  - `P`: Pause for break
  - `C`: Complete current task
  - `Tab`: Disabled in focus mode
- **Gestures:**
  - Swipe away distractions
  - Long-press for override
  - Shake to reset focus
- **Feedback:**
  - Gentle vibration on block
  - Soft sound for reminder
  - Visual pulse on active task
  - Celebration on completion

### State Management

- **Local State:**
  - Focus mode active
  - Current task context
  - Break timer state
  - Switch attempt count
- **Global State:**
  ```dart
  final wipEnforcerProvider = StateNotifierProvider<WIPNotifier, WIPState>
  final focusModeProvider = StateProvider<bool>
  final currentTaskProvider = StateProvider<Quest?>
  final taskContextProvider = StateNotifierProvider<ContextNotifier, Map<String, Context>>
  final distractionLogProvider = StateNotifierProvider<DistractionNotifier, List<Distraction>>
  final focusSessionProvider = StateNotifierProvider<SessionNotifier, FocusSession>
  ```
- **Persistence:**
  - Task contexts saved
  - Focus statistics logged
  - Distraction patterns tracked
  - Commitment history kept

### Responsive Behavior

- **Desktop:** Full focus mode with panels
- **Tablet:** Simplified focus view
- **Mobile:** Single task card only
- **Breakpoint Strategy:** Progressive simplification

### Accessibility Requirements

- **Screen Reader:**
  - Announce WIP violations
  - Describe focus mode state
  - Read switch reasons
- **Keyboard Navigation:**
  - Trap focus in mode
  - Clear escape route
- **Color Contrast:** Non-color indicators
- **Motion:** Reduced animation option
- **Font Sizing:** Adjustable focus text

### ADHD-Specific UI Requirements

- **Cognitive Load:**
  - One task visible
  - Clear boundaries
  - Simple yes/no choices
  - Minimal UI in focus
- **Focus Management:**
  - Can't see other tasks
  - No notification badges
  - Disabled navigation
  - Timer always visible
- **Forgiveness:**
  - Emergency override exists
  - Context preserved
  - Easy return to task
  - No shame language
- **Visual Hierarchy:**
  - Current task enormous
  - Everything else hidden
  - Break button prominent
- **Immediate Feedback:**
  - Instant focus mode
  - Clear blocking message
  - Quick context save

## Non-Functional Requirements

### Performance Targets

- Focus mode toggle <100ms
- Context save <200ms
- WIP check instant
- UI update <16ms
- Statistics calculation <500ms

### Technical Constraints

- State preservation required
- Notification system integration
- Window management control
- Full screen capability
- Audio/haptic support

### Security Requirements

- Context data encrypted
- No task data in logs
- Privacy mode option
- Secure state storage

## Implementation Details

### Code Structure

```
lib/
├── features/
│   └── wip_enforcement/
│       ├── presentation/
│       │   ├── widgets/
│       │   │   ├── wip_enforcer.dart
│       │   │   ├── focus_mode_overlay.dart
│       │   │   ├── task_switch_dialog.dart
│       │   │   ├── commitment_contract.dart
│       │   │   ├── distraction_logger.dart
│       │   │   └── focus_statistics.dart
│       │   ├── providers/
│       │   │   ├── wip_provider.dart
│       │   │   ├── focus_provider.dart
│       │   │   ├── context_provider.dart
│       │   │   └── distraction_provider.dart
│       │   └── screens/
│       │       ├── focus_mode_screen.dart
│       │       └── distraction_report_screen.dart
│       ├── domain/
│       │   ├── entities/
│       │   │   ├── wip_state.dart
│       │   │   ├── task_context.dart
│       │   │   ├── focus_session.dart
│       │   │   └── distraction.dart
│       │   └── services/
│       │       ├── wip_enforcer_service.dart
│       │       └── context_manager.dart
│       └── data/
│           ├── models/
│           │   └── focus_model.dart
│           └── repositories/
│               └── focus_repository.dart

backend/
├── src/
│   └── focus/
│       ├── wip_service.rs
│       └── statistics_tracker.rs
```

### Key Files to Create

- `wip_enforcer.dart` - WIP limit enforcement
- `focus_mode_overlay.dart` - Full screen focus
- `task_switch_dialog.dart` - Switching friction
- `context_manager.dart` - State preservation
- `distraction_logger.dart` - Pattern tracking

### Dependencies

```yaml
dependencies:
  flutter_riverpod: ^2.4.0
  window_manager: ^0.3.0
  flutter_local_notifications: ^15.1.0
  shared_preferences: ^2.2.0
  
dev_dependencies:
  patrol: ^3.0.0
```

### WIP Enforcement Logic

```dart
class WIPEnforcer {
  static const int WIP_LIMIT = 1;
  
  bool canMoveToInProgress(Quest quest, List<Quest> currentInProgress) {
    if (currentInProgress.isEmpty) return true;
    if (currentInProgress.length >= WIP_LIMIT) {
      _showBlockedDialog(quest, currentInProgress.first);
      return false;
    }
    return true;
  }
  
  void _showBlockedDialog(Quest blocked, Quest current) {
    showDialog(
      context: context,
      barrierDismissible: false,
      builder: (_) => WIPBlockedDialog(
        blocked: blocked,
        current: current,
        onSwitch: () => _handleTaskSwitch(blocked, current),
        onCancel: () => Navigator.pop(context),
        onEmergency: () => _handleEmergencyOverride(blocked),
      ),
    );
  }
  
  Future<void> _handleTaskSwitch(Quest newTask, Quest oldTask) async {
    // Save context
    await contextManager.saveContext(oldTask);
    
    // Log distraction
    await distractionLogger.log(DistractionEvent(
      from: oldTask,
      to: newTask,
      timestamp: DateTime.now(),
      reason: await _askForReason(),
    ));
    
    // Move tasks
    await moveToColumn(oldTask, Column.nextUp);
    await moveToColumn(newTask, Column.inProgress);
  }
}
```

### Focus Mode Implementation

```dart
class FocusModeOverlay extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final currentTask = ref.watch(currentTaskProvider);
    final focusSession = ref.watch(focusSessionProvider);
    
    if (currentTask == null) return Empty();
    
    return Scaffold(
      backgroundColor: Color(0xFF0D1117), // Dark focus theme
      body: Center(
        child: Container(
          constraints: BoxConstraints(maxWidth: 800),
          padding: EdgeInsets.all(32),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              // Minimal UI - just the task
              Text(
                currentTask.title,
                style: TextStyle(
                  fontSize: 32,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
              SizedBox(height: 40),
              
              // Timer always visible
              VisualTimer(
                duration: focusSession.duration,
                style: TimerStyle.progressRing,
              ),
              
              SizedBox(height: 40),
              
              // Only essential actions
              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  ElevatedButton.icon(
                    icon: Icon(Icons.check),
                    label: Text('Complete'),
                    onPressed: () => _completeTask(),
                  ),
                  SizedBox(width: 16),
                  OutlinedButton.icon(
                    icon: Icon(Icons.pause),
                    label: Text('Break'),
                    onPressed: () => _takeBreak(),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}
```

## Testing Requirements

### Unit Tests

- [ ] WIP limit enforcement
- [ ] Context preservation
- [ ] Distraction logging
- [ ] Focus session tracking
- [ ] Emergency override logic

### Widget Tests

- [ ] Focus mode UI
- [ ] Switch dialog interaction
- [ ] Context save trigger
- [ ] Statistics display
- [ ] Break mode toggle

### Integration Tests

- [ ] Task switch → Context save → Log
- [ ] Focus mode → Complete → Exit
- [ ] WIP block → Override → Switch
- [ ] Break → Return → Resume
- [ ] Multiple switch attempts

### Accessibility Tests

- [ ] Screen reader in focus mode
- [ ] Keyboard-only operation
- [ ] Escape routes clear
- [ ] Color-blind safe

## Definition of Done

- [ ] WIP=1 strictly enforced
- [ ] Focus mode implemented
- [ ] Context preservation working
- [ ] Distraction logging active
- [ ] Emergency override functional
- [ ] Break mode operational
- [ ] Statistics tracking
- [ ] All tests passing
- [ ] Accessibility verified
- [ ] Performance optimal
