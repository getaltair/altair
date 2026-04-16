<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    children?: Snippet;
  }

  let { children }: Props = $props();
</script>

<header class="topbar">
  {#if children}
    {@render children()}
  {/if}
</header>

<style>
  .topbar {
    height: 56px;
    background-color: var(--surface);
    display: flex;
    align-items: center;
    padding: 0 1.5rem;
    flex-shrink: 0;
  }
</style>
