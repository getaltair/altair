<script lang="ts">
	import { EntityType } from '$lib/contracts';
</script>

<svelte:head>
	<title>Guidance - Altair</title>
</svelte:head>

<div class="space-y-4">
	<div>
		<h1 class="text-2xl font-bold tracking-tight text-slate-900 dark:text-white">Guidance</h1>
		<p class="mt-1 text-base text-slate-600 dark:text-slate-400">
			Plan quests, build routines, and stay focused on what matters.
		</p>
	</div>

	<div
		class="rounded-lg border border-slate-200 bg-white p-6 dark:border-slate-700 dark:bg-slate-900"
	>
		<p class="text-sm text-slate-500 dark:text-slate-400">
			Entity type: <code
				class="rounded bg-slate-100 px-1.5 py-0.5 font-mono text-xs text-indigo-700 dark:bg-slate-800 dark:text-indigo-300"
				>{EntityType.GUIDANCE_QUEST}</code
			>
		</p>
		<p class="mt-3 text-sm text-slate-500 dark:text-slate-400">
			This page will list guidance quests, epics, routines, and focus sessions.
		</p>
	</div>
</div>
