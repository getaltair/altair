<script lang="ts">
	import type { Snippet } from 'svelte';
	import Sidebar from './Sidebar.svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	let sidebarOpen = $state(false);
</script>

<div class="min-h-screen bg-slate-50 dark:bg-slate-950">
	<Sidebar open={sidebarOpen} onclose={() => (sidebarOpen = false)} />

	<!-- Mobile header with hamburger -->
	<header
		class="sticky top-0 z-20 flex h-14 items-center border-b border-slate-200 bg-white/80 px-4 backdrop-blur-sm lg:hidden dark:border-slate-700 dark:bg-slate-900/80"
	>
		<button
			onclick={() => (sidebarOpen = true)}
			aria-label="Open navigation"
			class="rounded-lg p-2 text-slate-600 hover:bg-slate-100 focus:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500 dark:text-slate-400 dark:hover:bg-slate-800"
		>
			<!-- Hamburger icon -->
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="22"
				height="22"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				aria-hidden="true"
			>
				<line x1="3" y1="6" x2="21" y2="6" />
				<line x1="3" y1="12" x2="21" y2="12" />
				<line x1="3" y1="18" x2="21" y2="18" />
			</svg>
		</button>
		<span class="ml-3 text-base font-semibold text-slate-900 dark:text-white">Altair</span>
	</header>

	<!-- Main content area: offset by sidebar on desktop -->
	<main class="lg:pl-64">
		<div class="mx-auto max-w-5xl px-4 py-6 sm:px-6 lg:px-8 lg:py-8">
			{@render children()}
		</div>
	</main>
</div>
