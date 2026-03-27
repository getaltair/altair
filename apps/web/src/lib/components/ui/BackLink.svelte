<script lang="ts">
	import { resolve } from '$app/paths';

	interface Props {
		href: string;
		label: string;
	}

	let { href, label }: Props = $props();
</script>

<a
	href={resolve(href as '/')}
	class="transition-breathe mb-6 inline-flex items-center gap-1 font-body text-sm text-on-surface-muted hover:text-primary dark:text-on-surface-subtle dark:hover:text-primary-container"
>
	<span class="material-symbols-outlined text-[18px]" aria-hidden="true">arrow_back</span>
	{label}
</a>
