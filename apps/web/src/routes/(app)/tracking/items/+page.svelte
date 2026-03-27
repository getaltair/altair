<script lang="ts">
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import { syncStore } from '$lib/stores/sync.svelte';
	import type { TrackingItem, TrackingLocation, TrackingCategory } from '$lib/types/tracking.js';
	import ItemCard from '$lib/components/tracking/ItemCard.svelte';
	import { BackLink, Card, EmptyState } from '$lib/components/ui/index.js';

	let allItems = $state<TrackingItem[]>([]);
	let locations = $state<TrackingLocation[]>([]);
	let categories = $state<TrackingCategory[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	let selectedLocation = $state<string>('');
	let selectedCategory = $state<string>('');

	const filteredItems = $derived(
		allItems.filter((item) => {
			if (selectedLocation && item.location_id !== selectedLocation) return false;
			if (selectedCategory && item.category_id !== selectedCategory) return false;
			return true;
		})
	);

	onMount(async () => {
		error = null;
		try {
			allItems = await syncStore.queryItems();

			// Load locations and categories for filter dropdowns
			// Use empty string as householdId fallback
			const householdId = allItems[0]?.household_id ?? '';
			if (householdId) {
				locations = await syncStore.queryLocations(householdId);
				categories = await syncStore.queryCategories(householdId);
			}
		} catch (err) {
			console.error('[tracking-items] Failed to load:', err);
			error = 'Could not load data. Please try again.';
		}
		loading = false;
	});
</script>

<svelte:head>
	<title>Items - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<BackLink href="/tracking" label="Tracking" />

	<!-- Header -->
	<div class="mb-6">
		<h1 class="font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
			Items
		</h1>
	</div>

	<!-- Filter dropdowns -->
	{#if !loading && (locations.length > 0 || categories.length > 0)}
		<div class="mb-4 flex gap-3">
			{#if locations.length > 0}
				<select
					bind:value={selectedLocation}
					aria-label="Filter by location"
					class="transition-breathe rounded-xl bg-surface-low px-3 py-2 font-body text-sm text-on-surface focus:bg-pure-white focus:ring-2 focus:ring-primary/20 focus:outline-none dark:bg-[#1e2d2f] dark:text-[var(--text-primary)] dark:focus:bg-[var(--card-bg)]"
				>
					<option value="">All locations</option>
					{#each locations as loc (loc.id)}
						<option value={loc.id}>{loc.name}</option>
					{/each}
				</select>
			{/if}
			{#if categories.length > 0}
				<select
					bind:value={selectedCategory}
					aria-label="Filter by category"
					class="transition-breathe rounded-xl bg-surface-low px-3 py-2 font-body text-sm text-on-surface focus:bg-pure-white focus:ring-2 focus:ring-primary/20 focus:outline-none dark:bg-[#1e2d2f] dark:text-[var(--text-primary)] dark:focus:bg-[var(--card-bg)]"
				>
					<option value="">All categories</option>
					{#each categories as cat (cat.id)}
						<option value={cat.id}>{cat.name}</option>
					{/each}
				</select>
			{/if}
		</div>
	{/if}

	<!-- Item count -->
	{#if !loading && !error}
		<p class="mb-4 font-body text-xs text-outline-variant dark:text-on-surface-muted">
			{filteredItems.length} item{filteredItems.length !== 1 ? 's' : ''}
		</p>
	{/if}

	<!-- Item list -->
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
	{:else if filteredItems.length === 0}
		<EmptyState
			title="No items found"
			description={selectedLocation || selectedCategory
				? 'No items match the current filters.'
				: 'Start tracking items to manage your inventory.'}
			icon="inventory_2"
		/>
	{:else}
		<div class="space-y-3" role="list">
			{#each filteredItems as item (item.id)}
				<a href={resolve(`/tracking/items/${item.id}` as '/')} class="block">
					<ItemCard {item} />
				</a>
			{/each}
		</div>
	{/if}
</main>
