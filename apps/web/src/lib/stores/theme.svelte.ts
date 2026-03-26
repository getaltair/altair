/**
 * Reactive theme store using Svelte 5 runes.
 *
 * Persists the user's preference to localStorage and syncs
 * the `dark` class on `document.documentElement` so that
 * Tailwind CSS v4's class-based dark mode works correctly.
 */

export type Theme = 'light' | 'dark';

const STORAGE_KEY = 'altair-theme';

/**
 * Reactive state. Wrapped in an object because $state primitives cannot be
 * directly exported; the .current property remains reactive across imports.
 */
const theme = $state<{ current: Theme }>({ current: 'light' });

/**
 * Read the active theme reactively.
 *
 * Usage in a component: `theme.current`
 */
export { theme };

/**
 * Toggle between light and dark themes.
 */
export function toggle(): void {
	theme.current = theme.current === 'light' ? 'dark' : 'light';
}

/**
 * Explicitly set the theme.
 */
export function setTheme(value: Theme): void {
	theme.current = value;
}

/**
 * Initialise the theme from localStorage / OS preference.
 *
 * Call once in a top-level layout component. Safe to call during SSR
 * (returns early when window is unavailable).
 */
export function initTheme(): void {
	if (typeof window === 'undefined') return;

	let stored: Theme | null = null;
	try {
		stored = localStorage.getItem(STORAGE_KEY) as Theme | null;
	} catch {
		console.warn('[theme] localStorage unavailable, using OS preference.');
	}

	if (stored === 'light' || stored === 'dark') {
		theme.current = stored;
	} else if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
		theme.current = 'dark';
	}

	// Runs whenever theme.current changes: syncs the DOM class and persists to localStorage.
	$effect(() => {
		const value = theme.current;
		document.documentElement.classList.toggle('dark', value === 'dark');
		try {
			localStorage.setItem(STORAGE_KEY, value);
		} catch {
			console.warn('[theme] Could not persist theme preference.');
		}
	});
}
