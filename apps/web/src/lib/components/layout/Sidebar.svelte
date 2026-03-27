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
	}

	const navItems: NavItem[] = [
		{ label: 'Today', href: '/', icon: 'calendar_today' },
		{ label: 'Guidance', href: '/guidance', icon: 'explore' },
		{ label: 'Knowledge', href: '/knowledge', icon: 'menu_book' },
		{ label: 'Tracking', href: '/tracking', icon: 'track_changes' },
		{ label: 'Settings', href: '/settings', icon: 'settings' }
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
		class="fixed inset-0 z-30 bg-[#2a3435]/40 backdrop-blur-sm lg:hidden"
		onclick={onclose}
		aria-label="Close navigation"
		tabindex="-1"
	></button>
{/if}

<aside
	class="fixed top-0 left-0 z-40 flex h-full w-64 flex-col bg-surface-low transition-transform duration-300 ease-[cubic-bezier(0.4,0,0.2,1)] dark:bg-[#141e1f] {open
		? 'translate-x-0'
		: '-translate-x-full'} lg:translate-x-0"
	aria-label="Main navigation"
>
	<!-- Brand area -->
	<div class="flex h-16 items-center gap-2 px-5">
		<div class="h-2 w-2 rounded bg-primary"></div>
		<span class="font-display text-xl font-bold text-on-surface dark:text-[#f0f4f5]">Altair</span>
	</div>

	<!-- Navigation links -->
	<nav class="flex-1 space-y-1 overflow-y-auto px-3 py-4">
		{#each navItems as item (item.label)}
			<a
				href={resolve(item.href as '/')}
				onclick={onclose}
				class="transition-breathe flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm {isActive(
					item.href
				)
					? 'bg-primary-container font-medium text-primary dark:bg-primary/20'
					: 'text-on-surface-muted hover:bg-surface-container dark:text-on-surface-subtle dark:hover:bg-[#263638]'}"
				aria-current={isActive(item.href) ? 'page' : undefined}
			>
				<span class="material-symbols-outlined text-[22px]" aria-hidden="true">{item.icon}</span>
				<span>{item.label}</span>
			</a>
		{/each}
	</nav>

	<!-- Bottom section: theme toggle -->
	<div class="px-4 py-3">
		<div class="flex items-center justify-between">
			<span class="font-body text-xs font-medium text-on-surface-muted dark:text-on-surface-subtle"
				>Theme</span
			>
			<ThemeToggle />
		</div>
	</div>
</aside>
