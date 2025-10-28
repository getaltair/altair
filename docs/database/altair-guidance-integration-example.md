# Altair Database Service - Practical Integration Example

**TL;DR:** Complete example of integrating the shared database service into Guidance app, showing real-world usage patterns.

---

## 📋 Guidance App Integration

### Project Structure

```
apps/guidance/
├── lib/
│   ├── main.dart
│   ├── repositories/
│   │   ├── task_repository.dart          # Uses shared DB
│   │   └── project_repository.dart       # Uses shared DB
│   ├── services/
│   │   ├── database_service.dart         # Wrapper around shared DB
│   │   └── link_service.dart             # Cross-app linking
│   ├── models/
│   │   ├── task.dart
│   │   └── project.dart
│   └── screens/
│       ├── task_detail_screen.dart       # Shows linked resources
│       └── linked_resources_widget.dart  # Reusable widget
└── pubspec.yaml
```

### pubspec.yaml

```yaml
name: guidance
description: Task management with ADHD-friendly features
version: 1.0.0+1

environment:
  sdk: '>=3.0.0 <4.0.0'

dependencies:
  flutter:
    sdk: flutter

  # Shared packages
  altair_db_service:
    path: ../../packages/altair_db_service
  altair_db:
    path: ../../packages/altair_db
  altair_auth:
    path: ../../packages/altair_auth
  altair_ui:
    path: ../../packages/altair_ui

  # Other dependencies
  uuid: ^4.0.0
  intl: ^0.18.0
  provider: ^6.1.0
```

---

## 🔧 Database Service Wrapper

```dart
// lib/services/database_service.dart

import 'package:altair_db_service/altair_db_service.dart';
import 'package:surrealdb/surrealdb.dart';

/// Guidance-specific wrapper around the shared database
class GuidanceDatabaseService {
  static GuidanceDatabaseService? _instance;
  late AltairConnectionManager _connection;
  late Surreal _db;

  GuidanceDatabaseService._();

  static Future<GuidanceDatabaseService> getInstance() async {
    if (_instance == null) {
      _instance = GuidanceDatabaseService._();
      await _instance!._initialize();
    }
    return _instance!;
  }

  Future<void> _initialize() async {
    _connection = await AltairConnectionManager.getInstance();
    _db = _connection.client;
  }

  /// Get the raw database client for advanced queries
  Surreal get client => _db;

  /// Get connection manager for cross-app queries
  AltairConnectionManager get connection => _connection;

  /// Ensure database is ready
  Future<void> ensureReady() async {
    try {
      await _db.query('INFO FOR DB');
    } catch (e) {
      throw Exception('Database not ready: $e');
    }
  }
}
```

---

## 📦 Task Repository

```dart
// lib/repositories/task_repository.dart

import 'package:uuid/uuid.dart';
import '../models/task.dart';
import '../services/database_service.dart';

class TaskRepository {
  final GuidanceDatabaseService _dbService;

  TaskRepository(this._dbService);

  /// Create a new task
  Future<Task> create(Task task) async {
    final db = _dbService.client;

    final result = await db.create('task', {
      'id': task.id,
      'title': task.title,
      'description': task.description,
      'status': task.status,
      'priority': task.priority,
      'due_date': task.dueDate?.toIso8601String(),
      'project_id': task.projectId,
      'parent_task_id': task.parentTaskId,
      'tags': task.tags,
      'created_at': task.createdAt.toIso8601String(),
      'updated_at': task.updatedAt.toIso8601String(),
    });

    return Task.fromJson(result.first);
  }

  /// Get task by ID
  Future<Task?> getById(String id) async {
    final db = _dbService.client;

    final result = await db.select(id);
    if (result.isEmpty) return null;

    return Task.fromJson(result.first);
  }

  /// Get all tasks
  Future<List<Task>> getAll({
    String? status,
    String? projectId,
    bool includeDeleted = false,
  }) async {
    final db = _dbService.client;

    String query = 'SELECT * FROM task WHERE 1=1';
    Map<String, dynamic> vars = {};

    if (!includeDeleted) {
      query += ' AND deleted_at IS NULL';
    }

    if (status != null) {
      query += ' AND status = \$status';
      vars['status'] = status;
    }

    if (projectId != null) {
      query += ' AND project_id = \$projectId';
      vars['projectId'] = projectId;
    }

    query += ' ORDER BY created_at DESC';

    final result = await db.query(query, vars: vars);
    return (result[0] as List)
        .map((json) => Task.fromJson(json))
        .toList();
  }

  /// Update task
  Future<Task> update(Task task) async {
    final db = _dbService.client;

    final result = await db.merge(task.id, {
      'title': task.title,
      'description': task.description,
      'status': task.status,
      'priority': task.priority,
      'due_date': task.dueDate?.toIso8601String(),
      'project_id': task.projectId,
      'parent_task_id': task.parentTaskId,
      'tags': task.tags,
      'updated_at': DateTime.now().toIso8601String(),
    });

    return Task.fromJson(result.first);
  }

  /// Soft delete task
  Future<void> delete(String id) async {
    final db = _dbService.client;

    await db.merge(id, {
      'deleted_at': DateTime.now().toIso8601String(),
    });
  }

  /// Get subtasks of a task
  Future<List<Task>> getSubtasks(String parentId) async {
    final db = _dbService.client;

    final result = await db.query('''
      SELECT * FROM task
      WHERE parent_task_id = \$parentId
        AND deleted_at IS NULL
      ORDER BY created_at ASC
    ''', vars: {'parentId': parentId});

    return (result[0] as List)
        .map((json) => Task.fromJson(json))
        .toList();
  }

  /// Search tasks
  Future<List<Task>> search(String query) async {
    final db = _dbService.client;

    final result = await db.query('''
      SELECT * FROM task
      WHERE (title @@ \$query OR description @@ \$query)
        AND deleted_at IS NULL
      ORDER BY created_at DESC
      LIMIT 50
    ''', vars: {'query': query});

    return (result[0] as List)
        .map((json) => Task.fromJson(json))
        .toList();
  }
}
```

---

## 🔗 Link Service (Cross-App Integration)

```dart
// lib/services/link_service.dart

import 'package:altair_db/altair_db.dart';
import 'database_service.dart';

class LinkService {
  final GuidanceDatabaseService _dbService;

  LinkService(this._dbService);

  /// Create a link from task to another resource
  Future<void> linkTask({
    required String taskId,
    required String targetId,
    required String linkType,
  }) async {
    await _dbService.connection.createLink(
      sourceId: taskId,
      targetId: targetId,
      linkType: linkType,
    );
  }

  /// Get all resources linked to a task
  Future<List<LinkedResource>> getTaskLinks(String taskId) async {
    final queries = AltairQueries(_dbService.client);
    final links = await queries.getAllLinks(taskId);

    return links.map((link) {
      final resource = link['resource'];
      final resourceType = _getResourceType(link['linked_resource_id']);

      return LinkedResource(
        id: link['linked_resource_id'],
        type: resourceType,
        title: resource['title'] ?? resource['name'] ?? 'Untitled',
        linkType: link['link_type'],
        direction: link['direction'],
        createdAt: DateTime.parse(link['created_at']),
      );
    }).toList();
  }

  /// Get notes that reference this task
  Future<List<LinkedResource>> getReferencingNotes(String taskId) async {
    final queries = AltairQueries(_dbService.client);
    final backlinks = await queries.getBacklinks(taskId);

    return backlinks
        .where((link) => link['resource']['id'].toString().startsWith('note:'))
        .map((link) => LinkedResource(
          id: link['source'],
          type: ResourceType.note,
          title: link['resource']['title'],
          linkType: link['link_type'],
          direction: 'incoming',
          createdAt: DateTime.parse(link['created_at']),
        ))
        .toList();
  }

  /// Get items required by this task
  Future<List<LinkedResource>> getRequiredItems(String taskId) async {
    final queries = AltairQueries(_dbService.client);
    final links = await queries.getForwardLinks(taskId);

    return links
        .where((link) =>
            link['resource']['id'].toString().startsWith('item:') &&
            link['link_type'] == LinkType.requires)
        .map((link) => LinkedResource(
          id: link['target'],
          type: ResourceType.item,
          title: link['resource']['name'],
          linkType: link['link_type'],
          direction: 'outgoing',
          createdAt: DateTime.parse(link['created_at']),
        ))
        .toList();
  }

  /// Remove a link
  Future<void> removeLink({
    required String sourceId,
    required String targetId,
    required String linkType,
  }) async {
    final db = _dbService.client;

    await db.query('''
      DELETE FROM link
      WHERE source = \$source
        AND target = \$target
        AND link_type = \$linkType
    ''', vars: {
      'source': sourceId,
      'target': targetId,
      'linkType': linkType,
    });
  }

  ResourceType _getResourceType(String id) {
    if (id.startsWith('task:')) return ResourceType.task;
    if (id.startsWith('note:')) return ResourceType.note;
    if (id.startsWith('item:')) return ResourceType.item;
    if (id.startsWith('project:')) return ResourceType.project;
    return ResourceType.unknown;
  }
}

enum ResourceType {
  task,
  note,
  item,
  project,
  unknown,
}

class LinkedResource {
  final String id;
  final ResourceType type;
  final String title;
  final String linkType;
  final String direction; // 'incoming' or 'outgoing'
  final DateTime createdAt;

  LinkedResource({
    required this.id,
    required this.type,
    required this.title,
    required this.linkType,
    required this.direction,
    required this.createdAt,
  });

  String get icon {
    switch (type) {
      case ResourceType.task:
        return '✓';
      case ResourceType.note:
        return '📝';
      case ResourceType.item:
        return '📦';
      case ResourceType.project:
        return '📁';
      default:
        return '•';
    }
  }
}
```

---

## 🎨 UI: Task Detail with Linked Resources

```dart
// lib/screens/task_detail_screen.dart

import 'package:flutter/material.dart';
import '../models/task.dart';
import '../services/link_service.dart';
import '../widgets/linked_resources_widget.dart';

class TaskDetailScreen extends StatefulWidget {
  final Task task;

  const TaskDetailScreen({Key? key, required this.task}) : super(key: key);

  @override
  State<TaskDetailScreen> createState() => _TaskDetailScreenState();
}

class _TaskDetailScreenState extends State<TaskDetailScreen> {
  late LinkService _linkService;
  List<LinkedResource>? _linkedResources;
  bool _loadingLinks = false;

  @override
  void initState() {
    super.initState();
    _initializeServices();
  }

  Future<void> _initializeServices() async {
    final dbService = await GuidanceDatabaseService.getInstance();
    _linkService = LinkService(dbService);
    await _loadLinks();
  }

  Future<void> _loadLinks() async {
    setState(() => _loadingLinks = true);

    try {
      final links = await _linkService.getTaskLinks(widget.task.id);
      setState(() {
        _linkedResources = links;
        _loadingLinks = false;
      });
    } catch (e) {
      setState(() => _loadingLinks = false);
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Failed to load linked resources: $e')),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.task.title),
        actions: [
          IconButton(
            icon: Icon(Icons.link),
            onPressed: _showLinkDialog,
            tooltip: 'Add link to note or item',
          ),
        ],
      ),
      body: SingleChildScrollView(
        padding: EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Task details
            _buildTaskDetails(),

            SizedBox(height: 24),

            // Linked resources section
            Text(
              'Linked Resources',
              style: Theme.of(context).textTheme.titleLarge,
            ),
            SizedBox(height: 8),

            if (_loadingLinks)
              Center(child: CircularProgressIndicator())
            else if (_linkedResources == null || _linkedResources!.isEmpty)
              _buildEmptyState()
            else
              LinkedResourcesWidget(
                resources: _linkedResources!,
                onTap: _openLinkedResource,
                onRemove: _removeLink,
              ),
          ],
        ),
      ),
    );
  }

  Widget _buildTaskDetails() {
    return Card(
      child: Padding(
        padding: EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              widget.task.title,
              style: Theme.of(context).textTheme.headlineSmall,
            ),
            if (widget.task.description != null) ...[
              SizedBox(height: 8),
              Text(widget.task.description!),
            ],
            SizedBox(height: 16),
            Row(
              children: [
                Chip(
                  label: Text(widget.task.status),
                  backgroundColor: _getStatusColor(widget.task.status),
                ),
                if (widget.task.priority != null) ...[
                  SizedBox(width: 8),
                  Chip(
                    label: Text(widget.task.priority!),
                    backgroundColor: _getPriorityColor(widget.task.priority!),
                  ),
                ],
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildEmptyState() {
    return Card(
      child: Padding(
        padding: EdgeInsets.all(32),
        child: Column(
          children: [
            Icon(Icons.link_off, size: 48, color: Colors.grey),
            SizedBox(height: 16),
            Text(
              'No linked resources yet',
              style: Theme.of(context).textTheme.titleMedium,
            ),
            SizedBox(height: 8),
            Text(
              'Link this task to notes in Knowledge or items in Tracking',
              style: Theme.of(context).textTheme.bodySmall,
              textAlign: TextAlign.center,
            ),
            SizedBox(height: 16),
            ElevatedButton.icon(
              onPressed: _showLinkDialog,
              icon: Icon(Icons.add_link),
              label: Text('Add Link'),
            ),
          ],
        ),
      ),
    );
  }

  void _showLinkDialog() {
    // Show dialog to search and link to notes/items
    showDialog(
      context: context,
      builder: (context) => LinkResourceDialog(
        taskId: widget.task.id,
        onLinked: _loadLinks,
      ),
    );
  }

  void _openLinkedResource(LinkedResource resource) {
    // Navigate to appropriate app or show details
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('Opening ${resource.type.name}: ${resource.title}'),
        action: SnackBarAction(
          label: 'Open in ${resource.type.name}',
          onPressed: () {
            // TODO: Deep link to other app or show inline
          },
        ),
      ),
    );
  }

  Future<void> _removeLink(LinkedResource resource) async {
    await _linkService.removeLink(
      sourceId: widget.task.id,
      targetId: resource.id,
      linkType: resource.linkType,
    );
    await _loadLinks();
  }

  Color _getStatusColor(String status) {
    switch (status) {
      case 'done':
        return Colors.green.shade100;
      case 'in_progress':
        return Colors.blue.shade100;
      default:
        return Colors.grey.shade100;
    }
  }

  Color _getPriorityColor(String priority) {
    switch (priority) {
      case 'high':
        return Colors.red.shade100;
      case 'medium':
        return Colors.orange.shade100;
      default:
        return Colors.grey.shade100;
    }
  }
}
```

---

## 🎨 UI: Linked Resources Widget (Reusable)

```dart
// lib/widgets/linked_resources_widget.dart

import 'package:flutter/material.dart';
import '../services/link_service.dart';

class LinkedResourcesWidget extends StatelessWidget {
  final List<LinkedResource> resources;
  final Function(LinkedResource) onTap;
  final Function(LinkedResource)? onRemove;

  const LinkedResourcesWidget({
    Key? key,
    required this.resources,
    required this.onTap,
    this.onRemove,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    // Group by type
    final grouped = <ResourceType, List<LinkedResource>>{};
    for (var resource in resources) {
      grouped.putIfAbsent(resource.type, () => []).add(resource);
    }

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (grouped.containsKey(ResourceType.note)) ...[
          _buildSection(
            context,
            'Notes',
            grouped[ResourceType.note]!,
            Icons.description,
          ),
          SizedBox(height: 16),
        ],
        if (grouped.containsKey(ResourceType.item)) ...[
          _buildSection(
            context,
            'Items',
            grouped[ResourceType.item]!,
            Icons.inventory_2,
          ),
          SizedBox(height: 16),
        ],
        if (grouped.containsKey(ResourceType.project)) ...[
          _buildSection(
            context,
            'Projects',
            grouped[ResourceType.project]!,
            Icons.folder,
          ),
        ],
      ],
    );
  }

  Widget _buildSection(
    BuildContext context,
    String title,
    List<LinkedResource> items,
    IconData icon,
  ) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            Icon(icon, size: 20),
            SizedBox(width: 8),
            Text(
              title,
              style: Theme.of(context).textTheme.titleMedium,
            ),
            SizedBox(width: 8),
            Chip(
              label: Text('${items.length}'),
              padding: EdgeInsets.zero,
            ),
          ],
        ),
        SizedBox(height: 8),
        ...items.map((resource) => _buildResourceCard(context, resource)),
      ],
    );
  }

  Widget _buildResourceCard(BuildContext context, LinkedResource resource) {
    return Card(
      margin: EdgeInsets.only(bottom: 8),
      child: ListTile(
        leading: Text(
          resource.icon,
          style: TextStyle(fontSize: 24),
        ),
        title: Text(resource.title),
        subtitle: Text(
          '${resource.linkType} • ${resource.direction == 'incoming' ? '→ References this' : 'Referenced by this →'}',
        ),
        trailing: onRemove != null
            ? IconButton(
                icon: Icon(Icons.link_off, size: 20),
                onPressed: () => onRemove!(resource),
                tooltip: 'Remove link',
              )
            : null,
        onTap: () => onTap(resource),
      ),
    );
  }
}
```

---

## 🚀 App Initialization

```dart
// lib/main.dart

import 'package:flutter/material.dart';
import 'package:altair_db_service/altair_db_service.dart';
import 'services/database_service.dart';
import 'screens/home_screen.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // Initialize database service
  await _initializeDatabase();

  runApp(GuidanceApp());
}

Future<void> _initializeDatabase() async {
  try {
    // Get or start the database service
    final service = AltairDatabaseService();
    final status = await service.getStatus();

    if (!status.isRunning) {
      print('Starting Altair database service...');
      await service.start();
    } else {
      print('Altair database service already running');
    }

    // Connect to database
    final dbService = await GuidanceDatabaseService.getInstance();
    await dbService.ensureReady();

    print('✓ Database ready');
  } catch (e) {
    print('Error initializing database: $e');
    // Show error dialog to user
    throw Exception('Failed to initialize database: $e');
  }
}

class GuidanceApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Altair Guidance',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.indigo),
        useMaterial3: true,
      ),
      home: HomeScreen(),
    );
  }
}
```

---

## 🧪 Example Usage

```dart
// Example: Creating a task with links

Future<void> createTaskWithLinks() async {
  final dbService = await GuidanceDatabaseService.getInstance();
  final taskRepo = TaskRepository(dbService);
  final linkService = LinkService(dbService);

  // Create task
  final task = await taskRepo.create(Task(
    id: 'task:${Uuid().v4()}',
    title: 'Setup development environment',
    description: 'Install all necessary tools and dependencies',
    status: 'todo',
    priority: 'high',
    tags: ['setup', 'development'],
    createdAt: DateTime.now(),
    updatedAt: DateTime.now(),
  ));

  // Link to a note (assuming note exists in Knowledge app)
  await linkService.linkTask(
    taskId: task.id,
    targetId: 'note:dev-setup-guide',
    linkType: LinkType.references,
  );

  // Link to required items (assuming items exist in Tracking app)
  await linkService.linkTask(
    taskId: task.id,
    targetId: 'item:laptop',
    linkType: LinkType.requires,
  );

  await linkService.linkTask(
    taskId: task.id,
    targetId: 'item:monitor',
    linkType: LinkType.requires,
  );

  print('✓ Task created with cross-app links');
}
```

---

## 📝 Key Takeaways

### What This Gives You

1. **True Cross-App Integration**
   - Tasks can reference notes from Knowledge
   - Tasks can require items from Tracking
   - All data in one place

2. **Shared Data Access**
   - All apps use same database instance
   - No data duplication
   - Real-time consistency

3. **Clean Architecture**
   - Repository pattern for data access
   - Service layer for business logic
   - Clear separation of concerns

4. **Reusable Components**
   - `LinkedResourcesWidget` works in all apps
   - `LinkService` shared pattern
   - Consistent UX across ecosystem

5. **Easy Testing**
   - Mock database service
   - Test repositories independently
   - Integration tests for cross-app features

---

This practical example shows exactly how to integrate the shared database service into one of your apps. The same patterns apply to Knowledge and Tracking. Ready to start building?
