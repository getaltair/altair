<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { syncStore } from '$lib/stores/sync.svelte';
	import type { TrackingItem, TrackingItemEvent } from '$lib/types/tracking.js';
	import QuantityControl from '$lib/components/tracking/QuantityControl.svelte';
	import RelationsPanel from '$lib/components/relations/RelationsPanel.svelte';
	import { BackLink, Button, Card, EmptyState, SectionLabel } from '$lib/components/ui/index.js';
	import {
		itemStatusColor,
		formatDate,
		formatQuantity,
		eventTypeLabel,
		eventTypeIcon,
		stockLevelClass
	} from '$lib/utils/tracking-format.js';

	const id = $derived(page.params.id!);
	let item = $state<TrackingItem | null>(null);
	let events = $state<TrackingItemEvent[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let showDeleteConfirm = $state(false);

	onMount(async () => {
		error = null;
		try {
			item = await syncStore.queryItem(id);
			if (item) {
				events = await syncStore.queryItemEvents(id);
			}
		} catch (err) {
			console.error('[item-detail] Failed to load:', err);
			error = 'Could not load data. Please try again.';
		}
		loading = false;
	});

	async function handleAdjust(change: number) {
		if (!item) return;
		const eventType = change > 0 ? 'restocked' : 'consumed';
		try {
			await syncStore.adjustItemQuantity(item.id, change, eventType);
			// Reload item and events
			item = await syncStore.queryItem(id);
			events = await syncStore.queryItemEvents(id);
		} catch (err) {
			console.error('[item-detail] Failed to adjust quantity:', err);
			error = 'Could not update quantity. Please try again.';
		}
	}

	async function handleDelete() {
		if (!item) return;
		const database = syncStore.db;
		if (!database) return;

		try {
			await database.execute('DELETE FROM tracking_items WHERE id = ?', [item.id]);
			goto(resolve('/tracking/items' as '/'));
		} catch (err) {
			console.error('[item-detail] Failed to delete:', err);
			error = 'Could not delete item. Please try again.';
		}
	}

	const stockLevel = $derived(item ? stockLevelClass(item.quantity, item.min_quantity) : '');
</script>

<svelte:head>
	<title>{item?.name ?? 'Item'} - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<BackLink href="/tracking/items" label="Items" />

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
	{:else if !item}
		<EmptyState
			title="Item not found"
			description="This item may have been removed or is not available locally."
			icon="error_outline"
			ctaLabel="Back to items"
			ctaHref="/tracking/items"
		/>
	{:else}
		<!-- Item header -->
		<div class="mb-8">
			<h1 class="font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
				{item.name}
			</h1>

			<div class="mt-3 flex items-center gap-3">
				<span
					class="inline-flex items-center rounded-full px-3 py-1 font-body text-xs font-medium {itemStatusColor(
						item.status
					)}"
				>
					{item.status}
				</span>
				{#if stockLevel === 'low-stock'}
					<span
						class="inline-flex items-center gap-1 rounded-full bg-[#f0e6c8]/50 px-3 py-1 font-body text-xs font-medium text-[#8a6a2a]"
					>
						<span class="material-symbols-outlined text-[14px]" aria-hidden="true">warning</span>
						Low stock
					</span>
				{:else if stockLevel === 'out-of-stock'}
					<span
						class="inline-flex items-center gap-1 rounded-full bg-[#fe8983]/20 px-3 py-1 font-body text-xs font-medium text-warm-terracotta"
					>
						<span class="material-symbols-outlined text-[14px]" aria-hidden="true">error</span>
						Out of stock
					</span>
				{/if}
			</div>

			{#if item.description}
				<p
					class="mt-4 font-body text-sm leading-relaxed text-on-surface-muted dark:text-on-surface-subtle"
				>
					{item.description}
				</p>
			{/if}

			<div class="mt-4 flex flex-wrap items-center gap-4">
				{#if item.location_id}
					<span
						class="inline-flex items-center gap-1 font-body text-xs text-outline-variant dark:text-on-surface-muted"
					>
						<span class="material-symbols-outlined text-[14px]" aria-hidden="true">location_on</span
						>
						Location
					</span>
				{/if}
				{#if item.category_id}
					<span
						class="inline-flex items-center gap-1 font-body text-xs text-outline-variant dark:text-on-surface-muted"
					>
						<span class="material-symbols-outlined text-[14px]" aria-hidden="true">category</span>
						Category
					</span>
				{/if}
				{#if item.barcode}
					<span
						class="inline-flex items-center gap-1 font-body text-xs text-outline-variant dark:text-on-surface-muted"
					>
						<span class="material-symbols-outlined text-[14px]" aria-hidden="true"
							>qr_code_scanner</span
						>
						{item.barcode}
					</span>
				{/if}
				<span
					class="inline-flex items-center gap-1 font-body text-xs text-outline-variant dark:text-on-surface-muted"
				>
					<span class="material-symbols-outlined text-[14px]" aria-hidden="true"
						>calendar_today</span
					>
					Created {formatDate(item.created_at)}
				</span>
			</div>
		</div>

		<!-- Quantity control -->
		<section class="mb-8">
			<SectionLabel text="QUANTITY" />
			<Card>
				<div class="flex flex-col items-center gap-4 py-4">
					<span class="font-display text-4xl font-bold text-primary dark:text-primary-container">
						{formatQuantity(item.quantity, item.unit)}
					</span>
					{#if item.min_quantity != null}
						<p class="font-body text-xs text-on-surface-muted dark:text-on-surface-subtle">
							Minimum: {formatQuantity(item.min_quantity, item.unit)}
						</p>
					{/if}
					<QuantityControl quantity={item.quantity} unit={item.unit} onAdjust={handleAdjust} />
					<p class="font-body text-xs text-outline-variant dark:text-on-surface-muted">
						+ adds as restocked, - records as consumed
					</p>
				</div>
			</Card>
		</section>

		<!-- Event history -->
		<section class="mb-8">
			<SectionLabel text="EVENT HISTORY" />
			{#if events.length === 0}
				<Card surface="low">
					<p
						class="py-4 text-center font-body text-sm text-outline-variant dark:text-on-surface-muted"
					>
						No events recorded yet
					</p>
				</Card>
			{:else}
				<div class="space-y-3" role="list">
					{#each events as event (event.id)}
						<Card>
							<div class="flex items-center justify-between" role="listitem">
								<div class="flex items-center gap-3">
									<span
										class="material-symbols-outlined text-lg text-primary dark:text-primary-container"
										aria-hidden="true"
									>
										{eventTypeIcon(event.event_type)}
									</span>
									<div>
										<p
											class="font-body text-sm font-semibold text-on-surface dark:text-[var(--text-primary)]"
										>
											{eventTypeLabel(event.event_type)}
											<span
												class="font-normal {event.quantity_change >= 0
													? 'text-[#5a8a52]'
													: 'text-warm-terracotta'}"
											>
												{event.quantity_change >= 0 ? '+' : ''}{event.quantity_change}
											</span>
										</p>
										<p class="font-body text-xs text-on-surface-muted dark:text-on-surface-subtle">
											{formatDate(event.created_at)}
										</p>
									</div>
								</div>
								{#if event.notes}
									<p
										class="max-w-[200px] truncate font-body text-xs text-outline-variant dark:text-on-surface-muted"
									>
										{event.notes}
									</p>
								{/if}
							</div>
						</Card>
					{/each}
				</div>
			{/if}
		</section>

		<!-- Relations -->
		<section class="mb-8">
			<RelationsPanel entityType="tracking_item" entityId={item.id} />
		</section>

		<!-- Delete -->
		<section>
			{#if showDeleteConfirm}
				<Card>
					<div class="flex flex-col items-center gap-4 py-4 text-center">
						<span class="material-symbols-outlined text-3xl text-warm-terracotta">
							delete_forever
						</span>
						<p class="font-body text-sm text-on-surface dark:text-[var(--text-primary)]">
							Are you sure you want to delete <strong>{item.name}</strong>?
						</p>
						<div class="flex gap-3">
							<Button variant="ghost" onclick={() => (showDeleteConfirm = false)}>Cancel</Button>
							<button
								onclick={handleDelete}
								class="transition-breathe inline-flex min-h-[48px] items-center justify-center rounded-full bg-warm-terracotta px-6 py-3 font-body font-semibold text-white hover:opacity-90"
							>
								Delete
							</button>
						</div>
					</div>
				</Card>
			{:else}
				<button
					onclick={() => (showDeleteConfirm = true)}
					class="transition-breathe font-body text-sm text-outline-variant hover:text-warm-terracotta dark:text-on-surface-muted dark:hover:text-warm-terracotta"
				>
					Delete this item
				</button>
			{/if}
		</section>
	{/if}
</main>
