# Flutter Architecture - Altair

**ADHD-Friendly Task Management - Frontend Architecture**

> This document defines the architectural decisions, patterns, and structure for Altair's Flutter application. Written for developers with basic Flutter knowledge.

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Folder Structure](#folder-structure)
3. [State Management](#state-management)
4. [Offline-First Architecture](#offline-first-architecture)
5. [API Client Design](#api-client-design)
6. [Code Generation](#code-generation)
7. [Multi-Platform Strategy](#multi-platform-strategy)
8. [Key Patterns & Practices](#key-patterns--practices)

---

## Architecture Overview

### Core Principles

**ADHD-First Design:**
- **Zero data loss** - Offline-first ensures tasks are never lost
- **Instant feedback** - Optimistic updates, no waiting
- **Predictable behavior** - Clear loading/error states
- **Forgiving UX** - Easy undo, no destructive actions without confirmation

**Technical Approach:**
- **Feature-based architecture** - Each feature is self-contained
- **Offline-first with Drift** - SQLite for structured local storage
- **Reactive state with Riverpod** - Declarative UI updates
- **Type-safe everything** - Code generation for models and providers

### Architecture Layers

```
┌─────────────────────────────────────┐
│         Presentation Layer          │
│    (Widgets, Screens, UI Logic)     │
└─────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────┐
│      Application Layer              │
│   (Providers, State, Controllers)   │
└─────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────┐
│         Domain Layer                │
│     (Models, Repositories)          │
└─────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────┐
│         Data Layer                  │
│  (API Client, Drift DB, Services)   │
└─────────────────────────────────────┘
```

---

## Folder Structure

### Complete Structure

```
lib/
├── main.dart                     # App entry point
├── app.dart                      # Root widget with routing
│
├── core/                         # Core utilities & config
│   ├── config/
│   │   ├── app_config.dart      # Environment config
│   │   └── api_config.dart      # API endpoints
│   ├── constants/
│   │   ├── app_constants.dart   # App-wide constants
│   │   └── storage_keys.dart    # Local storage keys
│   ├── error/
│   │   ├── failures.dart        # Error types
│   │   └── error_handler.dart   # Global error handling
│   ├── network/
│   │   ├── api_client.dart      # Dio HTTP client
│   │   ├── api_interceptor.dart # Auth & logging
│   │   └── api_response.dart    # Response wrapper
│   ├── database/
│   │   ├── app_database.dart    # Drift database
│   │   └── database_tables.dart # Table definitions
│   └── utils/
│       ├── logger.dart          # Logging utility
│       └── date_utils.dart      # Date helpers
│
├── features/                     # Feature modules
│   ├── auth/
│   │   ├── data/
│   │   │   ├── models/
│   │   │   │   ├── user_model.dart        # API response model
│   │   │   │   └── auth_tokens_model.dart # Token model
│   │   │   ├── repositories/
│   │   │   │   └── auth_repository.dart   # Auth API calls
│   │   │   └── datasources/
│   │   │       ├── auth_remote_datasource.dart  # API
│   │   │       └── auth_local_datasource.dart   # Secure storage
│   │   ├── domain/
│   │   │   ├── entities/
│   │   │   │   └── user.dart              # Domain model
│   │   │   └── usecases/
│   │   │       ├── login_usecase.dart
│   │   │       └── register_usecase.dart
│   │   ├── presentation/
│   │   │   ├── providers/
│   │   │   │   └── auth_provider.dart     # State management
│   │   │   ├── screens/
│   │   │   │   ├── login_screen.dart
│   │   │   │   └── register_screen.dart
│   │   │   └── widgets/
│   │   │       └── auth_form.dart
│   │   └── auth.dart                      # Public exports
│   │
│   ├── tasks/
│   │   ├── data/
│   │   │   ├── models/
│   │   │   │   └── task_model.dart
│   │   │   ├── repositories/
│   │   │   │   └── task_repository.dart
│   │   │   └── datasources/
│   │   │       ├── task_remote_datasource.dart
│   │   │       └── task_local_datasource.dart
│   │   ├── domain/
│   │   │   ├── entities/
│   │   │   │   └── task.dart
│   │   │   └── usecases/
│   │   │       ├── create_task_usecase.dart
│   │   │       └── sync_tasks_usecase.dart
│   │   ├── presentation/
│   │   │   ├── providers/
│   │   │   │   ├── task_provider.dart
│   │   │   │   └── task_list_provider.dart
│   │   │   ├── screens/
│   │   │   │   ├── task_list_screen.dart
│   │   │   │   └── task_detail_screen.dart
│   │   │   └── widgets/
│   │   │       ├── task_card.dart
│   │   │       └── quick_capture_bar.dart
│   │   └── tasks.dart
│   │
│   └── sync/                     # Background sync feature
│       ├── data/
│       ├── domain/
│       └── presentation/
│
└── shared/                       # Shared across features
    ├── widgets/
    │   ├── loading_indicator.dart
    │   ├── error_view.dart
    │   └── empty_state.dart
    ├── theme/
    │   ├── app_theme.dart
    │   ├── app_colors.dart
    │   └── app_text_styles.dart
    └── extensions/
        ├── context_extensions.dart
        └── datetime_extensions.dart
```

### Why This Structure?

**Feature-based organization:**
- Each feature (`auth/`, `tasks/`) is independent
- Easy to find related code
- Can be extracted to packages later
- Clear boundaries between features

**Three-layer architecture per feature:**
- **Data** - External data (API, database)
- **Domain** - Business logic (entities, use cases)
- **Presentation** - UI (screens, widgets, providers)

**Benefits for ADHD development:**
- Predictable file locations
- Reduced cognitive load ("where does this go?")
- Clear separation of concerns
- Easy to navigate when hyperfocused

---

## State Management

### Riverpod Pattern

We use **Riverpod 2.0** with **code generation** for type safety and DX.

**Core Concepts:**

1. **Providers** - Reactive data sources
2. **Notifiers** - State controllers
3. **Consumers** - Widgets that watch providers

### Provider Types

```dart
// 1. Simple provider - Immutable, rarely changes
@riverpod
ApiClient apiClient(ApiClientRef ref) {
  return ApiClient(baseUrl: AppConfig.apiUrl);
}

// 2. Future provider - Async data fetching
@riverpod
Future<User> currentUser(CurrentUserRef ref) async {
  final authRepo = ref.watch(authRepositoryProvider);
  return authRepo.getCurrentUser();
}

// 3. Stream provider - Real-time updates
@riverpod
Stream<List<Task>> taskStream(TaskStreamRef ref) {
  final db = ref.watch(databaseProvider);
  return db.watchAllTasks();
}

// 4. Notifier - Complex stateful logic (RECOMMENDED)
@riverpod
class TaskList extends _$TaskList {
  @override
  FutureOr<List<Task>> build() async {
    // Initial state
    return _fetchTasks();
  }

  Future<void> createTask(String title) async {
    state = const AsyncValue.loading();

    state = await AsyncValue.guard(() async {
      final task = await _repository.createTask(title);
      final current = state.value ?? [];
      return [...current, task];
    });
  }
}
```

### State Management Rules

**DO:**
- Use `AsyncNotifier` for async state that changes
- Keep business logic in repositories/use cases
- Handle loading/error states with `AsyncValue`
- Use `ref.watch()` in build methods
- Use `ref.read()` in event handlers

**DON'T:**
- Put API calls directly in providers
- Forget to handle loading/error states
- Use `setState()` in complex widgets (use Riverpod)
- Share mutable state between features

### Example: Task Creation Flow

```dart
// 1. Repository (data layer)
class TaskRepository {
  final ApiClient _api;
  final AppDatabase _db;

  Future<Task> createTask(String title) async {
    // Optimistic update to local DB
    final localTask = await _db.insertTask(title);

    try {
      // Sync to server
      final serverTask = await _api.createTask(title);
      await _db.updateTaskId(localTask.id, serverTask.id);
      return serverTask;
    } catch (e) {
      // Keep local task, mark for retry
      await _db.markForSync(localTask.id);
      return localTask;
    }
  }
}

// 2. Provider (application layer)
@riverpod
class TaskList extends _$TaskList {
  @override
  FutureOr<List<Task>> build() {
    final db = ref.watch(databaseProvider);
    return db.getAllTasks();
  }

  Future<void> quickCapture(String title) async {
    final repo = ref.read(taskRepositoryProvider);

    state = const AsyncValue.loading();
    state = await AsyncValue.guard(() async {
      await repo.createTask(title);
      return ref.read(databaseProvider).getAllTasks();
    });
  }
}

// 3. UI (presentation layer)
class QuickCaptureWidget extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final taskList = ref.watch(taskListProvider);

    return taskList.when(
      data: (tasks) => TaskListView(tasks: tasks),
      loading: () => LoadingIndicator(),
      error: (err, stack) => ErrorView(error: err),
    );
  }

  void _onSubmit(WidgetRef ref, String title) {
    ref.read(taskListProvider.notifier).quickCapture(title);
  }
}
```

---

## Offline-First Architecture

### Why Drift?

**Drift = SQLite + Type Safety + Reactivity**

- Structured queries (filter, sort, join)
- Reactive streams (UI updates automatically)
- Type-safe queries at compile time
- Migration support
- Multi-platform (mobile, web, desktop)

### Database Schema

```dart
// lib/core/database/database_tables.dart
import 'package:drift/drift.dart';

class Tasks extends Table {
  TextColumn get id => text()(); // UUID from server
  TextColumn get localId => text()(); // Local UUID
  TextColumn get title => text()();
  TextColumn get description => text().nullable()();
  TextColumn get state => text()(); // TaskState enum
  IntColumn get cognitiveLoad => integer().withDefault(const Constant(5))();
  IntColumn get estimatedMinutes => integer().nullable()();
  TextColumn get userId => text()();

  // Sync metadata
  BoolColumn get isSynced => boolean().withDefault(const Constant(false))();
  BoolColumn get isPendingSync => boolean().withDefault(const Constant(false))();
  DateTimeColumn get lastSyncedAt => dateTime().nullable()();

  // Timestamps
  DateTimeColumn get createdAt => dateTime()();
  DateTimeColumn get updatedAt => dateTime()();

  @override
  Set<Column> get primaryKey => {localId};
}

class Users extends Table {
  TextColumn get id => text()();
  TextColumn get email => text()();
  TextColumn get username => text().nullable()();
  TextColumn get adhdProfile => text().map(const AdhdProfileConverter())();

  @override
  Set<Column> get primaryKey => {id};
}

// Database class
@DriftDatabase(tables: [Tasks, Users])
class AppDatabase extends _$AppDatabase {
  AppDatabase() : super(_openConnection());

  @override
  int get schemaVersion => 1;

  // Watch all tasks (reactive stream)
  Stream<List<Task>> watchAllTasks() {
    return select(tasks).watch();
  }

  // Watch tasks by state
  Stream<List<Task>> watchTasksByState(String state) {
    return (select(tasks)..where((t) => t.state.equals(state))).watch();
  }

  // Get unsynced tasks
  Future<List<Task>> getUnsyncedTasks() {
    return (select(tasks)..where((t) => t.isPendingSync.equals(true))).get();
  }
}

// Platform-specific connection
LazyDatabase _openConnection() {
  return LazyDatabase(() async {
    final dbFolder = await getApplicationDocumentsDirectory();
    final file = File(p.join(dbFolder.path, 'altair.sqlite'));
    return NativeDatabase.createInBackground(file);
  });
}
```

### Sync Strategy

**Optimistic Updates Pattern:**

```dart
class TaskRepository {
  final ApiClient _api;
  final AppDatabase _db;

  Future<Task> createTask(String title) async {
    // 1. Create local task immediately (optimistic)
    final localId = const Uuid().v4();
    final task = TasksCompanion(
      localId: Value(localId),
      title: Value(title),
      state: Value('inbox'),
      isSynced: Value(false),
      isPendingSync: Value(true),
      createdAt: Value(DateTime.now()),
    );

    await _db.into(_db.tasks).insert(task);

    // 2. Try to sync to server in background
    _syncTask(localId);

    return _db.getTaskByLocalId(localId);
  }

  Future<void> _syncTask(String localId) async {
    try {
      final task = await _db.getTaskByLocalId(localId);
      final response = await _api.createTask(task.toApiModel());

      // Update with server ID
      await _db.updateTask(
        task.copyWith(
          id: response.id,
          isSynced: true,
          isPendingSync: false,
          lastSyncedAt: DateTime.now(),
        ),
      );
    } catch (e) {
      // Keep pending, retry later
      print('Sync failed: $e');
    }
  }
}
```

**Background Sync Service:**

```dart
@riverpod
class SyncService extends _$SyncService {
  Timer? _syncTimer;

  @override
  void build() {
    // Start periodic sync every 30 seconds
    _syncTimer = Timer.periodic(Duration(seconds: 30), (_) => sync());
  }

  Future<void> sync() async {
    final db = ref.read(databaseProvider);
    final api = ref.read(apiClientProvider);

    // Get all unsynced tasks
    final unsynced = await db.getUnsyncedTasks();

    for (final task in unsynced) {
      try {
        final response = await api.createTask(task.toApiModel());
        await db.updateTask(task.copyWith(
          id: response.id,
          isSynced: true,
          isPendingSync: false,
        ));
      } catch (e) {
        // Continue to next task
      }
    }
  }
}
```

---

## API Client Design

### Dio Configuration

```dart
// lib/core/network/api_client.dart
class ApiClient {
  final Dio _dio;

  ApiClient({required String baseUrl}) : _dio = Dio(
    BaseOptions(
      baseUrl: baseUrl,
      connectTimeout: Duration(seconds: 10),
      receiveTimeout: Duration(seconds: 10),
      headers: {'Content-Type': 'application/json'},
    ),
  ) {
    _dio.interceptors.addAll([
      AuthInterceptor(),
      LoggingInterceptor(),
      ErrorInterceptor(),
    ]);
  }

  // Task endpoints
  Future<TaskModel> createTask(CreateTaskDto dto) async {
    final response = await _dio.post('/api/tasks', data: dto.toJson());
    return TaskModel.fromJson(response.data);
  }

  Future<List<TaskModel>> getTasks() async {
    final response = await _dio.get('/api/tasks');
    return (response.data as List)
      .map((json) => TaskModel.fromJson(json))
      .toList();
  }
}
```

### Auth Interceptor

```dart
class AuthInterceptor extends Interceptor {
  @override
  void onRequest(
    RequestOptions options,
    RequestInterceptorHandler handler,
  ) async {
    // Get token from secure storage
    final token = await SecureStorage.getAccessToken();

    if (token != null) {
      options.headers['Authorization'] = 'Bearer $token';
    }

    handler.next(options);
  }

  @override
  void onError(DioException err, ErrorInterceptorHandler handler) async {
    if (err.response?.statusCode == 401) {
      // Try to refresh token
      final refreshed = await _refreshToken();

      if (refreshed) {
        // Retry original request
        return handler.resolve(await _retry(err.requestOptions));
      }
    }

    handler.next(err);
  }
}
```

---

## Code Generation

### Required Packages

```yaml
dependencies:
  freezed_annotation: ^2.4.1
  json_annotation: ^4.8.1
  riverpod_annotation: ^2.3.0
  drift: ^2.14.0

dev_dependencies:
  build_runner: ^2.4.7
  freezed: ^2.4.5
  json_serializable: ^6.7.1
  riverpod_generator: ^2.3.9
  drift_dev: ^2.14.0
```

### Model with Freezed

```dart
import 'package:freezed_annotation/freezed_annotation.dart';

part 'task.freezed.dart';
part 'task.g.dart';

@freezed
class Task with _$Task {
  const factory Task({
    required String id,
    required String title,
    String? description,
    required TaskState state,
    @Default(5) int cognitiveLoad,
    int? estimatedMinutes,
    required DateTime createdAt,
    required DateTime updatedAt,
  }) = _Task;

  factory Task.fromJson(Map<String, dynamic> json) => _$TaskFromJson(json);
}

enum TaskState {
  inbox,
  triaged,
  active,
  blocked,
  done,
  archived,
}
```

### Generate Code

```bash
# Generate all (freezed, json, riverpod, drift)
dart run build_runner build --delete-conflicting-outputs

# Watch mode (regenerates on save)
dart run build_runner watch --delete-conflicting-outputs
```

---

## Multi-Platform Strategy

### Platform-Specific Code

```dart
// lib/core/platform/platform_info.dart
import 'dart:io' show Platform;
import 'package:flutter/foundation.dart' show kIsWeb;

class PlatformInfo {
  static bool get isWeb => kIsWeb;
  static bool get isAndroid => !kIsWeb && Platform.isAndroid;
  static bool get isIOS => !kIsWeb && Platform.isIOS;
  static bool get isDesktop => isMacOS || isWindows || isLinux;
  static bool get isMacOS => !kIsWeb && Platform.isMacOS;
  static bool get isWindows => !kIsWeb && Platform.isWindows;
  static bool get isLinux => !kIsWeb && Platform.isLinux;

  static bool get isMobile => isAndroid || isIOS;
}

// Usage in widgets
class QuickCaptureBar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    if (PlatformInfo.isDesktop) {
      return DesktopQuickCapture(); // Keyboard shortcuts
    } else {
      return MobileQuickCapture(); // Floating action button
    }
  }
}
```

### Responsive Layout

```dart
class ResponsiveLayout extends StatelessWidget {
  final Widget mobile;
  final Widget? tablet;
  final Widget? desktop;

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        if (constraints.maxWidth >= 1200) {
          return desktop ?? tablet ?? mobile;
        } else if (constraints.maxWidth >= 768) {
          return tablet ?? mobile;
        } else {
          return mobile;
        }
      },
    );
  }
}
```

---

## Key Patterns & Practices

### 1. Error Handling

```dart
// Sealed class for typed errors
@freezed
sealed class Failure with _$Failure {
  const factory Failure.network(String message) = NetworkFailure;
  const factory Failure.auth(String message) = AuthFailure;
  const factory Failure.validation(String message) = ValidationFailure;
  const factory Failure.unknown(String message) = UnknownFailure;
}

// In repositories
Future<Either<Failure, Task>> createTask(String title) async {
  try {
    final task = await _api.createTask(title);
    return Right(task);
  } on DioException catch (e) {
    if (e.type == DioExceptionType.connectionTimeout) {
      return Left(Failure.network('Connection timeout'));
    }
    return Left(Failure.unknown(e.message ?? 'Unknown error'));
  }
}
```

### 2. Loading States

```dart
class TaskListScreen extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tasksAsync = ref.watch(taskListProvider);

    return tasksAsync.when(
      data: (tasks) => ListView.builder(
        itemCount: tasks.length,
        itemBuilder: (context, index) => TaskCard(task: tasks[index]),
      ),
      loading: () => Center(child: CircularProgressIndicator()),
      error: (error, stack) => ErrorView(
        message: error.toString(),
        onRetry: () => ref.refresh(taskListProvider),
      ),
    );
  }
}
```

### 3. Dependency Injection

```dart
// All dependencies as providers
@riverpod
AppDatabase database(DatabaseRef ref) {
  return AppDatabase();
}

@riverpod
ApiClient apiClient(ApiClientRef ref) {
  return ApiClient(baseUrl: AppConfig.apiUrl);
}

@riverpod
TaskRepository taskRepository(TaskRepositoryRef ref) {
  final api = ref.watch(apiClientProvider);
  final db = ref.watch(databaseProvider);
  return TaskRepository(api: api, db: db);
}
```

### 4. Navigation

```dart
// lib/app.dart with go_router
final _router = GoRouter(
  routes: [
    GoRoute(
      path: '/',
      redirect: (context, state) {
        final isLoggedIn = /* check auth */;
        return isLoggedIn ? '/tasks' : '/login';
      },
    ),
    GoRoute(
      path: '/login',
      builder: (context, state) => LoginScreen(),
    ),
    GoRoute(
      path: '/tasks',
      builder: (context, state) => TaskListScreen(),
    ),
    GoRoute(
      path: '/tasks/:id',
      builder: (context, state) {
        final id = state.pathParameters['id']!;
        return TaskDetailScreen(taskId: id);
      },
    ),
  ],
);

class App extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      routerConfig: _router,
      theme: AppTheme.light,
      darkTheme: AppTheme.dark,
    );
  }
}
```

---

## Next Steps

With this architecture foundation:

1. **Set up folder structure** - Create all directories
2. **Configure code generation** - Update pubspec.yaml, run build_runner
3. **Build core layer** - API client, database, error handling
4. **Implement auth feature** - Complete vertical slice
5. **Add tasks feature** - With offline-first sync
6. **Test on all platforms** - Mobile, web, desktop

**Remember:**
- This architecture scales from MVP to production
- Each feature is independent and testable
- Offline-first prevents data loss (critical for ADHD users)
- Code generation reduces boilerplate and bugs

---

**Last Updated:** October 2025
**Status:** Foundation Architecture
**Next:** Implementation Phase
