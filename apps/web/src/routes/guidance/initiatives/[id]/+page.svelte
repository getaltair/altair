<script lang="ts">
  import Card from '$lib/components/primitives/Card.svelte';
  import Badge from '$lib/components/primitives/Badge.svelte';
  import Button from '$lib/components/primitives/Button.svelte';
  import { initiativeById } from '$lib/repositories/initiative.svelte';
  import { epicsByInitiative } from '$lib/repositories/epic.svelte';
  import { questsByEpic } from '$lib/repositories/quest.svelte';
  import { getSyncClient } from '$lib/sync';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  const initiative = $derived(initiativeById(data.id));
  const epics = $derived(initiative ? epicsByInitiative(initiative.id) : []);

  // Inline quest creation state
  let creatingForEpicId = $state<string | null>(null);
  let newQuestTitle = $state('');
  let creating = $state(false);

  async function createQuest(epicId: string) {
    if (!newQuestTitle.trim() || !initiative) return;
    creating = true;
    try {
      const client = getSyncClient();
      const id = crypto.randomUUID();
      const now = new Date().toISOString();
      await client.execute(
        `INSERT INTO quests (id, title, status, epic_id, initiative_id, user_id, created_at, updated_at)
         VALUES (?, ?, 'not_started', ?, ?, (SELECT user_id FROM initiatives WHERE id = ? LIMIT 1), ?, ?)`,
        [id, newQuestTitle.trim(), epicId, initiative.id, initiative.id, now, now]
      );
      newQuestTitle = '';
      creatingForEpicId = null;
    } finally {
      creating = false;
    }
  }
</script>

<div class="page">
  <nav class="breadcrumb">
    <a href="/guidance/initiatives" class="breadcrumb__link">Initiatives</a>
    <span class="breadcrumb__sep">/</span>
    <span class="breadcrumb__current">{initiative?.title ?? 'Initiative'}</span>
  </nav>

  {#if !initiative}
    <p class="empty-state">Initiative not found.</p>
  {:else}
    <header class="page__header">
      <div>
        <h1 class="page__title">{initiative.title}</h1>
        {#if initiative.description}
          <p class="page__description">{initiative.description}</p>
        {/if}
      </div>
      <Badge label={initiative.status} />
    </header>

    <section class="epics">
      {#if epics.length === 0}
        <p class="empty-state">No epics yet. Add epics to organise quests.</p>
      {:else}
        {#each epics as epic (epic.id)}
          {@const quests = questsByEpic(epic.id)}
          <div class="epic">
            <div class="epic__header">
              <a href="/guidance/epics/{epic.id}" class="epic__title">{epic.title}</a>
              <Badge label={epic.status} />
            </div>

            <ul class="quest-list">
              {#each quests as quest (quest.id)}
                <li>
                  <a href="/guidance/quests/{quest.id}" class="quest-link">
                    <span class="quest-title">{quest.title}</span>
                    <span class="quest-status">{quest.status}</span>
                  </a>
                </li>
              {/each}
            </ul>

            {#if creatingForEpicId === epic.id}
              <div class="create-quest">
                <input
                  class="quest-input"
                  type="text"
                  placeholder="Quest title"
                  bind:value={newQuestTitle}
                />
                <div class="create-quest__actions">
                  <Button
                    disabled={creating || !newQuestTitle.trim()}
                    onclick={() => createQuest(epic.id)}
                  >
                    {creating ? 'Adding…' : 'Add Quest'}
                  </Button>
                  <Button
                    variant="ghost"
                    onclick={() => {
                      creatingForEpicId = null;
                      newQuestTitle = '';
                    }}
                  >
                    Cancel
                  </Button>
                </div>
              </div>
            {:else}
              <button
                class="add-quest-btn"
                onclick={() => {
                  creatingForEpicId = epic.id;
                  newQuestTitle = '';
                }}
              >
                + Add Quest
              </button>
            {/if}
          </div>
        {/each}
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

  .empty-state {
    color: var(--on-surface-variant);
  }

  .epics {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .epic {
    background-color: var(--surface-container);
    border-radius: 1rem;
    padding: 1.25rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .epic__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
  }

  .epic__title {
    font-family: var(--font-display);
    font-weight: 600;
    color: var(--primary);
    text-decoration: none;
  }

  .epic__title:hover {
    text-decoration: underline;
  }

  .quest-list {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
    padding-left: 0.5rem;
  }

  .quest-link {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    text-decoration: none;
    padding: 0.375rem 0.5rem;
    border-radius: 0.5rem;
    transition: background-color var(--motion-standard);
  }

  .quest-link:hover {
    background-color: var(--surface-elevated);
  }

  .quest-title {
    color: var(--on-surface);
    font-size: 0.9375rem;
  }

  .quest-status {
    font-size: 0.8125rem;
    color: var(--on-surface-variant);
    text-transform: capitalize;
  }

  .create-quest {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .create-quest__actions {
    display: flex;
    gap: 0.5rem;
  }

  .add-quest-btn {
    background: none;
    border: none;
    color: var(--primary);
    font-family: var(--font-body);
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    padding: 0.25rem 0;
    text-align: left;
    transition: opacity var(--motion-standard);
  }

  .add-quest-btn:hover {
    opacity: 0.75;
  }

  .quest-input {
    background-color: var(--surface-zone);
    border: 1.5px solid transparent;
    border-radius: 0.5rem;
    padding: 0.625rem 0.875rem;
    font-family: var(--font-body);
    font-size: 0.9375rem;
    color: var(--on-surface);
    outline: none;
    width: 100%;
    transition: background-color var(--motion-standard), border-color var(--motion-standard);
  }

  .quest-input:focus {
    background-color: var(--surface-card);
    border-color: var(--primary);
  }
</style>
