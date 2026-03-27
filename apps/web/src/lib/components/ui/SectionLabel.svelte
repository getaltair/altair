<script lang="ts">
	interface Props {
		text?: string;
		class?: string;
	}

	let { text, class: className = '' }: Props = $props();
</script>

<div
	class="mb-3 font-body text-xs font-semibold tracking-widest text-on-surface-muted uppercase dark:text-on-surface-subtle {className}"
>
	{#if text}
		{text}
	{/if}
</div>
