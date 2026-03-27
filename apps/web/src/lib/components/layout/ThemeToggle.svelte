<script lang="ts">
	import { theme, toggle } from '$lib/stores/theme.svelte';
</script>

<button
	onclick={toggle}
	aria-label={theme.current === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'}
	class="transition-breathe rounded-xl p-2 text-on-surface-muted hover:bg-surface-container focus:outline-none focus-visible:ring-2 focus-visible:ring-primary dark:text-on-surface-subtle dark:hover:bg-[#263638]"
>
	{#if theme.current === 'dark'}
		<span class="material-symbols-outlined text-[20px]" aria-hidden="true">light_mode</span>
	{:else}
		<span class="material-symbols-outlined text-[20px]" aria-hidden="true">dark_mode</span>
	{/if}
</button>
