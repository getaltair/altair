<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { syncStore } from '$lib/stores/sync.svelte';
	import type { Initiative } from '$lib/types/core.js';
	import type { GuidanceEpic, GuidanceQuest } from '$lib/types/guidance.js';
	import { BackLink, Card, SectionLabel, EmptyState } from '$lib/components/ui/index.js';
	import { statusColor, formatDate } from '$lib/utils/guidance-format.js';

	const id = $derived(page.params.id);

	let initiative = $state<Initiative | null>(null);
	let epics = $state<GuidanceEpic[]>([]);
	let quests = $state<GuidanceQuest[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		error = null;
		try {
			const allInitiatives = await syncStore.queryInitiatives();
			initiative = allInitiatives.find((i) => i.id === id) ?? null;

			const db = syncStore.db;
			if (db) {
				epics = await db.getAll<GuidanceEpic>(
					'SELECT * FROM guidance_epics WHERE initiative_id = ? ORDER BY created_at DESC',
					[id]
				);

				quests = await db.getAll<GuidanceQuest>(
					'SELECT * FROM guidance_quests WHERE initiative_id = ? ORDER BY created_at DESC',
					[id]
				);
			}
		} catch (err) {
			console.error('[initiative-detail] Failed to load data:', err);
			error = 'Could not load data. Please try again.';
		}

		loading = false;
	});
</script>

<svelte:head>
	<title>{initiative?.name ?? 'Initiative'} - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<BackLink href="/guidance/initiatives" label="Initiatives" />

	{#if loading}
		<div class="space-y-4">
			<div class="h-10 w-64 animate-pulse rounded-lg bg-surface-low dark:bg-surface-high"></div>
			<div class="h-6 w-20 animate-pulse rounded-full bg-surface-low dark:bg-surface-high"></div>
			<div class="h-16 w-full animate-pulse rounded-2xl bg-surface-low dark:bg-surface-high"></div>
		</div>
	{:else if error}
		<Card>
			<div class="flex flex-col items-center gap-3 py-6 text-center">
				<span class="material-symbols-outlined text-3xl text-error">cloud_off</span>
				<p class="font-body text-sm text-on-surface-muted">{error}</p>
			</div>
		</Card>
	{:else if !initiative}
		<EmptyState
			title="Initiative not found"
			description="This initiative may have been removed or is not available locally."
			icon="error_outline"
			ctaLabel="Back to initiatives"
			ctaHref="/guidance/initiatives"
		/>
	{:else}
		<!-- Initiative header -->
		<div class="mb-8">
			<h1 class="font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
				{initiative.name}
			</h1>
			<div class="mt-3 flex items-center gap-3">
				<span
					class="inline-flex items-center rounded-full px-3 py-1 font-body text-xs font-medium {statusColor(
						initiative.status
					)}"
				>
					{initiative.status}
				</span>
				<span class="font-body text-xs text-outline-variant dark:text-on-surface-muted">
					Created {formatDate(initiative.created_at)}
				</span>
			</div>
			{#if initiative.description}
				<p
					class="mt-4 font-body text-sm leading-relaxed text-on-surface-muted dark:text-on-surface-subtle"
				>
					{initiative.description}
				</p>
			{/if}
		</div>

		<!-- EPICS section -->
		<section class="mb-8">
			<SectionLabel text="EPICS" />
			{#if epics.length === 0}
				<Card surface="low">
					<p
						class="py-4 text-center font-body text-sm text-outline-variant dark:text-on-surface-muted"
					>
						No epics linked to this initiative
					</p>
				</Card>
			{:else}
				<div class="space-y-3" role="list">
					{#each epics as epic (epic.id)}
						<Card>
							<div class="flex items-center justify-between" role="listitem">
								<div class="min-w-0 flex-1">
									<div class="flex items-center gap-2">
										<span
											class="material-symbols-outlined text-lg text-primary dark:text-primary-container"
											aria-hidden="true"
										>
											auto_awesome
										</span>
										<h3
											class="font-body text-sm font-semibold text-on-surface dark:text-[var(--text-primary)]"
										>
											{epic.name}
										</h3>
									</div>
									{#if epic.description}
										<p
											class="mt-1 line-clamp-1 pl-7 font-body text-xs text-on-surface-muted dark:text-on-surface-subtle"
										>
											{epic.description}
										</p>
									{/if}
								</div>
								<span
									class="inline-flex shrink-0 items-center rounded-full px-2.5 py-0.5 font-body text-xs font-medium {statusColor(
										epic.status
									)}"
								>
									{epic.status}
								</span>
							</div>
						</Card>
					{/each}
				</div>
			{/if}
		</section>

		<!-- QUESTS section -->
		<section class="mb-8">
			<SectionLabel text="QUESTS" />
			{#if quests.length === 0}
				<Card surface="low">
					<p
						class="py-4 text-center font-body text-sm text-outline-variant dark:text-on-surface-muted"
					>
						No quests linked to this initiative
					</p>
				</Card>
			{:else}
				<div class="space-y-3" role="list">
					{#each quests as quest (quest.id)}
						<Card>
							<div class="flex items-center justify-between" role="listitem">
								<div class="min-w-0 flex-1">
									<div class="flex items-center gap-2">
										<span
											class="material-symbols-outlined text-lg text-primary dark:text-primary-container"
											aria-hidden="true"
										>
											task_alt
										</span>
										<h3
											class="font-body text-sm font-semibold text-on-surface dark:text-[var(--text-primary)]"
										>
											{quest.name}
										</h3>
									</div>
									{#if quest.due_date}
										<p
											class="mt-1 pl-7 font-body text-xs text-on-surface-muted dark:text-on-surface-subtle"
										>
											Due {formatDate(quest.due_date)}
										</p>
									{/if}
								</div>
								<span
									class="inline-flex shrink-0 items-center rounded-full px-2.5 py-0.5 font-body text-xs font-medium {statusColor(
										quest.status
									)}"
								>
									{quest.status}
								</span>
							</div>
						</Card>
					{/each}
				</div>
			{/if}
		</section>
	{/if}
</main>
