<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { syncStore } from '$lib/stores/sync.svelte';
	import type { TrackingShoppingList, TrackingShoppingListItem } from '$lib/types/tracking.js';
	import ShoppingListItem from '$lib/components/tracking/ShoppingListItem.svelte';
	import { BackLink, Card, EmptyState, SectionLabel } from '$lib/components/ui/index.js';

	const id = $derived(page.params.id!);
	let list = $state<TrackingShoppingList | null>(null);
	let items = $state<TrackingShoppingListItem[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	const checkedCount = $derived(items.filter((i) => i.is_checked === 1).length);
	const totalCount = $derived(items.length);
	const progress = $derived(totalCount > 0 ? Math.round((checkedCount / totalCount) * 100) : 0);

	onMount(async () => {
		error = null;
		try {
			const db = syncStore.db;
			if (db) {
				list = await db.getOptional<TrackingShoppingList>(
					'SELECT * FROM tracking_shopping_lists WHERE id = ?',
					[id]
				);
			}
			if (list) {
				items = await syncStore.queryShoppingListItems(id);
			}
		} catch (err) {
			console.error('[shopping-list-detail] Failed to load:', err);
			error = 'Could not load data. Please try again.';
		}
		loading = false;
	});

	async function handleToggle(itemId: string) {
		try {
			await syncStore.toggleShoppingListItemCheck(itemId);
			// Reload items
			items = await syncStore.queryShoppingListItems(id);
		} catch (err) {
			console.error('[shopping-list-detail] Failed to toggle item:', err);
		}
	}
</script>

<svelte:head>
	<title>{list?.name ?? 'Shopping List'} - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<BackLink href="/tracking/shopping-lists" label="Shopping Lists" />

	{#if loading}
		<div class="space-y-4">
			<div class="h-10 w-64 animate-pulse rounded-lg bg-surface-low dark:bg-surface-high"></div>
			<div class="h-4 w-32 animate-pulse rounded bg-surface-low dark:bg-surface-high"></div>
			<div class="space-y-2">
				{#each [0, 1, 2] as i (i)}
					<div class="h-12 animate-pulse rounded-xl bg-surface-low dark:bg-surface-high"></div>
				{/each}
			</div>
		</div>
	{:else if error}
		<Card>
			<div class="flex flex-col items-center gap-3 py-6 text-center">
				<span class="material-symbols-outlined text-3xl text-error">cloud_off</span>
				<p class="font-body text-sm text-on-surface-muted">{error}</p>
			</div>
		</Card>
	{:else if !list}
		<EmptyState
			title="Shopping list not found"
			description="This list may have been removed or is not available locally."
			icon="error_outline"
			ctaLabel="Back to shopping lists"
			ctaHref="/tracking/shopping-lists"
		/>
	{:else}
		<!-- List header -->
		<div class="mb-6">
			<h1 class="font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
				{list.name}
			</h1>
		</div>

		<!-- Progress -->
		{#if totalCount > 0}
			<div class="mb-6">
				<div class="flex items-center justify-between">
					<span class="font-body text-sm text-on-surface-muted dark:text-on-surface-subtle">
						{checkedCount} of {totalCount} items checked
					</span>
					<span class="font-display text-lg font-bold text-primary dark:text-primary-container">
						{progress}%
					</span>
				</div>
				<div class="mt-2 h-2 w-full overflow-hidden rounded-full bg-[#a9b4b5]/30">
					<div
						class="h-full rounded-full bg-primary transition-all duration-300 dark:bg-primary-container"
						style="width: {progress}%"
					></div>
				</div>
			</div>
		{/if}

		<!-- Items -->
		<section>
			<SectionLabel text="ITEMS" />
			{#if items.length === 0}
				<EmptyState
					title="No items in this list"
					description="Add items to start planning your shopping."
					icon="add_shopping_cart"
				/>
			{:else}
				<Card>
					<div role="list">
						{#each items as item (item.id)}
							<ShoppingListItem {item} onToggle={() => handleToggle(item.id)} />
						{/each}
					</div>
				</Card>
			{/if}
		</section>
	{/if}
</main>
