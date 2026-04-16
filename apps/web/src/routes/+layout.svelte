<script lang="ts">
  import '../app.css';
  import favicon from '$lib/assets/favicon.svg';
  import Shell from '$lib/components/layout/Shell.svelte';
  import { getSyncClient, subscribeToStreams } from '$lib/sync/index.js';
  import type { LayoutData } from './$types';

  let { data, children }: { data: LayoutData; children: import('svelte').Snippet } = $props();

  $effect(() => {
    if (data.isAdmin !== undefined) {
      const client = getSyncClient();
      subscribeToStreams(client).catch((err: unknown) => {
        console.error('PowerSync stream subscription failed:', err);
      });
    }
  });
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
</svelte:head>

<Shell isAdmin={data.isAdmin ?? false}>
  {@render children()}
</Shell>
