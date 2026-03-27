<script lang="ts">
	import { onMount } from 'svelte';
	import { syncStore } from '$lib/stores/sync.svelte';
	import type { EntityRelation } from '$lib/types/relations.js';
	import { Card, EmptyState, SectionLabel } from '$lib/components/ui/index.js';
	import CreateRelationDialog from './CreateRelationDialog.svelte';

	interface Props {
		entityType: string;
		entityId: string;
	}

	let { entityType, entityId }: Props = $props();

	let outgoing = $state<EntityRelation[]>([]);
	let incoming = $state<EntityRelation[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let dialogOpen = $state(false);

	const hasRelations = $derived(outgoing.length > 0 || incoming.length > 0);

	async function loadRelations() {
		const db = syncStore.db;
		if (!db) return;

		try {
			outgoing = await db.getAll<EntityRelation>(
				'SELECT * FROM entity_relations WHERE from_entity_id = ? ORDER BY created_at DESC',
				[entityId]
			);
			incoming = await db.getAll<EntityRelation>(
				'SELECT * FROM entity_relations WHERE to_entity_id = ? ORDER BY created_at DESC',
				[entityId]
			);
		} catch (err) {
			console.error('[relations-panel] Failed to load relations:', err);
			error = 'Could not load relations.';
		}

		loading = false;
	}

	onMount(() => {
		loadRelations();
	});

	function handleCreated() {
		loadRelations();
	}

	function formatEntityType(type: string): string {
		return type
			.split('_')
			.map((w) => w.charAt(0).toUpperCase() + w.slice(1))
			.join(' ');
	}

	function formatRelationType(type: string): string {
		return type
			.split('_')
			.map((w) => w.charAt(0).toUpperCase() + w.slice(1))
			.join(' ');
	}

	function truncateId(id: string): string {
		return id.length > 8 ? id.slice(0, 8) : id;
	}
</script>

<section>
	<div class="mb-4 flex items-center justify-between">
		<SectionLabel text="RELATIONS" />
		<button
			type="button"
			class="transition-breathe inline-flex items-center gap-1.5 rounded-full px-4 py-2 font-body text-sm font-medium text-primary hover:bg-primary-container/40 dark:text-primary-container dark:hover:bg-[#2a4a5a]/40"
			onclick={() => (dialogOpen = true)}
		>
			<span class="material-symbols-outlined text-base" aria-hidden="true">add_link</span>
			Link entity
		</button>
	</div>

	{#if loading}
		<div class="space-y-3">
			<div class="h-14 animate-pulse rounded-2xl bg-surface-low dark:bg-surface-high"></div>
			<div class="h-14 animate-pulse rounded-2xl bg-surface-low dark:bg-surface-high"></div>
		</div>
	{:else if error}
		<Card>
			<div class="flex flex-col items-center gap-3 py-4 text-center">
				<span class="material-symbols-outlined text-2xl text-error">cloud_off</span>
				<p class="font-body text-sm text-on-surface-muted">{error}</p>
			</div>
		</Card>
	{:else if !hasRelations}
		<EmptyState
			title="No relations yet"
			description="Link this to other items to build connections across your workspace."
			icon="hub"
			ctaLabel="Link entity"
			onCta={() => (dialogOpen = true)}
		/>
	{:else}
		{#if outgoing.length > 0}
			<div class="mb-6">
				<p
					class="mb-2 flex items-center gap-1.5 font-body text-xs font-medium tracking-wide text-on-surface-muted uppercase dark:text-on-surface-subtle"
				>
					<span class="material-symbols-outlined text-sm" aria-hidden="true">arrow_forward</span>
					Links to
				</p>
				<div class="space-y-2" role="list">
					{#each outgoing as relation (relation.id)}
						<Card
							class={relation.source_type === 'ai' || relation.source_type === 'ai_suggested'
								? 'border border-dashed border-outline-variant/30 dark:border-[#3a5a5c]/50'
								: ''}
						>
							<div class="flex items-center justify-between" role="listitem">
								<div class="flex items-center gap-3">
									<span
										class="material-symbols-outlined text-lg text-primary dark:text-primary-container"
										aria-hidden="true">arrow_outward</span
									>
									<div>
										<p
											class="font-body text-sm font-semibold text-on-surface dark:text-[var(--text-primary)]"
										>
											{formatEntityType(relation.to_entity_type)}
										</p>
										<p class="font-body text-xs text-on-surface-muted dark:text-on-surface-subtle">
											{truncateId(relation.to_entity_id)}...
										</p>
									</div>
								</div>
								<div class="flex items-center gap-2">
									<span
										class="inline-flex items-center rounded-full bg-primary-container/40 px-2.5 py-0.5 font-body text-xs font-medium text-primary dark:bg-[#2a4a5a]/60 dark:text-primary-container"
									>
										{formatRelationType(relation.relation_type)}
									</span>
									{#if relation.source_type === 'ai' || relation.source_type === 'ai_suggested'}
										<span
											class="material-symbols-outlined text-sm text-outline-variant dark:text-on-surface-muted"
											title="AI suggested"
											aria-label="AI suggested relation">auto_awesome</span
										>
									{/if}
								</div>
							</div>
						</Card>
					{/each}
				</div>
			</div>
		{/if}

		{#if incoming.length > 0}
			<div>
				<p
					class="mb-2 flex items-center gap-1.5 font-body text-xs font-medium tracking-wide text-on-surface-muted uppercase dark:text-on-surface-subtle"
				>
					<span class="material-symbols-outlined text-sm" aria-hidden="true">arrow_back</span>
					Linked from
				</p>
				<div class="space-y-2" role="list">
					{#each incoming as relation (relation.id)}
						<Card
							class={relation.source_type === 'ai' || relation.source_type === 'ai_suggested'
								? 'border border-dashed border-outline-variant/30 dark:border-[#3a5a5c]/50'
								: ''}
						>
							<div class="flex items-center justify-between" role="listitem">
								<div class="flex items-center gap-3">
									<span
										class="material-symbols-outlined text-lg text-secondary dark:text-[#d1e6f0]"
										aria-hidden="true">arrow_inward</span
									>
									<div>
										<p
											class="font-body text-sm font-semibold text-on-surface dark:text-[var(--text-primary)]"
										>
											{formatEntityType(relation.from_entity_type)}
										</p>
										<p class="font-body text-xs text-on-surface-muted dark:text-on-surface-subtle">
											{truncateId(relation.from_entity_id)}...
										</p>
									</div>
								</div>
								<div class="flex items-center gap-2">
									<span
										class="inline-flex items-center rounded-full bg-secondary-container/40 px-2.5 py-0.5 font-body text-xs font-medium text-secondary dark:bg-[#2a3e48]/60 dark:text-[#d1e6f0]"
									>
										{formatRelationType(relation.relation_type)}
									</span>
									{#if relation.source_type === 'ai' || relation.source_type === 'ai_suggested'}
										<span
											class="material-symbols-outlined text-sm text-outline-variant dark:text-on-surface-muted"
											title="AI suggested"
											aria-label="AI suggested relation">auto_awesome</span
										>
									{/if}
								</div>
							</div>
						</Card>
					{/each}
				</div>
			</div>
		{/if}
	{/if}
</section>

<CreateRelationDialog
	fromEntityType={entityType}
	fromEntityId={entityId}
	open={dialogOpen}
	onClose={() => (dialogOpen = false)}
	onCreated={handleCreated}
/>
