<script lang="ts">
	import { initTheme } from '$lib/stores/theme.svelte';
	import AppShell from '$lib/components/layout/AppShell.svelte';

	let { children } = $props();

	// Initialize theme from localStorage / OS preference on the client.
	// This wires up the $effect that keeps the DOM class and localStorage in sync.
	initTheme();
</script>

<AppShell>
	{@render children()}
</AppShell>
