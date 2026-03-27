<script lang="ts">
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import { syncStore } from '$lib/stores/sync.svelte';
	import type { Initiative } from '$lib/types/core.js';
	import { BackLink, Card, EmptyState } from '$lib/components/ui/index.js';
	import { statusColor, formatDate } from '$lib/utils/guidance-format.js';

	let initiatives = $state<Initiative[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		error = null;
		try {
			initiatives = await syncStore.queryInitiatives();
		} catch (err) {
			console.error('[initiatives] Failed to load:', err);
			error = 'Could not load data. Please try again.';
		}
		loading = false;
	});
</script>

<svelte:head>
	<title>Initiatives - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<BackLink href="/guidance" label="Guidance" />

	<!-- Header -->
	<div class="mb-8">
		<h1 class="font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
			Initiatives
		</h1>
	</div>

	<!-- Initiative list -->
	{#if loading}
		<div class="space-y-3">
			{#each [0, 1, 2] as i (i)}
				<div class="h-24 animate-pulse rounded-2xl bg-surface-low dark:bg-surface-high"></div>
			{/each}
		</div>
	{:else if error}
		<Card>
			<div class="flex flex-col items-center gap-3 py-6 text-center">
				<span class="material-symbols-outlined text-3xl text-error">cloud_off</span>
				<p class="font-body text-sm text-on-surface-muted">{error}</p>
			</div>
		</Card>
	{:else if initiatives.length === 0}
		<EmptyState
			title="No initiatives yet"
			description="Initiatives are long-term goals. Create one to get started."
			icon="flag"
		/>
	{:else}
		<div class="space-y-3">
			{#each initiatives as initiative (initiative.id)}
				<a href={resolve(`/guidance/initiatives/${initiative.id}` as '/')} class="block">
					<Card class="transition-breathe hover:bg-surface-low dark:hover:bg-[var(--card-bg)]">
						<div class="flex items-start justify-between gap-3">
							<div class="min-w-0 flex-1">
								<div class="flex items-center gap-2">
									<h3
										class="font-display text-base font-bold text-on-surface dark:text-[var(--text-primary)]"
									>
										{initiative.name}
									</h3>
									<span
										class="inline-flex shrink-0 items-center rounded-full px-2.5 py-0.5 font-body text-xs font-medium {statusColor(
											initiative.status
										)}"
									>
										{initiative.status}
									</span>
								</div>
								{#if initiative.description}
									<p
										class="mt-1.5 line-clamp-2 font-body text-sm text-on-surface-muted dark:text-on-surface-subtle"
									>
										{initiative.description}
									</p>
								{/if}
								<p class="mt-2 font-body text-xs text-outline-variant dark:text-on-surface-muted">
									Created {formatDate(initiative.created_at)}
								</p>
							</div>
							<span
								class="material-symbols-outlined mt-1 shrink-0 text-xl text-outline-variant dark:text-on-surface-muted"
								aria-hidden="true"
							>
								chevron_right
							</span>
						</div>
					</Card>
				</a>
			{/each}
		</div>
	{/if}
</main>
