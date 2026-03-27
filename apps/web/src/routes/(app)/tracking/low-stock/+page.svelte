<script lang="ts">
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import { syncStore } from '$lib/stores/sync.svelte';
	import type { TrackingItem } from '$lib/types/tracking.js';
	import ItemCard from '$lib/components/tracking/ItemCard.svelte';
	import { BackLink, Card, EmptyState } from '$lib/components/ui/index.js';

	let lowStockItems = $state<TrackingItem[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		error = null;
		try {
			// Get householdId from all items as fallback
			const allItems = await syncStore.queryItems();
			const householdId = allItems[0]?.household_id ?? '';
			if (householdId) {
				lowStockItems = await syncStore.queryLowStockItems(householdId);
			} else {
				// Fallback: filter client-side if no household
				lowStockItems = allItems.filter(
					(i) => i.min_quantity != null && i.quantity < i.min_quantity
				);
			}
		} catch (err) {
			console.error('[low-stock] Failed to load:', err);
			error = 'Could not load data. Please try again.';
		}
		loading = false;
	});
</script>

<svelte:head>
	<title>Low Stock - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<BackLink href="/tracking" label="Tracking" />

	<!-- Header -->
	<div class="mb-6">
		<h1 class="font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
			Low Stock
		</h1>
	</div>

	<!-- Item count -->
	{#if !loading && !error}
		<p class="mb-4 font-body text-xs text-outline-variant dark:text-on-surface-muted">
			{lowStockItems.length} item{lowStockItems.length !== 1 ? 's' : ''} below threshold
		</p>
	{/if}

	<!-- Low stock items -->
	{#if loading}
		<div class="space-y-3">
			{#each [0, 1, 2] as i (i)}
				<div class="h-20 animate-pulse rounded-2xl bg-surface-low dark:bg-surface-high"></div>
			{/each}
		</div>
	{:else if error}
		<Card>
			<div class="flex flex-col items-center gap-3 py-6 text-center">
				<span class="material-symbols-outlined text-3xl text-error">cloud_off</span>
				<p class="font-body text-sm text-on-surface-muted">{error}</p>
			</div>
		</Card>
	{:else if lowStockItems.length === 0}
		<EmptyState
			title="All stocked up!"
			description="No items are below their minimum quantity. Everything looks good."
			icon="check_circle"
		/>
	{:else}
		<div class="space-y-3" role="list">
			{#each lowStockItems as item (item.id)}
				<a href={resolve(`/tracking/items/${item.id}` as '/')} class="block">
					<ItemCard {item} />
				</a>
			{/each}
		</div>
	{/if}
</main>
