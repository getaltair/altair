<script lang="ts">
  interface Props {
    children: import('svelte').Snippet;
  }

  let { children }: Props = $props();
</script>

<div class="card">
  {@render children()}
</div>

<style>
  .card {
    background-color: var(--surface-container);
    border-radius: 1rem;
    padding: 1.5rem;
  }
</style>
