<script lang="ts">
	import { onMount } from 'svelte';
	import { SvelteMap } from 'svelte/reactivity';
	import { resolve } from '$app/paths';
	import { syncStore } from '$lib/stores/sync.svelte';
	import type { TrackingShoppingList, ShoppingListStatus } from '$lib/types/tracking.js';
	import ShoppingListCard from '$lib/components/tracking/ShoppingListCard.svelte';
	import { BackLink, Card, EmptyState } from '$lib/components/ui/index.js';

	let allLists = $state<TrackingShoppingList[]>([]);
	let listItemCounts = $state<SvelteMap<string, { total: number; checked: number }>>(
		new SvelteMap()
	);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let statusFilter = $state<ShoppingListStatus | 'all'>('all');

	const filteredLists = $derived(
		statusFilter === 'all' ? allLists : allLists.filter((l) => l.status === statusFilter)
	);

	const filters: { label: string; value: ShoppingListStatus | 'all' }[] = [
		{ label: 'All', value: 'all' },
		{ label: 'Active', value: 'active' },
		{ label: 'Completed', value: 'completed' },
		{ label: 'Archived', value: 'archived' }
	];

	onMount(async () => {
		error = null;
		try {
			allLists = await syncStore.queryShoppingLists();

			// Load item counts for each list
			const counts = new SvelteMap<string, { total: number; checked: number }>();
			for (const list of allLists) {
				const items = await syncStore.queryShoppingListItems(list.id);
				counts.set(list.id, {
					total: items.length,
					checked: items.filter((i) => i.is_checked === 1).length
				});
			}
			listItemCounts = counts;
		} catch (err) {
			console.error('[shopping-lists] Failed to load:', err);
			error = 'Could not load data. Please try again.';
		}
		loading = false;
	});
</script>

<svelte:head>
	<title>Shopping Lists - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<BackLink href="/tracking" label="Tracking" />

	<!-- Header -->
	<div class="mb-6">
		<h1 class="font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
			Shopping Lists
		</h1>
	</div>

	<!-- Filter pills -->
	<div
		class="mb-4 flex gap-2 overflow-x-auto pb-1"
		role="tablist"
		aria-label="Shopping list status filter"
	>
		{#each filters as filter (filter.value)}
			<button
				role="tab"
				aria-selected={statusFilter === filter.value}
				onclick={() => (statusFilter = filter.value)}
				class="transition-breathe shrink-0 rounded-full px-4 py-1.5 font-body text-sm font-medium
					{statusFilter === filter.value
					? 'bg-primary text-white'
					: 'bg-surface-low text-on-surface-muted hover:bg-surface-container dark:bg-surface-high dark:text-on-surface-subtle dark:hover:bg-[#2e4446]'}"
			>
				{filter.label}
			</button>
		{/each}
	</div>

	<div role="tabpanel">
		<!-- List count -->
		{#if !loading && !error}
			<p class="mb-4 font-body text-xs text-outline-variant dark:text-on-surface-muted">
				{filteredLists.length} list{filteredLists.length !== 1 ? 's' : ''}
			</p>
		{/if}

		<!-- Shopping lists -->
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
		{:else if filteredLists.length === 0}
			<EmptyState
				title="No shopping lists"
				description={statusFilter === 'all'
					? 'Create a shopping list to plan your purchases.'
					: `No ${statusFilter} lists right now.`}
				icon="shopping_cart"
			/>
		{:else}
			<div class="space-y-3" role="list">
				{#each filteredLists as list (list.id)}
					{@const counts = listItemCounts.get(list.id)}
					<a href={resolve(`/tracking/shopping-lists/${list.id}` as '/')} class="block">
						<ShoppingListCard
							{list}
							itemCount={counts?.total ?? 0}
							checkedCount={counts?.checked ?? 0}
						/>
					</a>
				{/each}
			</div>
		{/if}
	</div>
</main>
