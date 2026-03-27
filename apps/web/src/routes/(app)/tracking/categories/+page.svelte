<script lang="ts">
	import { onMount } from 'svelte';
	import { syncStore } from '$lib/stores/sync.svelte';
	import type { TrackingCategory } from '$lib/types/tracking.js';
	import TreeView from '$lib/components/tracking/TreeView.svelte';
	import { BackLink, Card, EmptyState } from '$lib/components/ui/index.js';

	let categories = $state<TrackingCategory[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		error = null;
		try {
			const items = await syncStore.queryItems();
			const householdId = items[0]?.household_id ?? '';
			categories = await syncStore.queryCategories(householdId);
		} catch (err) {
			console.error('[tracking-categories] Failed to load:', err);
			error = 'Could not load data. Please try again.';
		}
		loading = false;
	});
</script>

<svelte:head>
	<title>Categories - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<BackLink href="/tracking" label="Tracking" />

	<!-- Header -->
	<div class="mb-6">
		<h1 class="font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
			Categories
		</h1>
	</div>

	{#if loading}
		<div class="space-y-3">
			{#each [0, 1, 2] as i (i)}
				<div class="h-12 animate-pulse rounded-2xl bg-surface-low dark:bg-surface-high"></div>
			{/each}
		</div>
	{:else if error}
		<Card>
			<div class="flex flex-col items-center gap-3 py-6 text-center">
				<span class="material-symbols-outlined text-3xl text-error">cloud_off</span>
				<p class="font-body text-sm text-on-surface-muted">{error}</p>
			</div>
		</Card>
	{:else if categories.length === 0}
		<EmptyState
			title="No categories yet"
			description="Add categories to organize your items by type."
			icon="category"
		/>
	{:else}
		<Card>
			<TreeView
				items={categories}
				getId={(item) => (item as TrackingCategory).id}
				getParentId={(item) => (item as TrackingCategory).parent_category_id}
				getName={(item) => (item as TrackingCategory).name}
				getDescription={(item) => (item as TrackingCategory).description ?? undefined}
			/>
		</Card>
	{/if}
</main>
