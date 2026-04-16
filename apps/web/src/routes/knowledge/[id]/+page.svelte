<script lang="ts">
  import { page } from '$app/state';
  import { noteById, searchNotes, backlinksFor } from '$lib/repositories/note.svelte.js';
  import { allTags, tagsForEntity } from '$lib/repositories/tag.svelte.js';
  import { getSyncClient } from '$lib/sync/index.js';
  import { RelationType } from '$lib/contracts/relationTypes.js';
  import { EntityType } from '$lib/contracts/entityTypes.js';
  import { detectLinkTrigger } from '$lib/utils/note-link-trigger.js';
  import Tag from '$lib/components/primitives/Tag.svelte';
  import type { Note } from '$lib/repositories/note.svelte.js';

  // ----------------------------------------------------------------
  // Route param
  // ----------------------------------------------------------------
  const noteId = $derived(page.params.id!);

  // ----------------------------------------------------------------
  // Repository data (reactive)
  // ----------------------------------------------------------------
  const note = $derived(noteById(noteId));
  const tags = $derived(tagsForEntity(noteId, EntityType.KnowledgeNote));
  const allTagsList = allTags();
  const backlinks = $derived(backlinksFor(noteId));

  // ----------------------------------------------------------------
  // Snapshots — direct watch since no repository function exists
  // ----------------------------------------------------------------
  interface NoteSnapshot {
    id: string;
    note_id: string;
    content: string;
    created_at: string;
  }

  let snapshots: NoteSnapshot[] = $state([]);

  $effect(() => {
    const db = getSyncClient();
    const controller = new AbortController();
    const currentNoteId = noteId;

    (async () => {
      const stream = db.watch(
        `SELECT * FROM note_snapshots WHERE note_id = ? ORDER BY created_at DESC`,
        [currentNoteId],
        { signal: controller.signal },
      );
      for await (const result of stream) {
        snapshots = (result.rows?._array ?? []) as NoteSnapshot[];
      }
    })();

    return () => controller.abort();
  });

  // ----------------------------------------------------------------
  // Editor state
  // ----------------------------------------------------------------
  let editorContent = $state('');

  $effect(() => {
    // Initialise content when note loads or changes
    if (note) {
      editorContent = note.content;
    }
  });

  async function handleBlur() {
    if (!note) return;
    const db = getSyncClient();
    await db.execute(
      `UPDATE notes SET content = ?, updated_at = ? WHERE id = ?`,
      [editorContent, new Date().toISOString(), note.id],
    );
  }

  // ----------------------------------------------------------------
  // [[  link trigger — dropdown
  // ----------------------------------------------------------------
  let triggerQuery = $state('');
  let triggerStart = $state(-1);
  let dropdownVisible = $state(false);

  // Search results when trigger is active — uses a one-shot query
  // to avoid the multi-instantiation lifecycle issue with searchNotes.
  let triggerResults: Note[] = $state([]);

  async function refreshTriggerResults(q: string) {
    const db = getSyncClient();
    const pattern = `%${q}%`;
    const result = await db.execute(
      `SELECT * FROM notes WHERE deleted_at IS NULL AND (title LIKE ? OR content LIKE ?) ORDER BY updated_at DESC LIMIT 10`,
      [pattern, pattern],
    );
    triggerResults = (result.rows?._array ?? []) as Note[];
  }

  function handleTextareaInput(e: Event) {
    const ta = e.target as HTMLTextAreaElement;
    editorContent = ta.value;

    const trigger = detectLinkTrigger(ta.value, ta.selectionStart);
    if (trigger) {
      triggerQuery = trigger.query;
      triggerStart = trigger.triggerStart;
      dropdownVisible = true;
      refreshTriggerResults(trigger.query);
    } else {
      dropdownVisible = false;
    }
  }

  async function selectLinkedNote(linkedNote: Note, textarea: HTMLTextAreaElement) {
    if (!note) return;

    // Build the insertion text
    const insertion = `[[${linkedNote.title}|${linkedNote.id}]]`;
    const before = textarea.value.slice(0, triggerStart);
    const after = textarea.value.slice(textarea.selectionStart);
    const newValue = before + insertion + after;

    editorContent = newValue;
    textarea.value = newValue;
    const cursor = before.length + insertion.length;
    textarea.setSelectionRange(cursor, cursor);

    dropdownVisible = false;

    // Write relation to entity_relations
    const db = getSyncClient();
    const now = new Date().toISOString();
    await db.execute(
      `INSERT INTO entity_relations (id, from_entity_id, from_entity_type, to_entity_id, to_entity_type, relation_type, user_id, created_at, updated_at)
       VALUES (?, ?, ?, ?, ?, ?, (SELECT user_id FROM notes WHERE id = ? LIMIT 1), ?, ?)`,
      [
        crypto.randomUUID(),
        note.id,
        EntityType.KnowledgeNote,
        linkedNote.id,
        EntityType.KnowledgeNote,
        RelationType.NoteLink,
        note.id,
        now,
        now,
      ],
    );
  }

  // ----------------------------------------------------------------
  // Tag management
  // ----------------------------------------------------------------
  let tagInput = $state('');
  let tagDropdownVisible = $state(false);

  const filteredTagSuggestions = $derived(
    tagInput.trim() === ''
      ? allTagsList
      : allTagsList.filter((t) =>
          t.name.toLowerCase().includes(tagInput.toLowerCase()),
        ),
  );

  async function addTag(tagId: string) {
    if (!note) return;
    const db = getSyncClient();
    const now = new Date().toISOString();
    await db.execute(
      `INSERT OR IGNORE INTO entity_tags (id, entity_id, entity_type, tag_id, created_at)
       VALUES (?, ?, ?, ?, ?)`,
      [crypto.randomUUID(), note.id, EntityType.KnowledgeNote, tagId, now],
    );
    tagInput = '';
    tagDropdownVisible = false;
  }

  async function removeTag(tagId: string) {
    if (!note) return;
    const db = getSyncClient();
    await db.execute(
      `DELETE FROM entity_tags WHERE entity_id = ? AND tag_id = ?`,
      [note.id, tagId],
    );
  }

  // Close dropdowns on outside click
  function handleWindowClick() {
    tagDropdownVisible = false;
  }

  // ----------------------------------------------------------------
  // Backlink note titles — resolve from entity relations
  // ----------------------------------------------------------------
  // backlinks gives EntityRelation[] where from_entity_id is the linking note.
  // We need to resolve titles. We'll do a reactive watch per backlink (simple approach
  // since backlinks are typically few).
  let backlinkNotes: Note[] = $state([]);

  $effect(() => {
    if (backlinks.length === 0) {
      backlinkNotes = [];
      return;
    }
    const db = getSyncClient();
    const ids = backlinks.map((r) => r.from_entity_id);
    const placeholders = ids.map(() => '?').join(', ');
    db.execute(
      `SELECT * FROM notes WHERE id IN (${placeholders}) AND deleted_at IS NULL`,
      ids,
    ).then((result) => {
      backlinkNotes = (result.rows?._array ?? []) as Note[];
    });
  });

  // ----------------------------------------------------------------
  // Helpers
  // ----------------------------------------------------------------
  function formatDate(iso: string): string {
    return new Date(iso).toLocaleDateString(undefined, {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
  }
</script>

<svelte:head>
  <title>{note?.title ?? 'Note'} — Altair</title>
</svelte:head>

<div
  class="page"
  role="presentation"
  onclick={handleWindowClick}
  onkeydown={(e) => { if (e.key === 'Escape') handleWindowClick(); }}
>
  {#if !note}
    <p class="loading">Loading…</p>
  {:else}
    <h1 class="note-title">{note.title}</h1>

    <div class="columns">
      <!-- ============================================================
           Wide column: editor + backlinks
      ============================================================ -->
      <div class="editor-column">
        <div class="editor-wrap">
          <textarea
            class="editor"
            value={editorContent}
            oninput={handleTextareaInput}
            onblur={handleBlur}
            aria-label="Note content"
            rows={20}
          ></textarea>

          {#if dropdownVisible && triggerResults.length > 0}
            <!-- Absolutely positioned dropdown below the textarea -->
            <ul class="link-dropdown" role="listbox" aria-label="Link suggestions">
              {#each triggerResults as result (result.id)}
                <li role="option" aria-selected="false">
                  <button
                    type="button"
                    class="link-option"
                    onmousedown={(e) => {
                      e.preventDefault();
                      const ta = document.querySelector<HTMLTextAreaElement>('.editor');
                      if (ta) selectLinkedNote(result, ta);
                    }}
                  >
                    {result.title}
                  </button>
                </li>
              {/each}
            </ul>
          {/if}
        </div>

        <!-- Backlinks -->
        {#if backlinkNotes.length > 0}
          <section class="backlinks" aria-label="Backlinks">
            <h2 class="section-heading">Backlinks</h2>
            <div class="backlink-chips">
              {#each backlinkNotes as bl (bl.id)}
                <a href="/knowledge/{bl.id}" class="backlink-chip">{bl.title}</a>
              {/each}
            </div>
          </section>
        {/if}
      </div>

      <!-- ============================================================
           Narrow column: metadata
      ============================================================ -->
      <aside class="metadata-panel">
        <!-- Tags -->
        <section class="meta-section">
          <h2 class="section-heading">Tags</h2>
          <div class="tag-chips">
            {#each tags as tag (tag.id)}
              <Tag label={tag.name} onremove={() => removeTag(tag.id)} />
            {/each}
          </div>
          <!-- Tag autocomplete input -->
          <div class="tag-input-wrap">
            <input
              class="tag-input"
              type="text"
              role="combobox"
              placeholder="Add tag…"
              bind:value={tagInput}
              onfocus={() => { tagDropdownVisible = true; }}
              oninput={() => { tagDropdownVisible = true; }}
              onclick={(e) => e.stopPropagation()}
              aria-label="Add tag"
              aria-haspopup="listbox"
              aria-expanded={tagDropdownVisible}
              aria-controls="tag-suggestions"
              aria-autocomplete="list"
              autocomplete="off"
            />
            {#if tagDropdownVisible && filteredTagSuggestions.length > 0}
              <ul
                id="tag-suggestions"
                class="tag-dropdown"
                role="listbox"
                aria-label="Tag suggestions"
                onclick={(e) => e.stopPropagation()}
                onkeydown={(e) => e.stopPropagation()}
              >
                {#each filteredTagSuggestions as suggestion (suggestion.id)}
                  <li role="option" aria-selected="false">
                    <button
                      type="button"
                      class="tag-option"
                      onmousedown={(e) => {
                        e.preventDefault();
                        addTag(suggestion.id);
                      }}
                    >
                      {suggestion.name}
                    </button>
                  </li>
                {/each}
              </ul>
            {/if}
          </div>
        </section>

        <!-- Snapshot history (view-only — invariant E-6) -->
        <section class="meta-section">
          <h2 class="section-heading">History</h2>
          {#if snapshots.length === 0}
            <p class="empty-meta">No snapshots yet.</p>
          {:else}
            <ul class="snapshot-list" aria-label="Note snapshots">
              {#each snapshots as snap (snap.id)}
                <li class="snapshot-item">
                  <span class="snapshot-date">{formatDate(snap.created_at)}</span>
                  <p class="snapshot-preview">{snap.content.slice(0, 80)}{snap.content.length > 80 ? '…' : ''}</p>
                </li>
              {/each}
            </ul>
          {/if}
        </section>
      </aside>
    </div>
  {/if}
</div>

<style>
  .page {
    padding: 2rem;
    max-width: 72rem;
    margin: 0 auto;
  }

  .loading {
    font-family: var(--font-body);
    color: var(--on-surface-variant);
    text-align: center;
    margin-top: 4rem;
  }

  .note-title {
    font-family: var(--font-display);
    font-size: 2rem;
    font-weight: 700;
    color: var(--on-surface);
    margin: 0 0 1.5rem;
  }

  /* Two-column layout */
  .columns {
    display: grid;
    grid-template-columns: 1fr 20rem;
    gap: 2rem;
    align-items: start;
  }

  @media (max-width: 768px) {
    .columns {
      grid-template-columns: 1fr;
    }
  }

  /* Editor */
  .editor-column {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .editor-wrap {
    position: relative;
  }

  .editor {
    width: 100%;
    min-height: 20rem;
    padding: 1rem;
    background-color: var(--surface-zone);
    border: 1.5px solid transparent;
    border-radius: 0.75rem;
    font-family: var(--font-body);
    font-size: 0.9375rem;
    color: var(--on-surface);
    line-height: 1.6;
    resize: vertical;
    outline: none;
    transition: border-color var(--motion-standard);
    box-sizing: border-box;
  }

  .editor:focus {
    border-color: var(--primary);
  }

  /* [[ dropdown */
  .link-dropdown {
    position: absolute;
    top: calc(100% + 0.25rem);
    left: 0;
    right: 0;
    z-index: 50;
    list-style: none;
    margin: 0;
    padding: 0.25rem 0;
    background-color: var(--surface-container);
    border: 1px solid var(--outline-variant, var(--primary-container));
    border-radius: 0.5rem;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
    max-height: 14rem;
    overflow-y: auto;
  }

  .link-option {
    display: block;
    width: 100%;
    padding: 0.5rem 0.875rem;
    background: none;
    border: none;
    text-align: left;
    font-family: var(--font-body);
    font-size: 0.9375rem;
    color: var(--on-surface);
    cursor: pointer;
  }

  .link-option:hover {
    background-color: var(--surface-zone);
  }

  /* Backlinks */
  .backlinks {
    margin-top: 0.5rem;
  }

  .backlink-chips {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-top: 0.5rem;
  }

  .backlink-chip {
    display: inline-flex;
    align-items: center;
    padding: 0.25rem 0.75rem;
    border-radius: 9999px;
    background-color: var(--surface-zone);
    color: var(--primary);
    font-family: var(--font-body);
    font-size: 0.8125rem;
    font-weight: 500;
    text-decoration: none;
    transition: background-color var(--motion-standard);
  }

  .backlink-chip:hover {
    background-color: var(--primary-container);
  }

  /* Metadata panel */
  .metadata-panel {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .meta-section {
    background-color: var(--surface-container);
    border-radius: 1rem;
    padding: 1rem 1.25rem;
  }

  .section-heading {
    font-family: var(--font-body);
    font-size: 0.8125rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--on-surface-variant);
    margin: 0 0 0.75rem;
  }

  /* Tag chips */
  .tag-chips {
    display: flex;
    flex-wrap: wrap;
    gap: 0.375rem;
    margin-bottom: 0.75rem;
  }

  /* Tag input */
  .tag-input-wrap {
    position: relative;
  }

  .tag-input {
    width: 100%;
    padding: 0.375rem 0.625rem;
    background-color: var(--surface-zone);
    border: 1.5px solid transparent;
    border-radius: 0.5rem;
    font-family: var(--font-body);
    font-size: 0.875rem;
    color: var(--on-surface);
    outline: none;
    box-sizing: border-box;
    transition: border-color var(--motion-standard);
  }

  .tag-input:focus {
    border-color: var(--primary);
  }

  .tag-dropdown {
    position: absolute;
    top: calc(100% + 0.25rem);
    left: 0;
    right: 0;
    z-index: 50;
    list-style: none;
    margin: 0;
    padding: 0.25rem 0;
    background-color: var(--surface-container);
    border: 1px solid var(--outline-variant, var(--primary-container));
    border-radius: 0.5rem;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
    max-height: 12rem;
    overflow-y: auto;
  }

  .tag-option {
    display: block;
    width: 100%;
    padding: 0.375rem 0.75rem;
    background: none;
    border: none;
    text-align: left;
    font-family: var(--font-body);
    font-size: 0.875rem;
    color: var(--on-surface);
    cursor: pointer;
  }

  .tag-option:hover {
    background-color: var(--surface-zone);
  }

  /* Snapshots */
  .snapshot-list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .snapshot-item {
    border-left: 2px solid var(--primary-container);
    padding-left: 0.625rem;
  }

  .snapshot-date {
    font-family: var(--font-body);
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--on-surface-variant);
  }

  .snapshot-preview {
    font-family: var(--font-body);
    font-size: 0.8125rem;
    color: var(--on-surface);
    margin: 0.25rem 0 0;
    line-height: 1.4;
  }

  .empty-meta {
    font-family: var(--font-body);
    font-size: 0.875rem;
    color: var(--on-surface-variant);
    margin: 0;
  }
</style>
