import { getSyncClient } from '$lib/sync';

// ============================================================
// Tracking item types (mirrors tracking_items schema columns)
// ============================================================

export interface TrackingItem {
  id: string;
  name: string;
  description: string | null;
  quantity: number;
  barcode: string | null;
  location_id: string | null;
  category_id: string | null;
  user_id: string;
  household_id: string | null;
  initiative_id: string | null;
  expires_at: string | null;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

// ============================================================
// allItems — reactive list, optionally filtered by category or location
// ============================================================

export function allItems(filters?: { categoryId?: string; locationId?: string }): {
  readonly items: TrackingItem[];
} {
  let _items = $state<TrackingItem[]>([]);

  $effect(() => {
    const client = getSyncClient();
    let active = true;

    const conditions: string[] = ['deleted_at IS NULL'];
    const params: string[] = [];

    if (filters?.categoryId) {
      conditions.push(`category_id = ?`);
      params.push(filters.categoryId);
    }
    if (filters?.locationId) {
      conditions.push(`location_id = ?`);
      params.push(filters.locationId);
    }

    const sql = `SELECT * FROM tracking_items WHERE ${conditions.join(' AND ')} ORDER BY name ASC`;

    (async () => {
      for await (const result of client.watch(sql, params)) {
        if (!active) break;
        _items = (result.rows?._array ?? []) as TrackingItem[];
      }
    })();

    return () => {
      active = false;
    };
  });

  return {
    get items() {
      return _items;
    },
  };
}

// ============================================================
// itemById — reactive single item lookup
// ============================================================

export function itemById(id: string): { readonly item: TrackingItem | null } {
  let _item = $state<TrackingItem | null>(null);

  $effect(() => {
    const client = getSyncClient();
    let active = true;

    (async () => {
      for await (const result of client.watch(
        'SELECT * FROM tracking_items WHERE id = ? AND deleted_at IS NULL LIMIT 1',
        [id],
      )) {
        if (!active) break;
        const rows = (result.rows?._array ?? []) as TrackingItem[];
        _item = rows[0] ?? null;
      }
    })();

    return () => {
      active = false;
    };
  });

  return {
    get item() {
      return _item;
    },
  };
}

// ============================================================
// lowStockItems — items where quantity is at or below zero.
// Note: a per-item low_stock_threshold column is not yet in the schema
// (planned as FR-4.10). This uses quantity <= 0 as a proxy until the
// column is added and synced.
// ============================================================

let _lowStockItems = $state<TrackingItem[]>([]);

$effect(() => {
  const client = getSyncClient();
  let active = true;

  (async () => {
    for await (const result of client.watch(
      'SELECT * FROM tracking_items WHERE deleted_at IS NULL AND quantity <= 0 ORDER BY name ASC',
      [],
    )) {
      if (!active) break;
      _lowStockItems = (result.rows?._array ?? []) as TrackingItem[];
    }
  })();

  return () => {
    active = false;
  };
});

export { _lowStockItems as lowStockItems };
