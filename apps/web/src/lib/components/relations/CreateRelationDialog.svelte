<script lang="ts">
	import { syncStore } from '$lib/stores/sync.svelte';

	interface Props {
		fromEntityType: string;
		fromEntityId: string;
		open: boolean;
		onClose: () => void;
		onCreated: () => void;
	}

	let { fromEntityType, fromEntityId, open, onClose, onCreated }: Props = $props();

	let targetType = $state('knowledge_note');
	let targetId = $state('');
	let relationType = $state('related_to');
	let submitting = $state(false);
	let error = $state<string | null>(null);

	const entityTypeOptions = [
		{ value: 'knowledge_note', label: 'Knowledge Note' },
		{ value: 'tracking_item', label: 'Tracking Item' },
		{ value: 'guidance_quest', label: 'Guidance Quest' },
		{ value: 'guidance_routine', label: 'Guidance Routine' },
		{ value: 'initiative', label: 'Initiative' },
		{ value: 'household', label: 'Household' }
	];

	const relationTypeOptions = [
		{ value: 'related_to', label: 'Related to' },
		{ value: 'depends_on', label: 'Depends on' },
		{ value: 'references', label: 'References' },
		{ value: 'supports', label: 'Supports' },
		{ value: 'requires', label: 'Requires' },
		{ value: 'duplicates', label: 'Duplicates' },
		{ value: 'similar_to', label: 'Similar to' },
		{ value: 'generated_from', label: 'Generated from' }
	];

	const canSubmit = $derived(targetId.trim().length > 0 && !submitting);

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		if (!canSubmit) return;

		const db = syncStore.db;
		if (!db) {
			error = 'Database not available.';
			return;
		}

		submitting = true;
		error = null;

		try {
			await db.execute(
				'INSERT INTO entity_relations (id, from_entity_id, from_entity_type, to_entity_id, to_entity_type, relation_type, source_type, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)',
				[
					crypto.randomUUID(),
					fromEntityId,
					fromEntityType,
					targetId.trim(),
					targetType,
					relationType,
					'user',
					'accepted',
					new Date().toISOString(),
					new Date().toISOString()
				]
			);
			targetId = '';
			targetType = 'knowledge_note';
			relationType = 'related_to';
			onCreated();
			onClose();
		} catch (err) {
			console.error('[create-relation] Failed to create relation:', err);
			error = 'Could not create relation. Please try again.';
		}

		submitting = false;
	}

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			onClose();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			onClose();
		}
	}
</script>

{#if open}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-[#2a3435]/40 backdrop-blur-[20px]"
		onclick={handleBackdropClick}
		onkeydown={handleKeydown}
		role="dialog"
		aria-modal="true"
		aria-label="Link entity"
		tabindex="-1"
	>
		<div
			class="mx-4 w-full max-w-md rounded-2xl bg-[#f8fafa] p-6 shadow-[0_20px_40px_rgba(42,52,53,0.06)] dark:bg-[#1a2c2e]"
		>
			<div class="mb-6 flex items-center justify-between">
				<h2 class="font-display text-xl font-bold text-on-surface dark:text-[var(--text-primary)]">
					Link Entity
				</h2>
				<button
					type="button"
					class="transition-breathe rounded-full p-1.5 text-on-surface-muted hover:bg-surface-low dark:hover:bg-[#263638]"
					onclick={onClose}
					aria-label="Close dialog"
				>
					<span class="material-symbols-outlined text-xl">close</span>
				</button>
			</div>

			<form onsubmit={handleSubmit} class="space-y-5">
				<div>
					<label
						for="target-type"
						class="mb-1.5 block font-body text-sm font-medium text-on-surface dark:text-[#f0f4f5]"
					>
						Target entity type
					</label>
					<select
						id="target-type"
						bind:value={targetType}
						class="transition-breathe w-full rounded-xl bg-surface-low px-4 py-3 font-body text-on-surface focus:bg-pure-white focus:ring-1 focus:ring-primary/20 focus:outline-none dark:bg-[#1e2d2f] dark:text-[#f0f4f5] dark:focus:bg-[#263638] dark:focus:ring-primary/30"
					>
						{#each entityTypeOptions as opt (opt.value)}
							<option value={opt.value}>{opt.label}</option>
						{/each}
					</select>
				</div>

				<div>
					<label
						for="target-id"
						class="mb-1.5 block font-body text-sm font-medium text-on-surface dark:text-[#f0f4f5]"
					>
						Target entity ID
					</label>
					<input
						id="target-id"
						type="text"
						bind:value={targetId}
						placeholder="Enter entity UUID"
						class="transition-breathe w-full rounded-xl bg-surface-low px-4 py-3 font-body text-on-surface placeholder:text-on-surface-subtle focus:bg-pure-white focus:ring-1 focus:ring-primary/20 focus:outline-none dark:bg-[#1e2d2f] dark:text-[#f0f4f5] dark:placeholder:text-[#5a7a7c] dark:focus:bg-[#263638] dark:focus:ring-primary/30"
					/>
				</div>

				<div>
					<label
						for="relation-type"
						class="mb-1.5 block font-body text-sm font-medium text-on-surface dark:text-[#f0f4f5]"
					>
						Relation type
					</label>
					<select
						id="relation-type"
						bind:value={relationType}
						class="transition-breathe w-full rounded-xl bg-surface-low px-4 py-3 font-body text-on-surface focus:bg-pure-white focus:ring-1 focus:ring-primary/20 focus:outline-none dark:bg-[#1e2d2f] dark:text-[#f0f4f5] dark:focus:bg-[#263638] dark:focus:ring-primary/30"
					>
						{#each relationTypeOptions as opt (opt.value)}
							<option value={opt.value}>{opt.label}</option>
						{/each}
					</select>
				</div>

				{#if error}
					<p class="font-body text-sm text-error">{error}</p>
				{/if}

				<div class="flex items-center justify-end gap-3 pt-2">
					<button
						type="button"
						class="transition-breathe rounded-full px-5 py-2.5 font-body text-sm font-medium text-on-surface-muted hover:bg-surface-low dark:hover:bg-[#1e2d2f]"
						onclick={onClose}
					>
						Cancel
					</button>
					<button
						type="submit"
						disabled={!canSubmit}
						class="transition-breathe inline-flex min-h-[44px] items-center justify-center rounded-full bg-primary px-6 py-2.5 font-body text-sm font-semibold text-white hover:bg-primary-dim disabled:cursor-not-allowed disabled:opacity-50"
					>
						{#if submitting}
							<span class="material-symbols-outlined mr-2 animate-spin text-base"
								>progress_activity</span
							>
							Linking...
						{:else}
							<span class="material-symbols-outlined mr-2 text-base" aria-hidden="true"
								>add_link</span
							>
							Link
						{/if}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
