<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		surface?: 'default' | 'low' | 'high' | 'highest';
		class?: string;
		children?: Snippet;
	}

	let { surface = 'default', class: className = '', children }: Props = $props();

	const surfaceClasses = $derived(
		{
			default: 'bg-pure-white dark:bg-[var(--card-bg)]',
			low: 'bg-surface-low dark:bg-[#1e2d2f]',
			high: 'bg-surface-high dark:bg-[#263638]',
			highest: 'bg-surface-highest dark:bg-[#2e4446]'
		}[surface] ?? 'bg-pure-white dark:bg-[var(--card-bg)]'
	);
</script>

<div class="rounded-2xl p-4 {surfaceClasses} {className}">
	{#if children}
		{@render children()}
	{/if}
</div>
