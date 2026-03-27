<script lang="ts">
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import { syncStore } from '$lib/stores/sync.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import SectionLabel from '$lib/components/ui/SectionLabel.svelte';

	let itemCount = $state(0);
	let lowStockCount = $state(0);
	let shoppingListCount = $state(0);
	let loading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		error = null;
		try {
			const items = await syncStore.queryItems();
			const shoppingLists = await syncStore.queryShoppingLists();

			itemCount = items.length;
			shoppingListCount = shoppingLists.length;

			// Count low-stock items client-side from all items
			lowStockCount = items.filter(
				(i) => i.min_quantity != null && i.quantity < i.min_quantity
			).length;
		} catch (err) {
			console.error('[tracking] Failed to load stats:', err);
			error = 'Could not load data. Please try again.';
		}
		loading = false;
	});

	const navItems = [
		{
			href: '/tracking/items',
			icon: 'inventory_2',
			title: 'Items',
			description: 'Track and manage your inventory'
		},
		{
			href: '/tracking/locations',
			icon: 'location_on',
			title: 'Locations',
			description: 'Where things are stored'
		},
		{
			href: '/tracking/categories',
			icon: 'category',
			title: 'Categories',
			description: 'Organize items by type'
		},
		{
			href: '/tracking/shopping-lists',
			icon: 'shopping_cart',
			title: 'Shopping Lists',
			description: 'Plan your purchases'
		},
		{
			href: '/tracking/low-stock',
			icon: 'warning',
			title: 'Low Stock',
			description: 'Items running low'
		}
	];
</script>

<svelte:head>
	<title>Tracking - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<!-- Header -->
	<div class="mb-10">
		<h1 class="font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
			Tracking
		</h1>
		<p class="mt-2 font-body text-sm text-on-surface-muted dark:text-on-surface-subtle">
			What do I have?
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
			<div class="grid grid-cols-3 gap-4">
				{#each [{ href: '/tracking/items', count: itemCount, label: 'Items' }, { href: '/tracking/low-stock', count: lowStockCount, label: 'Low Stock' }, { href: '/tracking/shopping-lists', count: shoppingListCount, label: 'Shopping Lists' }] as stat (stat.href)}
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
