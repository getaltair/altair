<script lang="ts">
	import type { TrackingShoppingList } from '$lib/types/tracking.js';

	interface Props {
		list: TrackingShoppingList;
		itemCount?: number;
		checkedCount?: number;
		class?: string;
	}

	let { list, itemCount = 0, checkedCount = 0, class: className = '' }: Props = $props();

	const statusClasses = $derived(
		{
			active: 'bg-[#a8c5a0]/30 text-[#5a8a52]',
			completed: 'bg-[#c7e7fa]/50 text-[#446273]',
			archived: 'bg-[#e8eef0] text-[#566162]'
		}[list.status] ?? 'bg-surface-low text-on-surface-muted'
	);

	const progress = $derived(itemCount > 0 ? Math.round((checkedCount / itemCount) * 100) : 0);
</script>

<div
	class="flex items-center gap-3 rounded-2xl bg-pure-white p-4 dark:bg-[var(--card-bg)] {className}"
	role="listitem"
>
	<span
		class="material-symbols-outlined text-xl text-primary dark:text-primary-container"
		aria-hidden="true"
	>
		shopping_cart
	</span>

	<div class="min-w-0 flex-1">
		<div class="flex items-center gap-2">
			<span class="font-body text-sm font-semibold text-on-surface dark:text-[var(--text-primary)]">
				{list.name}
			</span>
			<span
				class="inline-flex items-center rounded-full px-2 py-0.5 font-body text-[10px] font-medium {statusClasses}"
			>
				{list.status}
			</span>
		</div>

		{#if itemCount > 0}
			<div class="mt-2">
				<div class="flex items-center justify-between">
					<span class="font-body text-xs text-on-surface-muted dark:text-on-surface-subtle">
						{checkedCount}/{itemCount} items checked
					</span>
					<span class="font-body text-xs font-medium text-primary dark:text-primary-container">
						{progress}%
					</span>
				</div>
				<div class="mt-1 h-1.5 w-full overflow-hidden rounded-full bg-[#a9b4b5]/30">
					<div
						class="h-full rounded-full bg-primary transition-all duration-300 dark:bg-primary-container"
						style="width: {progress}%"
					></div>
				</div>
			</div>
		{:else}
			<p class="mt-1 font-body text-xs text-outline-variant dark:text-on-surface-muted">
				No items yet
			</p>
		{/if}
	</div>

	<span
		class="material-symbols-outlined shrink-0 text-xl text-outline-variant dark:text-on-surface-muted"
		aria-hidden="true"
	>
		chevron_right
	</span>
</div>
