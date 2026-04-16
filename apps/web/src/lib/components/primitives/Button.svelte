<script lang="ts">
  interface Props {
    variant?: 'primary' | 'secondary' | 'ghost';
    disabled?: boolean;
    onclick?: () => void;
    children?: import('svelte').Snippet;
  }

  let {
    variant = 'primary',
    disabled = false,
    onclick,
    children,
  }: Props = $props();
</script>

<button
  class="btn btn-{variant}"
  {disabled}
  onclick={onclick}
>
  {#if children}
    {@render children()}
  {/if}
</button>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.625rem 1.25rem;
    border-radius: 9999px;
    border: none;
    cursor: pointer;
    font-family: var(--font-body);
    font-size: 0.9375rem;
    font-weight: 600;
    line-height: 1;
    transition: background-color var(--motion-standard), color var(--motion-standard), opacity var(--motion-standard);
    text-decoration: none;
  }

  .btn:disabled {
    opacity: 0.38;
    cursor: not-allowed;
  }

  .btn-primary {
    background-color: var(--primary);
    color: var(--surface-card);
  }

  .btn-primary:hover:not(:disabled) {
    background-color: color-mix(in srgb, var(--primary) 85%, var(--surface-card));
  }

  .btn-secondary {
    background-color: var(--primary-container);
    color: var(--primary);
  }

  .btn-secondary:hover:not(:disabled) {
    background-color: color-mix(in srgb, var(--primary-container) 85%, var(--primary));
  }

  .btn-ghost {
    background-color: transparent;
    color: var(--primary);
  }

  .btn-ghost:hover:not(:disabled) {
    background-color: var(--surface-zone);
  }
</style>
