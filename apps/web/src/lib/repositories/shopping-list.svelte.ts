import { getSyncClient } from '$lib/sync';

// ============================================================
// Shopping list types (mirror shopping_lists schema columns)
// ============================================================

export interface ShoppingList {
  id: string;
  name: string;
  household_id: string;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

export interface ShoppingListItem {
  id: string;
  shopping_list_id: string;
  item_id: string | null;
  name: string;
  quantity: number;
  status: string;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

// ============================================================
// allShoppingLists — reactive list of all non-deleted shopping lists
// ============================================================

let _allShoppingLists = $state<ShoppingList[]>([]);

$effect(() => {
  const client = getSyncClient();
  let active = true;

  (async () => {
    for await (const result of client.watch(
      'SELECT * FROM shopping_lists WHERE deleted_at IS NULL ORDER BY name ASC',
      [],
    )) {
      if (!active) break;
      _allShoppingLists = (result.rows?._array ?? []) as ShoppingList[];
    }
  })().catch((err) => console.error('[shopping-list] watch failed:', err));

  return () => {
    active = false;
  };
});

export { _allShoppingLists as allShoppingLists };

// ============================================================
// shoppingListById — reactive single shopping list lookup
// ============================================================

export function shoppingListById(id: string): { readonly list: ShoppingList | null } {
  let _list = $state<ShoppingList | null>(null);

  $effect(() => {
    const client = getSyncClient();
    let active = true;

    (async () => {
      for await (const result of client.watch(
        'SELECT * FROM shopping_lists WHERE id = ? AND deleted_at IS NULL LIMIT 1',
        [id],
      )) {
        if (!active) break;
        const rows = (result.rows?._array ?? []) as ShoppingList[];
        _list = rows[0] ?? null;
      }
    })().catch((err) => console.error('[shopping-list] watch failed:', err));

    return () => {
      active = false;
    };
  });

  return {
    get list() {
      return _list;
    },
  };
}

// ============================================================
// itemsForShoppingList — reactive items for a given list, ordered by created_at
// ============================================================

export function itemsForShoppingList(listId: string): { readonly items: ShoppingListItem[] } {
  let _items = $state<ShoppingListItem[]>([]);

  $effect(() => {
    const client = getSyncClient();
    let active = true;

    (async () => {
      for await (const result of client.watch(
        'SELECT * FROM shopping_list_items WHERE shopping_list_id = ? AND deleted_at IS NULL ORDER BY created_at ASC',
        [listId],
      )) {
        if (!active) break;
        _items = (result.rows?._array ?? []) as ShoppingListItem[];
      }
    })().catch((err) => console.error('[shopping-list] watch failed:', err));

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
