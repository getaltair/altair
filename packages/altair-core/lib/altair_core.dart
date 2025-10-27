/// Core business logic, models, and data layer for Altair applications.
library altair_core;

// Models
export 'models/task.dart';
export 'models/project.dart';
export 'models/tag.dart';

// Repositories
export 'repositories/task_repository.dart';
export 'repositories/task_repository_surrealdb.dart';
export 'repositories/project_repository.dart';
export 'repositories/tag_repository.dart';

// Database
export 'database/database.dart';
export 'database/schema.dart';
