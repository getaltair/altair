<script lang="ts">
	import type { FormEventHandler } from 'svelte/elements';

	interface Props {
		type?: string;
		placeholder?: string;
		value?: string;
		label?: string;
		id?: string;
		oninput?: FormEventHandler<HTMLInputElement>;
		onchange?: FormEventHandler<HTMLInputElement>;
	}

	let {
		type = 'text',
		placeholder = '',
		value = $bindable(''),
		label,
		id,
		oninput,
		onchange
	}: Props = $props();

	const inputId = $derived(
		id ?? (label ? `input-${label.toLowerCase().replace(/\s+/g, '-')}` : undefined)
	);
</script>

{#if label}
	<label
		for={inputId}
		class="mb-1.5 block font-body text-sm font-medium text-on-surface dark:text-[#f0f4f5]"
	>
		{label}
	</label>
{/if}
<input
	id={inputId}
	{type}
	{placeholder}
	bind:value
	{oninput}
	{onchange}
	class="transition-breathe w-full rounded-xl bg-surface-low px-4 py-3 font-body text-on-surface placeholder:text-on-surface-subtle focus:bg-pure-white focus:ring-1 focus:ring-primary/20 focus:outline-none dark:bg-[#1e2d2f] dark:text-[#f0f4f5] dark:placeholder:text-[#5a7a7c] dark:focus:bg-[#263638] dark:focus:ring-primary/30"
/>
