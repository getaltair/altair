<script lang="ts">
	import type { SyncConnectionStatus } from '$lib/stores/sync.svelte.js';

	interface Props {
		status?: SyncConnectionStatus;
		class?: string;
	}

	let { status = 'connected', class: className = '' }: Props = $props();

	const visible = $derived(
		status === 'disconnected' || status === 'error' || status === 'connecting'
	);

	const barClasses = $derived(
		status === 'connecting'
			? 'bg-primary text-white'
			: status === 'error'
				? 'bg-error text-white'
				: 'bg-warm-terracotta text-white'
	);

	const message = $derived(
		status === 'connecting'
			? 'Syncing...'
			: status === 'error'
				? 'Sync error -- your data may not be saved'
				: 'Offline -- changes will sync when reconnected'
	);
</script>

{#if visible}
	<div
		class="transition-breathe px-4 py-1 text-center font-body text-xs {barClasses} {className}"
		role="status"
		aria-live="polite"
	>
		{message}
	</div>
{/if}
