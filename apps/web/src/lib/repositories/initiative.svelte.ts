import { getSyncClient } from '$lib/sync';

// ============================================================
// Initiative row type — mirrors the initiatives table in schema.ts
// ============================================================

export interface Initiative {
  id: string;
  title: string;
  description: string | null;
  status: string;
  user_id: string;
  household_id: string | null;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

// ============================================================
// Reactive state — module-level $state (Svelte 5 runes)
// ============================================================

let _allInitiatives = $state<Initiative[]>([]);

$effect(() => {
  const client = getSyncClient();
  (async () => {
    for await (const result of client.watch(
      'SELECT * FROM initiatives WHERE deleted_at IS NULL ORDER BY created_at DESC',
      []
    )) {
      _allInitiatives = (result.rows?._array ?? []) as Initiative[];
    }
  })();
});

// ============================================================
// Exported accessors
// ============================================================

/** All non-deleted initiatives. */
export const allInitiatives = (): Initiative[] => _allInitiatives;

/** Look up a single initiative by its UUID. */
export const initiativeById = (id: string): Initiative | undefined =>
  _allInitiatives.find((i) => i.id === id);
