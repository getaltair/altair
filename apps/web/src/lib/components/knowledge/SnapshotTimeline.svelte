<script lang="ts">
	import type { KnowledgeNoteSnapshot } from '$lib/types/knowledge.js';
	import { formatDate, truncateContent } from '$lib/utils/knowledge-format.js';

	interface Props {
		snapshots: KnowledgeNoteSnapshot[];
	}

	let { snapshots }: Props = $props();
</script>

<div class="relative" role="list" aria-label="Snapshot history">
	<!-- Timeline connector line -->
	{#if snapshots.length > 1}
		<div
			class="absolute top-3 bottom-3 left-3 w-0.5 bg-surface-container dark:bg-surface-high"
			aria-hidden="true"
		></div>
	{/if}

	<div class="space-y-6">
		{#each snapshots as snapshot, i (snapshot.id)}
			<div class="relative flex gap-4" role="listitem">
				<!-- Timeline dot -->
				<div
					class="relative z-10 mt-1 h-6 w-6 shrink-0 rounded-full {i === 0
						? 'bg-primary dark:bg-primary-container'
						: 'bg-surface-high dark:bg-[#263638]'}"
					aria-hidden="true"
				>
					<span
						class="material-symbols-outlined absolute inset-0 flex items-center justify-center text-[14px] {i ===
						0
							? 'text-white dark:text-[#1a2c2e]'
							: 'text-on-surface-muted dark:text-on-surface-subtle'}"
					>
						history
					</span>
				</div>

				<!-- Snapshot content -->
				<div class="min-w-0 flex-1 rounded-2xl bg-white p-4 dark:bg-[var(--card-bg)]">
					<div class="flex items-center gap-3">
						<span
							class="font-body text-sm font-semibold text-on-surface dark:text-[var(--text-primary)]"
						>
							{formatDate(snapshot.created_at)}
						</span>
						{#if snapshot.created_by_process}
							<span
								class="inline-flex rounded-full bg-surface-low px-2 py-0.5 font-body text-[10px] font-semibold tracking-wide text-on-surface-muted uppercase dark:bg-surface-high dark:text-on-surface-subtle"
							>
								{snapshot.created_by_process}
							</span>
						{/if}
					</div>

					<p
						class="mt-2 font-body text-sm leading-relaxed text-on-surface-muted dark:text-on-surface-subtle"
					>
						{truncateContent(snapshot.content, 200)}
					</p>
				</div>
			</div>
		{/each}
	</div>
</div>
