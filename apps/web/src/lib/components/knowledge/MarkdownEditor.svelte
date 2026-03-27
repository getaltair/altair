<script lang="ts">
	let {
		value,
		onchange,
		readonly = false
	}: {
		value: string;
		onchange: (v: string) => void;
		readonly?: boolean;
	} = $props();

	let mode = $state<'edit' | 'preview'>('edit');

	/**
	 * Basic markdown-to-HTML conversion via regex.
	 * Handles: headings, bold, italic, links, unordered lists, line breaks.
	 */
	const renderedHtml = $derived.by(() => {
		let html = value;

		// Escape HTML entities first
		html = html.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');

		// Headings (# through ######)
		html = html.replace(/^######\s+(.+)$/gm, '<h6>$1</h6>');
		html = html.replace(/^#####\s+(.+)$/gm, '<h5>$1</h5>');
		html = html.replace(/^####\s+(.+)$/gm, '<h4>$1</h4>');
		html = html.replace(/^###\s+(.+)$/gm, '<h3>$1</h3>');
		html = html.replace(/^##\s+(.+)$/gm, '<h2>$1</h2>');
		html = html.replace(/^#\s+(.+)$/gm, '<h1>$1</h1>');

		// Bold (**text**)
		html = html.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>');

		// Italic (*text*)
		html = html.replace(/\*(.+?)\*/g, '<em>$1</em>');

		// Links [text](url)
		html = html.replace(
			/\[([^\]]+)\]\(([^)]+)\)/g,
			'<a href="$2" class="text-primary underline" target="_blank" rel="noopener noreferrer">$1</a>'
		);

		// Unordered lists (- item)
		html = html.replace(/^-\s+(.+)$/gm, '<li>$1</li>');
		html = html.replace(/((?:<li>.*<\/li>\n?)+)/g, '<ul class="list-disc pl-5">$1</ul>');

		// Line breaks (double newline = paragraph break)
		html = html.replace(/\n\n/g, '</p><p>');
		html = `<p>${html}</p>`;

		// Clean up empty paragraphs
		html = html.replace(/<p>\s*<\/p>/g, '');

		return html;
	});
</script>

<div class="flex flex-col">
	<!-- Tab bar -->
	{#if !readonly}
		<div class="mb-3 flex gap-1" role="tablist" aria-label="Editor mode">
			<button
				role="tab"
				aria-selected={mode === 'edit'}
				onclick={() => (mode = 'edit')}
				class="transition-breathe rounded-full px-4 py-1.5 font-body text-sm font-medium
					{mode === 'edit'
					? 'bg-primary text-white'
					: 'bg-surface-low text-on-surface-muted hover:bg-surface-container dark:bg-surface-high dark:text-on-surface-subtle dark:hover:bg-[#2e4446]'}"
			>
				Edit
			</button>
			<button
				role="tab"
				aria-selected={mode === 'preview'}
				onclick={() => (mode = 'preview')}
				class="transition-breathe rounded-full px-4 py-1.5 font-body text-sm font-medium
					{mode === 'preview'
					? 'bg-primary text-white'
					: 'bg-surface-low text-on-surface-muted hover:bg-surface-container dark:bg-surface-high dark:text-on-surface-subtle dark:hover:bg-[#2e4446]'}"
			>
				Preview
			</button>
		</div>
	{/if}

	<!-- Edit mode -->
	{#if mode === 'edit' && !readonly}
		<textarea
			{value}
			oninput={(e) => onchange(e.currentTarget.value)}
			class="transition-breathe min-h-[200px] w-full resize-y rounded-2xl bg-surface-low p-4 font-body text-sm leading-relaxed text-on-surface outline-none focus:bg-pure-white focus:ring-2 focus:ring-primary/20 dark:bg-[#1e2d2f] dark:text-[var(--text-primary)] dark:focus:bg-[var(--card-bg)]"
			placeholder="Start writing..."
		></textarea>
	{:else}
		<!-- Preview / readonly mode -->
		<div
			class="prose prose-sm min-h-[200px] max-w-none rounded-2xl bg-surface-low p-4 font-body text-sm leading-relaxed text-on-surface dark:bg-[#1e2d2f] dark:text-[var(--text-primary)]"
		>
			<!-- eslint-disable-next-line svelte/no-at-html-tags -->
			{@html renderedHtml}
		</div>
	{/if}
</div>
