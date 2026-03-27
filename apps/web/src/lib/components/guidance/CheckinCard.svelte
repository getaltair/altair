<script lang="ts">
	import type { GuidanceDailyCheckin } from '$lib/types/guidance.js';
	import Button from '$lib/components/ui/Button.svelte';

	interface Props {
		checkin: GuidanceDailyCheckin | null;
		class?: string;
	}

	let { checkin, class: className = '' }: Props = $props();

	const energyDots = $derived.by(() => {
		const level = checkin?.energy_level ?? 0;
		return Array.from({ length: 5 }, (_, i) => i < level);
	});
</script>

<div class="rounded-2xl bg-white p-4 dark:bg-[var(--card-bg)] {className}">
	{#if !checkin}
		<!-- No check-in state -->
		<div class="flex items-center justify-between">
			<span class="font-body text-sm text-on-surface-muted dark:text-on-surface-subtle">
				No check-in yet today
			</span>
			<Button variant="ghost" size="sm">Add Check-in</Button>
		</div>
	{:else}
		<!-- Check-in data -->
		<div class="space-y-3">
			<!-- Energy level -->
			<div class="flex items-center gap-3">
				<span
					class="font-body text-xs font-semibold tracking-widest text-on-surface-muted uppercase dark:text-on-surface-subtle"
				>
					Energy
				</span>
				<div class="flex gap-1.5" aria-label="Energy level {checkin.energy_level ?? 0} out of 5">
					{#each energyDots as filled, i (i)}
						<span
							class="h-2.5 w-2.5 rounded-full {filled
								? 'bg-primary'
								: 'bg-[#d0dfe2] dark:bg-[#3a5052]'}"
						></span>
					{/each}
				</div>
			</div>

			<!-- Mood -->
			<div class="flex items-center gap-3">
				<span
					class="font-body text-xs font-semibold tracking-widest text-on-surface-muted uppercase dark:text-on-surface-subtle"
				>
					Mood
				</span>
				<span class="font-body text-sm text-on-surface dark:text-[var(--text-primary)]">
					{checkin.mood ?? '--'}
				</span>
			</div>

			<!-- Notes -->
			{#if checkin.notes}
				<p class="line-clamp-2 font-body text-sm text-on-surface-muted dark:text-on-surface-subtle">
					{checkin.notes}
				</p>
			{/if}
		</div>
	{/if}
</div>
