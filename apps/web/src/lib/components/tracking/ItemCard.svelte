<script lang="ts">
	import type { TrackingItem } from '$lib/types/tracking.js';
	import { itemStatusColor, formatQuantity, stockLevelClass } from '$lib/utils/tracking-format.js';

	interface Props {
		item: TrackingItem;
		class?: string;
	}

	let { item, class: className = '' }: Props = $props();

	const statusClasses = $derived(itemStatusColor(item.status));
	const stockLevel = $derived(stockLevelClass(item.quantity, item.min_quantity));
</script>

<div
	class="flex items-start gap-3 rounded-2xl bg-pure-white p-4 dark:bg-[var(--card-bg)] {className}"
	role="listitem"
>
	<!-- Stock level indicator -->
	<div class="mt-1 flex shrink-0 items-center">
		{#if stockLevel === 'out-of-stock'}
			<span
				class="material-symbols-outlined text-xl text-warm-terracotta"
				aria-label="Out of stock"
			>
				error
			</span>
		{:else if stockLevel === 'low-stock'}
			<span class="material-symbols-outlined text-xl text-[#d4a853]" aria-label="Low stock">
				warning
			</span>
		{:else}
			<span
				class="material-symbols-outlined text-xl text-primary dark:text-primary-container"
				aria-hidden="true"
			>
				inventory_2
			</span>
		{/if}
	</div>

	<!-- Item content -->
	<div class="min-w-0 flex-1">
		<div class="flex items-center gap-2">
			<span class="font-body text-sm font-semibold text-on-surface dark:text-[var(--text-primary)]">
				{item.name}
			</span>
			<span
				class="inline-flex items-center rounded-full px-2 py-0.5 font-body text-[10px] font-medium {statusClasses}"
			>
				{item.status}
			</span>
		</div>

		<div class="mt-1 flex items-center gap-3">
			<span class="font-display text-lg font-bold text-primary dark:text-primary-container">
				{formatQuantity(item.quantity, item.unit)}
			</span>
			{#if item.min_quantity != null && stockLevel}
				<span class="font-body text-xs text-warm-terracotta">
					min {item.min_quantity}
				</span>
			{/if}
		</div>

		<div class="mt-1 flex items-center gap-2">
			{#if item.location_id}
				<span
					class="inline-flex items-center gap-0.5 font-body text-xs text-on-surface-muted dark:text-on-surface-subtle"
				>
					<span class="material-symbols-outlined text-[12px]" aria-hidden="true">location_on</span>
					Location
				</span>
			{:else}
				<span class="font-body text-xs text-outline-variant dark:text-on-surface-muted">
					No location
				</span>
			{/if}
			{#if item.category_id}
				<span
					class="inline-flex items-center gap-0.5 font-body text-xs text-on-surface-muted dark:text-on-surface-subtle"
				>
					<span class="material-symbols-outlined text-[12px]" aria-hidden="true">category</span>
					Category
				</span>
			{/if}
		</div>
	</div>

	<!-- Chevron -->
	<span
		class="material-symbols-outlined mt-2 shrink-0 text-xl text-outline-variant dark:text-on-surface-muted"
		aria-hidden="true"
	>
		chevron_right
	</span>
</div>
