import { getSyncClient } from '$lib/sync';

// ============================================================
// Epic row type — mirrors the epics table in schema.ts
// ============================================================

export interface Epic {
  id: string;
  initiative_id: string;
  title: string;
  description: string | null;
  status: string;
  sort_order: number | null;
  user_id: string;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

// ============================================================
// Reactive state — module-level $state (Svelte 5 runes)
// ============================================================

let _allEpics = $state<Epic[]>([]);

$effect(() => {
  const client = getSyncClient();
  (async () => {
    for await (const result of client.watch(
      'SELECT * FROM epics WHERE deleted_at IS NULL ORDER BY sort_order ASC, created_at DESC',
      []
    )) {
      _allEpics = (result.rows?._array ?? []) as Epic[];
    }
  })();
});

// ============================================================
// Exported accessors
// ============================================================

/** All epics belonging to a given initiative. */
export const epicsByInitiative = (initiativeId: string): Epic[] =>
  _allEpics.filter((e) => e.initiative_id === initiativeId);

/** Look up a single epic by its UUID. */
export const epicById = (id: string): Epic | undefined =>
  _allEpics.find((e) => e.id === id);
