<script lang="ts">
  import { goto } from '$app/navigation';
  import { getSyncClient } from '$lib/sync';
  import Input from '$lib/components/primitives/Input.svelte';
  import Button from '$lib/components/primitives/Button.svelte';

  let title = $state('');
  let description = $state('');
  let dueDate = $state('');

  let titleError = $state('');
  let submitting = $state(false);

  function validate(): boolean {
    if (!title.trim()) {
      titleError = 'Title is required.';
      return false;
    }
    titleError = '';
    return true;
  }

  async function handleSubmit() {
    if (!validate()) return;

    submitting = true;
    try {
      const client = getSyncClient();
      const id = crypto.randomUUID();
      const now = new Date().toISOString();

      await client.execute(
        `INSERT INTO quests (id, title, description, status, due_date, user_id, created_at, updated_at)
         VALUES (?, ?, ?, 'not_started', ?, '', ?, ?)`,
        [id, title.trim(), description.trim() || null, dueDate || null, now, now],
      );

      await goto(`/guidance/quests/${id}`);
    } catch (err) {
      console.error('Quest creation error:', err);
    } finally {
      submitting = false;
    }
  }
</script>

<div class="page">
  <header class="page-header">
    <a href="/guidance" class="back-link">← Guidance</a>
    <h1 class="page-title">New Quest</h1>
  </header>

  <form class="form" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
    <Input bind:value={title} label="Title" error={titleError} />

    <Input bind:value={description} label="Description (optional)" />

    <div class="field">
      <label class="label" for="due-date-input">Due date (optional)</label>
      <input
        id="due-date-input"
        class="date-input"
        type="date"
        bind:value={dueDate}
      />
    </div>

    <div class="form-actions">
      <Button variant="secondary" onclick={() => goto('/guidance')} disabled={submitting}>
        Cancel
      </Button>
      <Button variant="primary" disabled={submitting}>
        {submitting ? 'Creating…' : 'Create Quest'}
      </Button>
    </div>
  </form>
</div>

<style>
  .page {
    max-width: 480px;
    margin: 0 auto;
    padding: 2rem 1.5rem;
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
