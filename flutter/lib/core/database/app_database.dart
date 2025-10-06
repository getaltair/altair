// Drift database schema for Altair's offline-first task management.
//
// This database implements an offline-first architecture specifically designed
// for ADHD users to prevent data loss and provide instant feedback. Key features:
//
// - [Tasks] table with cognitive load tracking and state machine
// - [Users] table with ADHD profile customization
// - Sync metadata (lastSyncedAt, version, pendingSync) for background synchronization
// - Local-first primary keys with server IDs added after sync
// - Optimized indexes for ADHD-specific query patterns
//
// The schema mirrors the backend PostgreSQL models but adds offline sync
// capabilities. See backend/altair/models/ for the source of truth.

import 'dart:convert';
import 'dart:io';

import 'package:drift/drift.dart';
import 'package:drift/native.dart';
import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';
import 'package:uuid/uuid.dart';

part 'app_database.g.dart';

// ============================================================================
// Enums - Domain types for type-safe state management
// ============================================================================

/// Task state machine for ADHD-friendly workflow.
///
/// Matches backend TaskState enum exactly.
/// Flow: inbox → triaged → active/blocked → done → archived
enum TaskState {
  /// Newly captured tasks, no decisions required
  inbox,

  /// Task has been reviewed and categorized
  triaged,

  /// Currently being worked on
  active,

  /// Cannot proceed, waiting on external dependency
  blocked,

  /// Completed (celebration time!)
  done,

  /// Completed tasks after 30 days (for historical reference)
  archived,
}

// ============================================================================
// Type Converters - Handle complex types in SQLite
// ============================================================================

/// Converts between Dart Map and JSON string for storage.
///
/// Used for storing ADHD profiles and task metadata as JSONB-like fields.
class JsonTypeConverter extends TypeConverter<Map<String, dynamic>, String> {
  const JsonTypeConverter();

  @override
  Map<String, dynamic> fromSql(String fromDb) {
    if (fromDb.isEmpty) return {};
    return json.decode(fromDb) as Map<String, dynamic>;
  }

  @override
  String toSql(Map<String, dynamic> value) {
    return json.encode(value);
  }
}

// ============================================================================
// Base Model - Common fields for all tables
// ============================================================================

/// Base mixin providing common fields for all domain tables.
///
/// Includes:
/// - UUID primary key (client-generated for offline-first)
/// - Timestamp tracking (created, updated)
/// - Sync metadata for offline-first architecture
mixin BaseModel on Table {
  /// Client-generated UUID, serves as primary key for offline-first.
  ///
  /// This is the local ID that persists even before syncing to server.
  TextColumn get id => text().clientDefault(() => const Uuid().v4())();

  /// Server-assigned ID after successful sync.
  ///
  /// Nullable until task is synced to backend. Used to map local tasks
  /// to their server counterparts.
  TextColumn get serverId => text().nullable()();

  /// Timestamp when record was created locally.
  DateTimeColumn get createdAt =>
      dateTime().clientDefault(() => DateTime.now())();

  /// Timestamp when record was last updated locally.
  DateTimeColumn get updatedAt =>
      dateTime().clientDefault(() => DateTime.now())();

  /// Timestamp of last successful sync with server.
  ///
  /// Null if never synced. Used to determine if local changes are newer
  /// than server state for conflict resolution.
  DateTimeColumn get lastSyncedAt => dateTime().nullable()();

  /// Version number for optimistic locking and conflict resolution.
  ///
  /// Incremented on each update. Server rejects updates if version
  /// doesn't match, triggering conflict resolution flow.
  IntColumn get version => integer().withDefault(const Constant(1))();

  /// Flag indicating this record has pending changes to sync.
  ///
  /// Set to true when record is created/updated locally.
  /// Set to false after successful sync to server.
  BoolColumn get pendingSync => boolean().withDefault(const Constant(true))();

  @override
  Set<Column> get primaryKey => {id};
}

// ============================================================================
// User Table
// ============================================================================

/// Users table storing authentication and ADHD profile data.
///
/// Matches backend User model with offline sync support.
@DataClassName('User')
class Users extends Table with BaseModel {
  /// User's email address (unique identifier for login).
  TextColumn get email => text().unique()();

  /// Optional display username (unique if provided).
  TextColumn get username => text().unique().nullable()();

  /// Argon2 hashed password (never exposed in API responses).
  TextColumn get passwordHash => text()();

  /// Account status flag (inactive users can't login).
  BoolColumn get isActive => boolean().withDefault(const Constant(true))();

  /// ADHD-specific preferences and settings stored as JSONB.
  ///
  /// Contains:
  /// - preferred_focus_duration: Minutes for Pomodoro sessions
  /// - break_duration: Minutes for break periods
  /// - notification_preferences: Sound/visual notification settings
  /// - sensory_preferences: UI customizations for sensory needs
  /// - best_focus_times: Times of day when user focuses best
  /// - common_distractions: Known distraction triggers
  TextColumn get adhdProfile =>
      text().map(const JsonTypeConverter()).clientDefault(() => '{}')();
}

// ============================================================================
// Task Table
// ============================================================================

/// Tasks table with ADHD-specific attributes for cognitive load management.
///
/// Matches backend Task model exactly with additional offline sync metadata.
@DataClassName('Task')
class Tasks extends Table with BaseModel {
  /// Brief task description (required, max 255 characters).
  ///
  /// Matches backend model constraint.
  TextColumn get title => text().withLength(min: 1, max: 255)();

  /// Detailed task information (optional long text).
  TextColumn get description => text().nullable()();

  /// Foreign key to user who owns this task.
  ///
  /// Uses CASCADE delete - when user is deleted, their tasks are removed.
  TextColumn get userId =>
      text().references(Users, #id, onDelete: KeyAction.cascade)();

  /// Current task state in the ADHD-friendly workflow.
  ///
  /// Uses IntEnum for better performance (stores as integer, not text).
  /// Defaults to inbox for friction-free capture.
  IntColumn get state =>
      intEnum<TaskState>().withDefault(Constant(TaskState.inbox.index))();

  /// Subjective mental effort rating (1-10 scale).
  ///
  /// Helps users choose tasks matching current energy levels.
  /// Defaults to 5 (medium difficulty).
  IntColumn get cognitiveLoad => integer()
      .customConstraint('CHECK(cognitive_load BETWEEN 1 AND 10)')
      .withDefault(const Constant(5))();

  /// User's time estimate in minutes (nullable).
  ///
  /// Backend uses integer, so we do too for exact match.
  IntColumn get estimatedMinutes => integer().nullable()();
}

// ============================================================================
// Database Definition
// ============================================================================

/// Main application database with offline-first support.
///
/// Provides type-safe queries and reactive streams for UI updates.
@DriftDatabase(tables: [Users, Tasks])
class AppDatabase extends _$AppDatabase {
  /// Creates database instance with the given executor.
  ///
  /// Use [createConnection()] helper to get platform-appropriate executor.
  AppDatabase(super.e);

  @override
  int get schemaVersion => 1;

  @override
  MigrationStrategy get migration => MigrationStrategy(
    onCreate: (Migrator m) async {
      // Create all tables on first run
      await m.createAll();
    },
    onUpgrade: (Migrator m, int from, int to) async {
      // Future migrations will go here
      // Example:
      // if (from < 2) {
      //   await m.addColumn(tasks, tasks.actualMinutes);
      // }
    },
    beforeOpen: (details) async {
      // Enable foreign key constraints
      await customStatement('PRAGMA foreign_keys = ON');

      if (details.wasCreated) {
        // Database was created for the first time
        // Could seed initial data here if needed
        print('Database created, version ${details.versionNow}');
      }
    },
  );

  /// Indexes for optimized ADHD-specific queries.
  ///
  /// These support common query patterns:
  /// - User's tasks filtered by state
  /// - Finding tasks pending sync
  /// - Cognitive load sorting for energy-based task selection
  @override
  List<DatabaseSchemaEntity> get allSchemaEntities => [
    ...super.allSchemaEntities,

    // Fast task list filtering by user and state
    Index(
      'idx_tasks_user_state',
      'CREATE INDEX idx_tasks_user_state ON tasks(user_id, state)',
    ),

    // Quick lookup of unsynced tasks for background sync
    Index(
      'idx_tasks_pending_sync',
      'CREATE INDEX idx_tasks_pending_sync ON tasks(pending_sync) WHERE pending_sync = 1',
    ),

    // Cognitive load sorting within states
    // Enables "show me easy tasks" queries
    Index(
      'idx_tasks_state_load',
      'CREATE INDEX idx_tasks_state_load ON tasks(state, cognitive_load)',
    ),
  ];

  // ==========================================================================
  // Query Methods - Task Operations
  // ==========================================================================

  /// Watch all tasks for a user, ordered by creation date (newest first).
  ///
  /// Returns a reactive stream that automatically updates the UI when tasks change.
  Stream<List<Task>> watchAllTasks(String userId) {
    return (select(tasks)
          ..where((t) => t.userId.equals(userId))
          ..orderBy([(t) => OrderingTerm.desc(t.createdAt)]))
        .watch();
  }

  /// Watch tasks filtered by state.
  ///
  /// Useful for inbox view, active tasks list, etc.
  Stream<List<Task>> watchTasksByState(String userId, TaskState state) {
    return (select(tasks)
          ..where((t) => t.userId.equals(userId) & t.state.equalsValue(state))
          ..orderBy([(t) => OrderingTerm.desc(t.createdAt)]))
        .watch();
  }

  /// Get tasks that haven't been synced to server yet.
  ///
  /// Used by background sync service to batch upload pending changes.
  Future<List<Task>> getUnsyncedTasks() {
    return (select(tasks)..where((t) => t.pendingSync.equals(true))).get();
  }

  /// Get a single task by its local ID.
  Future<Task?> getTaskById(String id) {
    return (select(tasks)..where((t) => t.id.equals(id))).getSingleOrNull();
  }

  /// Get a task by its server ID (after sync).
  Future<Task?> getTaskByServerId(String serverId) {
    return (select(
      tasks,
    )..where((t) => t.serverId.equals(serverId))).getSingleOrNull();
  }

  /// Create a new task with optimistic UI update.
  ///
  /// Returns the created task immediately for UI rendering.
  /// Marked as pending sync for background upload.
  Future<Task> createTask(TasksCompanion task) {
    return into(tasks).insertReturning(task);
  }

  /// Update an existing task.
  ///
  /// Returns true if task was found and updated.
  /// Automatically marks task as pending sync.
  Future<bool> updateTask(String id, TasksCompanion task) async {
    final updated = await (update(tasks)..where((t) => t.id.equals(id))).write(
      task.copyWith(
        updatedAt: Value(DateTime.now()),
        pendingSync: const Value(true),
      ),
    );
    return updated > 0;
  }

  /// Mark task as successfully synced with server.
  ///
  /// Updates server ID and sync metadata after successful API call.
  Future<void> markTaskSynced(String localId, String serverId) {
    return (update(tasks)..where((t) => t.id.equals(localId))).write(
      TasksCompanion(
        serverId: Value(serverId),
        pendingSync: const Value(false),
        lastSyncedAt: Value(DateTime.now()),
      ),
    );
  }

  /// Delete a task (consider soft delete for ADHD users - don't lose data!).
  Future<int> deleteTask(String id) {
    return (delete(tasks)..where((t) => t.id.equals(id))).go();
  }

  /// Transition task to a new state.
  ///
  /// Updates task state and marks for sync.
  Future<bool> transitionTaskState(String id, TaskState newState) async {
    final task = await getTaskById(id);
    if (task == null) return false;

    final update = TasksCompanion(
      state: Value(newState),
      updatedAt: Value(DateTime.now()),
      pendingSync: const Value(true),
    );

    return updateTask(id, update);
  }

  // ==========================================================================
  // Query Methods - User Operations
  // ==========================================================================

  /// Get user by ID.
  Future<User?> getUserById(String id) {
    return (select(users)..where((u) => u.id.equals(id))).getSingleOrNull();
  }

  /// Get user by email (for login).
  Future<User?> getUserByEmail(String email) {
    return (select(
      users,
    )..where((u) => u.email.equals(email))).getSingleOrNull();
  }

  /// Upsert user (insert or update if exists).
  ///
  /// Used after successful login to cache user data locally.
  Future<void> upsertUser(UsersCompanion user) {
    return into(users).insertOnConflictUpdate(user);
  }

  /// Update user's ADHD profile settings.
  Future<void> updateAdhdProfile(String userId, Map<String, dynamic> profile) {
    return (update(users)..where((u) => u.id.equals(userId))).write(
      UsersCompanion(
        adhdProfile: Value(profile),
        updatedAt: Value(DateTime.now()),
      ),
    );
  }
}

// ============================================================================
// Database Connection Helper
// ============================================================================

/// Creates a platform-appropriate database connection.
///
/// For mobile/desktop: SQLite file in app documents directory.
/// For web: Will need different implementation using drift/web_worker.
LazyDatabase createConnection() {
  return LazyDatabase(() async {
    final dbFolder = await getApplicationDocumentsDirectory();
    final file = File(p.join(dbFolder.path, 'altair.sqlite'));

    return NativeDatabase.createInBackground(
      file,
      logStatements: true, // Enable for development, disable in production
    );
  });
}
