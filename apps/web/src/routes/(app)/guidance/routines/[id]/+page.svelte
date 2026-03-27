<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { syncStore } from '$lib/stores/sync.svelte';
	import type { GuidanceRoutine } from '$lib/types/guidance.js';
	import { BackLink, Card, EmptyState, SectionLabel } from '$lib/components/ui/index.js';
	import { statusColor, frequencyLabel, formatDate } from '$lib/utils/guidance-format.js';

	const id = $derived(page.params.id);
	let routine = $state<GuidanceRoutine | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		error = null;
		try {
			const routines = await syncStore.queryRoutines();
			routine = routines.find((r) => r.id === id) ?? null;
		} catch (err) {
			console.error('[routine-detail] Failed to load:', err);
			error = 'Could not load data. Please try again.';
		}

		loading = false;
	});
</script>

<svelte:head>
	<title>{routine?.name ?? 'Routine'} - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<BackLink href="/guidance/routines" label="Routines" />

	{#if loading}
		<div class="space-y-4">
			<div class="h-10 w-64 animate-pulse rounded-lg bg-surface-low dark:bg-surface-high"></div>
			<div class="flex gap-2">
				<div class="h-6 w-20 animate-pulse rounded-full bg-surface-low dark:bg-surface-high"></div>
				<div class="h-6 w-16 animate-pulse rounded-full bg-surface-low dark:bg-surface-high"></div>
			</div>
		</div>
	{:else if error}
		<Card>
			<div class="flex flex-col items-center gap-3 py-6 text-center">
				<span class="material-symbols-outlined text-3xl text-error">cloud_off</span>
				<p class="font-body text-sm text-on-surface-muted">{error}</p>
			</div>
		</Card>
	{:else if !routine}
		<EmptyState
			title="Routine not found"
			description="This routine may have been removed or is not available locally."
			icon="error_outline"
			ctaLabel="Back to routines"
			ctaHref="/guidance/routines"
		/>
	{:else}
		<!-- Routine header -->
		<div class="mb-8">
			<h1 class="font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
				{routine.name}
			</h1>

			<div class="mt-3 flex items-center gap-3">
				<span
					class="inline-flex items-center rounded-full bg-surface-low px-3 py-1 font-body text-xs font-medium text-on-surface-muted dark:bg-surface-high dark:text-on-surface-subtle"
				>
					{frequencyLabel(routine.frequency)}
				</span>
				<span
					class="inline-flex items-center rounded-full px-3 py-1 font-body text-xs font-medium {statusColor(
						routine.status
					)}"
				>
					{routine.status}
				</span>
			</div>

			{#if routine.description}
				<p
					class="mt-4 font-body text-sm leading-relaxed text-on-surface-muted dark:text-on-surface-subtle"
				>
					{routine.description}
				</p>
			{/if}

			<p class="mt-4 font-body text-xs text-outline-variant dark:text-on-surface-muted">
				Created {formatDate(routine.created_at)}
			</p>
		</div>

		<!-- GENERATED QUESTS section -->
		<section class="mb-8">
			<SectionLabel text="GENERATED QUESTS" />
			<Card surface="low">
				<p
					class="py-4 text-center font-body text-sm text-outline-variant dark:text-on-surface-muted"
				>
					Quests generated from this routine will appear here
				</p>
			</Card>
		</section>
	{/if}
</main>
