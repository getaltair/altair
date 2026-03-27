<script lang="ts">
	import type { Snippet } from 'svelte';
	import Sidebar from './Sidebar.svelte';
	import OfflineIndicator from '$lib/components/ui/OfflineIndicator.svelte';
	import { syncStore } from '$lib/stores/sync.svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	let sidebarOpen = $state(false);
</script>

<svelte:head>
	<link
		rel="stylesheet"
		href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200"
	/>
</svelte:head>

<div class="min-h-screen bg-surface dark:bg-[#1a2526]">
	<Sidebar open={sidebarOpen} onclose={() => (sidebarOpen = false)} />

	<!-- Mobile header -->
	<header
		class="sticky top-0 z-20 flex h-14 items-center bg-surface-low/80 px-4 backdrop-blur-sm lg:hidden dark:bg-[#141e1f]/80"
	>
		<button
			onclick={() => (sidebarOpen = true)}
			aria-label="Open navigation"
			class="transition-breathe rounded-xl p-2 text-on-surface-muted hover:bg-surface-container focus:outline-none focus-visible:ring-2 focus-visible:ring-primary dark:text-on-surface-subtle dark:hover:bg-[#263638]"
		>
			<span class="material-symbols-outlined text-[22px]" aria-hidden="true">menu</span>
		</button>
		<div class="ml-3 flex items-center gap-2">
			<div class="h-2 w-2 rounded bg-primary"></div>
			<span class="font-display text-base font-bold text-on-surface dark:text-[#f0f4f5]"
				>Altair</span
			>
		</div>
	</header>

	<!-- Main content area: offset by sidebar on desktop -->
	<main class="lg:pl-64">
		<OfflineIndicator status={syncStore.syncStatus} />
		<div class="mx-auto max-w-[1200px] p-6 md:p-8">
			{@render children()}
		</div>
	</main>
</div>
