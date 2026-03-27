<script lang="ts" generics="T">
	import { SvelteMap, SvelteSet } from 'svelte/reactivity';

	interface Props {
		items: T[];
		getId: (item: T) => string;
		getParentId: (item: T) => string | undefined | null;
		getName: (item: T) => string;
		getDescription?: (item: T) => string | undefined;
		onSelect?: (item: T) => void;
	}

	let { items, getId, getParentId, getName, getDescription, onSelect }: Props = $props();

	// Build tree structure from flat list
	interface TreeNode {
		item: T;
		children: TreeNode[];
	}

	const tree = $derived.by(() => {
		const nodeMap = new SvelteMap<string, TreeNode>();
		const roots: TreeNode[] = [];

		// Create nodes
		for (const item of items) {
			nodeMap.set(getId(item), { item, children: [] });
		}

		// Build parent-child relationships
		for (const item of items) {
			const node = nodeMap.get(getId(item))!;
			const parentId = getParentId(item);
			if (parentId && nodeMap.has(parentId)) {
				nodeMap.get(parentId)!.children.push(node);
			} else {
				roots.push(node);
			}
		}

		return roots;
	});

	// Track expanded state per node ID
	let expanded = $state<SvelteSet<string>>(new SvelteSet());

	function toggleExpanded(id: string) {
		const next = new SvelteSet(expanded);
		if (next.has(id)) {
			next.delete(id);
		} else {
			next.add(id);
		}
		expanded = next;
	}
</script>

{#snippet treeNode(node: TreeNode, depth: number)}
	{@const id = getId(node.item)}
	{@const name = getName(node.item)}
	{@const description = getDescription?.(node.item)}
	{@const hasChildren = node.children.length > 0}
	{@const isExpanded = expanded.has(id)}

	<div class="group" style="padding-left: {depth * 1.5}rem;">
		<button
			onclick={() => {
				if (hasChildren) toggleExpanded(id);
				onSelect?.(node.item);
			}}
			class="transition-breathe flex w-full items-start gap-2 rounded-xl px-3 py-2.5 text-left hover:bg-surface-low dark:hover:bg-[#1e2d2f]"
		>
			<!-- Expand/collapse chevron -->
			{#if hasChildren}
				<span
					class="material-symbols-outlined mt-0.5 shrink-0 text-[18px] text-outline-variant transition-transform duration-300 dark:text-on-surface-muted {isExpanded
						? 'rotate-90'
						: ''}"
					aria-hidden="true"
				>
					chevron_right
				</span>
			{:else}
				<span class="w-[18px] shrink-0"></span>
			{/if}

			<div class="min-w-0 flex-1">
				<span
					class="font-body text-sm font-semibold text-on-surface dark:text-[var(--text-primary)]"
				>
					{name}
				</span>
				{#if description}
					<p class="mt-0.5 font-body text-xs text-on-surface-muted dark:text-on-surface-subtle">
						{description}
					</p>
				{/if}
			</div>

			{#if hasChildren}
				<span
					class="mt-0.5 shrink-0 font-body text-xs text-outline-variant dark:text-on-surface-muted"
				>
					{node.children.length}
				</span>
			{/if}
		</button>

		{#if hasChildren && isExpanded}
			<div>
				{#each node.children as child (getId(child.item))}
					{@render treeNode(child, depth + 1)}
				{/each}
			</div>
		{/if}
	</div>
{/snippet}

<div role="tree">
	{#each tree as node (getId(node.item))}
		{@render treeNode(node, 0)}
	{/each}
</div>
