<script lang="ts">
	import type { TrackingShoppingListItem } from '$lib/types/tracking.js';
	import { formatQuantity } from '$lib/utils/tracking-format.js';

	interface Props {
		item: TrackingShoppingListItem;
		onToggle: () => void;
	}

	let { item, onToggle }: Props = $props();

	const isChecked = $derived(item.is_checked === 1);
</script>

<button
	onclick={onToggle}
	class="transition-breathe flex w-full items-center gap-3 rounded-xl px-3 py-3 text-left hover:bg-surface-low dark:hover:bg-[#1e2d2f]"
>
	<!-- Checkbox circle -->
	<div
		class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full
			{isChecked ? 'bg-primary text-white' : 'border-2 border-[#d0dfe2] dark:border-[#3a5052]'}"
	>
		{#if isChecked}
			<span class="material-symbols-outlined text-[16px]" aria-hidden="true">check</span>
		{/if}
	</div>

	<!-- Item name -->
	<span
		class="min-w-0 flex-1 font-body text-sm text-on-surface dark:text-[var(--text-primary)]
			{isChecked ? 'line-through opacity-60' : ''}"
	>
		{item.name}
	</span>

	<!-- Quantity + unit -->
	{#if item.quantity > 0}
		<span
			class="shrink-0 font-body text-xs text-on-surface-muted dark:text-on-surface-subtle
				{isChecked ? 'opacity-60' : ''}"
		>
			{formatQuantity(item.quantity, item.unit)}
		</span>
	{/if}
</button>
