<script lang="ts">
	import type { Snippet } from 'svelte';
	import { resolve } from '$app/paths';

	interface Props {
		variant?: 'primary' | 'ghost' | 'tonal';
		size?: 'sm' | 'md' | 'lg';
		disabled?: boolean;
		href?: string;
		type?: 'button' | 'submit' | 'reset';
		onclick?: (e: MouseEvent) => void;
		children?: Snippet;
	}

	let {
		variant = 'primary',
		size = 'md',
		disabled = false,
		href,
		type = 'button',
		onclick,
		children
	}: Props = $props();

	const variantClasses = $derived(
		{
			primary:
				'bg-primary text-white hover:bg-primary-dim active:bg-primary-dim focus-visible:ring-2 focus-visible:ring-primary/40',
			ghost:
				'bg-transparent text-on-surface-muted hover:bg-surface-low active:bg-surface-container dark:hover:bg-[#1e2d2f] dark:active:bg-[#263638]',
			tonal:
				'bg-primary-container text-primary hover:bg-primary-fixed-dim active:bg-primary-fixed-dim dark:bg-[#2a4a5a] dark:text-primary-container dark:hover:bg-[#335566] dark:active:bg-[#335566]'
		}[variant]
	);

	const sizeClasses = $derived(
		{
			sm: 'px-4 py-2 text-sm',
			md: 'px-6 py-3',
			lg: 'px-8 py-4 text-lg'
		}[size]
	);
</script>

{#if href}
	<a
		href={resolve(href as '/')}
		class="transition-breathe inline-flex min-h-[48px] items-center justify-center rounded-full font-body font-semibold {variantClasses} {sizeClasses}"
		class:pointer-events-none={disabled}
		class:opacity-50={disabled}
		aria-disabled={disabled || undefined}
	>
		{#if children}
			{@render children()}
		{/if}
	</a>
{:else}
	<button
		{type}
		{disabled}
		{onclick}
		class="transition-breathe inline-flex min-h-[48px] cursor-pointer items-center justify-center rounded-full font-body font-semibold disabled:cursor-not-allowed disabled:opacity-50 {variantClasses} {sizeClasses}"
	>
		{#if children}
			{@render children()}
		{/if}
	</button>
{/if}
