import { getSyncClient } from '$lib/sync';

// ============================================================
// Routine row type — mirrors the routines table in schema.ts
// ============================================================

export interface Routine {
  id: string;
  title: string;
  description: string | null;
  frequency_type: string;
  frequency_config: string | null;
  status: string;
  user_id: string;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

// ============================================================
// Reactive state — module-level $state (Svelte 5 runes)
// ============================================================

let _allRoutines = $state<Routine[]>([]);

$effect(() => {
  const client = getSyncClient();
  (async () => {
    for await (const result of client.watch(
      'SELECT * FROM routines WHERE deleted_at IS NULL ORDER BY created_at DESC',
      []
    )) {
      _allRoutines = (result.rows?._array ?? []) as Routine[];
    }
  })().catch((err) => console.error('[routine] watch failed:', err));
});

// ============================================================
// Exported accessors
// ============================================================

/** All non-deleted routines. */
export const allRoutines = (): Routine[] => _allRoutines;

/**
 * Active routines that are due today.
 * A routine is considered "due today" when its status is 'active'.
 * Frequency-based scheduling (next occurrence) is handled by the server;
 * the client shows all active routines in the Today view.
 */
export const dueToday = (): Routine[] =>
  _allRoutines.filter((r) => r.status === 'active');
