<script lang="ts">
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import { syncStore } from '$lib/stores/sync.svelte';
	import type { GuidanceRoutine } from '$lib/types/guidance.js';
	import RoutineCard from '$lib/components/guidance/RoutineCard.svelte';
	import { BackLink, Card, EmptyState } from '$lib/components/ui/index.js';

	let routines = $state<GuidanceRoutine[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		error = null;
		try {
			routines = await syncStore.queryRoutines();
		} catch (err) {
			console.error('[routines] Failed to load:', err);
			error = 'Could not load data. Please try again.';
		}
		loading = false;
	});
</script>

<svelte:head>
	<title>Routines - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<BackLink href="/guidance" label="Guidance" />

	<!-- Header -->
	<div class="mb-8">
		<h1 class="font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
			Routines
		</h1>
	</div>

	<!-- Routine list -->
	{#if loading}
		<div class="space-y-3">
			{#each [0, 1, 2] as i (i)}
				<div class="h-14 animate-pulse rounded-2xl bg-surface-low dark:bg-surface-high"></div>
			{/each}
		</div>
	{:else if error}
		<Card>
			<div class="flex flex-col items-center gap-3 py-6 text-center">
				<span class="material-symbols-outlined text-3xl text-error">cloud_off</span>
				<p class="font-body text-sm text-on-surface-muted">{error}</p>
			</div>
		</Card>
	{:else if routines.length === 0}
		<EmptyState
			title="No routines yet"
			description="Routines help you build consistent habits."
			icon="repeat"
		/>
	{:else}
		<div class="space-y-3" role="list">
			{#each routines as routine (routine.id)}
				<a href={resolve(`/guidance/routines/${routine.id}` as '/')} class="block">
					<RoutineCard {routine} />
				</a>
			{/each}
		</div>
	{/if}
</main>
