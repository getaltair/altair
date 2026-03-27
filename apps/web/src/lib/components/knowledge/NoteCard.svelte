<script lang="ts">
	import type { KnowledgeNote } from '$lib/types/knowledge.js';
	import { contentTypeLabel, truncateContent, formatDate } from '$lib/utils/knowledge-format.js';

	interface Props {
		note: KnowledgeNote;
		class?: string;
	}

	let { note, class: className = '' }: Props = $props();

	const preview = $derived(note.content ? truncateContent(note.content) : '');
</script>

<div class="rounded-2xl bg-white p-4 dark:bg-[var(--card-bg)] {className}" role="listitem">
	<div class="flex items-start justify-between gap-3">
		<div class="min-w-0 flex-1">
			<div class="flex items-center gap-2">
				<span
					class="font-display text-sm font-bold text-on-surface dark:text-[var(--text-primary)]"
				>
					{note.title}
				</span>
				{#if note.is_pinned === 1}
					<span
						class="inline-flex items-center gap-0.5 rounded-full bg-[#f0e6c8]/50 px-2 py-0.5 font-body text-[10px] font-semibold tracking-wide text-[#8a6a2a] uppercase"
					>
						<span class="material-symbols-outlined text-[12px]" aria-hidden="true">push_pin</span>
						Pinned
					</span>
				{/if}
			</div>

			{#if preview}
				<p
					class="mt-1.5 font-body text-sm leading-relaxed text-on-surface-muted dark:text-on-surface-subtle"
				>
					{preview}
				</p>
			{/if}

			<div class="mt-2 flex items-center gap-2">
				<span
					class="inline-flex rounded-full bg-primary-container/40 px-2 py-0.5 font-body text-[10px] font-semibold tracking-wide text-primary uppercase dark:bg-[#2a4a5a] dark:text-primary-container"
				>
					{contentTypeLabel(note.content_type)}
				</span>
				<span class="font-body text-xs text-outline-variant dark:text-on-surface-muted">
					{formatDate(note.created_at)}
				</span>
			</div>
		</div>
	</div>
</div>
