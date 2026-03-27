<script lang="ts">
	import { onMount } from 'svelte';
	import { syncStore } from '$lib/stores/sync.svelte';
	import type { TrackingLocation } from '$lib/types/tracking.js';
	import TreeView from '$lib/components/tracking/TreeView.svelte';
	import { BackLink, Card, EmptyState } from '$lib/components/ui/index.js';

	let locations = $state<TrackingLocation[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		error = null;
		try {
			// Get householdId from items as a fallback
			const items = await syncStore.queryItems();
			const householdId = items[0]?.household_id ?? '';
			locations = await syncStore.queryLocations(householdId);
		} catch (err) {
			console.error('[tracking-locations] Failed to load:', err);
			error = 'Could not load data. Please try again.';
		}
		loading = false;
	});
</script>

<svelte:head>
	<title>Locations - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<BackLink href="/tracking" label="Tracking" />

	<!-- Header -->
	<div class="mb-6">
		<h1 class="font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
			Locations
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
	{:else if locations.length === 0}
		<EmptyState
			title="No locations yet"
			description="Add locations to organize where your items are stored."
			icon="location_on"
		/>
	{:else}
		<Card>
			<TreeView
				items={locations}
				getId={(item) => (item as TrackingLocation).id}
				getParentId={(item) => (item as TrackingLocation).parent_location_id}
				getName={(item) => (item as TrackingLocation).name}
				getDescription={(item) => (item as TrackingLocation).description ?? undefined}
			/>
		</Card>
	{/if}
</main>
