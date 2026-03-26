<script>
	import { page } from '$app/stores';
	import { resolve } from '$app/paths';
</script>

<div class="flex min-h-screen items-center justify-center bg-slate-50 dark:bg-slate-900">
	<div class="text-center">
		<h1 class="text-6xl font-bold text-indigo-600 dark:text-indigo-400">
			{$page.status}
		</h1>
		<p class="mt-4 text-lg text-slate-600 dark:text-slate-400">
			{$page.error?.message ?? 'Something went wrong'}
		</p>
		<a
			href={resolve('/')}
			class="mt-8 inline-block rounded-lg bg-indigo-600 px-6 py-3 text-sm font-medium text-white hover:bg-indigo-700"
		>
			Go home
		</a>
	</div>
</div>
