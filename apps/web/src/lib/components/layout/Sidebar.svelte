<script lang="ts">
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import ThemeToggle from './ThemeToggle.svelte';

	interface Props {
		open: boolean;
		onclose: () => void;
	}

	let { open, onclose }: Props = $props();

	interface NavItem {
		label: string;
		href: string;
		icon: string;
		disabled?: boolean;
		tooltip?: string;
	}

	const navItems: NavItem[] = [
		{ label: 'Guidance', href: '/guidance', icon: 'compass' },
		{ label: 'Knowledge', href: '/knowledge', icon: 'book' },
		{ label: 'Tracking', href: '/tracking', icon: 'checklist' },
		{ label: 'Search', href: '#', icon: 'search', disabled: true, tooltip: 'Coming soon' },
		{ label: 'Settings', href: '/settings', icon: 'gear' }
	];

	function isActive(href: string): boolean {
		const pathname = page.url.pathname;
		if (href === '/') return pathname === '/';
		return pathname === href || pathname.startsWith(href + '/');
	}
</script>

<!-- Mobile backdrop -->
{#if open}
	<button
		class="fixed inset-0 z-30 bg-black/40 backdrop-blur-sm lg:hidden"
		onclick={onclose}
		aria-label="Close navigation"
		tabindex="-1"
	></button>
{/if}

<aside
	class="fixed top-0 left-0 z-40 flex h-full w-64 flex-col border-r border-slate-200 bg-white transition-transform duration-200 ease-in-out dark:border-slate-700 dark:bg-slate-900 {open
		? 'translate-x-0'
		: '-translate-x-full'} lg:translate-x-0"
	aria-label="Main navigation"
>
	<!-- Logo / Brand -->
	<div class="flex h-16 items-center gap-3 border-b border-slate-200 px-5 dark:border-slate-700">
		<div
			class="flex h-8 w-8 items-center justify-center rounded-lg bg-indigo-600 text-sm font-bold text-white"
		>
			A
		</div>
		<span class="text-lg font-semibold tracking-tight text-slate-900 dark:text-white">Altair</span>
	</div>

	<!-- Navigation links -->
	<nav class="flex-1 space-y-1 overflow-y-auto px-3 py-4">
		{#each navItems as item (item.label)}
			{#if item.disabled}
				<span
					class="group flex cursor-not-allowed items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium text-slate-400 dark:text-slate-500"
					title={item.tooltip}
				>
					{@render navIcon(item.icon)}
					{item.label}
					<span
						class="ml-auto rounded-full bg-slate-100 px-2 py-0.5 text-xs text-slate-400 dark:bg-slate-800 dark:text-slate-500"
						>Soon</span
					>
				</span>
			{:else}
				<a
					href={resolve(item.href as '/')}
					onclick={onclose}
					class="group flex items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium transition-colors {isActive(
						item.href
					)
						? 'bg-indigo-50 text-indigo-700 dark:bg-indigo-950 dark:text-indigo-300'
						: 'text-slate-700 hover:bg-slate-100 dark:text-slate-300 dark:hover:bg-slate-800'}"
					aria-current={isActive(item.href) ? 'page' : undefined}
				>
					{@render navIcon(item.icon)}
					{item.label}
				</a>
			{/if}
		{/each}
	</nav>

	<!-- Bottom section: theme toggle -->
	<div class="border-t border-slate-200 px-4 py-3 dark:border-slate-700">
		<div class="flex items-center justify-between">
			<span class="text-xs font-medium text-slate-500 dark:text-slate-400">Theme</span>
			<ThemeToggle />
		</div>
	</div>
</aside>

{#snippet navIcon(icon: string)}
	{#if icon === 'compass'}
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="18"
			height="18"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			aria-hidden="true"
		>
			<circle cx="12" cy="12" r="10" />
			<polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76" />
		</svg>
	{:else if icon === 'book'}
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="18"
			height="18"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			aria-hidden="true"
		>
			<path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20" />
			<path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" />
		</svg>
	{:else if icon === 'checklist'}
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="18"
			height="18"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			aria-hidden="true"
		>
			<path d="M9 11l3 3L22 4" />
			<path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11" />
		</svg>
	{:else if icon === 'search'}
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="18"
			height="18"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			aria-hidden="true"
		>
			<circle cx="11" cy="11" r="8" />
			<line x1="21" y1="21" x2="16.65" y2="16.65" />
		</svg>
	{:else if icon === 'gear'}
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="18"
			height="18"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			aria-hidden="true"
		>
			<circle cx="12" cy="12" r="3" />
			<path
				d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"
			/>
		</svg>
	{/if}
{/snippet}
