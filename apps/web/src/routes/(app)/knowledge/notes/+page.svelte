<script lang="ts">
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import { syncStore } from '$lib/stores/sync.svelte';
	import type { KnowledgeNote } from '$lib/types/knowledge.js';
	import NoteCard from '$lib/components/knowledge/NoteCard.svelte';
	import { BackLink, Card, EmptyState } from '$lib/components/ui/index.js';

	let allNotes = $state<KnowledgeNote[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let searchQuery = $state('');
	let activeFilter = $state<'all' | 'pinned'>('all');

	const filteredNotes = $derived.by(() => {
		let notes = allNotes;

		if (activeFilter === 'pinned') {
			notes = notes.filter((n) => n.is_pinned === 1);
		}

		if (searchQuery.trim()) {
			const query = searchQuery.toLowerCase();
			notes = notes.filter((n) => n.title.toLowerCase().includes(query));
		}

		return notes;
	});

	const filters: { label: string; value: 'all' | 'pinned' }[] = [
		{ label: 'All', value: 'all' },
		{ label: 'Pinned', value: 'pinned' }
	];

	onMount(async () => {
		error = null;
		try {
			allNotes = await syncStore.queryNotes();
		} catch (err) {
			console.error('[knowledge/notes] Failed to load:', err);
			error = 'Could not load data. Please try again.';
		}
		loading = false;
	});
</script>

<svelte:head>
	<title>Notes - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<BackLink href="/knowledge" label="Knowledge" />

	<!-- Header -->
	<div class="mb-6">
		<h1 class="font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
			Notes
		</h1>
	</div>

	<!-- Search -->
	<div class="mb-4">
		<input
			type="text"
			bind:value={searchQuery}
			placeholder="Search notes..."
			class="transition-breathe w-full rounded-2xl bg-surface-low px-4 py-3 font-body text-sm text-on-surface outline-none placeholder:text-outline-variant focus:bg-pure-white focus:ring-2 focus:ring-primary/20 dark:bg-[#1e2d2f] dark:text-[var(--text-primary)] dark:placeholder:text-on-surface-muted dark:focus:bg-[var(--card-bg)]"
		/>
	</div>

	<!-- Filter pills -->
	<div class="mb-4 flex gap-2 overflow-x-auto pb-1" role="tablist" aria-label="Note filter">
		{#each filters as filter (filter.value)}
			<button
				role="tab"
				aria-selected={activeFilter === filter.value}
				onclick={() => (activeFilter = filter.value)}
				class="transition-breathe shrink-0 rounded-full px-4 py-1.5 font-body text-sm font-medium
					{activeFilter === filter.value
					? 'bg-primary text-white'
					: 'bg-surface-low text-on-surface-muted hover:bg-surface-container dark:bg-surface-high dark:text-on-surface-subtle dark:hover:bg-[#2e4446]'}"
			>
				{filter.label}
			</button>
		{/each}
	</div>

	<div role="tabpanel">
		<!-- Note count -->
		{#if !loading}
			<p class="mb-4 font-body text-xs text-outline-variant dark:text-on-surface-muted">
				{filteredNotes.length} note{filteredNotes.length !== 1 ? 's' : ''}
			</p>
		{/if}

		<!-- Note list -->
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
		{:else if filteredNotes.length === 0}
			<EmptyState
				title="No notes found"
				description={activeFilter === 'pinned'
					? 'No pinned notes yet.'
					: searchQuery
						? 'No notes match your search.'
						: 'Notes help you capture and organize knowledge.'}
				icon="description"
			/>
		{:else}
			<div class="space-y-3" role="list">
				{#each filteredNotes as note (note.id)}
					<a href={resolve(`/knowledge/notes/${note.id}` as '/')} class="block">
						<NoteCard {note} />
					</a>
				{/each}
			</div>
		{/if}
	</div>
</main>
