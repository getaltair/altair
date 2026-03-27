<script lang="ts">
	import { page } from '$app/state';
	import { BackLink } from '$lib/components/ui/index.js';
	import RelationsPanel from '$lib/components/relations/RelationsPanel.svelte';

	const id = $derived(page.params.id ?? '');
</script>

<svelte:head>
	<title>Note Relations - Altair</title>
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-8">
	<BackLink href="/knowledge/notes/{id}" label="Back to note" />

	<h1 class="mb-8 font-display text-3xl font-bold text-on-surface dark:text-[var(--text-primary)]">
		Note Relations
	</h1>

	<RelationsPanel entityType="knowledge_note" entityId={id} />
</main>
