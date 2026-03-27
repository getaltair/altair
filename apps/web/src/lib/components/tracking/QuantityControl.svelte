<script lang="ts">
	import { formatQuantity } from '$lib/utils/tracking-format.js';

	interface Props {
		quantity: number;
		unit?: string | null;
		onAdjust: (change: number) => void;
	}

	let { quantity, unit, onAdjust }: Props = $props();

	function decrement() {
		if (quantity > 0) {
			onAdjust(-1);
		}
	}

	function increment() {
		onAdjust(1);
	}
</script>

<div class="inline-flex items-center gap-3 rounded-full bg-surface-low p-1 dark:bg-[#1e2d2f]">
	<button
		onclick={decrement}
		disabled={quantity <= 0}
		aria-label="Decrease quantity"
		class="transition-breathe flex h-10 w-10 items-center justify-center rounded-full bg-primary text-white hover:bg-primary-dim disabled:cursor-not-allowed disabled:opacity-40"
	>
		<span class="material-symbols-outlined text-xl" aria-hidden="true">remove</span>
	</button>

	<span
		class="min-w-[4rem] text-center font-display text-lg font-bold text-on-surface dark:text-[var(--text-primary)]"
	>
		{formatQuantity(quantity, unit)}
	</span>

	<button
		onclick={increment}
		aria-label="Increase quantity"
		class="transition-breathe flex h-10 w-10 items-center justify-center rounded-full bg-primary text-white hover:bg-primary-dim"
	>
		<span class="material-symbols-outlined text-xl" aria-hidden="true">add</span>
	</button>
</div>
