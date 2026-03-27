<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import { syncStore } from '$lib/stores/sync.svelte';
	import type { KnowledgeNote } from '$lib/types/knowledge.js';
	import { contentTypeLabel, formatDate } from '$lib/utils/knowledge-format.js';
	import MarkdownEditor from '$lib/components/knowledge/MarkdownEditor.svelte';
	import RelationsPanel from '$lib/components/relations/RelationsPanel.svelte';
	import { BackLink, Button, Card, EmptyState, SectionLabel } from '$lib/components/ui/index.js';

	const id = $derived(page.params.id ?? '');
	let note = $state<KnowledgeNote | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let saving = $state(false);
	let showDeleteConfirm = $state(false);

	// Editable fields
	let editContent = $state('');
	let editContentType = $state('markdown');

	onMount(async () => {
		error = null;
		if (!id) {
			error = 'Invalid note ID.';
			loading = false;
			return;
		}
		try {
			note = await syncStore.queryNote(id);
			if (note) {
				editContent = note.content ?? '';
				editContentType = note.content_type;
			}
		} catch (err) {
			console.error('[note-detail] Failed to load:', err);
			error = 'Could not load data. Please try again.';
		}
		loading = false;
	});

	async function handleSave() {
		if (!note || saving) return;
		saving = true;
		try {
			const db = syncStore.db;
			if (db) {
				await db.execute(
					'UPDATE knowledge_notes SET content = ?, content_type = ?, updated_at = ? WHERE id = ?',
					[editContent, editContentType, new Date().toISOString(), id]
				);
				note = await syncStore.queryNote(id);
			}
		} catch (err) {
			console.error('[note-detail] Failed to save:', err);
			error = 'Could not save changes. Please try again.';
		}
		saving = false;
	}

	async function handleTogglePin() {
		if (!note) return;
		try {
			const db = syncStore.db;
			if (db) {
				const newPinned = note.is_pinned === 1 ? 0 : 1;
				await db.execute('UPDATE knowledge_notes SET is_pinned = ?, updated_at = ? WHERE id = ?', [
					newPinned,
					new Date().toISOString(),
					id
				]);
				note = await syncStore.queryNote(id);
			}
		} catch (err) {
			console.error('[note-detail] Failed to toggle pin:', err);
			error = 'Could not update pin status. Please try again.';
		}
	}

	async function handleDelete() {
		if (!note) return;
		try {
			const db = syncStore.db;
			if (db) {
				await db.execute('DELETE FROM knowledge_notes WHERE id = ?', [id]);
				goto(resolve('/knowledge/notes' as '/'));
			}
		} catch (err) {
			console.error('[note-detail] Failed to delete:', err);
			error = 'Could not delete note. Please try again.';
		}
	}
</script>

<svelte:head>
	<title>{note?.title ?? 'Note'} - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<BackLink href="/knowledge/notes" label="Notes" />

	{#if loading}
		<div class="space-y-4">
			<div class="h-10 w-64 animate-pulse rounded-lg bg-surface-low dark:bg-surface-high"></div>
			<div class="flex gap-2">
				<div class="h-6 w-20 animate-pulse rounded-full bg-surface-low dark:bg-surface-high"></div>
				<div class="h-6 w-16 animate-pulse rounded-full bg-surface-low dark:bg-surface-high"></div>
			</div>
			<div class="h-48 w-full animate-pulse rounded-2xl bg-surface-low dark:bg-surface-high"></div>
		</div>
	{:else if error}
		<Card>
			<div class="flex flex-col items-center gap-3 py-6 text-center">
				<span class="material-symbols-outlined text-3xl text-error">cloud_off</span>
				<p class="font-body text-sm text-on-surface-muted">{error}</p>
			</div>
		</Card>
	{:else if !note}
		<EmptyState
			title="Note not found"
			description="This note may have been removed or is not available locally."
			icon="error_outline"
			ctaLabel="Back to notes"
			ctaHref="/knowledge/notes"
		/>
	{:else}
		<!-- Note header -->
		<div class="mb-8">
			<h1 class="font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
				{note.title}
			</h1>

			<div class="mt-3 flex flex-wrap items-center gap-3">
				<span
					class="inline-flex rounded-full bg-primary-container/40 px-3 py-1 font-body text-xs font-medium text-primary dark:bg-[#2a4a5a] dark:text-primary-container"
				>
					{contentTypeLabel(note.content_type)}
				</span>

				<button
					onclick={handleTogglePin}
					class="transition-breathe inline-flex items-center gap-1 rounded-full px-3 py-1 font-body text-xs font-medium
						{note.is_pinned === 1
						? 'bg-[#f0e6c8]/50 text-[#8a6a2a]'
						: 'bg-surface-low text-on-surface-muted hover:bg-surface-container dark:bg-surface-high dark:text-on-surface-subtle'}"
				>
					<span class="material-symbols-outlined text-[14px]" aria-hidden="true">push_pin</span>
					{note.is_pinned === 1 ? 'Pinned' : 'Pin'}
				</button>
			</div>

			<div class="mt-4 flex flex-wrap items-center gap-4">
				<span
					class="inline-flex items-center gap-1 font-body text-xs text-outline-variant dark:text-on-surface-muted"
				>
					<span class="material-symbols-outlined text-[14px]" aria-hidden="true"
						>calendar_today</span
					>
					Created {formatDate(note.created_at)}
				</span>
				<span
					class="inline-flex items-center gap-1 font-body text-xs text-outline-variant dark:text-on-surface-muted"
				>
					<span class="material-symbols-outlined text-[14px]" aria-hidden="true">edit</span>
					Updated {formatDate(note.updated_at)}
				</span>
			</div>
		</div>

		<!-- Content editor -->
		<section class="mb-8">
			<SectionLabel text="CONTENT" />
			<MarkdownEditor value={editContent} onchange={(v) => (editContent = v)} />

			<div class="mt-4">
				<Button variant="primary" onclick={handleSave} disabled={saving}>
					{#if saving}
						<span class="material-symbols-outlined mr-2 animate-spin text-lg" aria-hidden="true"
							>progress_activity</span
						>
						Saving...
					{:else}
						<span class="material-symbols-outlined mr-2 text-lg" aria-hidden="true">save</span>
						Save
					{/if}
				</Button>
			</div>
		</section>

		<!-- Navigation links -->
		<section class="mb-8">
			<SectionLabel text="RELATED" />
			<div class="space-y-3">
				<a href={resolve(`/knowledge/notes/${id}/history` as '/')} class="block">
					<Card class="transition-breathe hover:bg-surface-low dark:hover:bg-[var(--card-bg)]">
						<div class="flex items-center gap-4">
							<span
								class="material-symbols-outlined text-2xl text-primary dark:text-primary-container"
								aria-hidden="true">history</span
							>
							<div class="min-w-0 flex-1">
								<h3
									class="font-display text-sm font-bold text-on-surface dark:text-[var(--text-primary)]"
								>
									History
								</h3>
								<p
									class="mt-0.5 font-body text-xs text-on-surface-muted dark:text-on-surface-subtle"
								>
									View snapshots and version history
								</p>
							</div>
							<span
								class="material-symbols-outlined text-xl text-outline-variant dark:text-on-surface-muted"
								aria-hidden="true">chevron_right</span
							>
						</div>
					</Card>
				</a>
				<a href={resolve(`/knowledge/notes/${id}/relations` as '/')} class="block">
					<Card class="transition-breathe hover:bg-surface-low dark:hover:bg-[var(--card-bg)]">
						<div class="flex items-center gap-4">
							<span
								class="material-symbols-outlined text-2xl text-primary dark:text-primary-container"
								aria-hidden="true">hub</span
							>
							<div class="min-w-0 flex-1">
								<h3
									class="font-display text-sm font-bold text-on-surface dark:text-[var(--text-primary)]"
								>
									Relations
								</h3>
								<p
									class="mt-0.5 font-body text-xs text-on-surface-muted dark:text-on-surface-subtle"
								>
									Connected notes and entities
								</p>
							</div>
							<span
								class="material-symbols-outlined text-xl text-outline-variant dark:text-on-surface-muted"
								aria-hidden="true">chevron_right</span
							>
						</div>
					</Card>
				</a>
			</div>
		</section>

		<!-- Relations -->
		<section class="mb-8">
			<RelationsPanel entityType="knowledge_note" entityId={note.id} />
		</section>

		<!-- Delete zone -->
		<section>
			{#if showDeleteConfirm}
				<Card surface="low">
					<div class="flex flex-col items-center gap-3 py-4 text-center">
						<span class="material-symbols-outlined text-3xl text-error">warning</span>
						<p class="font-body text-sm text-on-surface dark:text-[var(--text-primary)]">
							Delete this note permanently?
						</p>
						<div class="flex gap-3">
							<Button variant="ghost" onclick={() => (showDeleteConfirm = false)}>Cancel</Button>
							<button
								onclick={handleDelete}
								class="transition-breathe inline-flex min-h-[48px] items-center justify-center rounded-full bg-error px-6 py-3 font-body font-semibold text-white hover:opacity-90"
							>
								Delete
							</button>
						</div>
					</div>
				</Card>
			{:else}
				<button
					onclick={() => (showDeleteConfirm = true)}
					class="transition-breathe font-body text-sm text-error hover:underline"
				>
					Delete this note
				</button>
			{/if}
		</section>
	{/if}
</main>
