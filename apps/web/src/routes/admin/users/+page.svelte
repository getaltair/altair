<script lang="ts">
  interface User {
    id: string;
    email: string;
    display_name: string;
    status: string;
    is_admin: boolean;
  }

  let users: User[] = $state([]);
  let loading = $state(true);
  let error: string | null = $state(null);

  let inviteEmail = $state('');
  let inviteDisplayName = $state('');
  let inviteError: string | null = $state(null);
  let inviteSuccess: string | null = $state(null);
  let inviting = $state(false);

  $effect(() => {
    (async () => {
      try {
        const res = await fetch('/api/admin/users');
        if (!res.ok) {
          error = res.status === 404 ? 'User management endpoint not yet available.' : 'Failed to load users.';
          return;
        }
        const data = await res.json() as User[];
        users = data;
      } catch {
        error = 'Failed to load users.';
      } finally {
        loading = false;
      }
    })();
  });

  async function handleDeactivate(userId: string) {
    try {
      const res = await fetch(`/api/admin/users/${userId}/deactivate`, { method: 'POST' });
      if (res.ok) {
        users = users.map((u) =>
          u.id === userId ? { ...u, status: 'inactive' } : u,
        );
      }
    } catch {
      // silently ignore — no network connection
    }
  }

  async function handleInvite(e: SubmitEvent) {
    e.preventDefault();
    inviteError = null;
    inviteSuccess = null;
    inviting = true;

    try {
      const res = await fetch('/api/admin/users', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email: inviteEmail, display_name: inviteDisplayName }),
      });

      if (res.ok) {
        inviteSuccess = `Invitation sent to ${inviteEmail}.`;
        inviteEmail = '';
        inviteDisplayName = '';
        const newUser = await res.json() as User;
        users = [...users, newUser];
      } else if (res.status === 404) {
        inviteError = 'Invite endpoint not yet available.';
      } else {
        const data = await res.json() as { error?: string };
        inviteError = data.error ?? 'Failed to send invite.';
      }
    } catch {
      inviteError = 'Failed to send invite.';
    } finally {
      inviting = false;
    }
  }
</script>

<svelte:head>
  <title>Users — Admin — Altair</title>
</svelte:head>

<div class="page">
  <h1>Users</h1>

  <section class="invite-section">
    <h2>Invite User</h2>
    {#if inviteError}
      <p class="message message--error" role="alert">{inviteError}</p>
    {/if}
    {#if inviteSuccess}
      <p class="message message--success" role="status">{inviteSuccess}</p>
    {/if}
    <form onsubmit={handleInvite} class="invite-form">
      <div class="field">
        <label for="invite-email">Email</label>
        <input
          id="invite-email"
          type="email"
          bind:value={inviteEmail}
          required
          autocomplete="off"
          class="input"
        />
      </div>
      <div class="field">
        <label for="invite-name">Display Name</label>
        <input
          id="invite-name"
          type="text"
          bind:value={inviteDisplayName}
          required
          autocomplete="off"
          class="input"
        />
      </div>
      <button type="submit" class="btn-primary" disabled={inviting}>
        {inviting ? 'Sending…' : 'Send Invite'}
      </button>
    </form>
  </section>

  <section class="users-section">
    <h2>All Users</h2>
    {#if loading}
      <p class="state-text">Loading…</p>
    {:else if error}
      <p class="state-text state-text--muted">{error}</p>
    {:else if users.length === 0}
      <p class="state-text state-text--muted">No users found.</p>
    {:else}
      <div class="table-wrapper">
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>Email</th>
              <th>Status</th>
              <th>Role</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each users as user (user.id)}
              <tr>
                <td>{user.display_name}</td>
                <td>{user.email}</td>
                <td>
                  <span class="badge badge--{user.status}">{user.status}</span>
                </td>
                <td>{user.is_admin ? 'Admin' : 'User'}</td>
                <td>
                  {#if user.status !== 'inactive'}
                    <button
                      class="btn-danger"
                      onclick={() => handleDeactivate(user.id)}
                    >
                      Deactivate
                    </button>
                  {/if}
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </section>
</div>

<style>
  .page {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  h1 {
    font-family: var(--font-display);
    font-size: 1.75rem;
    font-weight: 700;
    color: var(--on-surface);
    margin: 0;
  }

  h2 {
    font-family: var(--font-display);
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--on-surface);
    margin: 0 0 1rem;
  }

  .invite-section,
  .users-section {
    background-color: var(--surface-container);
    border-radius: 1rem;
    padding: 1.5rem;
  }

  .invite-form {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    max-width: 28rem;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  label {
    font-family: var(--font-body);
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--on-surface-variant);
  }

  .input {
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

  .input:focus {
    border-color: var(--primary);
  }

  .btn-primary {
    align-self: flex-start;
    padding: 0.625rem 1.25rem;
    border-radius: 9999px;
    border: none;
    cursor: pointer;
    background-color: var(--primary);
    color: var(--surface-card);
    font-family: var(--font-body);
    font-size: 0.9375rem;
    font-weight: 600;
    transition: opacity var(--motion-standard);
  }

  .btn-primary:disabled {
    opacity: 0.38;
    cursor: not-allowed;
  }

  .btn-danger {
    padding: 0.375rem 0.875rem;
    border-radius: 9999px;
    border: none;
    cursor: pointer;
    background-color: var(--error-container, color-mix(in srgb, var(--error) 15%, transparent));
    color: var(--error);
    font-family: var(--font-body);
    font-size: 0.875rem;
    font-weight: 500;
    transition: opacity var(--motion-standard);
  }

  .btn-danger:hover {
    opacity: 0.8;
  }

  .message {
    font-family: var(--font-body);
    font-size: 0.875rem;
    padding: 0.625rem 0.875rem;
    border-radius: 0.5rem;
    margin-bottom: 0.75rem;
  }

  .message--error {
    background-color: color-mix(in srgb, var(--error) 12%, transparent);
    color: var(--error);
  }

  .message--success {
    background-color: color-mix(in srgb, var(--primary) 12%, transparent);
    color: var(--primary);
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

  .badge {
    display: inline-block;
    padding: 0.2rem 0.625rem;
    border-radius: 9999px;
    font-size: 0.8125rem;
    font-weight: 500;
  }

  .badge--active {
    background-color: color-mix(in srgb, var(--primary) 15%, transparent);
    color: var(--primary);
  }

  .badge--pending {
    background-color: color-mix(in srgb, var(--on-surface-variant) 15%, transparent);
    color: var(--on-surface-variant);
  }

  .badge--inactive {
    background-color: color-mix(in srgb, var(--error) 12%, transparent);
    color: var(--error);
  }
</style>
