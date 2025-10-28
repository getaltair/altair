# Altair Database Service - Schema Design & Development

**TL;DR:** Complete schema design for cross-app linking, testing strategy, backup/restore, and development workflow.

---

## 🗄️ Database Schema Design

### Core Principle: Everything is Linkable

Every resource (task, note, item) can link to any other resource. Use SurrealDB's native graph capabilities.

### Base Schema

```sql
-- ====================
-- CORE RESOURCE TYPES
-- ====================

-- Tasks (from Guidance)
DEFINE TABLE task SCHEMALESS
  PERMISSIONS FOR
    select, create, update, delete WHERE true; -- Local-only, no auth needed

DEFINE FIELD id ON task TYPE record<task>;
DEFINE FIELD title ON task TYPE string;
DEFINE FIELD description ON task TYPE option<string>;
DEFINE FIELD status ON task TYPE string; -- 'todo', 'in_progress', 'done'
DEFINE FIELD priority ON task TYPE option<string>; -- 'low', 'medium', 'high'
DEFINE FIELD due_date ON task TYPE option<datetime>;
DEFINE FIELD project_id ON task TYPE option<record<project>>;
DEFINE FIELD parent_task_id ON task TYPE option<record<task>>;
DEFINE FIELD tags ON task TYPE array<string>;
DEFINE FIELD created_at ON task TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON task TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON task TYPE option<datetime>; -- Soft delete

-- Projects (from Guidance)
DEFINE TABLE project SCHEMALESS;
DEFINE FIELD id ON project TYPE record<project>;
DEFINE FIELD name ON project TYPE string;
DEFINE FIELD description ON project TYPE option<string>;
DEFINE FIELD status ON project TYPE string;
DEFINE FIELD color ON project TYPE option<string>;
DEFINE FIELD created_at ON project TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON project TYPE datetime DEFAULT time::now();

-- Notes/Pages (from Knowledge)
DEFINE TABLE note SCHEMALESS;
DEFINE FIELD id ON note TYPE record<note>;
DEFINE FIELD title ON note TYPE string;
DEFINE FIELD content ON note TYPE string; -- Markdown
DEFINE FIELD tags ON note TYPE array<string>;
DEFINE FIELD folder ON note TYPE option<string>;
DEFINE FIELD is_daily ON note TYPE bool DEFAULT false;
DEFINE FIELD date ON note TYPE option<datetime>; -- For daily notes
DEFINE FIELD created_at ON note TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON note TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON note TYPE option<datetime>;

-- Items (from Tracking)
DEFINE TABLE item SCHEMALESS;
DEFINE FIELD id ON item TYPE record<item>;
DEFINE FIELD name ON item TYPE string;
DEFINE FIELD description ON item TYPE option<string>;
DEFINE FIELD quantity ON item TYPE int DEFAULT 1;
DEFINE FIELD location ON item TYPE option<record<location>>;
DEFINE FIELD barcode ON item TYPE option<string>;
DEFINE FIELD photo_url ON item TYPE option<string>;
DEFINE FIELD tags ON item TYPE array<string>;
DEFINE FIELD created_at ON item TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON item TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON item TYPE option<datetime>;

-- Locations (from Tracking)
DEFINE TABLE location SCHEMALESS;
DEFINE FIELD id ON location TYPE record<location>;
DEFINE FIELD name ON location TYPE string;
DEFINE FIELD parent_location_id ON location TYPE option<record<location>>;
DEFINE FIELD created_at ON location TYPE datetime DEFAULT time::now();

-- ====================
-- CROSS-APP LINKING
-- ====================

-- Generic link table for any resource linking
DEFINE TABLE link SCHEMALESS;
DEFINE FIELD id ON link TYPE record<link>;
DEFINE FIELD source ON link TYPE record; -- Can be task:*, note:*, item:*
DEFINE FIELD target ON link TYPE record; -- Can be task:*, note:*, item:*
DEFINE FIELD link_type ON link TYPE string; -- 'references', 'blocks', 'relates_to', 'requires'
DEFINE FIELD metadata ON link TYPE option<object>; -- Arbitrary metadata
DEFINE FIELD created_at ON link TYPE datetime DEFAULT time::now();
DEFINE FIELD created_by ON link TYPE option<string>; -- 'user' or 'ai'

-- Indexes for fast lookup
DEFINE INDEX link_source_idx ON link FIELDS source;
DEFINE INDEX link_target_idx ON link FIELDS target;
DEFINE INDEX link_type_idx ON link FIELDS link_type;

-- Prevent duplicate links
DEFINE INDEX link_unique ON link FIELDS source, target, link_type UNIQUE;

-- ====================
-- FULL-TEXT SEARCH
-- ====================

-- Full-text search analyzer
DEFINE ANALYZER altair_text TOKENIZERS blank, class FILTERS lowercase, ascii;

-- Search indexes
DEFINE INDEX task_search ON task FIELDS title, description SEARCH ANALYZER altair_text BM25;
DEFINE INDEX note_search ON note FIELDS title, content SEARCH ANALYZER altair_text BM25;
DEFINE INDEX item_search ON item FIELDS name, description SEARCH ANALYZER altair_text BM25;

-- ====================
-- AI / EMBEDDINGS (Future)
-- ====================

-- Store embeddings for semantic search
DEFINE TABLE embedding SCHEMALESS;
DEFINE FIELD id ON embedding TYPE record<embedding>;
DEFINE FIELD resource_id ON embedding TYPE record; -- task:*, note:*, item:*
DEFINE FIELD vector ON embedding TYPE array<float>; -- 768-dim for OpenAI ada-002
DEFINE FIELD model ON embedding TYPE string; -- 'openai-ada-002', 'ollama-nomic-embed'
DEFINE FIELD created_at ON embedding TYPE datetime DEFAULT time::now();

DEFINE INDEX embedding_resource_idx ON embedding FIELDS resource_id;
```

---

## 🔗 Common Link Types

```dart
// lib/src/models/link_types.dart

class LinkType {
  /// Note references task (e.g., meeting notes mention task)
  static const String references = 'references';

  /// Task blocks another task (dependency)
  static const String blocks = 'blocks';

  /// Task is blocked by another task
  static const String blockedBy = 'blocked_by';

  /// Generic relationship
  static const String relatesTo = 'relates_to';

  /// Task requires item (e.g., "Install software" requires laptop)
  static const String requires = 'requires';

  /// Note is about project/task (stronger than reference)
  static const String about = 'about';

  /// Auto-generated by AI
  static const String suggested = 'suggested';

  /// Task mentioned in note (backlink)
  static const String mentionedIn = 'mentioned_in';
}
```

---

## 🔍 Common Queries Library

```dart
// packages/altair_db/lib/src/queries.dart

class AltairQueries {
  final Surreal db;

  AltairQueries(this.db);

  /// Get all resources linked TO a given resource (backlinks)
  Future<List<Map<String, dynamic>>> getBacklinks(String resourceId) async {
    final result = await db.query('''
      SELECT
        source,
        link_type,
        created_at,
        (SELECT * FROM $source)[0] AS resource
      FROM link
      WHERE target = $resourceId
      ORDER BY created_at DESC;
    ''', vars: {'resourceId': resourceId});

    return List<Map<String, dynamic>>.from(result[0]);
  }

  /// Get all resources linked FROM a given resource (forward links)
  Future<List<Map<String, dynamic>>> getForwardLinks(String resourceId) async {
    final result = await db.query('''
      SELECT
        target,
        link_type,
        created_at,
        (SELECT * FROM $target)[0] AS resource
      FROM link
      WHERE source = $resourceId
      ORDER BY created_at DESC;
    ''', vars: {'resourceId': resourceId});

    return List<Map<String, dynamic>>.from(result[0]);
  }

  /// Get ALL links (both directions) for a resource
  Future<List<Map<String, dynamic>>> getAllLinks(String resourceId) async {
    final result = await db.query('''
      -- Get links where resource is source
      SELECT
        'outgoing' AS direction,
        target AS linked_resource_id,
        link_type,
        created_at,
        (SELECT * FROM $target)[0] AS resource
      FROM link
      WHERE source = $resourceId

      UNION

      -- Get links where resource is target
      SELECT
        'incoming' AS direction,
        source AS linked_resource_id,
        link_type,
        created_at,
        (SELECT * FROM $source)[0] AS resource
      FROM link
      WHERE target = $resourceId

      ORDER BY created_at DESC;
    ''', vars: {'resourceId': resourceId});

    return List<Map<String, dynamic>>.from(result[0]);
  }

  /// Search across all apps
  Future<List<Map<String, dynamic>>> searchAll(String query) async {
    final result = await db.query('''
      SELECT
        id,
        title,
        description,
        'task' AS resource_type,
        created_at
      FROM task
      WHERE title @@ $query OR description @@ $query

      UNION

      SELECT
        id,
        title,
        content AS description,
        'note' AS resource_type,
        created_at
      FROM note
      WHERE title @@ $query OR content @@ $query

      UNION

      SELECT
        id,
        name AS title,
        description,
        'item' AS resource_type,
        created_at
      FROM item
      WHERE name @@ $query OR description @@ $query

      ORDER BY created_at DESC
      LIMIT 50;
    ''', vars: {'query': query});

    return List<Map<String, dynamic>>.from(result[0]);
  }

  /// Get tasks blocking a given task
  Future<List<Map<String, dynamic>>> getBlockingTasks(String taskId) async {
    final result = await db.query('''
      SELECT
        (SELECT * FROM $source)[0] AS blocking_task
      FROM link
      WHERE target = $taskId AND link_type = 'blocks';
    ''', vars: {'taskId': taskId});

    return List<Map<String, dynamic>>.from(result[0]);
  }

  /// Get all tasks in a project, including linked notes and items
  Future<Map<String, dynamic>> getProjectWithLinks(String projectId) async {
    final result = await db.query('''
      LET $project = (SELECT * FROM $projectId)[0];
      LET $tasks = SELECT * FROM task WHERE project_id = $projectId;

      LET $linked_notes = SELECT DISTINCT
        (SELECT * FROM $target)[0] AS note
      FROM link
      WHERE source IN (SELECT VALUE id FROM $tasks)
        AND link_type = 'references'
        AND string::startsWith($target, 'note:');

      LET $linked_items = SELECT DISTINCT
        (SELECT * FROM $target)[0] AS item
      FROM link
      WHERE source IN (SELECT VALUE id FROM $tasks)
        AND link_type = 'requires'
        AND string::startsWith($target, 'item:');

      RETURN {
        project: $project,
        tasks: $tasks,
        linked_notes: $linked_notes,
        linked_items: $linked_items
      };
    ''', vars: {'projectId': projectId});

    return result[0][0];
  }

  /// Get graph data for visualization
  Future<Map<String, dynamic>> getGraphData({
    String? resourceId,
    int depth = 2,
  }) async {
    final result = await db.query('''
      -- Get the starting resource(s)
      LET $start = SELECT * FROM ${resourceId != null ? '\$resourceId' : 'task, note, item'};

      -- Get all links up to depth
      LET $links = SELECT * FROM link WHERE
        source IN (SELECT VALUE id FROM $start) OR
        target IN (SELECT VALUE id FROM $start);

      -- Get all connected resources
      LET $nodes = (
        SELECT VALUE id FROM $start
        UNION
        SELECT VALUE source FROM $links
        UNION
        SELECT VALUE target FROM $links
      );

      LET $node_details = SELECT
        id,
        title,
        type::table(id) AS type
      FROM $nodes;

      RETURN {
        nodes: $node_details,
        links: $links
      };
    ''', vars: resourceId != null ? {'resourceId': resourceId} : {});

    return result[0][0];
  }
}
```

---

## 🧪 Testing Strategy

### Unit Tests

```dart
// packages/altair_db_service/test/service_manager_test.dart

import 'package:test/test.dart';
import 'package:altair_db_service/altair_db_service.dart';

void main() {
  group('AltairDatabaseService', () {
    late AltairDatabaseService service;

    setUp(() {
      service = AltairDatabaseService();
    });

    test('can check service status', () async {
      final status = await service.getStatus();
      expect(status, isNotNull);
      expect(status.port, equals(8000));
    });

    test('can start and stop service', () async {
      await service.start();

      var status = await service.getStatus();
      expect(status.isRunning, isTrue);

      await service.stop();

      status = await service.getStatus();
      expect(status.isRunning, isFalse);
    });

    test('handles already running service gracefully', () async {
      await service.start();

      // Starting again should not throw
      await service.start();

      final status = await service.getStatus();
      expect(status.isRunning, isTrue);

      await service.stop();
    });
  });

  group('AltairConnectionManager', () {
    test('can connect to service', () async {
      final db = await AltairConnectionManager.getInstance();
      expect(db, isNotNull);

      // Should be able to query
      final result = await db.client.query('SELECT * FROM task LIMIT 1');
      expect(result, isNotNull);
    });

    test('can create and query tasks', () async {
      final db = await AltairConnectionManager.getInstance();

      // Create a task
      await db.client.create('task', {
        'title': 'Test task',
        'description': 'Testing cross-app integration',
        'status': 'todo',
      });

      // Query it back
      final result = await db.client.query(
        'SELECT * FROM task WHERE title = \$title',
        vars: {'title': 'Test task'},
      );

      expect(result[0], isNotEmpty);
      expect(result[0][0]['title'], equals('Test task'));
    });

    test('can create links between resources', () async {
      final db = await AltairConnectionManager.getInstance();

      // Create a task
      final task = await db.client.create('task', {
        'title': 'Review pull request',
        'status': 'todo',
      });

      // Create a note
      final note = await db.client.create('note', {
        'title': 'PR Review Notes',
        'content': 'Code looks good overall...',
      });

      // Link them
      await db.createLink(
        sourceId: note.first['id'],
        targetId: task.first['id'],
        linkType: 'references',
      );

      // Verify link exists
      final links = await db.getLinkedResources(task.first['id']);
      expect(links, isNotEmpty);
      expect(links[0]['linked_resource'], equals(note.first['id']));
    });
  });

  group('Cross-app queries', () {
    test('can search across all resource types', () async {
      final db = await AltairConnectionManager.getInstance();

      // Create resources
      await db.client.create('task', {'title': 'Test integration'});
      await db.client.create('note', {'title': 'Integration notes'});
      await db.client.create('item', {'name': 'Integration server'});

      // Search
      final results = await db.searchAll('integration');

      expect(results.length, greaterThanOrEqualTo(3));
      expect(results.any((r) => r['resource_type'] == 'task'), isTrue);
      expect(results.any((r) => r['resource_type'] == 'note'), isTrue);
      expect(results.any((r) => r['resource_type'] == 'item'), isTrue);
    });
  });
}
```

### Integration Tests

```dart
// packages/altair_db_service/test/integration/cross_app_test.dart

import 'package:test/test.dart';
import 'package:altair_db_service/altair_db_service.dart';

void main() {
  group('Cross-app integration', () {
    late AltairConnectionManager db;

    setUpAll(() async {
      // Start service
      final service = AltairDatabaseService();
      await service.start();

      // Connect
      db = await AltairConnectionManager.getInstance();
    });

    test('Guidance → Knowledge: Task mentioned in note', () async {
      // In Guidance: Create a task
      final task = await db.client.create('task', {
        'title': 'Implement feature X',
        'description': 'Add new authentication flow',
        'status': 'in_progress',
      });

      final taskId = task.first['id'];

      // In Knowledge: Create note that references the task
      final note = await db.client.create('note', {
        'title': 'Daily Note - Sprint Planning',
        'content': 'Discussed feature X implementation. See task:$taskId',
      });

      final noteId = note.first['id'];

      // Create link
      await db.createLink(
        sourceId: noteId,
        targetId: taskId,
        linkType: LinkType.references,
      );

      // From Guidance: Find all notes referencing this task
      final queries = AltairQueries(db.client);
      final backlinks = await queries.getBacklinks(taskId);

      expect(backlinks, isNotEmpty);
      expect(backlinks[0]['resource']['id'], equals(noteId));
      expect(backlinks[0]['link_type'], equals(LinkType.references));
    });

    test('Tracking → Guidance: Task requires item', () async {
      // In Tracking: Create an item
      final item = await db.client.create('item', {
        'name': 'Development Laptop',
        'location': 'Office',
        'quantity': 1,
      });

      final itemId = item.first['id'];

      // In Guidance: Create task that requires this item
      final task = await db.client.create('task', {
        'title': 'Setup development environment',
        'status': 'blocked',
      });

      final taskId = task.first['id'];

      // Link: Task requires item
      await db.createLink(
        sourceId: taskId,
        targetId: itemId,
        linkType: LinkType.requires,
      );

      // From Tracking: Find all tasks that need this item
      final queries = AltairQueries(db.client);
      final backlinks = await queries.getBacklinks(itemId);

      expect(backlinks, isNotEmpty);
      expect(backlinks[0]['link_type'], equals(LinkType.requires));
    });

    test('Complete ecosystem: Project → Tasks → Notes → Items', () async {
      // Create a project
      final project = await db.client.create('project', {
        'name': 'Website Redesign',
        'status': 'active',
      });
      final projectId = project.first['id'];

      // Create tasks in the project
      final task1 = await db.client.create('task', {
        'title': 'Design mockups',
        'project_id': projectId,
        'status': 'done',
      });

      final task2 = await db.client.create('task', {
        'title': 'Implement frontend',
        'project_id': projectId,
        'status': 'in_progress',
      });

      // Create note about the project
      final note = await db.client.create('note', {
        'title': 'Website Redesign - Design Decisions',
        'content': 'Decided on blue color scheme...',
      });

      // Create item needed for the project
      final item = await db.client.create('item', {
        'name': 'Figma License',
        'description': 'Design tool subscription',
      });

      // Create links
      await db.createLink(
        sourceId: note.first['id'],
        targetId: projectId,
        linkType: LinkType.about,
      );

      await db.createLink(
        sourceId: task1.first['id'],
        targetId: item.first['id'],
        linkType: LinkType.requires,
      );

      // Query the complete project ecosystem
      final queries = AltairQueries(db.client);
      final projectData = await queries.getProjectWithLinks(projectId);

      expect(projectData['project']['name'], equals('Website Redesign'));
      expect(projectData['tasks'].length, equals(2));
      expect(projectData['linked_notes'], isNotEmpty);
      expect(projectData['linked_items'], isNotEmpty);
    });
  });
}
```

---

## 💾 Backup & Restore

```dart
// packages/altair_db_service/lib/src/backup.dart

import 'dart:io';
import 'package:path/path.dart' as path;
import 'package:archive/archive_io.dart';

class AltairBackup {
  final ServiceConfig config;

  AltairBackup(this.config);

  /// Create a backup of the database
  Future<String> createBackup({String? backupPath}) async {
    final timestamp = DateTime.now().toIso8601String().replaceAll(':', '-');
    backupPath ??= path.join(
      await config.getServiceDirectory(),
      'backups',
      'altair-backup-$timestamp.tar.gz',
    );

    final backupDir = Directory(path.dirname(backupPath));
    await backupDir.create(recursive: true);

    // Create tar.gz of data directory
    final encoder = TarFileEncoder();
    encoder.tarDirectory(Directory(config.dataPath), filename: backupPath);

    print('✓ Backup created: $backupPath');
    return backupPath;
  }

  /// Restore from backup
  Future<void> restoreBackup(String backupPath) async {
    if (!await File(backupPath).exists()) {
      throw Exception('Backup file not found: $backupPath');
    }

    // Stop service before restore
    final service = AltairDatabaseService(config: config);
    await service.stop();

    // Clear existing data
    final dataDir = Directory(config.dataPath);
    if (await dataDir.exists()) {
      await dataDir.delete(recursive: true);
    }
    await dataDir.create(recursive: true);

    // Extract backup
    final decoder = TarDecoder();
    final archive = decoder.decodeBytes(
      GZipDecoder().decodeBytes(await File(backupPath).readAsBytes())
    );

    for (final file in archive) {
      final filename = path.join(config.dataPath, file.name);
      if (file.isFile) {
        final outFile = File(filename);
        await outFile.create(recursive: true);
        await outFile.writeAsBytes(file.content as List<int>);
      }
    }

    // Restart service
    await service.start();

    print('✓ Backup restored from: $backupPath');
  }

  /// List available backups
  Future<List<FileSystemEntity>> listBackups() async {
    final backupDir = Directory(path.join(
      await config.getServiceDirectory(),
      'backups',
    ));

    if (!await backupDir.exists()) {
      return [];
    }

    return backupDir
        .listSync()
        .where((f) => f.path.endsWith('.tar.gz'))
        .toList()
      ..sort((a, b) => b.statSync().modified.compareTo(a.statSync().modified));
  }

  /// Auto-backup on schedule
  Future<void> enableAutoBackup({
    Duration interval = const Duration(days: 1),
    int maxBackups = 7,
  }) async {
    Timer.periodic(interval, (timer) async {
      await createBackup();

      // Clean up old backups
      final backups = await listBackups();
      if (backups.length > maxBackups) {
        for (var i = maxBackups; i < backups.length; i++) {
          await backups[i].delete();
        }
      }
    });
  }
}
```

---

## 🔧 Development Workflow

### Local Development Setup

```bash
# 1. Download SurrealDB binary
cd packages/altair_db_service
dart run bin/download_surrealdb.dart

# 2. Start the service manually (for development)
~/.altair/bin/surrealdb start \
  --bind 127.0.0.1:8000 \
  --user altair \
  --pass dev-password \
  file://~/.altair/database/altair.db

# 3. Test connection
curl http://localhost:8000/health

# 4. Run tests
dart test
```

### Seed Data for Development

```dart
// scripts/seed_database.dart

import 'package:altair_db_service/altair_db_service.dart';

Future<void> main() async {
  final db = await AltairConnectionManager.getInstance();

  print('Seeding database...');

  // Create sample project
  final project = await db.client.create('project', {
    'name': 'Altair Development',
    'description': 'Building the Altair ecosystem',
    'status': 'active',
    'color': '#4F46E5',
  });

  final projectId = project.first['id'];

  // Create sample tasks
  final tasks = [
    {
      'title': 'Setup database service',
      'description': 'Implement shared SurrealDB service',
      'status': 'done',
      'project_id': projectId,
    },
    {
      'title': 'Implement cross-app linking',
      'description': 'Enable resources to link across apps',
      'status': 'in_progress',
      'project_id': projectId,
    },
    {
      'title': 'Build Knowledge app',
      'description': 'Personal wiki with [[wiki-links]]',
      'status': 'todo',
      'project_id': projectId,
    },
  ];

  final createdTasks = [];
  for (var taskData in tasks) {
    final task = await db.client.create('task', taskData);
    createdTasks.add(task.first);
    print('✓ Created task: ${taskData['title']}');
  }

  // Create sample notes
  final note = await db.client.create('note', {
    'title': 'Altair Architecture Decision',
    'content': '''
# Architecture Decision: Shared Database Service

## Context
We need all three Altair apps to share data seamlessly.

## Decision
Use a shared SurrealDB service running locally on user's device.

## Consequences
- True cross-app integration
- Single source of truth
- Easier backups
    ''',
    'tags': ['architecture', 'decision'],
  });

  print('✓ Created note: ${note.first['title']}');

  // Create sample items
  final item = await db.client.create('item', {
    'name': 'Development Laptop',
    'description': 'Main development machine',
    'location': 'Home Office',
    'quantity': 1,
  });

  print('✓ Created item: ${item.first['name']}');

  // Create links
  await db.createLink(
    sourceId: note.first['id'],
    targetId: projectId,
    linkType: 'about',
  );

  await db.createLink(
    sourceId: createdTasks[1]['id'],
    targetId: item.first['id'],
    linkType: 'requires',
  );

  print('\n✓ Database seeded successfully!');
  print('  - 1 project');
  print('  - ${tasks.length} tasks');
  print('  - 1 note');
  print('  - 1 item');
  print('  - 2 links');
}
```

---

## 📱 Mobile Considerations

### Android Service Implementation

```dart
// packages/altair_db_service/lib/src/platform/android_service.dart

// For Android, use a foreground service to keep SurrealDB running

import 'package:flutter/services.dart';

class AndroidDatabaseService {
  static const platform = MethodChannel('com.getaltair/database_service');

  Future<void> start() async {
    try {
      await platform.invokeMethod('startDatabaseService');
    } catch (e) {
      throw Exception('Failed to start Android service: $e');
    }
  }

  Future<void> stop() async {
    try {
      await platform.invokeMethod('stopDatabaseService');
    } catch (e) {
      throw Exception('Failed to stop Android service: $e');
    }
  }

  Future<bool> isRunning() async {
    try {
      return await platform.invokeMethod('isServiceRunning');
    } catch (e) {
      return false;
    }
  }
}
```

Corresponding Android native code needed in `android/app/src/main/kotlin/`:

```kotlin
// DatabaseService.kt
class DatabaseService : Service() {
    private var surrealProcess: Process? = null

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        startForeground(NOTIFICATION_ID, createNotification())
        startSurrealDB()
        return START_STICKY
    }

    private fun startSurrealDB() {
        // Extract binary from assets, start process
        // Implementation details...
    }
}
```

### iOS Limitations & Fallback

For iOS, background processes are heavily restricted. Best approach:

```dart
// For iOS: Fall back to SQLite, sync to desktop/cloud
if (Platform.isIOS) {
  // Use PowerSync or custom sync
  final localDb = await openSqliteDatabase();
  // Setup sync with cloud SurrealDB when online
} else {
  // Use shared SurrealDB service
  final db = await AltairConnectionManager.getInstance();
}
```

---

This gives you everything needed to build the shared database service. Ready to start implementing? Want me to create example code for one of the apps (Guidance/Knowledge) showing how to use this?
