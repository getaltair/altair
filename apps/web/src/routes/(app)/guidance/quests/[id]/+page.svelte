<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { syncStore } from '$lib/stores/sync.svelte';
	import type { GuidanceQuest, GuidanceFocusSession } from '$lib/types/guidance.js';
	import { BackLink, Button, Card, EmptyState, SectionLabel } from '$lib/components/ui/index.js';
	import {
		statusColor,
		priorityLabel,
		priorityColor,
		formatDate,
		formatDuration
	} from '$lib/utils/guidance-format.js';

	const id = $derived(page.params.id);
	let quest = $state<GuidanceQuest | null>(null);
	let focusSessions = $state<GuidanceFocusSession[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		error = null;
		const db = syncStore.db;

		try {
			const quests = await syncStore.queryQuests();
			quest = quests.find((q) => q.id === id) ?? null;

			if (db) {
				focusSessions = await db.getAll<GuidanceFocusSession>(
					'SELECT * FROM guidance_focus_sessions WHERE quest_id = ? ORDER BY started_at DESC',
					[id]
				);
			}
		} catch (err) {
			console.error('[quest-detail] Failed to load:', err);
			error = 'Could not load data. Please try again.';
		}

		loading = false;
	});

	async function handleComplete() {
		if (!quest || !id) return;
		try {
			await syncStore.completeQuest(id);
			const quests = await syncStore.queryQuests();
			quest = quests.find((q) => q.id === id) ?? null;
		} catch (err) {
			console.error('[quest-detail] Failed to complete quest:', err);
			error = 'Could not complete quest. Please try again.';
		}
	}
</script>

<svelte:head>
	<title>{quest?.name ?? 'Quest'} - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<BackLink href="/guidance/quests" label="Quests" />

	{#if loading}
		<div class="space-y-4">
			<div class="h-10 w-64 animate-pulse rounded-lg bg-surface-low dark:bg-surface-high"></div>
			<div class="flex gap-2">
				<div class="h-6 w-20 animate-pulse rounded-full bg-surface-low dark:bg-surface-high"></div>
				<div class="h-6 w-16 animate-pulse rounded-full bg-surface-low dark:bg-surface-high"></div>
			</div>
			<div class="h-16 w-full animate-pulse rounded-2xl bg-surface-low dark:bg-surface-high"></div>
		</div>
	{:else if error}
		<Card>
			<div class="flex flex-col items-center gap-3 py-6 text-center">
				<span class="material-symbols-outlined text-3xl text-error">cloud_off</span>
				<p class="font-body text-sm text-on-surface-muted">{error}</p>
			</div>
		</Card>
	{:else if !quest}
		<EmptyState
			title="Quest not found"
			description="This quest may have been removed or is not available locally."
			icon="error_outline"
			ctaLabel="Back to quests"
			ctaHref="/guidance/quests"
		/>
	{:else}
		<!-- Quest header -->
		<div class="mb-8">
			<h1 class="font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
				{quest.name}
			</h1>

			<div class="mt-3 flex items-center gap-3">
				<span
					class="inline-flex items-center rounded-full px-3 py-1 font-body text-xs font-medium {statusColor(
						quest.status
					)}"
				>
					{quest.status.replace('_', ' ')}
				</span>
				<span class="font-body text-xs font-medium {priorityColor(quest.priority)}">
					{priorityLabel(quest.priority)} priority
				</span>
			</div>

			{#if quest.description}
				<p
					class="mt-4 font-body text-sm leading-relaxed text-on-surface-muted dark:text-on-surface-subtle"
				>
					{quest.description}
				</p>
			{/if}

			<div class="mt-4 flex flex-wrap items-center gap-4">
				{#if quest.due_date}
					<span
						class="inline-flex items-center gap-1 font-body text-xs text-outline-variant dark:text-on-surface-muted"
					>
						<span class="material-symbols-outlined text-[14px]" aria-hidden="true">event</span>
						Due {formatDate(quest.due_date)}
					</span>
				{/if}
				{#if quest.estimated_minutes}
					<span
						class="inline-flex items-center gap-1 font-body text-xs text-outline-variant dark:text-on-surface-muted"
					>
						<span class="material-symbols-outlined text-[14px]" aria-hidden="true">timer</span>
						{formatDuration(quest.estimated_minutes)} estimated
					</span>
				{/if}
				<span
					class="inline-flex items-center gap-1 font-body text-xs text-outline-variant dark:text-on-surface-muted"
				>
					<span class="material-symbols-outlined text-[14px]" aria-hidden="true"
						>calendar_today</span
					>
					Created {formatDate(quest.created_at)}
				</span>
			</div>
		</div>

		{#if quest.status === 'pending' || quest.status === 'in_progress'}
			<div class="mb-8">
				<Button variant="primary" onclick={handleComplete}>
					<span class="material-symbols-outlined mr-2 text-lg" aria-hidden="true">check_circle</span
					>
					Complete Quest
				</Button>
			</div>
		{/if}

		<!-- FOCUS SESSIONS section -->
		<section class="mb-8">
			<SectionLabel text="FOCUS SESSIONS" />
			{#if focusSessions.length === 0}
				<Card surface="low">
					<p
						class="py-4 text-center font-body text-sm text-outline-variant dark:text-on-surface-muted"
					>
						No focus sessions yet
					</p>
				</Card>
			{:else}
				<div class="space-y-3" role="list">
					{#each focusSessions as session (session.id)}
						<Card>
							<div class="flex items-center justify-between" role="listitem">
								<div class="flex items-center gap-3">
									<span
										class="material-symbols-outlined text-lg text-primary dark:text-primary-container"
										aria-hidden="true">timer</span
									>
									<div>
										<p
											class="font-body text-sm font-semibold text-on-surface dark:text-[var(--text-primary)]"
										>
											{formatDuration(session.duration_minutes)}
										</p>
										<p class="font-body text-xs text-on-surface-muted dark:text-on-surface-subtle">
											{formatDate(session.started_at)}
										</p>
									</div>
								</div>
								{#if session.notes}
									<p
										class="max-w-[200px] truncate font-body text-xs text-outline-variant dark:text-on-surface-muted"
									>
										{session.notes}
									</p>
								{/if}
							</div>
						</Card>
					{/each}
				</div>
			{/if}
		</section>
	{/if}
</main>
