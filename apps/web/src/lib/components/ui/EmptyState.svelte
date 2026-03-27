<script lang="ts">
	import Button from './Button.svelte';

	interface Props {
		title: string;
		description?: string;
		ctaLabel?: string;
		ctaHref?: string;
		onCta?: () => void;
		icon?: string;
	}

	let { title, description, ctaLabel, ctaHref, onCta, icon }: Props = $props();
</script>

<div class="flex flex-col items-center justify-center px-8 py-16 text-center">
	{#if icon}
		<span class="material-symbols-outlined mb-4 text-4xl text-on-surface-subtle">{icon}</span>
	{/if}

	<h3 class="font-body text-lg font-semibold text-on-surface dark:text-[#f0f4f5]">
		{title}
	</h3>

	{#if description}
		<p class="mt-2 max-w-sm font-body text-sm text-on-surface-muted dark:text-on-surface-subtle">
			{description}
		</p>
	{/if}

	{#if ctaLabel}
		<div class="mt-6">
			{#if ctaHref}
				<Button variant="primary" href={ctaHref}>{ctaLabel}</Button>
			{:else}
				<Button variant="primary" onclick={onCta ? () => onCta() : undefined}>{ctaLabel}</Button>
			{/if}
		</div>
	{/if}
</div>
