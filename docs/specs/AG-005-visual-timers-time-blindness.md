# Feature AG-005: Visual Timers and Time Blindness Support

## What it does

Provides multiple visual timer implementations to combat time blindness, including analog clocks, progress rings, color-changing backgrounds, and time estimation calibration to help users develop accurate time perception.

## User Journey

GIVEN user starts working on a quest with time blindness challenges
WHEN user activates a visual timer for the task
THEN system provides continuous visual feedback about time passage and helps calibrate time estimation skills

## Functional Requirements

- Multiple timer visualization styles
- Pomodoro technique with modifications
- Time estimation vs actual tracking
- Buffer time auto-calculation
- Break reminders with enforcement
- Hyperfocus interruption system
- Time passage animations
- Historical time accuracy data
- Smart timer suggestions
- Multi-timer support
- Audio/haptic alerts
- Calendar integration for deadlines

## UI/UX Requirements

### Components

- `AnalogTimer` - Clock face visualization
- `ProgressRing` - Circular progress indicator
- `ColorFadeTimer` - Background color transition
- `TimelineBar` - Linear progress bar
- `EstimationTrainer` - Guess vs actual display
- `BreakEnforcer` - Mandatory break screen
- `HyperfocusGuard` - Attention breaker
- `TimerSelector` - Visual style picker
- `TimeHistory` - Accuracy tracking graph
- `MultiTimerDock` - Multiple timer management

### Visual Design

- **Layout:**
  - Timer overlay: 200px diameter circle
  - Position: Draggable, default top-right
  - Timeline bar: Bottom of screen, 40px
  - Break screen: Full screen takeover
- **Colors:**
  - Start: `#4CAF50` (Green)
  - Middle: `#FFC107` (Yellow) 
  - Warning: `#FF9800` (Orange)
  - Ending: `#F44336` (Red)
  - Break: `#2196F3` (Blue)
  - Overtime: `#9C27B0` (Purple pulse)
- **Typography:**
  - Time remaining: 24px bold
  - Estimation: 18px regular
  - Break message: 32px light
  - History stats: 14px medium
- **Iconography:**
  - Clock faces for analog
  - Progress rings
  - Hourglass for time running
  - Bell for alerts
  - Eye for hyperfocus guard
- **Borders/Shadows:**
  - Timer: 2px border, color matches state
  - Active timer: Pulsing glow
  - Break screen: Blur background

### User Interactions

- **Input Methods:**
  - Click to start/pause
  - Drag to reposition
  - Scroll to adjust time
  - Right-click for options
- **Keyboard Shortcuts:**
  - `T`: Start/stop timer
  - `B`: Take break now
  - `+/-`: Add/remove 5 minutes
  - `R`: Reset timer
  - `Space`: Pause/resume
- **Gestures:**
  - Swipe to change timer style
  - Long-press to reset
  - Pinch to resize
  - Shake to cancel (mobile)
- **Feedback:**
  - Tick sound every minute
  - Vibration at intervals
  - Screen flash warnings
  - Voice announcements (optional)

### State Management

- **Local State:**
  - Active timers
  - Current visualization
  - Pause state
  - Break status
- **Global State:**
  ```dart
  final timerProvider = StateNotifierProvider<TimerNotifier, TimerState>
  final multiTimerProvider = StateNotifierProvider<MultiTimerNotifier, List<Timer>>
  final timeEstimationProvider = StateNotifierProvider<EstimationNotifier, EstimationData>
  final breakScheduleProvider = StateProvider<BreakSchedule>
  final hyperfocusGuardProvider = StateProvider<HyperfocusSettings>
  final timeHistoryProvider = FutureProvider<TimeAccuracyHistory>
  ```
- **Persistence:**
  - Timer preferences per task type
  - Estimation history
  - Break patterns
  - Accuracy statistics

### Responsive Behavior

- **Desktop:** Multiple floating timers
- **Tablet:** Docked timer bar
- **Mobile:** Notification bar timer
- **Breakpoint Strategy:** Adapt timer size to screen

### Accessibility Requirements

- **Screen Reader:**
  - Announce time remaining
  - Describe timer state
  - Read break messages
- **Keyboard Navigation:**
  - Full timer control
  - Skip to timer widget
- **Color Contrast:** Shape changes supplement color
- **Motion:** Static timer option
- **Font Sizing:** Large time display

### ADHD-Specific UI Requirements

- **Cognitive Load:**
  - Default timer auto-starts
  - Simple 25-minute default
  - One-click extensions
  - Preset timer options
- **Focus Management:**
  - Always visible option
  - Can't minimize during focus
  - Break screen blocks work
- **Forgiveness:**
  - Pause anytime
  - Extend easily
  - Skip breaks (limited)
  - No penalty for overtime
- **Visual Hierarchy:**
  - Time remaining largest
  - Color coding obvious
  - Break alerts unmissable
- **Immediate Feedback:**
  - Instant start
  - Continuous animation
  - Clear state changes

## Non-Functional Requirements

### Performance Targets

- Timer update <16ms (60fps)
- Animation smoothness constant
- Multi-timer overhead <5% CPU
- Battery impact minimal
- Memory usage <20MB

### Technical Constraints

- Flutter animation framework
- Platform timer APIs
- Background service for breaks
- System notification access
- Audio playback capability

### Security Requirements

- No network requirements
- Local data only
- Permission for notifications
- Optional calendar access

## Implementation Details

### Code Structure

```
lib/
├── features/
│   └── visual_timers/
│       ├── presentation/
│       │   ├── widgets/
│       │   │   ├── analog_timer.dart
│       │   │   ├── progress_ring_timer.dart
│       │   │   ├── color_fade_timer.dart
│       │   │   ├── timeline_bar.dart
│       │   │   ├── break_enforcer.dart
│       │   │   └── hyperfocus_guard.dart
│       │   ├── providers/
│       │   │   ├── timer_provider.dart
│       │   │   ├── estimation_provider.dart
│       │   │   └── break_provider.dart
│       │   └── screens/
│       │       ├── timer_overlay.dart
│       │       └── time_history_screen.dart
│       ├── domain/
│       │   ├── entities/
│       │   │   ├── timer.dart
│       │   │   ├── time_estimation.dart
│       │   │   └── break_schedule.dart
│       │   └── services/
│       │       ├── timer_service.dart
│       │       └── estimation_calibrator.dart
│       └── data/
│           ├── models/
│           │   └── timer_model.dart
│           └── repositories/
│               └── timer_repository.dart

backend/
├── src/
│   └── timers/
│       ├── timer_service.rs
│       └── statistics_calculator.rs
```

### Key Files to Create

- `analog_timer.dart` - Clock face widget
- `progress_ring_timer.dart` - Circular progress
- `timer_provider.dart` - Timer state management
- `estimation_calibrator.dart` - Time accuracy service
- `break_enforcer.dart` - Break screen implementation

### Dependencies

```yaml
dependencies:
  flutter_riverpod: ^2.4.0
  circular_countdown_timer: ^0.2.0
  flutter_local_notifications: ^15.1.0
  pausable_timer: ^2.0.0
  audioplayers: ^5.0.0
  vibration: ^1.8.0
  
dev_dependencies:
  golden_toolkit: ^0.15.0
```

### Timer Visualization Styles

```dart
enum TimerStyle {
  analogClock,    // Traditional clock face
  progressRing,   // Circular progress
  colorFade,      // Background color change
  timelineBar,    // Linear progress
  sandTimer,      // Hourglass animation
  textOnly,       // Digital countdown
  pulsingDot,     // Minimal indicator
}

class TimerVisualizer {
  Widget build(TimerStyle style, Duration remaining, Duration total) {
    switch (style) {
      case TimerStyle.analogClock:
        return AnalogClockTimer(remaining, total);
      case TimerStyle.progressRing:
        return CircularProgressTimer(
          progress: remaining.inSeconds / total.inSeconds,
          color: _getColorForProgress(remaining, total),
        );
      // ... other styles
    }
  }
  
  Color _getColorForProgress(Duration remaining, Duration total) {
    final ratio = remaining.inSeconds / total.inSeconds;
    if (ratio > 0.5) return Colors.green;
    if (ratio > 0.25) return Colors.yellow;
    if (ratio > 0.1) return Colors.orange;
    return Colors.red;
  }
}
```

## Testing Requirements

### Unit Tests

- [x] Timer accuracy
- [x] State transitions
- [x] Break scheduling
- [x] Estimation calculations
- [x] Multi-timer management

### Widget Tests

- [x] Timer visualizations
- [x] Animation performance
- [x] User interactions
- [x] Break screen blocking
- [x] Alert triggering

### Integration Tests

- [x] Start timer → Complete → Record
- [x] Estimation → Actual → Calibration
- [x] Break reminder → Enforcement
- [x] Multiple timers simultaneously
- [x] Background notifications

### Accessibility Tests

- [x] Screen reader time announcements
- [x] Keyboard timer control
- [x] Non-visual alerts
- [x] High contrast modes

## Definition of Done

- [x] All timer styles implemented
- [x] Smooth 60fps animations
- [x] Break system working
- [x] Estimation tracking functional
- [x] Hyperfocus guard active
- [x] Multi-timer support complete
- [x] Accessibility verified
- [x] Battery efficient
- [x] All tests passing
- [x] Performance optimized
