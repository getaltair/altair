<script lang="ts">
  interface PageData {
    db: string;
    powersync: string;
    storage?: string;
  }

  let { data }: { data: PageData } = $props();

  function statusClass(value: string): string {
    if (value === 'ok' || value === 'healthy') return 'status--ok';
    if (value === 'unknown') return 'status--unknown';
    return 'status--error';
  }
</script>

<svelte:head>
  <title>Health — Admin — Altair</title>
</svelte:head>

<div class="page">
  <h1>System Health</h1>

  <div class="health-grid">
    <div class="health-card">
      <p class="health-card__label">Database</p>
      <p class="health-card__value {statusClass(data.db)}">{data.db}</p>
    </div>

    <div class="health-card">
      <p class="health-card__label">PowerSync</p>
      <p class="health-card__value {statusClass(data.powersync)}">{data.powersync}</p>
    </div>

    {#if data.storage !== undefined}
      <div class="health-card">
        <p class="health-card__label">Storage</p>
        <p class="health-card__value {statusClass(data.storage)}">{data.storage}</p>
      </div>
    {/if}
  </div>
</div>

<style>
  .page {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  h1 {
    font-family: var(--font-display);
    font-size: 1.75rem;
    font-weight: 700;
    color: var(--on-surface);
    margin: 0;
  }

  .health-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(14rem, 1fr));
    gap: 1rem;
  }

  .health-card {
    background-color: var(--surface-container);
    border-radius: 1rem;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .health-card__label {
    font-family: var(--font-body);
    font-size: 0.8125rem;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--on-surface-variant);
    margin: 0;
  }

  .health-card__value {
    font-family: var(--font-body);
    font-size: 1.125rem;
    font-weight: 700;
    margin: 0;
    text-transform: capitalize;
  }

  .status--ok {
    color: var(--primary);
  }

  .status--unknown {
    color: var(--on-surface-variant);
  }

  .status--error {
    color: var(--error);
  }
</style>
