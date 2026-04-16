<script lang="ts">
  import { getSyncClient } from '$lib/sync';
  import Button from '$lib/components/primitives/Button.svelte';
  import Input from '$lib/components/primitives/Input.svelte';

  interface TrackingLocation {
    id: string;
    name: string;
    household_id: string;
    created_at: string;
    updated_at: string;
    deleted_at: string | null;
  }

  let _locations = $state<TrackingLocation[]>([]);

  $effect(() => {
    const client = getSyncClient();
    let active = true;
    (async () => {
      for await (const result of client.watch(
        'SELECT * FROM tracking_locations WHERE deleted_at IS NULL ORDER BY name ASC',
        [],
      )) {
        if (!active) break;
        _locations = (result.rows?._array ?? []) as TrackingLocation[];
      }
    })();
    return () => { active = false; };
  });

  // ---- Create form ----
  let newName = $state('');
  let newNameError = $state('');
  let creating = $state(false);

  async function createLocation() {
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
        `INSERT INTO tracking_locations (id, name, household_id, created_at, updated_at, deleted_at)
         VALUES (?, ?, '', ?, ?, NULL)`,
        [id, newName.trim(), now, now],
      );
      newName = '';
    } catch (err) {
      console.error('createLocation error:', err);
    } finally {
      creating = false;
    }
  }

  // ---- Edit state ----
  let editingId = $state<string | null>(null);
  let editName = $state('');
  let editError = $state('');
  let saving = $state(false);

  function startEdit(loc: TrackingLocation) {
    editingId = loc.id;
    editName = loc.name;
    editError = '';
  }

  function cancelEdit() {
    editingId = null;
    editName = '';
    editError = '';
  }

  async function saveEdit(loc: TrackingLocation) {
    if (!editName.trim()) {
      editError = 'Name is required.';
      return;
    }
    saving = true;
    try {
      const client = getSyncClient();
      const now = new Date().toISOString();
      await client.execute(
        'UPDATE tracking_locations SET name = ?, updated_at = ? WHERE id = ?',
        [editName.trim(), now, loc.id],
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

  async function deleteLocation(id: string) {
    deleting = id;
    try {
      const client = getSyncClient();
      const now = new Date().toISOString();
      await client.execute(
        'UPDATE tracking_locations SET deleted_at = ?, updated_at = ? WHERE id = ?',
        [now, now, id],
      );
    } catch (err) {
      console.error('deleteLocation error:', err);
    } finally {
      deleting = null;
    }
  }
</script>

<div class="page">
  <header class="page-header">
    <h1 class="page-title">Locations</h1>
  </header>

  <!-- Create form -->
  <section class="create-section">
    <form
      class="create-form"
      onsubmit={(e) => { e.preventDefault(); createLocation(); }}
    >
      <Input bind:value={newName} label="New location name" error={newNameError} />
      <Button variant="primary" disabled={creating}>Add</Button>
    </form>
  </section>

  <!-- Location list -->
  {#if _locations.length === 0}
    <p class="empty-state">No locations yet. Add one above.</p>
  {:else}
    <ul class="list">
      {#each _locations as loc}
        <li class="list-item">
          {#if editingId === loc.id}
            <form
              class="edit-form"
              onsubmit={(e) => { e.preventDefault(); saveEdit(loc); }}
            >
              <Input bind:value={editName} error={editError} />
              <Button variant="primary" disabled={saving}>Save</Button>
              <Button variant="ghost" onclick={cancelEdit} disabled={saving}>Cancel</Button>
            </form>
          {:else}
            <span class="loc-name">{loc.name}</span>
            <div class="actions">
              <Button variant="ghost" onclick={() => startEdit(loc)}>Edit</Button>
              <Button
                variant="ghost"
                disabled={deleting === loc.id}
                onclick={() => deleteLocation(loc.id)}
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

  .loc-name {
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
