<script lang="ts">
  import { page } from '$app/state';
  import { getSyncClient } from '$lib/sync';
  import { itemById } from '$lib/repositories/item.svelte';
  import { eventsForItem } from '$lib/repositories/item-event.svelte';
  import Button from '$lib/components/primitives/Button.svelte';

  const id = $derived(page.params.id!);
  const itemRepo = $derived(itemById(id));
  const item = $derived(itemRepo.item);
  const eventRepo = $derived(eventsForItem(id));
  const events = $derived(eventRepo.events);

  // ---- Consumption form state ----
  let consumptionInput = $state('');
  let consumptionError = $state('');
  let submitting = $state(false);

  function validateConsumption(): number | null {
    const amount = Number(consumptionInput);
    if (!consumptionInput || isNaN(amount) || amount <= 0) {
      consumptionError = 'Enter a positive quantity to consume.';
      return null;
    }
    if (!item) {
      consumptionError = 'Item not found.';
      return null;
    }
    if (item.quantity - amount < 0) {
      consumptionError = `Cannot consume ${amount} — only ${item.quantity} in stock.`;
      return null;
    }
    consumptionError = '';
    return amount;
  }

  async function logConsumption() {
    const amount = validateConsumption();
    if (amount === null) return;
    if (!item) return;

    submitting = true;
    try {
      const client = getSyncClient();
      const now = new Date().toISOString();
      const eventId = crypto.randomUUID();

      await client.execute(
        `INSERT INTO tracking_item_events
          (id, item_id, event_type, quantity_change, from_location_id, to_location_id, notes, occurred_at, created_at)
         VALUES (?, ?, 'consumption', ?, NULL, NULL, NULL, ?, ?)`,
        [eventId, item.id, -amount, now, now],
      );

      await client.execute(
        `UPDATE tracking_items SET quantity = ?, updated_at = ? WHERE id = ?`,
        [item.quantity - amount, now, item.id],
      );

      consumptionInput = '';
    } catch (err) {
      consumptionError = 'Failed to log consumption. Please try again.';
      console.error('logConsumption error:', err);
    } finally {
      submitting = false;
    }
  }

  // ---- Helpers ----
  function formatDate(iso: string): string {
    return new Date(iso).toLocaleString();
  }

  function eventLabel(type: string): string {
    const labels: Record<string, string> = {
      consumption: 'Consumed',
      restock: 'Restocked',
      transfer: 'Transferred',
      adjustment: 'Adjusted',
    };
    return labels[type] ?? type;
  }

  function eventQtyClass(change: number): string {
    return change < 0 ? 'qty-negative' : 'qty-positive';
  }
</script>

<div class="page">
  {#if !item}
    <p class="loading">Loading…</p>
  {:else}
    <header class="page-header">
      <a href="/tracking" class="back-link">← Inventory</a>
      <h1 class="page-title">{item.name}</h1>
    </header>

    <!-- Item metadata -->
    <div class="meta-grid">
      <div class="meta-item">
        <span class="meta-label">Quantity</span>
        <span class="meta-value" class:qty-low={item.quantity <= 0}>{item.quantity}</span>
      </div>
      {#if item.description}
        <div class="meta-item">
          <span class="meta-label">Description</span>
          <span class="meta-value">{item.description}</span>
        </div>
      {/if}
      {#if item.expires_at}
        <div class="meta-item">
          <span class="meta-label">Expires</span>
          <span class="meta-value">{formatDate(item.expires_at)}</span>
        </div>
      {/if}
    </div>

    <!-- Consumption logging form -->
    <section class="section">
      <h2 class="section-title">Log consumption</h2>
      <div class="consume-form">
        <input
          class="qty-input"
          class:error-state={!!consumptionError}
          type="number"
          min="0.01"
          step="any"
          placeholder="Quantity consumed"
          bind:value={consumptionInput}
          aria-label="Quantity consumed"
          aria-describedby={consumptionError ? 'consume-error' : undefined}
          aria-invalid={consumptionError ? true : undefined}
        />
        <Button variant="primary" disabled={submitting} onclick={logConsumption}>
          Log
        </Button>
      </div>
      {#if consumptionError}
        <p id="consume-error" class="inline-error" role="alert">{consumptionError}</p>
      {/if}
    </section>

    <!-- Event timeline -->
    <section class="section">
      <h2 class="section-title">Event history</h2>
      {#if events.length === 0}
        <p class="empty-state">No events recorded yet.</p>
      {:else}
        <ol class="timeline" reversed>
          {#each [...events].reverse() as event}
            <li class="timeline-item">
              <span class="event-label">{eventLabel(event.event_type)}</span>
              <span class="event-qty {eventQtyClass(event.quantity_change)}">
                {event.quantity_change > 0 ? '+' : ''}{event.quantity_change}
              </span>
              <span class="event-date">{formatDate(event.occurred_at)}</span>
              {#if event.notes}
                <span class="event-notes">{event.notes}</span>
              {/if}
            </li>
          {/each}
        </ol>
      {/if}
    </section>
  {/if}
</div>

<style>
  .page {
    max-width: 720px;
    margin: 0 auto;
  }

  .loading {
    color: var(--on-surface-variant);
    font-family: var(--font-body);
    padding: 2rem 0;
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

  .meta-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 1.25rem;
    margin-bottom: 2rem;
    padding: 1.25rem;
    border-radius: 1rem;
    background-color: var(--surface-container);
  }

  .meta-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .meta-label {
    font-family: var(--font-body);
    font-size: 0.75rem;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--on-surface-variant);
  }

  .meta-value {
    font-family: var(--font-body);
    font-size: 1rem;
    color: var(--on-surface);
  }

  .meta-value.qty-low {
    color: #9f403d;
    font-weight: 700;
  }

  .section {
    margin-bottom: 2rem;
  }

  .section-title {
    font-family: var(--font-body);
    font-size: 1.0625rem;
    font-weight: 600;
    color: var(--on-surface);
    margin: 0 0 0.75rem;
  }

  .consume-form {
    display: flex;
    gap: 0.75rem;
    align-items: flex-start;
  }

  .qty-input {
    flex: 1;
    max-width: 180px;
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

  .qty-input:focus {
    border-color: var(--primary);
  }

  .qty-input.error-state {
    border-color: var(--error);
  }

  .inline-error {
    margin-top: 0.5rem;
    font-family: var(--font-body);
    font-size: 0.8125rem;
    color: var(--error);
  }

  .empty-state {
    color: var(--on-surface-variant);
    font-family: var(--font-body);
  }

  .timeline {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .timeline-item {
    display: grid;
    grid-template-columns: auto auto 1fr;
    gap: 0.5rem 1rem;
    align-items: baseline;
    padding: 0.75rem 1rem;
    border-radius: 0.75rem;
    background-color: var(--surface-container);
  }

  .event-label {
    font-family: var(--font-body);
    font-size: 0.9375rem;
    font-weight: 600;
    color: var(--on-surface);
  }

  .event-qty {
    font-family: var(--font-body);
    font-size: 0.9375rem;
    font-weight: 700;
  }

  .qty-negative {
    color: #9f403d;
  }

  .qty-positive {
    color: var(--primary);
  }

  .event-date {
    font-family: var(--font-body);
    font-size: 0.8125rem;
    color: var(--on-surface-variant);
    text-align: right;
  }

  .event-notes {
    grid-column: 1 / -1;
    font-family: var(--font-body);
    font-size: 0.875rem;
    color: var(--on-surface-variant);
    font-style: italic;
  }
</style>
