<script lang="ts">
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import { syncStore } from '$lib/stores/sync.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import SectionLabel from '$lib/components/ui/SectionLabel.svelte';

	let noteCount = $state(0);
	let pinnedCount = $state(0);
	let loading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		error = null;
		try {
			const notes = await syncStore.queryNotes();
			noteCount = notes.length;
			pinnedCount = notes.filter((n) => n.is_pinned === 1).length;
		} catch (err) {
			console.error('[knowledge] Failed to load stats:', err);
			error = 'Could not load data. Please try again.';
		}
		loading = false;
	});

	const navItems = [
		{
			href: '/knowledge/notes',
			icon: 'description',
			title: 'Notes',
			description: 'Browse and manage your knowledge notes'
		}
	];
</script>

<svelte:head>
	<title>Knowledge - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<!-- Header -->
	<div class="mb-10">
		<h1 class="font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
			Knowledge
		</h1>
		<p class="mt-2 font-body text-sm text-on-surface-muted dark:text-on-surface-subtle">
			What do I know?
		</p>
	</div>

	<!-- Stat cards grid -->
	<section class="mb-10">
		<SectionLabel text="OVERVIEW" />
		{#if error}
			<Card>
				<div class="flex flex-col items-center gap-3 py-6 text-center">
					<span class="material-symbols-outlined text-3xl text-error">cloud_off</span>
					<p class="font-body text-sm text-on-surface-muted">{error}</p>
				</div>
			</Card>
		{:else}
			<div class="grid grid-cols-2 gap-4">
				{#each [{ href: '/knowledge/notes', count: noteCount, label: 'Notes' }, { href: '/knowledge/notes', count: pinnedCount, label: 'Pinned' }] as stat (stat.label)}
					<a href={resolve(stat.href as '/')} class="block">
						<Card>
							<div class="text-center">
								{#if loading}
									<div
										class="mx-auto h-9 w-12 animate-pulse rounded-lg bg-surface-low dark:bg-surface-high"
									></div>
								{:else}
									<span
										class="font-display text-3xl font-bold text-primary dark:text-primary-container"
									>
										{stat.count}
									</span>
								{/if}
								<p
									class="mt-1 font-body text-xs font-semibold tracking-widest text-on-surface-muted uppercase dark:text-on-surface-subtle"
								>
									{stat.label}
								</p>
							</div>
						</Card>
					</a>
				{/each}
			</div>
		{/if}
	</section>

	<!-- Navigation cards -->
	<section>
		<SectionLabel text="EXPLORE" />
		<div class="space-y-3">
			{#each navItems as item (item.href)}
				<a href={resolve(item.href as '/')} class="block">
					<Card class="transition-breathe hover:bg-surface-low dark:hover:bg-[var(--card-bg)]">
						<div class="flex items-center gap-4">
							<span
								class="material-symbols-outlined text-4xl text-primary dark:text-primary-container"
								aria-hidden="true"
							>
								{item.icon}
							</span>
							<div class="min-w-0 flex-1">
								<h3
									class="font-display text-base font-bold text-on-surface dark:text-[var(--text-primary)]"
								>
									{item.title}
								</h3>
								<p
									class="mt-0.5 font-body text-sm text-on-surface-muted dark:text-on-surface-subtle"
								>
									{item.description}
								</p>
							</div>
							<span
								class="material-symbols-outlined text-xl text-outline-variant dark:text-on-surface-muted"
								aria-hidden="true"
							>
								chevron_right
							</span>
						</div>
					</Card>
				</a>
			{/each}
		</div>
	</section>
</main>
