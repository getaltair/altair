<script lang="ts">
	import { page } from '$app/state';
	import { syncStore } from '$lib/stores/sync.svelte';
	import { theme, setTheme, type Theme } from '$lib/stores/theme.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import SectionLabel from '$lib/components/ui/SectionLabel.svelte';

	// User data from the layout server load (auth hooks)
	const user = $derived(page.data?.user as { id: string; email: string; name: string } | null);

	// Avatar initials derived from name or email
	const initials = $derived.by(() => {
		if (user?.name) {
			const parts = user.name.trim().split(/\s+/);
			if (parts.length >= 2) return (parts[0][0] + parts[parts.length - 1][0]).toUpperCase();
			return parts[0][0].toUpperCase();
		}
		if (user?.email) return user.email[0].toUpperCase();
		return '?';
	});

	// Sync status display helpers
	const statusDotColor = $derived(
		{
			connected: 'bg-sage-whisper',
			connecting: 'bg-morning-gold',
			disconnected: 'bg-outline-variant',
			error: 'bg-error'
		}[syncStore.syncStatus] ?? 'bg-outline-variant'
	);

	const statusLabel = $derived(
		{
			connected: 'Connected',
			connecting: 'Connecting...',
			disconnected: 'Disconnected',
			error: 'Error'
		}[syncStore.syncStatus] ?? 'Disconnected'
	);

	// Format the last synced timestamp into a human-readable string
	const lastSyncedDisplay = $derived.by(() => {
		if (!syncStore.lastSyncedAt) return 'Never';
		try {
			const date = new Date(syncStore.lastSyncedAt);
			return date.toLocaleString(undefined, {
				month: 'short',
				day: 'numeric',
				hour: 'numeric',
				minute: '2-digit'
			});
		} catch {
			return syncStore.lastSyncedAt;
		}
	});

	// Theme options
	const themeOptions: { value: Theme; label: string }[] = [
		{ value: 'light', label: 'Light' },
		{ value: 'dark', label: 'Dark' }
	];

	// Refresh sync -- re-initialize to reconnect
	let isRefreshing = $state(false);

	async function handleRefreshSync() {
		isRefreshing = true;
		try {
			await syncStore.initialize();
		} catch (err) {
			console.error('[settings] Sync refresh failed:', err);
		} finally {
			setTimeout(() => {
				isRefreshing = false;
			}, 600);
		}
	}
</script>

<svelte:head>
	<title>Settings - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<!-- Page heading -->
	<h1 class="font-display text-2xl font-bold text-on-surface dark:text-[var(--text-primary)]">
		Settings
	</h1>
	<p class="mt-1 font-body text-sm text-on-surface-muted dark:text-on-surface-subtle">
		Manage your profile, household, and preferences.
	</p>

	<div class="mt-8 space-y-8">
		<!-- ================================================================== -->
		<!-- 1. PROFILE -->
		<!-- ================================================================== -->
		<Card>
			<SectionLabel text="PROFILE" />

			<div class="mt-4 flex items-center gap-4">
				<!-- Avatar circle with initials -->
				<div
					class="flex h-12 w-12 shrink-0 items-center justify-center rounded-full bg-primary-container font-display text-base font-bold text-primary dark:bg-[#2a4a5a] dark:text-primary-container"
				>
					{initials}
				</div>

				<div class="min-w-0">
					<p
						class="truncate font-display text-base font-semibold text-on-surface dark:text-[var(--text-primary)]"
					>
						{user?.name ?? 'Not signed in'}
					</p>
					<p class="truncate font-body text-sm text-on-surface-muted dark:text-on-surface-subtle">
						{user?.email ?? '--'}
					</p>
				</div>
			</div>
		</Card>

		<!-- ================================================================== -->
		<!-- 2. HOUSEHOLD -->
		<!-- ================================================================== -->
		<Card>
			<SectionLabel text="HOUSEHOLD" />

			<div class="mt-4 flex items-center justify-between gap-4">
				<div class="min-w-0">
					<p class="truncate font-body text-sm text-on-surface dark:text-[var(--text-primary)]">
						Default Household
					</p>
					<p class="font-body text-xs text-on-surface-muted dark:text-on-surface-subtle">
						Household management coming soon
					</p>
				</div>

				<Button variant="ghost" size="sm" disabled>Manage</Button>
			</div>
		</Card>

		<!-- ================================================================== -->
		<!-- 3. SYNC STATUS -->
		<!-- ================================================================== -->
		<Card>
			<SectionLabel text="SYNC STATUS" />

			<div class="mt-4 space-y-4">
				<!-- Status row -->
				<div class="flex items-center justify-between gap-4">
					<div class="flex items-center gap-2.5">
						<!-- Animated status dot -->
						<span class="relative flex h-2.5 w-2.5">
							{#if syncStore.syncStatus === 'connecting'}
								<span
									class="absolute inline-flex h-full w-full animate-ping rounded-full bg-morning-gold opacity-75"
								></span>
							{/if}
							<span class="relative inline-flex h-2.5 w-2.5 rounded-full {statusDotColor}"></span>
						</span>

						<span
							class="font-body text-sm font-medium text-on-surface dark:text-[var(--text-primary)]"
						>
							{statusLabel}
						</span>
					</div>

					<!-- Refresh button -->
					<button
						onclick={handleRefreshSync}
						disabled={isRefreshing}
						class="transition-breathe flex h-8 w-8 items-center justify-center rounded-full text-on-surface-muted hover:bg-surface-low hover:text-on-surface disabled:opacity-40 dark:hover:bg-[var(--card-bg)] dark:hover:text-[var(--text-primary)]"
						aria-label="Refresh sync"
					>
						<span
							class="material-symbols-outlined text-base"
							class:animate-spin={isRefreshing}
							aria-hidden="true">sync</span
						>
					</button>
				</div>

				<!-- Last synced -->
				<div>
					<p class="font-body text-xs text-on-surface-muted dark:text-on-surface-subtle">
						Last synced
					</p>
					<p class="mt-0.5 font-body text-sm text-on-surface dark:text-[var(--text-primary)]">
						{lastSyncedDisplay}
					</p>
				</div>

				<!-- Init error (if any) -->
				{#if syncStore.initError}
					<div class="rounded-xl bg-error-container/10 px-3 py-2">
						<p class="font-body text-xs text-error">
							{syncStore.initError}
						</p>
					</div>
				{/if}
			</div>
		</Card>

		<!-- ================================================================== -->
		<!-- 4. APPEARANCE -->
		<!-- ================================================================== -->
		<Card>
			<SectionLabel text="APPEARANCE" />

			<div class="mt-4">
				<p class="mb-3 font-body text-sm text-on-surface-muted dark:text-on-surface-subtle">
					Theme
				</p>

				<!-- Theme toggle buttons -->
				<div class="flex gap-2">
					{#each themeOptions as option (option.value)}
						<button
							onclick={() => setTheme(option.value)}
							class="group transition-breathe flex items-center gap-2 rounded-full px-5 py-2 font-body text-sm font-medium
								{theme.current === option.value
								? 'bg-primary text-white dark:bg-primary-container dark:text-primary'
								: 'bg-surface-low text-on-surface-muted hover:bg-surface-container hover:text-on-surface dark:bg-[var(--card-bg)] dark:text-on-surface-subtle dark:hover:bg-[#263638] dark:hover:text-[var(--text-primary)]'}"
							aria-pressed={theme.current === option.value}
						>
							<!-- Icon for each theme -->
							{#if option.value === 'light'}
								<span class="material-symbols-outlined text-sm" aria-hidden="true">light_mode</span>
							{:else}
								<span class="material-symbols-outlined text-sm" aria-hidden="true">dark_mode</span>
							{/if}
							{option.label}
						</button>
					{/each}
				</div>
			</div>
		</Card>
	</div>
</main>
