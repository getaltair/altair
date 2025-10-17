# Altair Core

Core business logic, models, and data layer shared across all Altair applications.

## Features

- **Models**: Data models for tasks, projects, tags
- **Repositories**: Data access layer with SQLite
- **Database**: SQLite schema and migrations
- **Services**: Business logic services
- **Utilities**: Common utilities and helpers

## Usage

Add to your `pubspec.yaml`:

```yaml
dependencies:
  altair_core:
    path: ../../packages/altair-core
```

### Creating a Task

```dart
import 'package:altair_core/altair_core.dart';

final repository = TaskRepository();

final task = Task(
  id: '',  // Auto-generated
  title: 'Buy groceries',
  description: 'Milk, eggs, bread',
  createdAt: DateTime.now(),
  updatedAt: DateTime.now(),
);

final created = await repository.create(task);
```

### Querying Tasks

```dart
// Get all tasks
final allTasks = await repository.findAll();

// Get tasks by status
final todoTasks = await repository.findAll(status: TaskStatus.todo);

// Search tasks
final searchResults = await repository.search('groceries');

// Get a specific task
final task = await repository.findById(taskId);
```

## Development

### Generate JSON Serialization Code

```bash
cd packages/altair-core
flutter pub run build_runner build --delete-conflicting-outputs
```

### Run Tests

```bash
flutter test
```

## License

AGPL-3.0-or-later
