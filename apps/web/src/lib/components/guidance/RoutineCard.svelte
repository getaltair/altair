<script lang="ts">
	import type { GuidanceRoutine } from '$lib/types/guidance.js';

	interface Props {
		routine: GuidanceRoutine;
		class?: string;
	}

	let { routine, class: className = '' }: Props = $props();

	const statusColor = $derived(
		{
			active: 'bg-[#a8c5a0]',
			paused: 'bg-[#d4a853]',
			archived: 'bg-[#a9b4b5]'
		}[routine.status] ?? 'bg-outline-variant'
	);
</script>

<div
	class="flex items-center justify-between rounded-2xl bg-white p-4 dark:bg-[var(--card-bg)] {className}"
	role="listitem"
>
	<div class="flex items-center gap-3">
		<!-- Status dot -->
		<span class="h-2 w-2 shrink-0 rounded-full {statusColor}" aria-label="{routine.status} status"
		></span>
		<span class="font-body text-sm font-semibold text-on-surface dark:text-[var(--text-primary)]">
			{routine.name}
		</span>
	</div>

	<!-- Frequency badge -->
	<span
		class="rounded-full bg-surface-low px-2 py-0.5 font-body text-xs text-on-surface-muted dark:bg-surface-high dark:text-on-surface-subtle"
	>
		{routine.frequency}
	</span>
</div>
