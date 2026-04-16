import { getSyncClient } from '$lib/sync';

// ============================================================
// Tag types derived from the PowerSync schema
// ============================================================

export interface Tag {
  id: string;
  name: string;
  user_id: string;
  created_at: string;
  updated_at: string;
}

export interface EntityTag {
  id: string;
  entity_id: string;
  entity_type: string;
  tag_id: string;
  created_at: string;
}

// ============================================================
// allTags — all tags sorted by name
// ============================================================

export function allTags(): Tag[] {
  const db = getSyncClient();
  let tags: Tag[] = $state([]);

  $effect(() => {
    const controller = new AbortController();

    (async () => {
      const stream = db.watch(
        `SELECT * FROM tags ORDER BY name ASC`,
        [],
        { signal: controller.signal },
      );
      for await (const result of stream) {
        tags = (result.rows?._array ?? []) as Tag[];
      }
    })().catch((err) => console.error('[tag] watch failed:', err));

    return () => controller.abort();
  });

  return tags;
}

// ============================================================
// tagsForEntity — tags associated with a given entity
// ============================================================

export function tagsForEntity(entityId: string, entityType: string): Tag[] {
  const db = getSyncClient();
  let tags: Tag[] = $state([]);

  $effect(() => {
    const controller = new AbortController();

    (async () => {
      const stream = db.watch(
        `SELECT t.*
         FROM tags t
         INNER JOIN entity_tags et ON et.tag_id = t.id
         WHERE et.entity_id = ? AND et.entity_type = ?
         ORDER BY t.name ASC`,
        [entityId, entityType],
        { signal: controller.signal },
      );
      for await (const result of stream) {
        tags = (result.rows?._array ?? []) as Tag[];
      }
    })().catch((err) => console.error('[tag] watch failed:', err));

    return () => controller.abort();
  });

  return tags;
}
