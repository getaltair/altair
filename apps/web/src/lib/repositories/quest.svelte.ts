import { getSyncClient } from '$lib/sync';
import { isToday } from '$lib/utils/date';

// ============================================================
// Quest row type — mirrors the quests table in schema.ts
// ============================================================

export interface Quest {
  id: string;
  title: string;
  description: string | null;
  status: string;
  priority: string | null;
  due_date: string | null;
  epic_id: string | null;
  initiative_id: string | null;
  routine_id: string | null;
  user_id: string;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

// ============================================================
// Reactive state — module-level $state (Svelte 5 runes)
// ============================================================

let _allQuests = $state<Quest[]>([]);

$effect(() => {
  const client = getSyncClient();
  (async () => {
    for await (const result of client.watch(
      'SELECT * FROM quests WHERE deleted_at IS NULL ORDER BY created_at DESC',
      []
    )) {
      _allQuests = (result.rows?._array ?? []) as Quest[];
    }
  })();
});

// ============================================================
// Exported accessors
// ============================================================

/** All non-deleted quests, ordered by created_at descending. */
export const allQuests = (): Quest[] => _allQuests;

/**
 * Quests due today or with no due date, excluding completed and cancelled quests.
 * Intended for the Today view.
 */
export const todayQuests = (): Quest[] =>
  _allQuests.filter(
    (q) =>
      q.status !== 'completed' &&
      q.status !== 'cancelled' &&
      (q.due_date === null || isToday(q.due_date))
  );

/** Look up a single quest by its UUID. */
export const questById = (id: string): Quest | undefined =>
  _allQuests.find((q) => q.id === id);

/** All quests belonging to a given epic. */
export const questsByEpic = (epicId: string): Quest[] =>
  _allQuests.filter((q) => q.epic_id === epicId);
