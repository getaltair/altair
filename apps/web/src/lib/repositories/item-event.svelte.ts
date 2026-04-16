import { getSyncClient } from '$lib/sync';

// ============================================================
// Tracking item event types (mirrors tracking_item_events schema columns)
// tracking_item_events is append-only — no updated_at, no deleted_at
// ============================================================

export interface TrackingItemEvent {
  id: string;
  item_id: string;
  event_type: string;
  quantity_change: number;
  from_location_id: string | null;
  to_location_id: string | null;
  notes: string | null;
  occurred_at: string;
  created_at: string;
}

// ============================================================
// eventsForItem — reactive event timeline for a given item, oldest first
// ============================================================

export function eventsForItem(itemId: string): { readonly events: TrackingItemEvent[] } {
  let _events = $state<TrackingItemEvent[]>([]);

  $effect(() => {
    const client = getSyncClient();
    let active = true;

    (async () => {
      for await (const result of client.watch(
        'SELECT * FROM tracking_item_events WHERE item_id = ? ORDER BY created_at ASC',
        [itemId],
      )) {
        if (!active) break;
        _events = (result.rows?._array ?? []) as TrackingItemEvent[];
      }
    })();

    return () => {
      active = false;
    };
  });

  return {
    get events() {
      return _events;
    },
  };
}
