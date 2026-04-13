<script lang="ts">
    import type { ActionData } from './$types';

    let { form }: { form: ActionData } = $props();
</script>

<svelte:head>
    <title>Sign In — Altair</title>
</svelte:head>

<div class="login-container">
    <div class="login-card">
        <h1>Sign In</h1>

        {#if form?.error}
            <div class="error-message" role="alert">
                {form.error}
            </div>
        {/if}

        <form method="POST">
            <div class="field">
                <label for="email">Email</label>
                <input
                    id="email"
                    name="email"
                    type="email"
                    autocomplete="email"
                    required
                />
            </div>

            <div class="field">
                <label for="password">Password</label>
                <input
                    id="password"
                    name="password"
                    type="password"
                    autocomplete="current-password"
                    required
                />
            </div>

            <button type="submit">Sign In</button>
        </form>

        <p class="register-link">
            Don't have an account? <a href="/auth/register">Create one</a>
        </p>
    </div>
</div>

<style>
    .login-container {
        display: flex;
        align-items: center;
        justify-content: center;
        min-height: 100vh;
        background: var(--color-background, #0a0a0f);
    }

    .login-card {
        width: 100%;
        max-width: 400px;
        padding: 2rem;
        background: var(--color-surface, #12121a);
        border-radius: var(--radius-lg, 12px);
        border: 1px solid var(--color-border, rgba(255, 255, 255, 0.08));
    }

    h1 {
        font-size: 1.5rem;
        font-weight: 600;
        margin-bottom: 1.5rem;
        color: var(--color-text-primary, #f0f0f5);
    }

    .error-message {
        padding: 0.75rem 1rem;
        background: var(--color-error-subtle, rgba(220, 38, 38, 0.1));
        border: 1px solid var(--color-error, rgba(220, 38, 38, 0.3));
        border-radius: var(--radius-md, 8px);
        color: var(--color-error-text, #fca5a5);
        font-size: 0.875rem;
        margin-bottom: 1rem;
    }

    .field {
        display: flex;
        flex-direction: column;
        gap: 0.375rem;
        margin-bottom: 1rem;
    }

    label {
        font-size: 0.875rem;
        font-weight: 500;
        color: var(--color-text-secondary, rgba(240, 240, 245, 0.7));
    }

    input {
        padding: 0.625rem 0.75rem;
        background: var(--color-input-bg, rgba(255, 255, 255, 0.05));
        border: 1px solid var(--color-border, rgba(255, 255, 255, 0.08));
        border-radius: var(--radius-sm, 6px);
        color: var(--color-text-primary, #f0f0f5);
        font-size: 0.9375rem;
    }

    input:focus {
        outline: none;
        border-color: var(--color-accent, #6d5dfc);
    }

    button {
        width: 100%;
        padding: 0.625rem;
        background: var(--color-accent, #6d5dfc);
        color: white;
        border: none;
        border-radius: var(--radius-sm, 6px);
        font-size: 0.9375rem;
        font-weight: 500;
        cursor: pointer;
        margin-top: 0.5rem;
    }

    button:hover {
        opacity: 0.9;
    }

    .register-link {
        text-align: center;
        margin-top: 1rem;
        font-size: 0.875rem;
        color: var(--color-text-secondary, rgba(240, 240, 245, 0.7));
    }

    .register-link a {
        color: var(--color-accent, #6d5dfc);
    }
</style>
