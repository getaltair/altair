/**
 * @altair/db
 *
 * SurrealDB schema types and query utilities for the Altair ecosystem.
 *
 * This package provides:
 * - Schema type definitions matching SurrealDB tables
 * - Query builder utilities (coming in future specs)
 * - Connection helpers for embedded and cloud SurrealDB
 *
 * @see docs/domain-model.md for entity relationships
 */

// ============================================================================
// Table Names
// ============================================================================

/**
 * SurrealDB table names used across the Altair ecosystem.
 * All tables use SCHEMAFULL mode and have CHANGEFEED 7d enabled.
 */
export const TABLES = {
  // Guidance app
  quest: 'quest',
  campaign: 'campaign',

  // Knowledge app
  note: 'note',
  folder: 'folder',

  // Tracking app
  item: 'item',
  location: 'location',

  // Shared
  capture: 'capture',
  user: 'user',
} as const;

export type TableName = (typeof TABLES)[keyof typeof TABLES];

// ============================================================================
// Edge Table Names (Graph Relationships)
// ============================================================================

/**
 * SurrealDB edge table names for graph relationships.
 * Edges follow the pattern: source ->edge-> target
 */
export const EDGES = {
  // Quest relationships
  contains: 'contains', // campaign ->contains-> quest
  references: 'references', // quest ->references-> note
  requires: 'requires', // quest ->requires-> item

  // Note relationships
  links_to: 'links_to', // note ->links_to-> note (wiki-links)
  documents: 'documents', // note ->documents-> item

  // Location relationships
  stored_in: 'stored_in', // item ->stored_in-> location
} as const;

export type EdgeName = (typeof EDGES)[keyof typeof EDGES];

// ============================================================================
// Schema Types (matching SurrealDB table schemas)
// ============================================================================

/**
 * SurrealDB record ID format.
 * Example: "quest:abc123" or "note:xyz789"
 */
export type RecordId<T extends TableName = TableName> = `${T}:${string}`;

/**
 * User reference for owner field.
 * All entities are owned by a user for record-level auth.
 */
export type UserId = RecordId<'user'>;

/**
 * Base fields present on all SurrealDB records.
 * These are auto-managed by the database.
 */
export interface SurrealRecord {
  id: string;
  created_at: string;
  updated_at: string;
}

/**
 * Base fields for all Altair entities.
 * Extends SurrealRecord with owner and soft-delete status.
 */
export interface BaseSchema extends SurrealRecord {
  owner: UserId;
  status: 'active' | 'archived';
}

// ============================================================================
// Guidance Schemas
// ============================================================================

export interface QuestSchema extends BaseSchema {
  title: string;
  description: string | null;
  energy_cost: 'low' | 'medium' | 'high' | 'epic';
  due_date: string | null;
  completed_at: string | null;
}

export interface CampaignSchema extends BaseSchema {
  title: string;
  description: string | null;
}

// ============================================================================
// Knowledge Schemas
// ============================================================================

export interface NoteSchema extends BaseSchema {
  title: string;
  content: string;
  tags: string[];
}

export interface FolderSchema extends BaseSchema {
  name: string;
  parent_id: RecordId<'folder'> | null;
}

// ============================================================================
// Tracking Schemas
// ============================================================================

export interface ItemSchema extends BaseSchema {
  name: string;
  description: string | null;
  quantity: number;
  image_url: string | null;
}

export interface LocationSchema extends BaseSchema {
  name: string;
  description: string | null;
  parent_id: RecordId<'location'> | null;
}

// ============================================================================
// Quick Capture Schema
// ============================================================================

export type CaptureType = 'quest' | 'note' | 'item' | 'unknown';

export interface CaptureSchema extends BaseSchema {
  content: string;
  capture_type: CaptureType;
  processed_at: string | null;
  target_id: string | null;
}

// ============================================================================
// User Schema
// ============================================================================

export interface UserSchema extends SurrealRecord {
  email: string;
  name: string | null;
  avatar_url: string | null;
}

// ============================================================================
// Query Result Types
// ============================================================================

/**
 * Generic query result wrapper.
 * SurrealDB returns results in this format.
 */
export interface QueryResult<T> {
  result: T[];
  status: string;
  time: string;
}

/**
 * Type-safe query result for a specific table.
 */
export type TableQueryResult<T extends TableName> = T extends 'quest'
  ? QueryResult<QuestSchema>
  : T extends 'campaign'
    ? QueryResult<CampaignSchema>
    : T extends 'note'
      ? QueryResult<NoteSchema>
      : T extends 'folder'
        ? QueryResult<FolderSchema>
        : T extends 'item'
          ? QueryResult<ItemSchema>
          : T extends 'location'
            ? QueryResult<LocationSchema>
            : T extends 'capture'
              ? QueryResult<CaptureSchema>
              : T extends 'user'
                ? QueryResult<UserSchema>
                : never;

// ============================================================================
// Connection Config Types (for future implementation)
// ============================================================================

/**
 * SurrealDB connection configuration.
 * Supports both embedded (local) and cloud connections.
 */
export interface ConnectionConfig {
  /** Connection URL (e.g., "ws://localhost:8000" or embedded path) */
  url: string;
  /** Namespace for multi-tenancy */
  namespace: string;
  /** Database name */
  database: string;
  /** Optional authentication */
  auth?: {
    username: string;
    password: string;
  };
}

/**
 * Default connection config for local development.
 * Uses embedded SurrealDB via surrealkv.
 */
export const DEFAULT_CONFIG: ConnectionConfig = {
  url: 'ws://localhost:8000',
  namespace: 'altair',
  database: 'main',
};
