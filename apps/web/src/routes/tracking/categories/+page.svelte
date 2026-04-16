<script lang="ts">
  import { getSyncClient } from '$lib/sync';
  import Button from '$lib/components/primitives/Button.svelte';
  import Input from '$lib/components/primitives/Input.svelte';

  interface TrackingCategory {
    id: string;
    name: string;
    household_id: string;
    created_at: string;
    updated_at: string;
    deleted_at: string | null;
  }

  let _categories = $state<TrackingCategory[]>([]);

  $effect(() => {
    const client = getSyncClient();
    let active = true;
    (async () => {
      for await (const result of client.watch(
        'SELECT * FROM tracking_categories WHERE deleted_at IS NULL ORDER BY name ASC',
        [],
      )) {
        if (!active) break;
        _categories = (result.rows?._array ?? []) as TrackingCategory[];
      }
    })();
    return () => { active = false; };
  });

  // ---- Create form ----
  let newName = $state('');
  let newNameError = $state('');
  let creating = $state(false);

  async function createCategory() {
    if (!newName.trim()) {
      newNameError = 'Name is required.';
      return;
    }
    newNameError = '';
    creating = true;
    try {
      const client = getSyncClient();
      const id = crypto.randomUUID();
      const now = new Date().toISOString();
      await client.execute(
        `INSERT INTO tracking_categories (id, name, household_id, created_at, updated_at, deleted_at)
         VALUES (?, ?, '', ?, ?, NULL)`,
        [id, newName.trim(), now, now],
      );
      newName = '';
    } catch (err) {
      console.error('createCategory error:', err);
    } finally {
      creating = false;
    }
  }

  // ---- Edit state ----
  let editingId = $state<string | null>(null);
  let editName = $state('');
  let editError = $state('');
  let saving = $state(false);

  function startEdit(cat: TrackingCategory) {
    editingId = cat.id;
    editName = cat.name;
    editError = '';
  }

  function cancelEdit() {
    editingId = null;
    editName = '';
    editError = '';
  }

  async function saveEdit(cat: TrackingCategory) {
    if (!editName.trim()) {
      editError = 'Name is required.';
      return;
    }
    saving = true;
    try {
      const client = getSyncClient();
      const now = new Date().toISOString();
      await client.execute(
        'UPDATE tracking_categories SET name = ?, updated_at = ? WHERE id = ?',
        [editName.trim(), now, cat.id],
      );
      editingId = null;
    } catch (err) {
      console.error('saveEdit error:', err);
    } finally {
      saving = false;
    }
  }

  // ---- Delete (soft) ----
  let deleting = $state<string | null>(null);

  async function deleteCategory(id: string) {
    deleting = id;
    try {
      const client = getSyncClient();
      const now = new Date().toISOString();
      await client.execute(
        'UPDATE tracking_categories SET deleted_at = ?, updated_at = ? WHERE id = ?',
        [now, now, id],
      );
    } catch (err) {
      console.error('deleteCategory error:', err);
    } finally {
      deleting = null;
    }
  }
</script>

<div class="page">
  <header class="page-header">
    <h1 class="page-title">Categories</h1>
  </header>

  <!-- Create form -->
  <section class="create-section">
    <form
      class="create-form"
      onsubmit={(e) => { e.preventDefault(); createCategory(); }}
    >
      <Input bind:value={newName} label="New category name" error={newNameError} />
      <Button variant="primary" disabled={creating}>Add</Button>
    </form>
  </section>

  <!-- Category list -->
  {#if _categories.length === 0}
    <p class="empty-state">No categories yet. Add one above.</p>
  {:else}
    <ul class="list">
      {#each _categories as cat}
        <li class="list-item">
          {#if editingId === cat.id}
            <form
              class="edit-form"
              onsubmit={(e) => { e.preventDefault(); saveEdit(cat); }}
            >
              <Input bind:value={editName} error={editError} />
              <Button variant="primary" disabled={saving}>Save</Button>
              <Button variant="ghost" onclick={cancelEdit} disabled={saving}>Cancel</Button>
            </form>
          {:else}
            <span class="cat-name">{cat.name}</span>
            <div class="actions">
              <Button variant="ghost" onclick={() => startEdit(cat)}>Edit</Button>
              <Button
                variant="ghost"
                disabled={deleting === cat.id}
                onclick={() => deleteCategory(cat.id)}
              >Delete</Button>
            </div>
          {/if}
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .page {
    max-width: 600px;
    margin: 0 auto;
  }

  .page-header {
    margin-bottom: 1.5rem;
  }

  .page-title {
    font-family: var(--font-display);
    font-size: 2rem;
    font-weight: 700;
    color: var(--on-surface);
    margin: 0;
  }

  .create-section {
    margin-bottom: 2rem;
  }

  .create-form {
    display: flex;
    gap: 0.75rem;
    align-items: flex-end;
  }

  .empty-state {
    color: var(--on-surface-variant);
    font-family: var(--font-body);
    padding: 2rem 0;
  }

  .list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .list-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    border-radius: 0.75rem;
    background-color: var(--surface-container);
  }

  .cat-name {
    font-family: var(--font-body);
    font-size: 0.9375rem;
    color: var(--on-surface);
    font-weight: 500;
    flex: 1;
  }

  .actions {
    display: flex;
    gap: 0.25rem;
  }

  .edit-form {
    display: flex;
    gap: 0.5rem;
    align-items: flex-end;
    width: 100%;
  }
</style>
