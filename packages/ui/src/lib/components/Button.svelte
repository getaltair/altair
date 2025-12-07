<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    variant?: 'primary' | 'secondary' | 'ghost';
    size?: 'sm' | 'md' | 'lg';
    disabled?: boolean;
    onclick?: () => void;
    children: Snippet;
  }

  let { variant = 'primary', size = 'md', disabled = false, onclick, children }: Props = $props();
</script>

<button class="btn btn-{variant} btn-{size}" {disabled} {onclick}>
  {@render children()}
</button>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: 0.375rem;
    font-weight: 500;
    transition: all 150ms ease;
    cursor: pointer;
    border: none;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-sm {
    padding: 0.25rem 0.75rem;
    font-size: 0.875rem;
  }

  .btn-md {
    padding: 0.5rem 1rem;
    font-size: 1rem;
  }

  .btn-lg {
    padding: 0.75rem 1.5rem;
    font-size: 1.125rem;
  }

  .btn-primary {
    background-color: var(--color-primary, #3b82f6);
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background-color: var(--color-primary-hover, #2563eb);
  }

  .btn-secondary {
    background-color: var(--color-secondary, #6b7280);
    color: white;
  }

  .btn-secondary:hover:not(:disabled) {
    background-color: var(--color-secondary-hover, #4b5563);
  }

  .btn-ghost {
    background-color: transparent;
    color: var(--color-text, #374151);
  }

  .btn-ghost:hover:not(:disabled) {
    background-color: var(--color-ghost-hover, #f3f4f6);
  }
</style>
