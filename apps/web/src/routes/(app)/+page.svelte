<script lang="ts">
	import { onMount } from 'svelte';
	import { syncStore } from '$lib/stores/sync.svelte';
	import { page } from '$app/state';
	import QuestCard from '$lib/components/guidance/QuestCard.svelte';
	import RoutineCard from '$lib/components/guidance/RoutineCard.svelte';
	import CheckinCard from '$lib/components/guidance/CheckinCard.svelte';
	import SectionLabel from '$lib/components/ui/SectionLabel.svelte';
	import EmptyState from '$lib/components/ui/EmptyState.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import type {
		GuidanceQuest,
		GuidanceRoutine,
		GuidanceDailyCheckin
	} from '$lib/types/guidance.js';

	let quests = $state<GuidanceQuest[]>([]);
	let routines = $state<GuidanceRoutine[]>([]);
	let checkin = $state<GuidanceDailyCheckin | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);

	// Greeting based on time of day
	const greeting = $derived.by(() => {
		const hour = new Date().getHours();
		if (hour < 12) return 'Good morning';
		if (hour < 17) return 'Good afternoon';
		return 'Good evening';
	});

	const user = $derived(page.data?.user as { id: string; email: string; name: string } | null);
	const userName = $derived(user?.name?.split(' ')[0] ?? 'there');

	// Today's date in editorial format: "Thursday, March 26"
	const dateDisplay = $derived(
		new Date().toLocaleDateString('en-US', { weekday: 'long', month: 'long', day: 'numeric' })
	);

	const completedCount = $derived(quests.filter((q) => q.status === 'completed').length);
	const totalCount = $derived(quests.length);

	async function loadData() {
		loading = true;
		error = null;
		const [questsResult, routinesResult, checkinResult] = await Promise.allSettled([
			syncStore.queryTodayQuests(),
			syncStore.queryTodayRoutines(),
			syncStore.queryTodayCheckin()
		]);
		if (questsResult.status === 'fulfilled') quests = questsResult.value;
		if (routinesResult.status === 'fulfilled') routines = routinesResult.value;
		if (checkinResult.status === 'fulfilled') checkin = checkinResult.value;
		const anyFailed = [questsResult, routinesResult, checkinResult].some(
			(r) => r.status === 'rejected'
		);
		if (anyFailed) error = 'Some data could not be loaded. Pull to refresh.';
		loading = false;
	}

	async function handleQuestComplete(id: string) {
		try {
			await syncStore.completeQuest(id);
			quests = await syncStore.queryTodayQuests();
		} catch (err) {
			console.error('[today] Failed to complete quest:', err);
			error = 'Could not complete quest. Please try again.';
			await loadData();
		}
	}

	onMount(() => {
		loadData();
	});
</script>

<svelte:head>
	<title>Today - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<!-- Header: Greeting + Date -->
	<div class="mb-10 flex items-start justify-between">
		<div>
			<h1 class="font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
				{greeting}, {userName}
			</h1>
		</div>
		<p class="mt-1 font-body text-sm text-on-surface-muted dark:text-on-surface-subtle">
			{dateDisplay}
		</p>
	</div>

	<!-- TODAY'S QUESTS section -->
	<section class="mb-8">
		<div class="mb-3 flex items-center justify-between">
			<SectionLabel text="TODAY'S QUESTS" />
			{#if totalCount > 0}
				<span class="font-body text-xs text-on-surface-muted"
					>{completedCount} of {totalCount} done</span
				>
			{/if}
		</div>

		{#if loading}
			<!-- Loading skeleton -->
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
		{:else if quests.length === 0}
			<EmptyState
				title="No quests for today"
				description="You're all caught up! Add quests to see them here."
				icon="check_circle"
			/>
		{:else}
			<div class="space-y-3" role="list">
				{#each quests as quest (quest.id)}
					<QuestCard {quest} onComplete={handleQuestComplete} />
				{/each}
			</div>
		{/if}
	</section>

	<!-- ROUTINES section -->
	<section class="mb-8">
		<SectionLabel text="ROUTINES" />
		{#if loading}
			<div class="space-y-3">
				{#each [0, 1] as i (i)}
					<div class="h-14 animate-pulse rounded-2xl bg-surface-low dark:bg-surface-high"></div>
				{/each}
			</div>
		{:else if routines.length === 0}
			<EmptyState
				title="No active routines"
				description="Set up routines to build consistent habits."
				icon="repeat"
			/>
		{:else}
			<div class="space-y-3" role="list">
				{#each routines as routine (routine.id)}
					<RoutineCard {routine} />
				{/each}
			</div>
		{/if}
	</section>

	<!-- CHECK-IN section -->
	<section class="mb-8">
		<SectionLabel text="CHECK-IN" />
		{#if loading}
			<div class="h-20 animate-pulse rounded-2xl bg-surface-low dark:bg-surface-high"></div>
		{:else}
			<CheckinCard {checkin} />
		{/if}
	</section>
</main>
