<script lang="ts">
  import { getSyncClient } from '$lib/sync';
  import { allItems, lowStockItems } from '$lib/repositories/item.svelte';
  import Badge from '$lib/components/primitives/Badge.svelte';
  import Card from '$lib/components/primitives/Card.svelte';

  // ---- Filter state ----
  let locationFilter = $state('');
  let categoryFilter = $state('');
  let viewMode = $state<'table' | 'card'>('table');

  // ---- Locations and categories for filter dropdowns ----
  interface TrackingLocation {
    id: string;
    name: string;
  }

  interface TrackingCategory {
    id: string;
    name: string;
  }

  let _locations = $state<TrackingLocation[]>([]);
  let _categories = $state<TrackingCategory[]>([]);

  $effect(() => {
    const client = getSyncClient();
    let active = true;

    (async () => {
      for await (const result of client.watch(
        'SELECT id, name FROM tracking_locations WHERE deleted_at IS NULL ORDER BY name ASC',
        [],
      )) {
        if (!active) break;
        _locations = (result.rows?._array ?? []) as TrackingLocation[];
      }
    })();

    return () => { active = false; };
  });

  $effect(() => {
    const client = getSyncClient();
    let active = true;

    (async () => {
      for await (const result of client.watch(
        'SELECT id, name FROM tracking_categories WHERE deleted_at IS NULL ORDER BY name ASC',
        [],
      )) {
        if (!active) break;
        _categories = (result.rows?._array ?? []) as TrackingCategory[];
      }
    })();

    return () => { active = false; };
  });

  // ---- Reactive items filtered by dropdowns ----
  const repo = $derived(
    allItems({
      locationId: locationFilter || undefined,
      categoryId: categoryFilter || undefined,
    }),
  );

  const items = $derived(repo.items);
  const lowStock = $derived(lowStockItems);

  // ---- Helpers ----
  function locationName(id: string | null): string {
    if (!id) return '—';
    return _locations.find((l) => l.id === id)?.name ?? '—';
  }

  function categoryName(id: string | null): string {
    if (!id) return '—';
    return _categories.find((c) => c.id === id)?.name ?? '—';
  }

  function isLowStock(itemId: string): boolean {
    return lowStock.some((i) => i.id === itemId);
  }
</script>

<div class="page">
  <header class="page-header">
    <h1 class="page-title">Inventory</h1>
    <a href="/tracking/items/new" class="btn-link">+ New item</a>
  </header>

  <!-- Low-stock alert summary -->
  {#if lowStock.length > 0}
    <div class="low-stock-banner">
      <Badge label="{lowStock.length} low stock" color="#9f403d" />
      <span class="low-stock-hint">Items at or below zero quantity.</span>
    </div>
  {/if}

  <!-- Filters + view toggle -->
  <div class="controls">
    <div class="filters">
      <select class="filter-select" bind:value={locationFilter} aria-label="Filter by location">
        <option value="">All locations</option>
        {#each _locations as loc}
          <option value={loc.id}>{loc.name}</option>
        {/each}
      </select>

      <select class="filter-select" bind:value={categoryFilter} aria-label="Filter by category">
        <option value="">All categories</option>
        {#each _categories as cat}
          <option value={cat.id}>{cat.name}</option>
        {/each}
      </select>
    </div>

    <div class="view-toggle" role="group" aria-label="View mode">
      <button
        class="toggle-btn"
        class:active={viewMode === 'table'}
        onclick={() => (viewMode = 'table')}
      >Table</button>
      <button
        class="toggle-btn"
        class:active={viewMode === 'card'}
        onclick={() => (viewMode = 'card')}
      >Cards</button>
    </div>
  </div>

  <!-- Table view -->
  {#if viewMode === 'table'}
    {#if items.length === 0}
      <p class="empty-state">No items found.</p>
    {:else}
      <div class="table-wrap">
        <table class="items-table">
          <thead>
            <tr>
              <th>Name</th>
              <th>Quantity</th>
              <th>Location</th>
              <th>Category</th>
            </tr>
          </thead>
          <tbody>
            {#each items as item}
              <tr>
                <td>
                  <a href="/tracking/items/{item.id}" class="item-link">{item.name}</a>
                </td>
                <td>
                  {#if isLowStock(item.id)}
                    <Badge label={String(item.quantity)} color="#9f403d" />
                  {:else}
                    {item.quantity}
                  {/if}
                </td>
                <td>{locationName(item.location_id)}</td>
                <td>{categoryName(item.category_id)}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  {/if}

  <!-- Card view -->
  {#if viewMode === 'card'}
    {#if items.length === 0}
      <p class="empty-state">No items found.</p>
    {:else}
      <div class="card-grid">
        {#each items as item}
          <a href="/tracking/items/{item.id}" class="card-link">
            <Card>
              <div class="card-body">
                <div class="card-row">
                  <span class="item-name">{item.name}</span>
                  {#if isLowStock(item.id)}
                    <Badge label={String(item.quantity)} color="#9f403d" />
                  {:else}
                    <span class="qty-label">Qty: {item.quantity}</span>
                  {/if}
                </div>
                <div class="card-meta">
                  <span>{locationName(item.location_id)}</span>
                  {#if item.category_id}
                    <span>· {categoryName(item.category_id)}</span>
                  {/if}
                </div>
              </div>
            </Card>
          </a>
        {/each}
      </div>
    {/if}
  {/if}
</div>

<style>
  .page {
    max-width: 960px;
    margin: 0 auto;
  }

  .page-header {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    margin-bottom: 1.5rem;
  }

  .page-title {
    font-family: var(--font-display);
    font-size: 2rem;
    font-weight: 700;
    color: var(--on-surface);
    margin: 0;
  }

  .btn-link {
    display: inline-flex;
    align-items: center;
    padding: 0.5rem 1.25rem;
    border-radius: 9999px;
    background-color: var(--primary);
    color: var(--surface-card);
    font-family: var(--font-body);
    font-size: 0.9375rem;
    font-weight: 600;
    text-decoration: none;
    transition: background-color var(--motion-standard);
  }

  .btn-link:hover {
    background-color: color-mix(in srgb, var(--primary) 85%, var(--surface-card));
  }

  .low-stock-banner {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 1.25rem;
    padding: 0.75rem 1rem;
    border-radius: 0.75rem;
    background-color: var(--surface-zone);
  }

  .low-stock-hint {
    font-family: var(--font-body);
    font-size: 0.875rem;
    color: var(--on-surface-variant);
  }

  .controls {
    display: flex;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
    margin-bottom: 1.25rem;
  }

  .filters {
    display: flex;
    gap: 0.75rem;
    flex: 1;
    flex-wrap: wrap;
  }

  .filter-select {
    background-color: var(--surface-zone);
    border: 1.5px solid transparent;
    border-radius: 0.5rem;
    padding: 0.5rem 0.75rem;
    font-family: var(--font-body);
    font-size: 0.875rem;
    color: var(--on-surface);
    cursor: pointer;
    outline: none;
    transition: border-color var(--motion-standard);
  }

  .filter-select:focus {
    border-color: var(--primary);
  }

  .view-toggle {
    display: flex;
    gap: 0.25rem;
  }

  .toggle-btn {
    padding: 0.375rem 0.875rem;
    border-radius: 9999px;
    border: none;
    background-color: var(--surface-zone);
    color: var(--on-surface-variant);
    font-family: var(--font-body);
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: background-color var(--motion-standard), color var(--motion-standard);
  }

  .toggle-btn.active {
    background-color: var(--secondary-container);
    color: var(--primary);
    font-weight: 600;
  }

  .empty-state {
    text-align: center;
    color: var(--on-surface-variant);
    font-family: var(--font-body);
    padding: 3rem 0;
  }

  .table-wrap {
    overflow-x: auto;
  }

  .items-table {
    width: 100%;
    border-collapse: collapse;
    font-family: var(--font-body);
    font-size: 0.9375rem;
    color: var(--on-surface);
  }

  .items-table th {
    text-align: left;
    padding: 0.625rem 0.75rem;
    font-size: 0.8125rem;
    font-weight: 600;
    letter-spacing: 0.05em;
    text-transform: uppercase;
    color: var(--on-surface-variant);
    border-bottom: 1px solid var(--surface-zone);
  }

  .items-table td {
    padding: 0.75rem;
    vertical-align: middle;
    border-bottom: 1px solid var(--surface-zone);
  }

  .items-table tr:last-child td {
    border-bottom: none;
  }

  .item-link {
    color: var(--primary);
    text-decoration: none;
    font-weight: 500;
  }

  .item-link:hover {
    text-decoration: underline;
  }

  .card-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 1rem;
  }

  .card-link {
    text-decoration: none;
  }

  .card-body {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .card-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
  }

  .item-name {
    font-family: var(--font-body);
    font-size: 1rem;
    font-weight: 600;
    color: var(--on-surface);
  }

  .qty-label {
    font-family: var(--font-body);
    font-size: 0.875rem;
    color: var(--on-surface-variant);
  }

  .card-meta {
    font-family: var(--font-body);
    font-size: 0.8125rem;
    color: var(--on-surface-variant);
    display: flex;
    gap: 0.25rem;
    flex-wrap: wrap;
  }
</style>
