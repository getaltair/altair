<script lang="ts">
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import { syncStore } from '$lib/stores/sync.svelte';
	import type { GuidanceQuest, QuestStatus } from '$lib/types/guidance.js';
	import QuestCard from '$lib/components/guidance/QuestCard.svelte';
	import { BackLink, Card, EmptyState } from '$lib/components/ui/index.js';

	let allQuests = $state<GuidanceQuest[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let statusFilter = $state<QuestStatus | 'all'>('all');

	const filteredQuests = $derived(
		statusFilter === 'all' ? allQuests : allQuests.filter((q) => q.status === statusFilter)
	);

	const filters: { label: string; value: QuestStatus | 'all' }[] = [
		{ label: 'All', value: 'all' },
		{ label: 'Pending', value: 'pending' },
		{ label: 'In Progress', value: 'in_progress' },
		{ label: 'Completed', value: 'completed' },
		{ label: 'Cancelled', value: 'cancelled' }
	];

	onMount(async () => {
		error = null;
		try {
			allQuests = await syncStore.queryQuests();
		} catch (err) {
			console.error('[quests] Failed to load:', err);
			error = 'Could not load data. Please try again.';
		}
		loading = false;
	});

	async function handleComplete(id: string) {
		try {
			await syncStore.completeQuest(id);
			allQuests = await syncStore.queryQuests();
		} catch (err) {
			console.error('[quests] Failed to complete quest:', err);
			error = 'Could not complete quest. Please try again.';
		}
	}
</script>

<svelte:head>
	<title>Quests - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<BackLink href="/guidance" label="Guidance" />

	<!-- Header -->
	<div class="mb-6">
		<h1 class="font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
			Quests
		</h1>
	</div>

	<!-- Filter pills -->
	<div class="mb-4 flex gap-2 overflow-x-auto pb-1" role="tablist" aria-label="Quest status filter">
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

	<!-- Quest count -->
	{#if !loading}
		<p class="mb-4 font-body text-xs text-outline-variant dark:text-on-surface-muted">
			{filteredQuests.length} quest{filteredQuests.length !== 1 ? 's' : ''}
		</p>
	{/if}

	<!-- Quest list -->
	{#if loading}
		<div class="space-y-3">
			{#each [0, 1, 2] as i (i)}
				<div class="h-16 animate-pulse rounded-2xl bg-surface-low dark:bg-surface-high"></div>
			{/each}
		</div>
	{:else if error}
		<Card>
			<div class="flex flex-col items-center gap-3 py-6 text-center">
				<span class="material-symbols-outlined text-3xl text-error">cloud_off</span>
				<p class="font-body text-sm text-on-surface-muted">{error}</p>
			</div>
		</Card>
	{:else if filteredQuests.length === 0}
		<EmptyState
			title="No quests found"
			description={statusFilter === 'all'
				? 'Quests help you break down goals into actionable steps.'
				: `No ${statusFilter.replace('_', ' ')} quests right now.`}
			icon="task_alt"
		/>
	{:else}
		<div class="space-y-3" role="list">
			{#each filteredQuests as quest (quest.id)}
				<a href={resolve(`/guidance/quests/${quest.id}` as '/')} class="block">
					<QuestCard {quest} onComplete={handleComplete} />
				</a>
			{/each}
		</div>
	{/if}
</main>
