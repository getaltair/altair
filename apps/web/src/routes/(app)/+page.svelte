<script lang="ts">
	import { EntityType } from '$lib/contracts';
	import { resolve } from '$app/paths';
</script>

<svelte:head>
	<title>Altair</title>
</svelte:head>

<div class="space-y-8">
	<div>
		<h1 class="text-2xl font-bold tracking-tight text-slate-900 dark:text-white">
			Welcome to Altair
		</h1>
		<p class="mt-2 text-base text-slate-600 dark:text-slate-400">
			Your household command center. Organize guidance quests, capture knowledge, and track what
			matters.
		</p>
	</div>

	<nav class="grid gap-4 sm:grid-cols-3" aria-label="Quick navigation">
		<a
			href={resolve('/guidance')}
			class="group rounded-xl border border-slate-200 bg-white p-5 transition-shadow hover:shadow-md dark:border-slate-700 dark:bg-slate-900"
		>
			<h2
				class="text-lg font-semibold text-slate-900 group-hover:text-indigo-600 dark:text-white dark:group-hover:text-indigo-400"
			>
				Guidance
			</h2>
			<p class="mt-1 text-sm text-slate-500 dark:text-slate-400">
				Manage quests, routines, and focus sessions.
			</p>
			<span
				class="mt-3 inline-block rounded-full bg-indigo-50 px-2.5 py-0.5 text-xs font-medium text-indigo-700 dark:bg-indigo-950 dark:text-indigo-300"
			>
				{EntityType.GUIDANCE_QUEST}
			</span>
		</a>

		<a
			href={resolve('/knowledge')}
			class="group rounded-xl border border-slate-200 bg-white p-5 transition-shadow hover:shadow-md dark:border-slate-700 dark:bg-slate-900"
		>
			<h2
				class="text-lg font-semibold text-slate-900 group-hover:text-indigo-600 dark:text-white dark:group-hover:text-indigo-400"
			>
				Knowledge
			</h2>
			<p class="mt-1 text-sm text-slate-500 dark:text-slate-400">
				Capture notes, ideas, and reference material.
			</p>
			<span
				class="mt-3 inline-block rounded-full bg-indigo-50 px-2.5 py-0.5 text-xs font-medium text-indigo-700 dark:bg-indigo-950 dark:text-indigo-300"
			>
				{EntityType.KNOWLEDGE_NOTE}
			</span>
		</a>

		<a
			href={resolve('/tracking')}
			class="group rounded-xl border border-slate-200 bg-white p-5 transition-shadow hover:shadow-md dark:border-slate-700 dark:bg-slate-900"
		>
			<h2
				class="text-lg font-semibold text-slate-900 group-hover:text-indigo-600 dark:text-white dark:group-hover:text-indigo-400"
			>
				Tracking
			</h2>
			<p class="mt-1 text-sm text-slate-500 dark:text-slate-400">
				Track items, locations, and shopping lists.
			</p>
			<span
				class="mt-3 inline-block rounded-full bg-indigo-50 px-2.5 py-0.5 text-xs font-medium text-indigo-700 dark:bg-indigo-950 dark:text-indigo-300"
			>
				{EntityType.TRACKING_ITEM}
			</span>
		</a>
	</nav>
</div>
