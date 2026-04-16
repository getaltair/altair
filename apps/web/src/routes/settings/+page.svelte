<script lang="ts">
  import Input from '$lib/components/primitives/Input.svelte';
  import { enhance } from '$app/forms';

  interface PageData {
    displayName: string;
    email: string;
  }

  interface FormResult {
    updateSuccess?: boolean;
    updateError?: string;
    passwordSuccess?: boolean;
    passwordError?: string;
  }

  let { data, form }: { data: PageData; form: FormResult | null } = $props();

  // These local state variables are intentionally initialized from the server
  // load data and then edited by the user independently (mutable form inputs).
  // svelte-ignore state_referenced_locally
  let displayName = $state(data.displayName);
  // svelte-ignore state_referenced_locally
  let email = $state(data.email);
  let oldPassword = $state('');
  let newPassword = $state('');
  let confirmPassword = $state('');
</script>

<svelte:head>
  <title>Settings — Altair</title>
</svelte:head>

<div class="page">
  <h1>Settings</h1>

  <!-- Profile Section -->
  <section class="section">
    <h2>Profile</h2>

    {#if form?.updateSuccess}
      <p class="message message--success" role="status">Profile updated successfully.</p>
    {/if}
    {#if form?.updateError}
      <p class="message message--error" role="alert">{form.updateError}</p>
    {/if}

    <form method="POST" action="?/updateProfile" use:enhance class="form">
      <Input label="Display Name" bind:value={displayName} id="display_name" />
      <input type="hidden" name="display_name" value={displayName} />

      <Input label="Email" type="email" bind:value={email} id="email" />
      <input type="hidden" name="email" value={email} />

      <button type="submit" class="btn-primary">Save Profile</button>
    </form>
  </section>

  <!-- Password Section -->
  <section class="section">
    <h2>Change Password</h2>

    {#if form?.passwordSuccess}
      <p class="message message--success" role="status">Password changed successfully.</p>
    {/if}
    {#if form?.passwordError}
      <p class="message message--error" role="alert">{form.passwordError}</p>
    {/if}

    <form method="POST" action="?/changePassword" use:enhance class="form">
      <Input
        label="Current Password"
        type="password"
        bind:value={oldPassword}
        id="old_password"
      />
      <input type="hidden" name="old_password" value={oldPassword} />

      <Input
        label="New Password"
        type="password"
        bind:value={newPassword}
        id="new_password"
      />
      <input type="hidden" name="new_password" value={newPassword} />

      <Input
        label="Confirm New Password"
        type="password"
        bind:value={confirmPassword}
        id="confirm_password"
      />
      <input type="hidden" name="confirm_password" value={confirmPassword} />

      <button type="submit" class="btn-primary">Change Password</button>
    </form>
  </section>

  <!-- Notifications Section -->
  <section class="section">
    <h2>Notification Preferences</h2>
    <p class="coming-soon">Coming soon</p>
  </section>

  <!-- Logout Section -->
  <section class="section section--danger">
    <h2>Session</h2>
    <form method="POST" action="?/logout">
      <button type="submit" class="btn-ghost-danger">Sign Out</button>
    </form>
  </section>
</div>

<style>
  .page {
    max-width: 40rem;
    margin: 0 auto;
    padding: 2rem 1.5rem;
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

  .section {
    background-color: var(--surface-container);
    border-radius: 1rem;
    padding: 1.5rem;
  }

  .section--danger {
    border: 1px solid color-mix(in srgb, var(--error) 25%, transparent);
  }

  .form {
    display: flex;
    flex-direction: column;
    gap: 0.875rem;
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
    margin-top: 0.25rem;
  }

  .btn-primary:hover {
    opacity: 0.88;
  }

  .btn-ghost-danger {
    padding: 0.625rem 1.25rem;
    border-radius: 9999px;
    border: none;
    cursor: pointer;
    background-color: transparent;
    color: var(--error);
    font-family: var(--font-body);
    font-size: 0.9375rem;
    font-weight: 600;
    transition: background-color var(--motion-standard);
  }

  .btn-ghost-danger:hover {
    background-color: color-mix(in srgb, var(--error) 10%, transparent);
  }

  .message {
    font-family: var(--font-body);
    font-size: 0.875rem;
    padding: 0.625rem 0.875rem;
    border-radius: 0.5rem;
    margin-bottom: 0.75rem;
  }

  .message--success {
    background-color: color-mix(in srgb, var(--primary) 12%, transparent);
    color: var(--primary);
  }

  .message--error {
    background-color: color-mix(in srgb, var(--error) 12%, transparent);
    color: var(--error);
  }

  .coming-soon {
    font-family: var(--font-body);
    font-size: 0.9375rem;
    color: var(--on-surface-variant);
    margin: 0;
  }
</style>
