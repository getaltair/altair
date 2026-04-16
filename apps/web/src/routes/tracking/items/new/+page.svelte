<script lang="ts">
  import { goto } from '$app/navigation';
  import { getSyncClient } from '$lib/sync';
  import Input from '$lib/components/primitives/Input.svelte';
  import Button from '$lib/components/primitives/Button.svelte';

  // ---- Locations and categories for selects ----
  interface TrackingLocation { id: string; name: string; }
  interface TrackingCategory { id: string; name: string; }

  let _locations = $state<TrackingLocation[]>([]);
  let _categories = $state<TrackingCategory[]>([]);

  $effect(() => {
    const client = getSyncClient();
    let active = true;
    (async () => {
      for await (const result of client.watch(
        'SELECT id, name FROM tracking_locations WHERE deleted_at IS NULL ORDER BY name ASC',
        [],
      )) {
        if (!active) break;
        _locations = (result.rows?._array ?? []) as TrackingLocation[];
      }
    })();
    return () => { active = false; };
  });

  $effect(() => {
    const client = getSyncClient();
    let active = true;
    (async () => {
      for await (const result of client.watch(
        'SELECT id, name FROM tracking_categories WHERE deleted_at IS NULL ORDER BY name ASC',
        [],
      )) {
        if (!active) break;
        _categories = (result.rows?._array ?? []) as TrackingCategory[];
      }
    })();
    return () => { active = false; };
  });

  // ---- Form state ----
  let name = $state('');
  let description = $state('');
  let quantityStr = $state('0');
  let locationId = $state('');
  let categoryId = $state('');
  let expiresAt = $state('');

  let nameError = $state('');
  let quantityError = $state('');
  let submitting = $state(false);

  function validate(): boolean {
    let valid = true;
    if (!name.trim()) {
      nameError = 'Name is required.';
      valid = false;
    } else {
      nameError = '';
    }
    const qty = Number(quantityStr);
    if (isNaN(qty) || qty < 0) {
      quantityError = 'Quantity must be zero or greater.';
      valid = false;
    } else {
      quantityError = '';
    }
    return valid;
  }

  async function handleSubmit() {
    if (!validate()) return;

    submitting = true;
    try {
      const client = getSyncClient();
      const id = crypto.randomUUID();
      const now = new Date().toISOString();

      await client.execute(
        `INSERT INTO tracking_items
          (id, name, description, quantity, barcode, location_id, category_id,
           user_id, household_id, initiative_id, expires_at, created_at, updated_at, deleted_at)
         VALUES (?, ?, ?, ?, NULL, ?, ?, '', NULL, NULL, ?, ?, ?, NULL)`,
        [
          id,
          name.trim(),
          description.trim() || null,
          Number(quantityStr),
          locationId || null,
          categoryId || null,
          expiresAt || null,
          now,
          now,
        ],
      );

      await goto(`/tracking/items/${id}`);
    } catch (err) {
      console.error('Item creation error:', err);
    } finally {
      submitting = false;
    }
  }
</script>

<div class="page">
  <header class="page-header">
    <a href="/tracking" class="back-link">← Inventory</a>
    <h1 class="page-title">New item</h1>
  </header>

  <form class="form" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
    <Input bind:value={name} label="Name" error={nameError} />

    <Input bind:value={description} label="Description (optional)" />

    <Input bind:value={quantityStr} label="Starting quantity" type="number" error={quantityError} />

    <div class="field">
      <label class="label" for="location-select">Location (optional)</label>
      <select id="location-select" class="select" bind:value={locationId}>
        <option value="">None</option>
        {#each _locations as loc}
          <option value={loc.id}>{loc.name}</option>
        {/each}
      </select>
    </div>

    <div class="field">
      <label class="label" for="category-select">Category (optional)</label>
      <select id="category-select" class="select" bind:value={categoryId}>
        <option value="">None</option>
        {#each _categories as cat}
          <option value={cat.id}>{cat.name}</option>
        {/each}
      </select>
    </div>

    <div class="field">
      <label class="label" for="expires-input">Expires (optional)</label>
      <input
        id="expires-input"
        class="date-input"
        type="date"
        bind:value={expiresAt}
      />
    </div>

    <div class="form-actions">
      <Button variant="secondary" onclick={() => history.back()} disabled={submitting}>
        Cancel
      </Button>
      <Button variant="primary" disabled={submitting}>
        Create item
      </Button>
    </div>
  </form>
</div>

<style>
  .page {
    max-width: 480px;
    margin: 0 auto;
  }

  .page-header {
    margin-bottom: 1.5rem;
  }

  .back-link {
    font-family: var(--font-body);
    font-size: 0.875rem;
    color: var(--primary);
    text-decoration: none;
    display: inline-block;
    margin-bottom: 0.5rem;
  }

  .back-link:hover {
    text-decoration: underline;
  }

  .page-title {
    font-family: var(--font-display);
    font-size: 2rem;
    font-weight: 700;
    color: var(--on-surface);
    margin: 0;
  }

  .form {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .label {
    font-family: var(--font-body);
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--on-surface-variant);
  }

  .select,
  .date-input {
    background-color: var(--surface-zone);
    border: 1.5px solid transparent;
    border-radius: 0.5rem;
    padding: 0.625rem 0.875rem;
    font-family: var(--font-body);
    font-size: 0.9375rem;
    color: var(--on-surface);
    outline: none;
    transition: border-color var(--motion-standard);
  }

  .select:focus,
  .date-input:focus {
    border-color: var(--primary);
  }

  .form-actions {
    display: flex;
    gap: 0.75rem;
    justify-content: flex-end;
    padding-top: 0.5rem;
  }
</style>
