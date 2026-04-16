<script lang="ts">
  import { allNotes } from '$lib/repositories/note.svelte.js';
  import Card from '$lib/components/primitives/Card.svelte';

  const notes = allNotes();

  let searchQuery = $state('');
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
  let debouncedQuery = $state('');

  function handleSearchInput(e: Event) {
    const value = (e.target as HTMLInputElement).value;
    searchQuery = value;
    if (debounceTimer !== null) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      debouncedQuery = value;
    }, 300);
  }

  const filteredNotes = $derived(
    debouncedQuery.trim() === ''
      ? notes
      : notes.filter((n) => {
          const q = debouncedQuery.toLowerCase();
          return n.title.toLowerCase().includes(q) || n.content.toLowerCase().includes(q);
        }),
  );

  function formatDate(iso: string): string {
    return new Date(iso).toLocaleDateString(undefined, {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
  }
</script>

<svelte:head>
  <title>Notes — Altair</title>
</svelte:head>

<div class="page">
  <header class="page-header">
    <h1>Notes</h1>
    <input
      class="search-input"
      type="search"
      placeholder="Search notes…"
      value={searchQuery}
      oninput={handleSearchInput}
      aria-label="Search notes"
    />
  </header>

  {#if filteredNotes.length === 0}
    <p class="empty">No notes found.</p>
  {:else}
    <ul class="notes-list">
      {#each filteredNotes as note (note.id)}
        <li>
          <a href="/knowledge/{note.id}" class="note-link">
            <Card>
              <div class="note-card-body">
                <span class="note-title">{note.title}</span>
                <span class="note-date">{formatDate(note.updated_at)}</span>
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
    padding: 2rem;
    max-width: 56rem;
    margin: 0 auto;
  }

  .page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
  }

  h1 {
    font-family: var(--font-display);
    font-size: 1.75rem;
    font-weight: 700;
    color: var(--on-surface);
    margin: 0;
  }

  .search-input {
    flex: 1;
    min-width: 12rem;
    max-width: 20rem;
    padding: 0.5rem 0.875rem;
    background-color: var(--surface-zone);
    border: 1.5px solid transparent;
    border-radius: 0.5rem;
    font-family: var(--font-body);
    font-size: 0.9375rem;
    color: var(--on-surface);
    outline: none;
    transition: border-color var(--motion-standard);
  }

  .search-input:focus {
    border-color: var(--primary);
  }

  .notes-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .note-link {
    display: block;
    text-decoration: none;
  }

  .note-card-body {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 1rem;
  }

  .note-title {
    font-family: var(--font-body);
    font-size: 1rem;
    font-weight: 600;
    color: var(--on-surface);
  }

  .note-date {
    font-family: var(--font-body);
    font-size: 0.8125rem;
    color: var(--on-surface-variant);
    white-space: nowrap;
    flex-shrink: 0;
  }

  .empty {
    font-family: var(--font-body);
    color: var(--on-surface-variant);
    text-align: center;
    margin-top: 3rem;
  }
</style>
