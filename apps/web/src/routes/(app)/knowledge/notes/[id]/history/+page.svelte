<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { syncStore } from '$lib/stores/sync.svelte';
	import type { KnowledgeNoteSnapshot } from '$lib/types/knowledge.js';
	import SnapshotTimeline from '$lib/components/knowledge/SnapshotTimeline.svelte';
	import { BackLink, Card, EmptyState } from '$lib/components/ui/index.js';

	const id = $derived(page.params.id ?? '');
	let snapshots = $state<KnowledgeNoteSnapshot[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		error = null;
		if (!id) {
			error = 'Invalid note ID.';
			loading = false;
			return;
		}
		try {
			snapshots = await syncStore.queryNoteSnapshots(id);
		} catch (err) {
			console.error('[note-history] Failed to load:', err);
			error = 'Could not load data. Please try again.';
		}
		loading = false;
	});
</script>

<svelte:head>
	<title>Note History - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<BackLink href="/knowledge/notes/{id}" label="Note" />

	<!-- Header -->
	<div class="mb-8">
		<h1 class="font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
			Note History
		</h1>
		<p class="mt-2 font-body text-sm text-on-surface-muted dark:text-on-surface-subtle">
			Snapshots and version history
		</p>
	</div>

	{#if loading}
		<div class="space-y-4">
			{#each [0, 1, 2] as i (i)}
				<div class="flex gap-4">
					<div
						class="h-6 w-6 shrink-0 animate-pulse rounded-full bg-surface-low dark:bg-surface-high"
					></div>
					<div
						class="h-20 flex-1 animate-pulse rounded-2xl bg-surface-low dark:bg-surface-high"
					></div>
				</div>
			{/each}
		</div>
	{:else if error}
		<Card>
			<div class="flex flex-col items-center gap-3 py-6 text-center">
				<span class="material-symbols-outlined text-3xl text-error">cloud_off</span>
				<p class="font-body text-sm text-on-surface-muted">{error}</p>
			</div>
		</Card>
	{:else if snapshots.length === 0}
		<EmptyState
			title="No history yet"
			description="Snapshots will appear here as the note is updated over time."
			icon="history"
		/>
	{:else}
		<SnapshotTimeline {snapshots} />
	{/if}
</main>
