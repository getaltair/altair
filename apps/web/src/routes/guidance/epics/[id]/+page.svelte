<script lang="ts">
  import Card from '$lib/components/primitives/Card.svelte';
  import Badge from '$lib/components/primitives/Badge.svelte';
  import { epicById } from '$lib/repositories/epic.svelte';
  import { questsByEpic } from '$lib/repositories/quest.svelte';

  let { data }: { data: { id: string } } = $props();

  const epic = $derived(epicById(data.id));
  const quests = $derived(epic ? questsByEpic(epic.id) : []);
</script>

<div class="page">
  <nav class="breadcrumb">
    {#if epic}
      <a href="/guidance/initiatives/{epic.initiative_id}" class="breadcrumb__link">Initiative</a>
      <span class="breadcrumb__sep">/</span>
    {/if}
    <span class="breadcrumb__current">{epic?.title ?? 'Epic'}</span>
  </nav>

  {#if !epic}
    <p class="empty-state">Epic not found.</p>
  {:else}
    <header class="page__header">
      <div>
        <h1 class="page__title">{epic.title}</h1>
        {#if epic.description}
          <p class="page__description">{epic.description}</p>
        {/if}
      </div>
      <Badge label={epic.status} />
    </header>

    <section>
      <h2 class="section-title">Quests</h2>
      {#if quests.length === 0}
        <p class="empty-state">No quests in this epic yet.</p>
      {:else}
        <ul class="quest-list">
          {#each quests as quest (quest.id)}
            <li>
              <a href="/guidance/quests/{quest.id}" class="quest-link">
                <Card>
                  <div class="quest-row">
                    <span class="quest-title">{quest.title}</span>
                    <Badge label={quest.status} />
                  </div>
                  {#if quest.description}
                    <p class="quest-description">{quest.description}</p>
                  {/if}
                </Card>
              </a>
            </li>
          {/each}
        </ul>
      {/if}
    </section>
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

  .breadcrumb {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    font-size: 0.875rem;
  }

  .breadcrumb__link {
    color: var(--primary);
    text-decoration: none;
  }

  .breadcrumb__link:hover {
    text-decoration: underline;
  }

  .breadcrumb__sep {
    color: var(--outline-variant);
  }

  .breadcrumb__current {
    color: var(--on-surface-variant);
  }

  .page__header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 1rem;
  }

  .page__title {
    font-family: var(--font-display);
    font-size: 1.75rem;
    font-weight: 700;
    color: var(--on-surface);
  }

  .page__description {
    margin-top: 0.5rem;
    color: var(--on-surface-variant);
    font-size: 0.9375rem;
  }

  .section-title {
    font-family: var(--font-display);
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--on-surface);
    margin-bottom: 0.75rem;
  }

  .empty-state {
    color: var(--on-surface-variant);
  }

  .quest-list {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .quest-link {
    text-decoration: none;
    display: block;
  }

  .quest-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
  }

  .quest-title {
    font-weight: 600;
    color: var(--on-surface);
  }

  .quest-description {
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
