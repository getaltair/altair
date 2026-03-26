<svelte:head>
	<title>Sign in - Altair</title>
</svelte:head>

<div class="flex min-h-screen items-center justify-center bg-slate-50 px-4 dark:bg-slate-950">
	<div class="w-full max-w-sm space-y-6">
		<div class="text-center">
			<div
				class="mx-auto flex h-12 w-12 items-center justify-center rounded-xl bg-indigo-600 text-lg font-bold text-white"
			>
				A
			</div>
			<h1 class="mt-4 text-2xl font-bold tracking-tight text-slate-900 dark:text-white">
				Sign in to Altair
			</h1>
			<p class="mt-1 text-sm text-slate-500 dark:text-slate-400">
				Enter your credentials to continue.
			</p>
		</div>

		<form class="space-y-4">
			<div>
				<label for="email" class="block text-sm font-medium text-slate-700 dark:text-slate-300">
					Email
				</label>
				<input
					id="email"
					type="email"
					autocomplete="email"
					required
					placeholder="you@example.com"
					class="mt-1 block w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 placeholder:text-slate-400 focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500/20 focus:outline-none dark:border-slate-600 dark:bg-slate-800 dark:text-white dark:placeholder:text-slate-500"
				/>
			</div>

			<div>
				<label for="password" class="block text-sm font-medium text-slate-700 dark:text-slate-300">
					Password
				</label>
				<input
					id="password"
					type="password"
					autocomplete="current-password"
					required
					placeholder="Your password"
					class="mt-1 block w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 placeholder:text-slate-400 focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500/20 focus:outline-none dark:border-slate-600 dark:bg-slate-800 dark:text-white dark:placeholder:text-slate-500"
				/>
			</div>

			<button
				type="submit"
				disabled
				class="w-full cursor-not-allowed rounded-lg bg-indigo-600 px-4 py-2.5 text-sm font-semibold text-white opacity-60 focus:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:ring-offset-2 dark:focus-visible:ring-offset-slate-950"
			>
				Sign in (coming soon)
			</button>
		</form>

		<p class="text-center text-xs text-slate-400 dark:text-slate-500">
			Authentication is not yet wired. This is a placeholder.
		</p>
	</div>
</div>
