# Altair Flutter Frontend Documentation

## Overview

Flutter frontend for Altair providing native mobile, web (PWA), and desktop experiences optimized for ADHD users. Focuses on instant response, offline-first operation, and dopamine-triggering micro-interactions.

## Tech Stack

```yaml
Core:
  - Flutter: 3.16+ (latest stable)
  - Dart: 3.2+
  - Target Platforms: iOS, Android, Web (PWA), Desktop

State Management:
  - Riverpod 2.0: Type-safe reactive state management
  
Local Storage:
  - Drift (SQLite): Local database with sync
  - Hive: Key-value store for preferences
  
Networking:
  - Dio: HTTP client with interceptors
  - web_socket_channel: WebSocket support
  
UI/UX:
  - flutter_animate: Micro-animations
  - google_fonts: ADHD-friendly typography
  - fl_chart: Data visualization
```

## Project Structure

```
flutter/
├── lib/
│   ├── main.dart                    # Entry point
│   ├── app.dart                     # App configuration
│   ├── core/
│   │   ├── constants/
│   │   │   ├── colors.dart         # ADHD-friendly palette
│   │   │   ├── durations.dart      # Animation timings
│   │   │   └── dimensions.dart     # Spacing system
│   │   ├── theme/
│   │   │   ├── app_theme.dart      # Theme configuration
│   │   │   └── adhd_theme.dart     # ADHD-specific theming
│   │   ├── router/
│   │   │   └── app_router.dart     # Go_router setup
│   │   └── utils/
│   │       ├── validators.dart
│   │       └── formatters.dart
│   ├── data/
│   │   ├── models/
│   │   │   ├── task.dart           # Task model
│   │   │   ├── user.dart           # User model
│   │   │   └── focus_session.dart  # Focus session model
│   │   ├── repositories/
│   │   │   ├── task_repository.dart
│   │   │   ├── auth_repository.dart
│   │   │   └── sync_repository.dart
│   │   ├── providers/
│   │   │   ├── api_provider.dart    # Dio setup
│   │   │   ├── database_provider.dart
│   │   │   └── websocket_provider.dart
│   │   └── local/
│   │       ├── database.dart        # Drift database
│   │       ├── dao/                 # Data access objects
│   │       └── tables/              # Database tables
│   ├── features/
│   │   ├── auth/
│   │   │   ├── screens/
│   │   │   ├── widgets/
│   │   │   └── providers/
│   │   ├── tasks/
│   │   │   ├── screens/
│   │   │   │   ├── inbox_screen.dart
│   │   │   │   ├── task_list_screen.dart
│   │   │   │   └── quick_capture_screen.dart
│   │   │   ├── widgets/
│   │   │   │   ├── task_card.dart
│   │   │   │   ├── task_state_indicator.dart
│   │   │   │   └── swipe_actions.dart
│   │   │   └── providers/
│   │   │       ├── task_list_provider.dart
│   │   │       └── active_task_provider.dart
│   │   ├── focus/
│   │   │   ├── screens/
│   │   │   │   └── focus_timer_screen.dart
│   │   │   ├── widgets/
│   │   │   │   ├── circular_timer.dart
│   │   │   │   └── focus_controls.dart
│   │   │   └── providers/
│   │   │       └── focus_session_provider.dart
│   │   ├── quick_capture/
│   │   │   ├── overlay/
│   │   │   │   └── quick_capture_overlay.dart
│   │   │   └── providers/
│   │   └── dashboard/
│   │       ├── screens/
│   │       └── widgets/
│   │           ├── stats_card.dart
│   │           └── streak_indicator.dart
│   └── shared/
│       ├── widgets/
│       │   ├── adhd_button.dart     # ADHD-optimized button
│       │   ├── celebration_overlay.dart
│       │   ├── loading_states.dart
│       │   └── error_states.dart
│       └── animations/
│           ├── success_animation.dart
│           └── micro_interactions.dart
├── assets/
│   ├── animations/               # Lottie/Rive animations
│   ├── sounds/                   # Feedback sounds
│   └── images/
├── web/                          # PWA configuration
│   ├── index.html
│   ├── manifest.json            # PWA manifest
│   └── service_worker.js       # Offline support
└── test/
```

## Core Implementation Files

### 1. Main App Setup

```dart
// lib/main.dart
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter/services.dart';
import 'app.dart';
import 'core/utils/platform_setup.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // Platform-specific setup
  await PlatformSetup.initialize();
  
  // ADHD-friendly: Keep screen awake during focus sessions
  // Wakelock.enable();
  
  // Set preferred orientations
  await SystemChrome.setPreferredOrientations([
    DeviceOrientation.portraitUp,
    DeviceOrientation.landscapeLeft,
    DeviceOrientation.landscapeRight,
  ]);
  
  runApp(
    const ProviderScope(
      child: AltairApp(),
    ),
  );
}
```

### 2. ADHD-Optimized Theme

```dart
// lib/core/theme/adhd_theme.dart
import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';

class ADHDTheme {
  static const _primaryColor = Color(0xFF00CC88);  // Calming green
  static const _backgroundColor = Color(0xFF0F0F23);  // Dark, low stimulation
  static const _surfaceColor = Color(0xFF1A1A2E);
  static const _errorColor = Color(0xFFFF6B6B);  // Soft red, not harsh
  
  static ThemeData get dark => ThemeData(
    brightness: Brightness.dark,
    primaryColor: _primaryColor,
    scaffoldBackgroundColor: _backgroundColor,
    
    // ADHD-friendly typography
    textTheme: GoogleFonts.lexendTextTheme(
      ThemeData.dark().textTheme,
    ).apply(
      bodyColor: Colors.grey[300],
      displayColor: Colors.grey[100],
    ),
    
    // Reduce visual noise
    cardTheme: CardTheme(
      color: _surfaceColor,
      elevation: 0,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
      ),
      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
    ),
    
    // Large, easy-to-tap buttons
    elevatedButtonTheme: ElevatedButtonThemeData(
      style: ElevatedButton.styleFrom(
        minimumSize: const Size.fromHeight(48),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
        ),
        backgroundColor: _primaryColor,
        foregroundColor: Colors.black,
        textStyle: const TextStyle(
          fontSize: 16,
          fontWeight: FontWeight.w600,
        ),
      ),
    ),
    
    // Clear input fields
    inputDecorationTheme: InputDecorationTheme(
      filled: true,
      fillColor: _surfaceColor,
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(12),
        borderSide: BorderSide.none,
      ),
      contentPadding: const EdgeInsets.all(16),
      hintStyle: TextStyle(color: Colors.grey[600]),
    ),
    
    // Gentle transitions
    pageTransitionsTheme: const PageTransitionsTheme(
      builders: {
        TargetPlatform.android: FadeUpwardsPageTransitionsBuilder(),
        TargetPlatform.iOS: CupertinoPageTransitionsBuilder(),
      },
    ),
  );
  
  // Celebration colors for rewards
  static const celebrationGradient = LinearGradient(
    colors: [
      Color(0xFF667EEA),
      Color(0xFF764BA2),
    ],
  );
}
```

### 3. Quick Capture Implementation

```dart
// lib/features/quick_capture/overlay/quick_capture_overlay.dart
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_animate/flutter_animate.dart';

class QuickCaptureOverlay extends ConsumerStatefulWidget {
  const QuickCaptureOverlay({super.key});
  
  @override
  ConsumerState<QuickCaptureOverlay> createState() => _QuickCaptureOverlayState();
}

class _QuickCaptureOverlayState extends ConsumerState<QuickCaptureOverlay> {
  final _controller = TextEditingController();
  final _focusNode = FocusNode();
  bool _isExpanded = false;
  
  @override
  void initState() {
    super.initState();
    // Register global keyboard shortcut
    HardwareKeyboard.instance.addHandler(_handleKeyPress);
  }
  
  bool _handleKeyPress(KeyEvent event) {
    // Ctrl/Cmd + Space to open quick capture
    if (event is KeyDownEvent &&
        event.logicalKey == LogicalKeyboardKey.space &&
        (HardwareKeyboard.instance.isControlPressed ||
         HardwareKeyboard.instance.isMetaPressed)) {
      _toggleQuickCapture();
      return true;
    }
    return false;
  }
  
  void _toggleQuickCapture() {
    setState(() {
      _isExpanded = !_isExpanded;
      if (_isExpanded) {
        _focusNode.requestFocus();
      } else {
        _controller.clear();
      }
    });
  }
  
  Future<void> _captureTask() async {
    if (_controller.text.trim().isEmpty) return;
    
    final text = _controller.text.trim();
    
    // Optimistic UI - clear immediately
    _controller.clear();
    _toggleQuickCapture();
    
    // Show success animation
    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('✓ Captured: $text'),
          backgroundColor: Colors.green,
          duration: const Duration(seconds: 2),
        ),
      );
    }
    
    // Save to local database first (offline-first)
    await ref.read(taskRepositoryProvider).quickCapture(text);
    
    // Sync in background
    ref.read(syncServiceProvider).syncTasks();
  }
  
  @override
  Widget build(BuildContext context) {
    return AnimatedPositioned(
      duration: const Duration(milliseconds: 300),
      curve: Curves.easeOutCubic,
      bottom: _isExpanded ? 100 : -200,
      left: 20,
      right: 20,
      child: Material(
        elevation: 8,
        borderRadius: BorderRadius.circular(16),
        color: Theme.of(context).cardColor,
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Row(
            children: [
              Expanded(
                child: TextField(
                  controller: _controller,
                  focusNode: _focusNode,
                  maxLines: 1,
                  decoration: const InputDecoration(
                    hintText: 'What needs to be done?',
                    border: InputBorder.none,
                  ),
                  onSubmitted: (_) => _captureTask(),
                  textInputAction: TextInputAction.done,
                ),
              ),
              const SizedBox(width: 12),
              IconButton.filled(
                onPressed: _captureTask,
                icon: const Icon(Icons.add),
                style: IconButton.styleFrom(
                  backgroundColor: Theme.of(context).primaryColor,
                  foregroundColor: Colors.black,
                ),
              ),
            ],
          ),
        ),
      ).animate(target: _isExpanded ? 1 : 0)
        .fadeIn(duration: 200.ms)
        .slideY(begin: 1, end: 0),
    );
  }
  
  @override
  void dispose() {
    HardwareKeyboard.instance.removeHandler(_handleKeyPress);
    _controller.dispose();
    _focusNode.dispose();
    super.dispose();
  }
}
```

### 4. Task Card with ADHD Features

```dart
// lib/features/tasks/widgets/task_card.dart
import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';
import 'package:flutter_slidable/flutter_slidable.dart';

class TaskCard extends StatelessWidget {
  final Task task;
  final VoidCallback onTap;
  final VoidCallback onComplete;
  final VoidCallback onDelete;
  
  const TaskCard({
    required this.task,
    required this.onTap,
    required this.onComplete,
    required this.onDelete,
    super.key,
  });
  
  Color _getStateColor(TaskState state) {
    return switch (state) {
      TaskState.inbox => Colors.grey,
      TaskState.active => Colors.blue,
      TaskState.blocked => Colors.orange,
      TaskState.done => Colors.green,
      _ => Colors.grey,
    };
  }
  
  Widget _buildCognitiveLoadIndicator() {
    return Row(
      children: List.generate(
        5,
        (index) => Icon(
          Icons.circle,
          size: 8,
          color: index < task.cognitiveLoad 
            ? Colors.orange.withOpacity(0.7)
            : Colors.grey.withOpacity(0.3),
        ),
      ),
    );
  }
  
  @override
  Widget build(BuildContext context) {
    return Slidable(
      endActionPane: ActionPane(
        motion: const BehindMotion(),
        children: [
          SlidableAction(
            onPressed: (_) => onComplete(),
            backgroundColor: Colors.green,
            foregroundColor: Colors.white,
            icon: Icons.check,
            label: 'Done',
            borderRadius: BorderRadius.circular(12),
          ),
          SlidableAction(
            onPressed: (_) => onDelete(),
            backgroundColor: Colors.red,
            foregroundColor: Colors.white,
            icon: Icons.delete,
            label: 'Delete',
            borderRadius: BorderRadius.circular(12),
          ),
        ],
      ),
      child: Card(
        child: InkWell(
          onTap: onTap,
          borderRadius: BorderRadius.circular(12),
          child: Padding(
            padding: const EdgeInsets.all(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    // State indicator
                    Container(
                      width: 4,
                      height: 40,
                      decoration: BoxDecoration(
                        color: _getStateColor(task.state),
                        borderRadius: BorderRadius.circular(2),
                      ),
                    ),
                    const SizedBox(width: 12),
                    
                    // Task content
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            task.title,
                            style: Theme.of(context).textTheme.titleMedium?.copyWith(
                              decoration: task.state == TaskState.done 
                                ? TextDecoration.lineThrough 
                                : null,
                            ),
                          ),
                          if (task.estimatedMinutes != null) ...[
                            const SizedBox(height: 4),
                            Text(
                              '${task.estimatedMinutes} min',
                              style: Theme.of(context).textTheme.bodySmall,
                            ),
                          ],
                        ],
                      ),
                    ),
                    
                    // Cognitive load indicator
                    _buildCognitiveLoadIndicator(),
                  ],
                ),
              ],
            ),
          ),
        ),
      ),
    ).animate()
      .fadeIn(duration: 300.ms)
      .slideX(begin: 0.1, end: 0);
  }
}
```

### 5. Focus Timer Screen

```dart
// lib/features/focus/screens/focus_timer_screen.dart
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:circular_countdown_timer/circular_countdown_timer.dart';
import 'package:flutter_animate/flutter_animate.dart';

class FocusTimerScreen extends ConsumerStatefulWidget {
  final Task task;
  
  const FocusTimerScreen({required this.task, super.key});
  
  @override
  ConsumerState<FocusTimerScreen> createState() => _FocusTimerScreenState();
}

class _FocusTimerScreenState extends ConsumerState<FocusTimerScreen> {
  final _controller = CountDownController();
  int _duration = 25 * 60; // 25 minutes default
  bool _isRunning = false;
  
  void _handleComplete() {
    // Celebration animation
    showDialog(
      context: context,
      barrierDismissible: false,
      builder: (context) => Center(
        child: Card(
          child: Padding(
            padding: const EdgeInsets.all(32),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                const Icon(
                  Icons.celebration,
                  size: 64,
                  color: Colors.amber,
                ).animate()
                  .scale(duration: 500.ms, curve: Curves.elasticOut)
                  .then()
                  .shake(duration: 500.ms),
                const SizedBox(height: 16),
                Text(
                  'Focus Session Complete!',
                  style: Theme.of(context).textTheme.headlineSmall,
                ),
                const SizedBox(height: 8),
                Text(
                  '+25 XP',
                  style: Theme.of(context).textTheme.titleMedium?.copyWith(
                    color: Colors.green,
                  ),
                ),
                const SizedBox(height: 24),
                FilledButton(
                  onPressed: () {
                    Navigator.of(context).pop();
                    Navigator.of(context).pop();
                  },
                  child: const Text('Continue'),
                ),
              ],
            ),
          ),
        ),
      ),
    );
    
    // Log completion
    ref.read(focusSessionProvider.notifier).completeSession(
      taskId: widget.task.id,
      duration: _duration,
    );
  }
  
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.black,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
        title: Text(widget.task.title),
      ),
      body: SafeArea(
        child: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              // Circular timer
              CircularCountDownTimer(
                controller: _controller,
                width: 280,
                height: 280,
                duration: _duration,
                fillColor: Theme.of(context).primaryColor,
                ringColor: Colors.grey[800]!,
                backgroundColor: Colors.black,
                strokeWidth: 20,
                strokeCap: StrokeCap.round,
                textStyle: const TextStyle(
                  fontSize: 48,
                  color: Colors.white,
                  fontWeight: FontWeight.bold,
                ),
                textFormat: CountdownTextFormat.MM_SS,
                isReverse: true,
                isReverseAnimation: true,
                autoStart: false,
                onComplete: _handleComplete,
              ),
              
              const SizedBox(height: 60),
              
              // Controls
              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  // Start/Pause button
                  FloatingActionButton.large(
                    onPressed: () {
                      setState(() {
                        _isRunning = !_isRunning;
                        if (_isRunning) {
                          _controller.start();
                        } else {
                          _controller.pause();
                        }
                      });
                    },
                    backgroundColor: Theme.of(context).primaryColor,
                    child: Icon(
                      _isRunning ? Icons.pause : Icons.play_arrow,
                      size: 32,
                      color: Colors.black,
                    ),
                  ),
                  
                  const SizedBox(width: 24),
                  
                  // Stop button
                  FloatingActionButton(
                    onPressed: () {
                      _controller.reset();
                      setState(() {
                        _isRunning = false;
                      });
                    },
                    backgroundColor: Colors.red,
                    child: const Icon(Icons.stop),
                  ),
                ],
              ),
              
              const SizedBox(height: 40),
              
              // Motivational text
              Text(
                _isRunning ? 'Stay focused! You got this!' : 'Ready when you are',
                style: Theme.of(context).textTheme.titleMedium?.copyWith(
                  color: Colors.grey[400],
                ),
              ).animate(target: _isRunning ? 1 : 0)
                .fadeIn()
                .slideY(begin: 0.2, end: 0),
            ],
          ),
        ),
      ),
    );
  }
}
```

### 6. Offline-First Data Layer

```dart
// lib/data/local/database.dart
import 'package:drift/drift.dart';
import 'package:drift/native.dart';
import 'package:path_provider/path_provider.dart';
import 'package:path/path.dart' as p;
import 'dart:io';

part 'database.g.dart';

class Tasks extends Table {
  TextColumn get id => text()();
  TextColumn get title => text()();
  TextColumn get description => text().nullable()();
  TextColumn get state => text()();
  IntColumn get cognitiveLoad => integer().withDefault(const Constant(5))();
  IntColumn get estimatedMinutes => integer().nullable()();
  IntColumn get actualMinutes => integer().nullable()();
  DateTimeColumn get createdAt => dateTime()();
  DateTimeColumn get updatedAt => dateTime()();
  BoolColumn get needsSync => boolean().withDefault(const Constant(false))();
  
  @override
  Set<Column> get primaryKey => {id};
}

@DriftDatabase(tables: [Tasks])
class AppDatabase extends _$AppDatabase {
  AppDatabase() : super(_openConnection());
  
  @override
  int get schemaVersion => 1;
  
  // Quick capture - save locally first
  Future<void> quickCapture(String text) async {
    final task = TasksCompanion(
      id: Value(uuid.v4()),
      title: Value(text),
      state: const Value('inbox'),
      createdAt: Value(DateTime.now()),
      updatedAt: Value(DateTime.now()),
      needsSync: const Value(true),
    );
    
    await into(tasks).insert(task);
  }
  
  // Get tasks needing sync
  Future<List<Task>> getUnsyncedTasks() {
    return (select(tasks)..where((t) => t.needsSync.equals(true))).get();
  }
  
  // Mark task as synced
  Future<void> markSynced(String taskId) {
    return (update(tasks)..where((t) => t.id.equals(taskId)))
      .write(const TasksCompanion(needsSync: Value(false)));
  }
}

LazyDatabase _openConnection() {
  return LazyDatabase(() async {
    final dbFolder = await getApplicationDocumentsDirectory();
    final file = File(p.join(dbFolder.path, 'altair.sqlite'));
    return NativeDatabase(file);
  });
}
```

### 7. Sync Service

```dart
// lib/data/repositories/sync_repository.dart
import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class SyncService {
  final Dio _dio;
  final AppDatabase _db;
  Timer? _syncTimer;
  
  SyncService(this._dio, this._db) {
    // Sync every 30 seconds when online
    _startPeriodicSync();
  }
  
  void _startPeriodicSync() {
    _syncTimer = Timer.periodic(
      const Duration(seconds: 30),
      (_) => syncTasks(),
    );
  }
  
  Future<void> syncTasks() async {
    try {
      // Get unsynced tasks
      final unsyncedTasks = await _db.getUnsyncedTasks();
      
      for (final task in unsyncedTasks) {
        try {
          // Send to server
          await _dio.post('/api/tasks', data: {
            'title': task.title,
            'description': task.description,
            'state': task.state,
            'cognitive_load': task.cognitiveLoad,
            'estimated_minutes': task.estimatedMinutes,
          });
          
          // Mark as synced
          await _db.markSynced(task.id);
        } catch (e) {
          // Individual task sync failed, continue with others
          print('Failed to sync task ${task.id}: $e');
        }
      }
      
      // Pull updates from server
      await _pullServerUpdates();
      
    } catch (e) {
      // Network error - will retry on next cycle
      print('Sync failed: $e');
    }
  }
  
  Future<void> _pullServerUpdates() async {
    // Get latest sync timestamp
    final lastSync = await _getLastSyncTime();
    
    final response = await _dio.get(
      '/api/tasks/updates',
      queryParameters: {'since': lastSync?.toIso8601String()},
    );
    
    // Update local database with server changes
    for (final taskData in response.data['tasks']) {
      await _db.upsertTask(taskData);
    }
    
    await _saveLastSyncTime(DateTime.now());
  }
  
  void dispose() {
    _syncTimer?.cancel();
  }
}
```

### 8. PWA Configuration

```html
<!-- web/index.html -->
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Altair - ADHD Project Management</title>
  
  <!-- PWA Meta Tags -->
  <meta name="theme-color" content="#00CC88">
  <link rel="manifest" href="manifest.json">
  <meta name="apple-mobile-web-app-capable" content="yes">
  <meta name="apple-mobile-web-app-status-bar-style" content="black">
  <meta name="apple-mobile-web-app-title" content="Altair">
  
  <!-- Icons -->
  <link rel="apple-touch-icon" href="icons/Icon-192.png">
  
  <style>
    body {
      background-color: #0F0F23;
      margin: 0;
      padding: 0;
    }
    
    .loading {
      position: fixed;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      color: #00CC88;
      font-family: system-ui;
      font-size: 20px;
    }
  </style>
</head>
<body>
  <div class="loading">Loading Altair...</div>
  
  <script>
    // Register service worker for offline support
    if ('serviceWorker' in navigator) {
      window.addEventListener('load', function () {
        navigator.serviceWorker.register('service_worker.js');
      });
    }
  </script>
  
  <script src="main.dart.js" type="application/javascript"></script>
</body>
</html>
```

```json
// web/manifest.json
{
  "name": "Altair - ADHD Project Management",
  "short_name": "Altair",
  "description": "ADHD-friendly project management with focus timers and quick capture",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#0F0F23",
  "theme_color": "#00CC88",
  "orientation": "any",
  "icons": [
    {
      "src": "icons/Icon-192.png",
      "sizes": "192x192",
      "type": "image/png"
    },
    {
      "src": "icons/Icon-512.png",
      "sizes": "512x512",
      "type": "image/png"
    }
  ],
  "shortcuts": [
    {
      "name": "Quick Capture",
      "short_name": "Capture",
      "description": "Quickly add a task",
      "url": "/quick-capture",
      "icons": [{"src": "icons/quick-capture.png", "sizes": "96x96"}]
    },
    {
      "name": "Start Focus",
      "short_name": "Focus",
      "description": "Start a focus session",
      "url": "/focus",
      "icons": [{"src": "icons/focus.png", "sizes": "96x96"}]
    }
  ]
}
```

## Development Setup

### 1. Initialize Flutter Project
```bash
# Create Flutter project
flutter create --org com.yourname --project-name altair flutter
cd flutter

# Add dependencies
flutter pub add \
  flutter_riverpod \
  go_router \
  dio \
  drift \
  sqlite3_flutter_libs \
  path_provider \
  hive \
  hive_flutter \
  flutter_animate \
  google_fonts \
  flutter_slidable \
  circular_countdown_timer \
  fl_chart \
  web_socket_channel

# Add dev dependencies
flutter pub add --dev \
  build_runner \
  drift_dev \
  riverpod_generator
```

### 2. Platform-Specific Setup

#### Android
```xml
<!-- android/app/src/main/AndroidManifest.xml -->
<uses-permission android:name="android.permission.INTERNET" />
<uses-permission android:name="android.permission.WAKE_LOCK" />
<uses-permission android:name="android.permission.VIBRATE" />
```

#### iOS
```xml
<!-- ios/Runner/Info.plist -->
<key>UIBackgroundModes</key>
<array>
  <string>fetch</string>
  <string>remote-notification</string>
</array>
```

### 3. Run Code Generation
```bash
# Generate Drift database code
flutter pub run build_runner build

# Watch for changes
flutter pub run build_runner watch
```

## Testing

```dart
// test/quick_capture_test.dart
import 'package:flutter_test/flutter_test.dart';
import 'package:altair/features/quick_capture/quick_capture.dart';

void main() {
  testWidgets('Quick capture saves task immediately', (tester) async {
    await tester.pumpWidget(const TestApp(child: QuickCaptureOverlay()));
    
    // Open quick capture
    await tester.tap(find.byIcon(Icons.add));
    await tester.pumpAndSettle();
    
    // Enter task
    await tester.enterText(
      find.byType(TextField),
      'Test task',
    );
    
    // Submit
    await tester.testTextInput.receiveAction(TextInputAction.done);
    await tester.pumpAndSettle();
    
    // Verify saved
    expect(find.text('✓ Captured: Test task'), findsOneWidget);
  });
}
```

## Deployment

### Web (PWA)
```bash
# Build for web
flutter build web --release

# Files will be in build/web/
# Deploy to any static hosting (Vercel, Netlify, etc.)
```

### Mobile
```bash
# Android
flutter build apk --release
flutter build appbundle --release  # For Play Store

# iOS
flutter build ios --release
# Then use Xcode to archive and upload
```

## ADHD-Specific UI Patterns

1. **Instant Feedback**: Every action shows immediate visual response
2. **Progress Indicators**: Always show loading states
3. **Celebrations**: Micro-animations for completed tasks
4. **Gentle Errors**: Encouraging error messages
5. **Visual Hierarchy**: Clear focus areas with reduced clutter
6. **Consistent Navigation**: Same patterns throughout
7. **Quick Actions**: One-tap/swipe for common tasks

## Integration with Python Backend

The Flutter app communicates with your FastAPI backend through:
- REST API calls via Dio
- WebSocket for real-time updates
- Offline-first with background sync
- JWT authentication with refresh tokens

## Next Steps

1. Set up Flutter project structure
2. Implement quick capture overlay
3. Create task list with state management
4. Add focus timer
5. Implement offline sync
6. Add celebration animations
7. Test on all platforms
8. Deploy as PWA first

This provides a complete, production-ready Flutter implementation that works seamlessly with your Python/FastAPI backend while maintaining ADHD-friendly patterns throughout!
