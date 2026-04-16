<script lang="ts">
  interface Household {
    id: string;
    name: string;
    member_count: number;
    created_at: string;
  }

  let households: Household[] = $state([]);
  let loading = $state(true);
  let error: string | null = $state(null);

  $effect(() => {
    (async () => {
      try {
        const res = await fetch('/api/admin/households');
        if (!res.ok) {
          error = res.status === 404
            ? 'Household management endpoint not yet available.'
            : 'Failed to load households.';
          return;
        }
        const data = await res.json() as Household[];
        households = data;
      } catch {
        error = 'Failed to load households.';
      } finally {
        loading = false;
      }
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
  <title>Households — Admin — Altair</title>
</svelte:head>

<div class="page">
  <h1>Households</h1>

  {#if loading}
    <p class="state-text">Loading…</p>
  {:else if error}
    <p class="state-text state-text--muted">{error}</p>
  {:else if households.length === 0}
    <p class="state-text state-text--muted">No households found.</p>
  {:else}
    <div class="table-wrapper">
      <table>
        <thead>
          <tr>
            <th>Name</th>
            <th>Members</th>
            <th>Created</th>
          </tr>
        </thead>
        <tbody>
          {#each households as household (household.id)}
            <tr>
              <td>{household.name}</td>
              <td>{household.member_count}</td>
              <td>{formatDate(household.created_at)}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
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

  .state-text {
    font-family: var(--font-body);
    font-size: 0.9375rem;
    color: var(--on-surface);
  }

  .state-text--muted {
    color: var(--on-surface-variant);
  }

  .table-wrapper {
    overflow-x: auto;
    background-color: var(--surface-container);
    border-radius: 1rem;
    padding: 1.5rem;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    font-family: var(--font-body);
    font-size: 0.9375rem;
  }

  th {
    text-align: left;
    padding: 0.625rem 0.875rem;
    font-size: 0.8125rem;
    font-weight: 600;
    color: var(--on-surface-variant);
    border-bottom: 1px solid var(--surface-zone);
    white-space: nowrap;
  }

  td {
    padding: 0.75rem 0.875rem;
    color: var(--on-surface);
    border-bottom: 1px solid var(--surface-zone);
  }
</style>
