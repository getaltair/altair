<script lang="ts">
  import Card from '$lib/components/primitives/Card.svelte';
  import Badge from '$lib/components/primitives/Badge.svelte';
  import Button from '$lib/components/primitives/Button.svelte';
  import { allInitiatives } from '$lib/repositories/initiative.svelte';

  const STATUSES = ['all', 'active', 'completed', 'cancelled', 'draft'] as const;
  type StatusFilter = (typeof STATUSES)[number];

  let filter = $state<StatusFilter>('all');
  const initiatives = $derived(allInitiatives());
  const filtered = $derived(
    filter === 'all' ? initiatives : initiatives.filter((i) => i.status === filter)
  );
</script>

<div class="page">
  <header class="page__header">
    <h1 class="page__title">Initiatives</h1>
    <Button onclick={() => (window.location.href = '/guidance/initiatives/new')}>
      New Initiative
    </Button>
  </header>

  <div class="filters">
    {#each STATUSES as s}
      <button
        class="filter-chip"
        class:filter-chip--active={filter === s}
        onclick={() => (filter = s)}
      >
        {s === 'all' ? 'All' : s.charAt(0).toUpperCase() + s.slice(1)}
      </button>
    {/each}
  </div>

  {#if filtered.length === 0}
    <p class="empty-state">No initiatives found.</p>
  {:else}
    <ul class="list">
      {#each filtered as initiative (initiative.id)}
        <li>
          <a href="/guidance/initiatives/{initiative.id}" class="item-link">
            <Card>
              <div class="item-row">
                <span class="item-title">{initiative.title}</span>
                <Badge label={initiative.status} />
              </div>
              {#if initiative.description}
                <p class="item-description">{initiative.description}</p>
              {/if}
            </Card>
          </a>
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .page {
    max-width: 48rem;
    margin: 0 auto;
    padding: 2rem 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .page__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
  }

  .page__title {
    font-family: var(--font-display);
    font-size: 1.75rem;
    font-weight: 700;
    color: var(--on-surface);
  }

  .filters {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .filter-chip {
    padding: 0.375rem 0.875rem;
    border-radius: 9999px;
    border: 1.5px solid var(--outline-variant);
    background-color: transparent;
    color: var(--on-surface-variant);
    font-family: var(--font-body);
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    transition: background-color var(--motion-standard), color var(--motion-standard), border-color var(--motion-standard);
  }

  .filter-chip:hover {
    border-color: var(--primary);
    color: var(--primary);
  }

  .filter-chip--active {
    background-color: var(--primary);
    color: var(--surface-card);
    border-color: var(--primary);
  }

  .empty-state {
    color: var(--on-surface-variant);
  }

  .list {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .item-link {
    text-decoration: none;
    display: block;
  }

  .item-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
  }

  .item-title {
    font-weight: 600;
    color: var(--on-surface);
  }

  .item-description {
    margin-top: 0.5rem;
    font-size: 0.875rem;
    color: var(--on-surface-variant);
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
</style>
