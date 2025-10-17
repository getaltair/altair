/// SQLite database schema for Altair
///
/// This file defines the database structure for the local-first storage.
/// PowerSync will use these schemas for synchronization when sync is enabled.
library;

/// SQL schema version
const int schemaVersion = 1;

/// Create tables SQL statements
const List<String> createTableStatements = [
  // Projects table
  '''
  CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'active',
    tags TEXT NOT NULL DEFAULT '[]',
    color TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    target_date INTEGER,
    completed_at INTEGER,
    metadata TEXT
  )
  ''',

  // Tasks table
  '''
  CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'todo',
    tags TEXT NOT NULL DEFAULT '[]',
    project_id TEXT,
    parent_task_id TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    completed_at INTEGER,
    estimated_minutes INTEGER,
    actual_minutes INTEGER,
    priority INTEGER NOT NULL DEFAULT 3,
    metadata TEXT,
    FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE,
    FOREIGN KEY (parent_task_id) REFERENCES tasks (id) ON DELETE CASCADE
  )
  ''',

  // Tags table
  '''
  CREATE TABLE IF NOT EXISTS tags (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    color TEXT,
    created_at INTEGER NOT NULL,
    usage_count INTEGER NOT NULL DEFAULT 0
  )
  ''',

  // Indexes for better query performance
  '''
  CREATE INDEX IF NOT EXISTS idx_tasks_project_id ON tasks(project_id)
  ''',
  '''
  CREATE INDEX IF NOT EXISTS idx_tasks_parent_task_id ON tasks(parent_task_id)
  ''',
  '''
  CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status)
  ''',
  '''
  CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks(created_at DESC)
  ''',
  '''
  CREATE INDEX IF NOT EXISTS idx_projects_status ON projects(status)
  ''',
  '''
  CREATE INDEX IF NOT EXISTS idx_tags_name ON tags(name)
  ''',
];

/// Migration statements for future schema updates
const Map<int, List<String>> migrations = {
  // Version 2 migrations would go here
  // 2: [
  //   'ALTER TABLE tasks ADD COLUMN new_field TEXT',
  // ],
};
