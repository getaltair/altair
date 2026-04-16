<script lang="ts">
  import { goto } from '$app/navigation';
  import { getSyncClient } from '$lib/sync/index.js';

  let title = $state('');
  let submitting = $state(false);
  let error = $state('');

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!title.trim()) return;

    submitting = true;
    error = '';

    try {
      const db = getSyncClient();
      const id = crypto.randomUUID();
      const now = new Date().toISOString();

      // Retrieve the current user_id from an existing note (PowerSync sets this via sync rules).
      // Fall back to a placeholder that will be corrected on next sync.
      const userResult = await db.execute(
        `SELECT user_id FROM notes WHERE deleted_at IS NULL LIMIT 1`,
        [],
      );
      const userId: string = (userResult.rows?._array?.[0] as { user_id: string } | undefined)?.user_id ?? 'pending';

      await db.execute(
        `INSERT INTO notes (id, title, content, user_id, initiative_id, created_at, updated_at, deleted_at)
         VALUES (?, ?, '', ?, NULL, ?, ?, NULL)`,
        [id, title.trim(), userId, now, now],
      );

      await goto(`/knowledge/${id}`);
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to create note.';
      submitting = false;
    }
  }

  function handleCancel() {
    goto('/knowledge');
  }
</script>

<svelte:head>
  <title>New Note — Altair</title>
</svelte:head>

<div class="page">
  <h1 class="page-title">New Note</h1>

  {#if error}
    <p class="error" role="alert">{error}</p>
  {/if}

  <form class="form" onsubmit={handleSubmit}>
    <label class="label" for="note-title">Title</label>
    <input
      id="note-title"
      class="input"
      type="text"
      bind:value={title}
      placeholder="Note title…"
      required
      aria-required="true"
    />

    <div class="actions">
      <button type="button" class="btn-cancel" onclick={handleCancel} disabled={submitting}>
        Cancel
      </button>
      <button type="submit" class="btn-submit" disabled={submitting || !title.trim()}>
        {submitting ? 'Creating…' : 'Create Note'}
      </button>
    </div>
  </form>
</div>

<style>
  .page {
    padding: 2rem;
    max-width: 36rem;
    margin: 0 auto;
  }

  .page-title {
    font-family: var(--font-display);
    font-size: 1.75rem;
    font-weight: 700;
    color: var(--on-surface);
    margin: 0 0 1.5rem;
  }

  .error {
    font-family: var(--font-body);
    font-size: 0.875rem;
    color: var(--error, #b00020);
    margin: 0 0 1rem;
  }

  .form {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .label {
    font-family: var(--font-body);
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--on-surface-variant);
  }

  .input {
    width: 100%;
    padding: 0.625rem 0.875rem;
    background-color: var(--surface-zone);
    border: 1.5px solid transparent;
    border-radius: 0.5rem;
    font-family: var(--font-body);
    font-size: 1rem;
    color: var(--on-surface);
    outline: none;
    box-sizing: border-box;
    transition: border-color var(--motion-standard);
  }

  .input:focus {
    border-color: var(--primary);
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    margin-top: 0.5rem;
  }

  .btn-cancel {
    padding: 0.5rem 1.25rem;
    background: none;
    border: 1.5px solid var(--outline-variant, var(--primary-container));
    border-radius: 0.5rem;
    font-family: var(--font-body);
    font-size: 0.9375rem;
    color: var(--on-surface);
    cursor: pointer;
    transition: background-color var(--motion-standard);
  }

  .btn-cancel:hover:not(:disabled) {
    background-color: var(--surface-zone);
  }

  .btn-cancel:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-submit {
    padding: 0.5rem 1.25rem;
    background-color: var(--primary);
    border: none;
    border-radius: 0.5rem;
    font-family: var(--font-body);
    font-size: 0.9375rem;
    font-weight: 600;
    color: var(--on-primary, #fff);
    cursor: pointer;
    transition: opacity var(--motion-standard);
  }

  .btn-submit:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
