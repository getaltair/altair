# Altair UI Implementation Guide for Claude Code

## Project Context
- **Apps**: Guidance (tasks), Knowledge (PKM), Tracking (inventory)
- **Stack**: Flutter + FastAPI + SurrealDB
- **Priority**: Desktop-first, mobile secondary
- **Architecture**: Local-first with shared SurrealDB service
- **Design Goal**: ADHD-optimized interfaces reducing cognitive load

## Core Implementation Requirements

### 1. Design System Package Structure
```
packages/altair_design_system/
├── lib/
│   ├── tokens/
│   │   ├── colors.dart
│   │   ├── spacing.dart
│   │   ├── typography.dart
│   │   └── timing.dart
│   ├── primitives/
│   │   ├── buttons.dart
│   │   ├── inputs.dart
│   │   ├── cards.dart
│   │   └── navigation.dart
│   ├── patterns/
│   │   ├── quick_capture.dart
│   │   ├── progressive_disclosure.dart
│   │   └── focus_mode.dart
│   └── themes/
│       ├── light_theme.dart
│       └── dark_theme.dart
```

### 2. Color System Implementation
```dart
// colors.dart
class AltairColors {
  // Primary palette - muted for large areas
  static const Color primary = Color(0xFF4A6FA5);  // Muted blue
  static const Color secondary = Color(0xFF7BA05B); // Muted green
  static const Color neutral = Color(0xFF6B7280);   // Gray
  
  // Accent colors - small areas only
  static const Color accent = Color(0xFFF59E0B);    // Amber for CTAs
  static const Color warning = Color(0xFFEF4444);   // Red for critical only
  
  // Backgrounds
  static const Color bgLight = Color(0xFFFAFAFA);
  static const Color bgDark = Color(0xFF1E1E1E);   // Not pure black
  
  // Borders for neo-brutalist style
  static const Color border = Color(0xFF000000);
  static const double borderWidth = 3.0;
}
```

### 3. Typography Configuration
```dart
// typography.dart
class AltairTypography {
  static const String fontFamily = 'Inter'; // Or Lexend for better readability
  
  static const TextStyle h1 = TextStyle(
    fontSize: 32,
    fontWeight: FontWeight.w700,
    height: 1.2,
  );
  
  static const TextStyle body = TextStyle(
    fontSize: 16,  // Minimum 14pt
    fontWeight: FontWeight.w400,
    height: 1.5,   // 1.5x line spacing minimum
  );
  
  // Max 70 characters per line
  static double maxLineWidth = 560.0;
}
```

### 4. Quick Capture Widget Implementation
```dart
// quick_capture.dart
class QuickCaptureWidget extends StatefulWidget {
  final Function(String) onCapture;
  final String placeholder;
  final bool enableVoice;
  final bool enableNaturalLanguage;
  
  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        border: Border.all(
          color: AltairColors.border,
          width: AltairColors.borderWidth,
        ),
        color: Colors.white,
        boxShadow: [
          BoxShadow(
            offset: Offset(4, 4),
            color: Colors.black,
            spreadRadius: 0,
          ),
        ],
      ),
      child: TextField(
        autofocus: true,  // Critical for ADHD
        maxLines: null,
        decoration: InputDecoration(
          hintText: placeholder,
          border: InputBorder.none,
          contentPadding: EdgeInsets.all(16),
        ),
        onSubmitted: (text) {
          if (enableNaturalLanguage) {
            // Parse: "Call Pete tomorrow 3pm #Work p1"
            final parsed = NaturalLanguageParser.parse(text);
            onCapture(parsed);
          } else {
            onCapture(text);
          }
        },
      ),
    );
  }
}
```

### 5. Progressive Disclosure Pattern
```dart
// progressive_disclosure.dart
class ProgressiveDisclosure extends StatefulWidget {
  final Widget primary;  // Always visible
  final Widget secondary; // Hidden by default
  final String expandLabel;
  
  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        primary,
        if (!_expanded)
          TextButton(
            onPressed: () => setState(() => _expanded = true),
            child: Text(expandLabel ?? 'More options'),
          ),
        if (_expanded) ...[
          SizedBox(height: 16),
          secondary,
        ],
      ],
    );
  }
}

// Usage example for task creation
ProgressiveDisclosure(
  primary: Column(children: [
    TextField(decoration: InputDecoration(labelText: 'Task title')),
    ElevatedButton(onPressed: _save, child: Text('Add Task')),
  ]),
  secondary: Column(children: [
    DropdownButton(...), // Project selection
    DatePicker(...),      // Due date
    TagSelector(...),     // Tags
  ]),
);
```

### 6. Window Management (Desktop)
```dart
// main.dart
import 'package:bitsdojo_window/bitsdojo_window.dart';

void main() {
  runApp(MyApp());
  
  doWhenWindowReady(() {
    final win = appWindow;
    const initialSize = Size(1200, 800);
    win.minSize = Size(800, 600);
    win.size = initialSize;
    win.alignment = Alignment.center;
    win.title = "Altair - Where Focus Takes Flight";
    win.show();
  });
}

// Custom title bar
class CustomTitleBar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return WindowTitleBarBox(
      child: Row(
        children: [
          Expanded(
            child: MoveWindow(
              child: Container(
                padding: EdgeInsets.symmetric(horizontal: 16),
                child: Text('Altair'),
              ),
            ),
          ),
          Row(
            children: [
              MinimizeWindowButton(),
              MaximizeWindowButton(),
              CloseWindowButton(),
            ],
          ),
        ],
      ),
    );
  }
}
```

### 7. Keyboard Shortcuts System
```dart
// keyboard_shortcuts.dart
class AltairShortcuts extends StatelessWidget {
  final Widget child;
  
  @override
  Widget build(BuildContext context) {
    return CallbackShortcuts(
      bindings: {
        // Global shortcuts
        SingleActivator(LogicalKeyboardKey.keyN, control: true): 
          () => _showQuickCapture(context),
        SingleActivator(LogicalKeyboardKey.keyK, control: true): 
          () => _showCommandPalette(context),
        SingleActivator(LogicalKeyboardKey.escape): 
          () => Navigator.of(context).pop(),
        
        // App switching
        SingleActivator(LogicalKeyboardKey.digit1, control: true): 
          () => _switchToApp('guidance'),
        SingleActivator(LogicalKeyboardKey.digit2, control: true): 
          () => _switchToApp('knowledge'),
        SingleActivator(LogicalKeyboardKey.digit3, control: true): 
          () => _switchToApp('tracking'),
      },
      child: Focus(
        autofocus: true,
        child: child,
      ),
    );
  }
}
```

### 8. State Management Pattern (Riverpod)
```dart
// quick_capture_provider.dart
@riverpod
class QuickCapture extends _$QuickCapture {
  @override
  Future<List<CaptureItem>> build() async {
    return await _loadFromDatabase();
  }
  
  Future<void> capture(String content) async {
    // Optimistic update
    state = AsyncValue.data([
      ...state.value ?? [],
      CaptureItem(content: content, timestamp: DateTime.now()),
    ]);
    
    // Save to database with debouncing
    _debounceTimer?.cancel();
    _debounceTimer = Timer(Duration(milliseconds: 500), () async {
      try {
        await _saveToDatabase(content);
      } catch (e) {
        // Rollback on error
        state = AsyncValue.data(
          state.value?.where((item) => item.content != content).toList() ?? []
        );
      }
    });
  }
}
```

### 9. Focus Mode Implementation
```dart
// focus_mode.dart
class FocusMode extends StatefulWidget {
  final Duration sessionLength;
  final Duration breakLength;
  
  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        // Main content
        widget.child,
        
        // Pomodoro timer overlay
        Positioned(
          top: 16,
          right: 16,
          child: Container(
            width: 120,
            height: 120,
            decoration: BoxDecoration(
              shape: BoxShape.circle,
              border: Border.all(width: 3, color: Colors.black),
              color: _isBreak ? Colors.green.shade100 : Colors.red.shade100,
            ),
            child: Center(
              child: Text(
                _formatTime(_remainingTime),
                style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
              ),
            ),
          ),
        ),
        
        // Focus mode controls
        Positioned(
          bottom: 16,
          right: 16,
          child: Column(
            children: [
              IconButton(
                icon: Icon(_isPaused ? Icons.play_arrow : Icons.pause),
                onPressed: _togglePause,
              ),
              IconButton(
                icon: Icon(Icons.skip_next),
                onPressed: _skipSession,
              ),
            ],
          ),
        ),
      ],
    );
  }
}
```

### 10. Animation Configuration
```dart
// animation_config.dart
class AltairAnimations {
  static const Duration standard = Duration(milliseconds: 250);
  static const Curve standardCurve = Curves.easeInOut;
  
  // Check system preference
  static bool get reducedMotion => 
    MediaQuery.of(context).disableAnimations;
  
  // Conditional animation wrapper
  static Widget conditionalAnimation({
    required Widget child,
    required bool animate,
  }) {
    if (!animate || reducedMotion) return child;
    
    return AnimatedSwitcher(
      duration: standard,
      child: child,
    );
  }
}

// Usage
AnimatedContainer(
  duration: AltairAnimations.reducedMotion 
    ? Duration.zero 
    : AltairAnimations.standard,
  // ... rest of properties
);
```

### 11. Notification Management
```dart
// notification_manager.dart
class NotificationManager {
  static final _notifications = FlutterLocalNotificationsPlugin();
  
  // Batch notifications at specific times
  static Future<void> scheduleBatchedSummary() async {
    final times = ['09:00', '13:00', '17:00']; // Morning, lunch, evening
    
    for (final time in times) {
      await _notifications.zonedSchedule(
        0,
        'Task Summary',
        await _generateSummary(),
        _nextInstanceOfTime(time),
        const NotificationDetails(
          android: AndroidNotificationDetails(
            'summary',
            'Batched Summaries',
            importance: Importance.low,
            priority: Priority.low,
          ),
        ),
        androidAllowWhileIdle: true,
        uiLocalNotificationDateInterpretation:
          UILocalNotificationDateInterpretation.absoluteTime,
        matchDateTimeComponents: DateTimeComponents.time,
      );
    }
  }
  
  // Only immediate notifications for critical items
  static Future<void> showCritical(String message) async {
    await _notifications.show(
      1,
      'Critical',
      message,
      const NotificationDetails(
        android: AndroidNotificationDetails(
          'critical',
          'Critical Alerts',
          importance: Importance.high,
          priority: Priority.high,
        ),
      ),
    );
  }
}
```

### 12. Cross-App Navigation
```dart
// app_navigator.dart
class AltairNavigator extends StatelessWidget {
  final String currentApp;
  
  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        // Side navigation for desktop
        if (!Platform.isMobile)
          Container(
            width: 64,
            color: AltairColors.bgDark,
            child: Column(
              children: [
                _NavButton(
                  icon: Icons.check_circle,
                  label: 'Guidance',
                  isActive: currentApp == 'guidance',
                  onTap: () => _switchTo('guidance'),
                ),
                _NavButton(
                  icon: Icons.book,
                  label: 'Knowledge',
                  isActive: currentApp == 'knowledge',
                  onTap: () => _switchTo('knowledge'),
                ),
                _NavButton(
                  icon: Icons.inventory,
                  label: 'Tracking',
                  isActive: currentApp == 'tracking',
                  onTap: () => _switchTo('tracking'),
                ),
              ],
            ),
          ),
        
        // Main content area
        Expanded(
          child: _getAppWidget(currentApp),
        ),
      ],
    );
  }
}
```

### 13. Database Configuration (Drift)
```dart
// database.dart
@DriftDatabase(tables: [Tasks, Notes, Items])
class AltairDatabase extends _$AltairDatabase {
  AltairDatabase() : super(_openConnection());
  
  @override
  int get schemaVersion => 1;
  
  // Quick capture across all apps
  Future<void> quickCapture(String content, CaptureType type) async {
    switch (type) {
      case CaptureType.task:
        await into(tasks).insert(TasksCompanion(
          content: Value(content),
          createdAt: Value(DateTime.now()),
        ));
        break;
      case CaptureType.note:
        await into(notes).insert(NotesCompanion(
          content: Value(content),
          createdAt: Value(DateTime.now()),
        ));
        break;
      case CaptureType.item:
        await into(items).insert(ItemsCompanion(
          name: Value(content),
          createdAt: Value(DateTime.now()),
        ));
        break;
    }
  }
  
  // Cross-app search
  Stream<List<SearchResult>> search(String query) {
    // Implement full-text search across all tables
    return customSelect(
      'SELECT * FROM tasks WHERE content LIKE ? '
      'UNION SELECT * FROM notes WHERE content LIKE ? '
      'UNION SELECT * FROM items WHERE name LIKE ?',
      variables: [
        Variable('%$query%'),
        Variable('%$query%'),
        Variable('%$query%'),
      ],
    ).watch().map((rows) => rows.map((row) => 
      SearchResult.fromData(row.data)).toList());
  }
}
```

### 14. Accessibility Implementation
```dart
// accessibility.dart
class AccessibleWidget extends StatelessWidget {
  final Widget child;
  final String label;
  final String? hint;
  final bool isButton;
  final bool isHeader;
  
  @override
  Widget build(BuildContext context) {
    return Semantics(
      label: label,
      hint: hint,
      button: isButton,
      header: isHeader,
      child: ExcludeSemantics(
        excluding: false,
        child: child,
      ),
    );
  }
}

// Usage
AccessibleWidget(
  label: 'Create new task',
  hint: 'Press to open quick capture',
  isButton: true,
  child: ElevatedButton(...),
);
```

### 15. Performance Optimization Patterns
```dart
// performance.dart
class OptimizedList extends StatelessWidget {
  final List<Item> items;
  
  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      itemCount: items.length,
      itemBuilder: (context, index) {
        return RepaintBoundary(
          child: ItemWidget(item: items[index]),
        );
      },
    );
  }
}

// Optimized animation
class OptimizedAnimation extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: _controller,
      builder: (context, child) {
        return Transform.scale(
          scale: _controller.value,
          child: child,
        );
      },
      child: ExpensiveWidget(), // Built once
    );
  }
}
```

## Implementation Checklist

### Phase 1: Foundation (Immediate)
- [ ] Create design_system package with tokens
- [ ] Implement custom window frame with bitsdojo_window
- [ ] Build QuickCaptureWidget with autofocus
- [ ] Setup keyboard shortcut system
- [ ] Configure Drift database with basic schema
- [ ] Implement global hotkey (Alt+Space) for capture

### Phase 2: Core Features
- [ ] Progressive disclosure components
- [ ] Cross-app navigation system
- [ ] Universal search (Cmd+K)
- [ ] Notification batching system
- [ ] Focus mode with Pomodoro timer
- [ ] Dark mode with proper contrast

### Phase 3: Optimization
- [ ] Animation performance with RepaintBoundary
- [ ] Implement prefers-reduced-motion
- [ ] Add accessibility labels
- [ ] Keyboard-only navigation
- [ ] Offline support with sync queue
- [ ] 60fps performance target

## Critical Implementation Rules

1. **Always autofocus** first input field
2. **Never exceed** 5 visible options without progressive disclosure
3. **Always provide** keyboard shortcuts for primary actions
4. **Never use** pure black (#000000) for backgrounds - use #1E1E1E
5. **Always include** undo/redo functionality
6. **Never animate** for more than 300ms
7. **Always save** automatically with debouncing
8. **Never require** more than 2 clicks to reach any feature
9. **Always show** progress indicators for multi-step processes
10. **Never use** red except for critical errors

## Testing Requirements

### Automated Tests
```dart
testWidgets('Quick capture autofocuses', (tester) async {
  await tester.pumpWidget(QuickCaptureWidget());
  final textField = find.byType(TextField);
  expect(tester.widget<TextField>(textField).autofocus, true);
});

testWidgets('Keyboard shortcuts work', (tester) async {
  await tester.pumpWidget(AltairShortcuts(child: Container()));
  await tester.sendKeyDownEvent(LogicalKeyboardKey.control);
  await tester.sendKeyEvent(LogicalKeyboardKey.keyN);
  await tester.sendKeyUpEvent(LogicalKeyboardKey.control);
  expect(find.byType(QuickCaptureWidget), findsOneWidget);
});
```

### Performance Metrics
- First contentful paint < 1 second
- Time to interactive < 2 seconds
- Frame rate >= 60fps
- Memory usage < 200MB idle
- Database query response < 100ms

## Package Dependencies
```yaml
dependencies:
  flutter:
    sdk: flutter
  bitsdojo_window: ^0.1.5      # Custom window management
  flutter_riverpod: ^2.4.0      # State management
  drift: ^2.12.0                # Database
  sqlite3_flutter_libs: ^0.5.0  # SQLite support
  flutter_local_notifications: ^16.0.0  # Notifications
  speech_to_text: ^6.3.0        # Voice input
  hotkey_manager: ^0.1.7        # Global hotkeys
```
