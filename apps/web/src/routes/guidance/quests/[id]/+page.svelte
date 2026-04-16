<script lang="ts">
  import Badge from '$lib/components/primitives/Badge.svelte';
  import Button from '$lib/components/primitives/Button.svelte';
  import { questById } from '$lib/repositories/quest.svelte';
  import { validNextStatuses, type QuestStatus } from '$lib/utils/quest-transitions';
  import { getSyncClient } from '$lib/sync';

  let { data }: { data: { id: string } } = $props();

  const quest = $derived(questById(data.id));
  const nextStatuses = $derived(
    quest ? validNextStatuses(quest.status as QuestStatus) : []
  );

  let transitioning = $state(false);

  async function applyTransition(newStatus: QuestStatus) {
    if (!quest) return;
    transitioning = true;
    try {
      const client = getSyncClient();
      const now = new Date().toISOString();
      await client.execute(
        'UPDATE quests SET status = ?, updated_at = ? WHERE id = ?',
        [newStatus, now, quest.id]
      );
    } finally {
      transitioning = false;
    }
  }

  const STATUS_LABELS: Record<QuestStatus, string> = {
    not_started: 'Not Started',
    in_progress: 'In Progress',
    deferred: 'Deferred',
    completed: 'Complete',
    cancelled: 'Cancel',
  };
</script>

<div class="page">
  <nav class="breadcrumb">
    <a href="/guidance/initiatives" class="breadcrumb__link">Guidance</a>
    <span class="breadcrumb__sep">/</span>
    {#if quest?.epic_id}
      <a href="/guidance/epics/{quest.epic_id}" class="breadcrumb__link">Epic</a>
      <span class="breadcrumb__sep">/</span>
    {/if}
    <span class="breadcrumb__current">{quest?.title ?? 'Quest'}</span>
  </nav>

  {#if !quest}
    <p class="empty-state">Quest not found.</p>
  {:else}
    <header class="page__header">
      <div class="page__title-group">
        <h1 class="page__title">{quest.title}</h1>
        <Badge label={quest.status.replace(/_/g, ' ')} />
      </div>
      <a href="/guidance/quests/{quest.id}/focus" class="focus-link">
        <Button variant="secondary">Start Focus Session</Button>
      </a>
    </header>

    {#if quest.description}
      <p class="page__description">{quest.description}</p>
    {/if}

    {#if nextStatuses.length > 0}
      <section class="transitions">
        <h2 class="section-title">Move to</h2>
        <div class="transition-buttons">
          {#each nextStatuses as status}
            <Button
              variant={status === 'completed' ? 'primary' : status === 'cancelled' ? 'ghost' : 'secondary'}
              disabled={transitioning}
              onclick={() => applyTransition(status)}
            >
              {STATUS_LABELS[status]}
            </Button>
          {/each}
        </div>
      </section>
    {/if}

    <section class="meta">
      <dl class="meta-list">
        {#if quest.priority}
          <div class="meta-row">
            <dt>Priority</dt>
            <dd>{quest.priority}</dd>
          </div>
        {/if}
        {#if quest.due_date}
          <div class="meta-row">
            <dt>Due</dt>
            <dd>{quest.due_date}</dd>
          </div>
        {/if}
        <div class="meta-row">
          <dt>Created</dt>
          <dd>{new Date(quest.created_at).toLocaleDateString()}</dd>
        </div>
        <div class="meta-row">
          <dt>Updated</dt>
          <dd>{new Date(quest.updated_at).toLocaleDateString()}</dd>
        </div>
      </dl>
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
    flex-wrap: wrap;
  }

  .page__title-group {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  .page__title {
    font-family: var(--font-display);
    font-size: 1.75rem;
    font-weight: 700;
    color: var(--on-surface);
  }

  .focus-link {
    text-decoration: none;
  }

  .page__description {
    color: var(--on-surface-variant);
    font-size: 0.9375rem;
    line-height: 1.6;
  }

  .empty-state {
    color: var(--on-surface-variant);
  }

  .section-title {
    font-family: var(--font-display);
    font-size: 1rem;
    font-weight: 600;
    color: var(--on-surface-variant);
    margin-bottom: 0.75rem;
  }

  .transitions {
    background-color: var(--surface-container);
    border-radius: 1rem;
    padding: 1.25rem;
  }

  .transition-buttons {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .meta {
    background-color: var(--surface-container);
    border-radius: 1rem;
    padding: 1.25rem;
  }

  .meta-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .meta-row {
    display: flex;
    gap: 1rem;
    align-items: baseline;
  }

  .meta-row dt {
    font-size: 0.8125rem;
    font-weight: 600;
    color: var(--on-surface-variant);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    min-width: 5rem;
  }

  .meta-row dd {
    color: var(--on-surface);
    font-size: 0.9375rem;
    text-transform: capitalize;
  }
</style>
