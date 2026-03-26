<script lang="ts">
	import { EntityType } from '$lib/contracts';
</script>

<svelte:head>
	<title>Knowledge - Altair</title>
</svelte:head>

<div class="space-y-4">
	<div>
		<h1 class="text-2xl font-bold tracking-tight text-slate-900 dark:text-white">Knowledge</h1>
		<p class="mt-1 text-base text-slate-600 dark:text-slate-400">
			Capture notes, bookmark ideas, and organize reference material.
		</p>
	</div>

	<div
		class="rounded-lg border border-slate-200 bg-white p-6 dark:border-slate-700 dark:bg-slate-900"
	>
		<p class="text-sm text-slate-500 dark:text-slate-400">
			Entity type: <code
				class="rounded bg-slate-100 px-1.5 py-0.5 font-mono text-xs text-indigo-700 dark:bg-slate-800 dark:text-indigo-300"
				>{EntityType.KNOWLEDGE_NOTE}</code
			>
		</p>
		<p class="mt-3 text-sm text-slate-500 dark:text-slate-400">
			This page will display knowledge notes and their snapshots.
		</p>
	</div>
</div>
