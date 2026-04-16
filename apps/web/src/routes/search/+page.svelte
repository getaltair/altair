<script lang="ts">
  import { browser } from '$app/environment';
  import Input from '$lib/components/primitives/Input.svelte';
  import type { Note } from '$lib/repositories/note.svelte.js';

  let query = $state('');
  let results: Note[] = $state([]);

  $effect(() => {
    if (!browser || !query.trim()) {
      results = [];
      return;
    }

    (async () => {
      const { searchNotes } = await import('$lib/repositories/note.svelte.js');
      results = searchNotes(query);
    })();
  });

  function formatDate(iso: string): string {
    return new Date(iso).toLocaleDateString(undefined, {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
  }
</script>

<svelte:head>
  <title>Search — Altair</title>
</svelte:head>

<div class="page">
  <h1>Search</h1>

  <div class="search-box">
    <Input
      label="Search everything..."
      bind:value={query}
      id="global-search"
    />

    <p class="empty-state">Search not yet available.</p>

    {#if query.trim() && results.length > 0}
      <ul class="results-list">
        {#each results as note (note.id)}
          <li>
            <a href="/knowledge/{note.id}" class="result-link">
              <span class="result-title">{note.title}</span>
              <span class="result-date">{formatDate(note.updated_at)}</span>
            </a>
          </li>
        {/each}
      </ul>
    {:else if query.trim() && results.length === 0}
      <p class="no-results">No notes match that filter.</p>
    {/if}
  </div>
</div>

<style>
  .page {
    max-width: 48rem;
    margin: 0 auto;
    padding: 2rem 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  h1 {
    font-family: var(--font-display);
    font-size: 1.75rem;
    font-weight: 700;
    color: var(--on-surface);
    margin: 0;
  }

  .search-box {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .empty-state {
    font-family: var(--font-body);
    font-size: 0.9375rem;
    color: var(--on-surface-variant);
    margin: 0;
  }

  .results-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .result-link {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.625rem 0.75rem;
    border-radius: 0.5rem;
    text-decoration: none;
    background-color: var(--surface-card);
    transition: background-color var(--motion-standard);
  }

  .result-link:hover {
    background-color: var(--surface-zone);
  }

  .result-title {
    font-family: var(--font-body);
    font-size: 1rem;
    font-weight: 600;
    color: var(--on-surface);
  }

  .result-date {
    font-family: var(--font-body);
    font-size: 0.8125rem;
    color: var(--on-surface-variant);
    white-space: nowrap;
    flex-shrink: 0;
  }

  .no-results {
    font-family: var(--font-body);
    font-size: 0.9375rem;
    color: var(--on-surface-variant);
    margin: 0;
  }
</style>
