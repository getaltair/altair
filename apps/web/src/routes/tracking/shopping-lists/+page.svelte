<script lang="ts">
  import { getSyncClient } from '$lib/sync';
  import { allShoppingLists, itemsForShoppingList } from '$lib/repositories/shopping-list.svelte';
  import { itemById } from '$lib/repositories/item.svelte';
  import type { ShoppingList, ShoppingListItem } from '$lib/repositories/shopping-list.svelte';

  // ---- Shopping lists ----
  const lists = $derived(allShoppingLists);

  let selectedListId = $state<string | null>(null);

  // Select first list by default when lists load
  $effect(() => {
    if (lists.length > 0 && !selectedListId) {
      selectedListId = lists[0].id;
    }
  });

  const selectedList = $derived(
    selectedListId ? lists.find((l) => l.id === selectedListId) ?? null : null,
  );

  // ---- Items for selected list ----
  const itemsRepo = $derived(
    selectedListId ? itemsForShoppingList(selectedListId) : null,
  );
  const listItems = $derived(itemsRepo?.items ?? []);

  // ---- Linked inventory quantities ----
  // We build a lookup: item_id -> quantity by watching tracking_items
  interface ItemQtyRow { id: string; quantity: number; name: string; }
  let _inventoryQtys = $state<Record<string, number>>({});

  $effect(() => {
    const linkedIds = listItems.filter((i) => i.item_id).map((i) => i.item_id as string);
    if (linkedIds.length === 0) {
      _inventoryQtys = {};
      return;
    }

    const client = getSyncClient();
    let active = true;
    const placeholders = linkedIds.map(() => '?').join(', ');

    (async () => {
      for await (const result of client.watch(
        `SELECT id, quantity FROM tracking_items WHERE id IN (${placeholders}) AND deleted_at IS NULL`,
        linkedIds,
      )) {
        if (!active) break;
        const rows = (result.rows?._array ?? []) as ItemQtyRow[];
        const map: Record<string, number> = {};
        for (const row of rows) {
          map[row.id] = row.quantity;
        }
        _inventoryQtys = map;
      }
    })();

    return () => { active = false; };
  });

  // ---- Toggle item status ----
  async function toggleItem(item: ShoppingListItem) {
    const client = getSyncClient();
    const now = new Date().toISOString();
    const newStatus = item.status === 'purchased' ? 'pending' : 'purchased';
    await client.execute(
      'UPDATE shopping_list_items SET status = ?, updated_at = ? WHERE id = ?',
      [newStatus, now, item.id],
    );
  }
</script>

<div class="page">
  <h1 class="page-title">Shopping lists</h1>

  {#if lists.length === 0}
    <p class="empty-state">No shopping lists yet.</p>
  {:else}
    <div class="layout">
      <!-- List selector sidebar -->
      <nav class="list-nav" aria-label="Shopping lists">
        <ul>
          {#each lists as list}
            <li>
              <button
                class="list-btn"
                class:active={selectedListId === list.id}
                onclick={() => (selectedListId = list.id)}
              >
                {list.name}
              </button>
            </li>
          {/each}
        </ul>
      </nav>

      <!-- Selected list items -->
      <div class="list-content">
        {#if selectedList}
          <h2 class="list-title">{selectedList.name}</h2>

          {#if listItems.length === 0}
            <p class="empty-state">No items in this list.</p>
          {:else}
            <ul class="items-list">
              {#each listItems as item}
                <li class="item-row" class:purchased={item.status === 'purchased'}>
                  <label class="item-label">
                    <input
                      class="pill-checkbox"
                      type="checkbox"
                      checked={item.status === 'purchased'}
                      onchange={() => toggleItem(item)}
                      aria-label="Mark {item.name} as {item.status === 'purchased' ? 'pending' : 'purchased'}"
                    />
                    <span class="item-name">{item.name}</span>
                    <span class="item-qty">×{item.quantity}</span>
                    {#if item.item_id && _inventoryQtys[item.item_id] !== undefined}
                      <span class="inventory-qty">
                        In stock: {_inventoryQtys[item.item_id]}
                      </span>
                    {/if}
                  </label>
                </li>
              {/each}
            </ul>
          {/if}
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .page {
    max-width: 800px;
    margin: 0 auto;
  }

  .page-title {
    font-family: var(--font-display);
    font-size: 2rem;
    font-weight: 700;
    color: var(--on-surface);
    margin: 0 0 1.5rem;
  }

  .empty-state {
    color: var(--on-surface-variant);
    font-family: var(--font-body);
    padding: 2rem 0;
  }

  .layout {
    display: grid;
    grid-template-columns: 200px 1fr;
    gap: 1.5rem;
  }

  @media (max-width: 600px) {
    .layout {
      grid-template-columns: 1fr;
    }
  }

  .list-nav ul {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .list-btn {
    width: 100%;
    text-align: left;
    padding: 0.5rem 0.75rem;
    border-radius: 9999px;
    border: none;
    background: transparent;
    color: var(--on-surface);
    font-family: var(--font-body);
    font-size: 0.9375rem;
    font-weight: 500;
    cursor: pointer;
    transition: background-color var(--motion-standard), color var(--motion-standard);
  }

  .list-btn:hover {
    background-color: var(--surface-zone);
  }

  .list-btn.active {
    background-color: var(--secondary-container);
    color: var(--primary);
    font-weight: 600;
  }

  .list-content {
    min-width: 0;
  }

  .list-title {
    font-family: var(--font-body);
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--on-surface);
    margin: 0 0 1rem;
  }

  .items-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .item-row {
    padding: 0.75rem 1rem;
    border-radius: 0.75rem;
    background-color: var(--surface-container);
    transition: opacity var(--motion-standard);
  }

  .item-row.purchased {
    opacity: 0.45;
  }

  .item-label {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    cursor: pointer;
  }

  /* Pill-shaped checkbox — border-radius: 9999px */
  .pill-checkbox {
    appearance: none;
    -webkit-appearance: none;
    width: 1.25rem;
    height: 1.25rem;
    border-radius: 9999px;
    border: 2px solid var(--on-surface-variant);
    background-color: var(--surface-card);
    cursor: pointer;
    flex-shrink: 0;
    transition: background-color var(--motion-standard), border-color var(--motion-standard);
    position: relative;
  }

  .pill-checkbox:checked {
    background-color: var(--primary);
    border-color: var(--primary);
  }

  .pill-checkbox:checked::after {
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    width: 0.4rem;
    height: 0.65rem;
    border: 2px solid var(--surface-card);
    border-top: none;
    border-left: none;
    transform: translate(-50%, -60%) rotate(45deg);
  }

  .item-name {
    font-family: var(--font-body);
    font-size: 0.9375rem;
    color: var(--on-surface);
    font-weight: 500;
    flex: 1;
  }

  .item-qty {
    font-family: var(--font-body);
    font-size: 0.875rem;
    color: var(--on-surface-variant);
  }

  .inventory-qty {
    font-family: var(--font-body);
    font-size: 0.8125rem;
    color: var(--on-surface-variant);
    font-style: italic;
  }
</style>
