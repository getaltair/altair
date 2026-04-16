import { getSyncClient } from '$lib/sync';
import { RelationType } from '$lib/contracts';

// ============================================================
// Note type derived from the PowerSync schema
// ============================================================

export interface Note {
  id: string;
  title: string;
  content: string;
  user_id: string;
  initiative_id: string | null;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

export interface EntityRelation {
  id: string;
  from_entity_type: string;
  from_entity_id: string;
  to_entity_type: string;
  to_entity_id: string;
  relation_type: string;
  source_type: string | null;
  status: string | null;
  confidence: number | null;
  evidence: string | null;
  user_id: string;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

// ============================================================
// allNotes — all non-deleted notes, sorted by updated_at desc
// ============================================================

export function allNotes(): Note[] {
  const db = getSyncClient();
  let notes: Note[] = $state([]);

  $effect(() => {
    const controller = new AbortController();

    (async () => {
      const stream = db.watch(
        `SELECT * FROM notes WHERE deleted_at IS NULL ORDER BY updated_at DESC`,
        [],
        { signal: controller.signal },
      );
      for await (const result of stream) {
        notes = (result.rows?._array ?? []) as Note[];
      }
    })();

    return () => controller.abort();
  });

  return notes;
}

// ============================================================
// noteById — single note by id
// ============================================================

export function noteById(id: string): Note | null {
  const db = getSyncClient();
  let note: Note | null = $state(null);

  $effect(() => {
    const controller = new AbortController();

    (async () => {
      const stream = db.watch(
        `SELECT * FROM notes WHERE id = ? AND deleted_at IS NULL LIMIT 1`,
        [id],
        { signal: controller.signal },
      );
      for await (const result of stream) {
        note = ((result.rows?._array ?? [])[0] ?? null) as Note | null;
      }
    })();

    return () => controller.abort();
  });

  return note;
}

// ============================================================
// searchNotes — local text filter across title and content
// ============================================================

export function searchNotes(query: string): Note[] {
  const db = getSyncClient();
  let notes: Note[] = $state([]);

  $effect(() => {
    const controller = new AbortController();
    const pattern = `%${query}%`;

    (async () => {
      const stream = db.watch(
        `SELECT * FROM notes
         WHERE deleted_at IS NULL
           AND (title LIKE ? OR content LIKE ?)
         ORDER BY updated_at DESC`,
        [pattern, pattern],
        { signal: controller.signal },
      );
      for await (const result of stream) {
        notes = (result.rows?._array ?? []) as Note[];
      }
    })();

    return () => controller.abort();
  });

  return notes;
}

// ============================================================
// backlinksFor — entity_relations where to_entity_id = noteId
//                and relation_type = note_link
// ============================================================

export function backlinksFor(noteId: string): EntityRelation[] {
  const db = getSyncClient();
  let relations: EntityRelation[] = $state([]);

  $effect(() => {
    const controller = new AbortController();

    (async () => {
      const stream = db.watch(
        `SELECT * FROM entity_relations
         WHERE to_entity_id = ?
           AND relation_type = ?
           AND deleted_at IS NULL
         ORDER BY created_at DESC`,
        [noteId, RelationType.NoteLink],
        { signal: controller.signal },
      );
      for await (const result of stream) {
        relations = (result.rows?._array ?? []) as EntityRelation[];
      }
    })();

    return () => controller.abort();
  });

  return relations;
}
