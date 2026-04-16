<script lang="ts">
  import Card from '$lib/components/primitives/Card.svelte';
  import Badge from '$lib/components/primitives/Badge.svelte';
  import Button from '$lib/components/primitives/Button.svelte';
  import { allRoutines } from '$lib/repositories/routine.svelte';

  const routines = $derived(allRoutines());

  const FREQUENCY_LABELS: Record<string, string> = {
    daily: 'Daily',
    weekly: 'Weekly',
    monthly: 'Monthly',
    custom: 'Custom',
  };
</script>

<div class="page">
  <header class="page__header">
    <h1 class="page__title">Routines</h1>
    <Button onclick={() => (window.location.href = '/guidance/routines/new')}>
      New Routine
    </Button>
  </header>

  {#if routines.length === 0}
    <p class="empty-state">No routines yet. Create one to build consistent habits.</p>
  {:else}
    <ul class="list">
      {#each routines as routine (routine.id)}
        <li>
          <a href="/guidance/routines/{routine.id}" class="item-link">
            <Card>
              <div class="item-row">
                <div class="item-left">
                  <span class="item-title">{routine.title}</span>
                  {#if routine.description}
                    <p class="item-description">{routine.description}</p>
                  {/if}
                </div>
                <div class="item-right">
                  <Badge label={FREQUENCY_LABELS[routine.frequency_type] ?? routine.frequency_type} />
                  <Badge label={routine.status} />
                </div>
              </div>
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
    align-items: flex-start;
    justify-content: space-between;
    gap: 1rem;
  }

  .item-left {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    flex: 1;
    min-width: 0;
  }

  .item-title {
    font-weight: 600;
    color: var(--on-surface);
  }

  .item-description {
    font-size: 0.875rem;
    color: var(--on-surface-variant);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .item-right {
    display: flex;
    gap: 0.375rem;
    flex-shrink: 0;
    flex-wrap: wrap;
    justify-content: flex-end;
  }
</style>
