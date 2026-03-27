<script lang="ts">
	import type { GuidanceQuest } from '$lib/types/guidance.js';

	interface Props {
		quest: GuidanceQuest;
		onComplete?: (id: string) => Promise<void>;
		class?: string;
	}

	let { quest, onComplete, class: className = '' }: Props = $props();

	// Optimistic override: null means "use quest.status", true means "user marked complete"
	let optimisticComplete = $state<boolean | null>(null);

	const localCompleted = $derived(
		optimisticComplete !== null ? optimisticComplete : quest.status === 'completed'
	);

	async function handleToggle() {
		if (localCompleted) return;
		optimisticComplete = true;
		try {
			await onComplete?.(quest.id);
		} catch {
			optimisticComplete = null;
		}
	}

	const priorityColor = $derived(
		{
			critical: 'bg-[#c97c5d]',
			high: 'bg-[#c97c5d]',
			medium: 'bg-[#d4a853]',
			low: 'bg-[#a8c5a0]'
		}[quest.priority] ?? 'bg-outline-variant'
	);

	const formattedDueDate = $derived.by(() => {
		if (!quest.due_date) return null;
		const d = new Date(quest.due_date);
		return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
	});
</script>

<div
	class="flex items-start gap-3 rounded-2xl bg-white p-4 dark:bg-[var(--card-bg)] {className}"
	role="listitem"
>
	<!-- Completion circle button -->
	<button
		onclick={handleToggle}
		disabled={localCompleted}
		aria-label={localCompleted ? 'Quest completed' : 'Mark quest as complete'}
		class="transition-breathe mt-0.5 flex h-6 w-6 shrink-0 items-center justify-center rounded-full
			{localCompleted
			? 'bg-primary text-white'
			: 'border-2 border-[#d0dfe2] hover:border-primary dark:border-[#3a5052]'}"
	>
		{#if localCompleted}
			<span class="material-symbols-outlined text-[16px]" aria-hidden="true">check</span>
		{/if}
	</button>

	<!-- Quest content -->
	<div class="min-w-0 flex-1 {localCompleted ? 'opacity-60' : ''}">
		<div class="flex items-center gap-2">
			<span
				class="font-body text-sm font-semibold text-on-surface dark:text-[var(--text-primary)]
					{localCompleted ? 'line-through' : ''}"
			>
				{quest.name}
			</span>
			<!-- Priority dot -->
			<span
				class="h-1.5 w-1.5 shrink-0 rounded-full {priorityColor}"
				aria-label="{quest.priority} priority"
			></span>
		</div>

		<div class="mt-1 flex items-center gap-2">
			{#if quest.initiative_id}
				<span class="font-body text-xs text-on-surface-muted dark:text-on-surface-subtle">
					Initiative
				</span>
			{/if}
			{#if formattedDueDate}
				<span class="font-body text-xs text-on-surface-muted dark:text-on-surface-subtle">
					Due {formattedDueDate}
				</span>
			{/if}
		</div>
	</div>
</div>
