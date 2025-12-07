/**
 * @altair/sync
 *
 * Change feed sync utilities for the Altair ecosystem.
 *
 * This package provides:
 * - SurrealDB CHANGEFEED processing utilities
 * - Last-Write-Wins (LWW) conflict resolution
 * - Sync state management
 * - Offline queue handling
 *
 * Architecture:
 * - All SurrealDB tables have `CHANGEFEED 7d` enabled
 * - Changes are processed via change feed subscriptions
 * - LWW resolution uses updated_at timestamps
 * - Single-user focus (no complex CRDT needed)
 *
 * @see docs/technical-architecture.md for sync architecture details
 */

// ============================================================================
// Change Types
// ============================================================================

/**
 * Types of changes captured by SurrealDB CHANGEFEED.
 */
export type ChangeAction = 'CREATE' | 'UPDATE' | 'DELETE';

/**
 * A single change from the SurrealDB change feed.
 */
export interface Change<T = unknown> {
  /** Change action type */
  action: ChangeAction;
  /** Table name where change occurred */
  table: string;
  /** Record ID that changed */
  id: string;
  /** Record data (null for DELETE) */
  data: T | null;
  /** Timestamp of the change */
  timestamp: string;
  /** Version sequence number */
  versionstamp: string;
}

/**
 * Batch of changes from a change feed query.
 */
export interface ChangeBatch<T = unknown> {
  /** Array of changes in chronological order */
  changes: Change<T>[];
  /** Cursor for fetching next batch */
  cursor: string | null;
  /** Whether more changes are available */
  hasMore: boolean;
}

// ============================================================================
// Sync State
// ============================================================================

/**
 * Current sync status.
 */
export type SyncStatus = 'idle' | 'syncing' | 'error' | 'offline';

/**
 * Sync state for a single table.
 */
export interface TableSyncState {
  /** Table name */
  table: string;
  /** Last processed versionstamp */
  lastVersionstamp: string | null;
  /** Last successful sync timestamp */
  lastSyncedAt: string | null;
  /** Current sync status */
  status: SyncStatus;
  /** Error message if status is 'error' */
  error: string | null;
  /** Number of pending local changes */
  pendingCount: number;
}

/**
 * Overall sync state across all tables.
 */
export interface SyncState {
  /** Per-table sync states */
  tables: Record<string, TableSyncState>;
  /** Overall status (worst status across all tables) */
  overallStatus: SyncStatus;
  /** Whether device is online */
  isOnline: boolean;
  /** Last connectivity check timestamp */
  lastConnectivityCheck: string | null;
}

// ============================================================================
// Conflict Resolution
// ============================================================================

/**
 * Conflict detection result.
 */
export interface ConflictInfo<T = unknown> {
  /** The conflicting local record */
  local: T;
  /** The conflicting remote record */
  remote: T;
  /** Which record wins based on LWW */
  winner: 'local' | 'remote';
  /** Timestamp comparison result */
  localUpdatedAt: string;
  remoteUpdatedAt: string;
}

/**
 * Last-Write-Wins conflict resolver.
 * Compares updated_at timestamps to determine winner.
 *
 * @param local - Local record
 * @param remote - Remote record
 * @returns The winning record
 */
export function resolveConflictLWW<T extends { updated_at: string }>(local: T, remote: T): T {
  const localTime = new Date(local.updated_at).getTime();
  const remoteTime = new Date(remote.updated_at).getTime();

  // Remote wins on tie (server authority)
  return localTime > remoteTime ? local : remote;
}

/**
 * Detect if two records are in conflict.
 *
 * @param local - Local record
 * @param remote - Remote record with same ID
 * @returns Conflict info or null if no conflict
 */
export function detectConflict<T extends { id: string; updated_at: string }>(
  local: T,
  remote: T
): ConflictInfo<T> | null {
  if (local.id !== remote.id) {
    return null; // Not the same record
  }

  if (local.updated_at === remote.updated_at) {
    return null; // Same version, no conflict
  }

  const localTime = new Date(local.updated_at).getTime();
  const remoteTime = new Date(remote.updated_at).getTime();

  return {
    local,
    remote,
    winner: localTime > remoteTime ? 'local' : 'remote',
    localUpdatedAt: local.updated_at,
    remoteUpdatedAt: remote.updated_at,
  };
}

// ============================================================================
// Offline Queue
// ============================================================================

/**
 * A queued operation to be synced when online.
 */
export interface QueuedOperation<T = unknown> {
  /** Unique operation ID */
  id: string;
  /** Operation type */
  action: ChangeAction;
  /** Target table */
  table: string;
  /** Record ID */
  recordId: string;
  /** Operation data */
  data: T | null;
  /** When operation was queued */
  queuedAt: string;
  /** Number of retry attempts */
  retryCount: number;
  /** Last error if any */
  lastError: string | null;
}

/**
 * Offline operation queue state.
 */
export interface OfflineQueue {
  /** Queued operations in order */
  operations: QueuedOperation[];
  /** Whether queue is being processed */
  isProcessing: boolean;
  /** Last processing attempt timestamp */
  lastProcessedAt: string | null;
}

// ============================================================================
// Sync Configuration
// ============================================================================

/**
 * Configuration for sync behavior.
 */
export interface SyncConfig {
  /** Polling interval in milliseconds (for change feed) */
  pollIntervalMs: number;
  /** Maximum batch size for change feed queries */
  batchSize: number;
  /** Maximum retry attempts for failed operations */
  maxRetries: number;
  /** Retry delay in milliseconds (exponential backoff base) */
  retryDelayMs: number;
  /** Tables to sync (empty = all tables) */
  tables: string[];
  /** Whether to sync automatically on connectivity restore */
  autoSyncOnReconnect: boolean;
}

/**
 * Default sync configuration.
 */
export const DEFAULT_SYNC_CONFIG: SyncConfig = {
  pollIntervalMs: 5000, // 5 seconds
  batchSize: 100,
  maxRetries: 3,
  retryDelayMs: 1000, // 1 second base
  tables: [], // All tables
  autoSyncOnReconnect: true,
};

// ============================================================================
// Sync Events
// ============================================================================

/**
 * Events emitted during sync operations.
 */
export type SyncEvent =
  | { type: 'sync:start'; table: string }
  | { type: 'sync:complete'; table: string; changesProcessed: number }
  | { type: 'sync:error'; table: string; error: string }
  | { type: 'sync:conflict'; table: string; conflict: ConflictInfo }
  | { type: 'connectivity:online' }
  | { type: 'connectivity:offline' }
  | { type: 'queue:added'; operation: QueuedOperation }
  | { type: 'queue:processed'; operationId: string }
  | { type: 'queue:failed'; operationId: string; error: string };

/**
 * Sync event handler type.
 */
export type SyncEventHandler = (event: SyncEvent) => void;

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Calculate exponential backoff delay.
 *
 * @param attempt - Current attempt number (0-indexed)
 * @param baseDelayMs - Base delay in milliseconds
 * @param maxDelayMs - Maximum delay cap
 * @returns Delay in milliseconds
 */
export function calculateBackoff(
  attempt: number,
  baseDelayMs: number = 1000,
  maxDelayMs: number = 30000
): number {
  const delay = baseDelayMs * Math.pow(2, attempt);
  return Math.min(delay, maxDelayMs);
}

/**
 * Generate a unique operation ID.
 */
export function generateOperationId(): string {
  return `op_${Date.now()}_${Math.random().toString(36).substring(2, 9)}`;
}

/**
 * Check if a change feed cursor is still valid (within 7 day window).
 *
 * @param cursor - The cursor timestamp
 * @returns Whether cursor is still valid
 */
export function isCursorValid(cursor: string): boolean {
  const cursorTime = new Date(cursor).getTime();
  const sevenDaysAgo = Date.now() - 7 * 24 * 60 * 60 * 1000;
  return cursorTime > sevenDaysAgo;
}
