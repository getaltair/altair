<script lang="ts">
	import { onMount } from 'svelte';
	import { initPowerSync, AltairConnector, SYNCED_TABLE_NAMES } from '$lib/sync/index.js';
	import type { PowerSyncDatabase } from '@powersync/web';

	let db = $state<PowerSyncDatabase | null>(null);
	let status = $state<'initializing' | 'connected' | 'connecting' | 'disconnected' | 'error'>(
		'initializing'
	);
	let lastSyncedAt = $state<string | null>(null);
	let rowCounts = $state<Record<string, number>>({});
	let initiatives = $state<Record<string, unknown>[]>([]);
	let errorMessage = $state<string | null>(null);
	let isReconnecting = $state(false);
	let hasSynced = $state<boolean | undefined>(undefined);

	function updateSyncStatus(database: PowerSyncDatabase) {
		const syncStatus = database.currentStatus;

		if (syncStatus.connected) {
			status = 'connected';
		} else if (syncStatus.connecting) {
			status = 'connecting';
		} else {
			status = 'disconnected';
		}

		lastSyncedAt = syncStatus.lastSyncedAt?.toISOString() ?? null;
		hasSynced = syncStatus.hasSynced;
	}

	async function refreshRowCounts(database: PowerSyncDatabase) {
		const counts: Record<string, number> = {};
		for (const table of SYNCED_TABLE_NAMES) {
			if (!/^[a-z_]+$/.test(table)) continue;
			try {
				const result = await database.get<{ count: number }>(
					`SELECT COUNT(*) as count FROM ${table}`
				);
				counts[table] = result.count;
			} catch (err) {
				console.error(`[sync-debug] Failed to count ${table}:`, err);
				counts[table] = -1;
			}
		}
		rowCounts = counts;
	}

	async function refreshInitiatives(database: PowerSyncDatabase) {
		try {
			const rows = await database.getAll<Record<string, unknown>>(
				'SELECT * FROM initiatives LIMIT 10'
			);
			initiatives = rows;
		} catch (err) {
			console.error('[sync-debug] Failed to query initiatives:', err);
			initiatives = [];
		}
	}

	async function handleReconnect() {
		if (!db) return;
		isReconnecting = true;
		errorMessage = null;

		try {
			await db.disconnect();
			await db.connect(new AltairConnector());
			status = 'connecting';
		} catch (err) {
			errorMessage = err instanceof Error ? err.message : String(err);
			status = 'error';
		} finally {
			isReconnecting = false;
		}
	}

	onMount(() => {
		let disposed = false;
		let statusUnsubscribe: (() => void) | undefined;
		let interval: ReturnType<typeof setInterval> | undefined;

		(async () => {
			try {
				const database = await initPowerSync();
				if (disposed) return;

				db = database;
				updateSyncStatus(database);
				await refreshRowCounts(database);
				await refreshInitiatives(database);

				// Listen for sync status changes via registerListener
				statusUnsubscribe = database.registerListener({
					statusChanged: () => {
						if (!disposed) {
							updateSyncStatus(database);
						}
					}
				});

				// Periodically refresh row counts (every 5 seconds)
				interval = setInterval(async () => {
					if (disposed) return;
					try {
						await refreshRowCounts(database);
						await refreshInitiatives(database);
					} catch (err) {
						console.error('[sync-debug] Periodic refresh failed:', err);
					}
				}, 5000);
			} catch (err) {
				if (disposed) return;
				status = 'error';
				errorMessage = err instanceof Error ? err.message : String(err);
			}
		})();

		return () => {
			disposed = true;
			statusUnsubscribe?.();
			if (interval) clearInterval(interval);
		};
	});

	const STATUS_COLOR: Record<string, string> = {
		connected: 'bg-emerald-100 text-emerald-800 dark:bg-emerald-900 dark:text-emerald-200',
		connecting: 'bg-amber-100 text-amber-800 dark:bg-amber-900 dark:text-amber-200',
		error: 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200',
		initializing: 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200',
		disconnected: 'bg-slate-100 text-slate-800 dark:bg-slate-700 dark:text-slate-200'
	};

	const STATUS_LABEL: Record<string, string> = {
		connected: 'Connected',
		connecting: 'Connecting...',
		error: 'Error',
		initializing: 'Initializing...',
		disconnected: 'Disconnected'
	};

	const statusColor = $derived(STATUS_COLOR[status] ?? STATUS_COLOR.disconnected);
	const statusLabel = $derived(STATUS_LABEL[status] ?? STATUS_LABEL.disconnected);
</script>

<svelte:head>
	<title>Sync Debug - Altair</title>
</svelte:head>

<div class="space-y-6">
	<div>
		<h1 class="text-2xl font-bold tracking-tight text-slate-900 dark:text-white">Sync Debug</h1>
		<p class="mt-1 text-base text-slate-600 dark:text-slate-400">
			PowerSync connection status and local SQLite data.
		</p>
	</div>

	<div
		class="rounded-lg border border-slate-200 bg-white p-6 dark:border-slate-700 dark:bg-slate-900"
	>
		<div class="flex items-center justify-between">
			<div class="flex items-center gap-3">
				<h2 class="text-lg font-semibold text-slate-900 dark:text-white">Connection Status</h2>
				<span class="inline-flex rounded-full px-2.5 py-0.5 text-xs font-medium {statusColor}">
					{statusLabel}
				</span>
			</div>
			<button
				onclick={handleReconnect}
				disabled={isReconnecting || status === 'initializing'}
				class="rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-medium text-white transition-colors hover:bg-indigo-700 disabled:cursor-not-allowed disabled:opacity-50"
			>
				{isReconnecting ? 'Reconnecting...' : 'Reconnect'}
			</button>
		</div>

		<dl class="mt-4 grid grid-cols-1 gap-4 sm:grid-cols-2">
			<div>
				<dt class="text-sm font-medium text-slate-500 dark:text-slate-400">Last synced</dt>
				<dd class="mt-1 text-sm text-slate-900 dark:text-white">
					{lastSyncedAt ?? 'Never'}
				</dd>
			</div>
			<div>
				<dt class="text-sm font-medium text-slate-500 dark:text-slate-400">
					Initial sync complete
				</dt>
				<dd class="mt-1 text-sm text-slate-900 dark:text-white">
					{#if hasSynced === undefined}Loading...{:else if hasSynced}Yes{:else}No{/if}
				</dd>
			</div>
		</dl>

		{#if errorMessage}
			<div
				class="mt-4 rounded-md border border-red-200 bg-red-50 p-3 dark:border-red-800 dark:bg-red-950"
			>
				<p class="text-sm text-red-700 dark:text-red-300">{errorMessage}</p>
			</div>
		{/if}
	</div>

	<div
		class="rounded-lg border border-slate-200 bg-white p-6 dark:border-slate-700 dark:bg-slate-900"
	>
		<h2 class="text-lg font-semibold text-slate-900 dark:text-white">Table Row Counts</h2>
		<p class="mt-1 text-sm text-slate-500 dark:text-slate-400">
			Number of rows in each locally synced SQLite table.
		</p>

		{#if Object.keys(rowCounts).length === 0}
			<p class="mt-4 text-sm text-slate-400 dark:text-slate-500">Waiting for data...</p>
		{:else}
			<div class="mt-4 overflow-hidden rounded-md border border-slate-200 dark:border-slate-700">
				<table class="min-w-full divide-y divide-slate-200 dark:divide-slate-700">
					<thead class="bg-slate-50 dark:bg-slate-800">
						<tr>
							<th
								class="px-4 py-2 text-left text-xs font-medium tracking-wider text-slate-500 uppercase dark:text-slate-400"
								>Table</th
							>
							<th
								class="px-4 py-2 text-right text-xs font-medium tracking-wider text-slate-500 uppercase dark:text-slate-400"
								>Rows</th
							>
						</tr>
					</thead>
					<tbody class="divide-y divide-slate-200 bg-white dark:divide-slate-700 dark:bg-slate-900">
						{#each Object.entries(rowCounts) as [table, count] (table)}
							<tr>
								<td
									class="px-4 py-2 font-mono text-sm whitespace-nowrap text-slate-900 dark:text-white"
									>{table}</td
								>
								<td
									class="px-4 py-2 text-right text-sm whitespace-nowrap text-slate-600 dark:text-slate-300"
								>
									{count === -1 ? 'Error' : count}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</div>

	<div
		class="rounded-lg border border-slate-200 bg-white p-6 dark:border-slate-700 dark:bg-slate-900"
	>
		<h2 class="text-lg font-semibold text-slate-900 dark:text-white">
			Initiatives (first 10 rows)
		</h2>
		<p class="mt-1 text-sm text-slate-500 dark:text-slate-400">
			Raw data from the local initiatives table for proof-of-life.
		</p>

		{#if initiatives.length === 0}
			<p class="mt-4 text-sm text-slate-400 dark:text-slate-500">No initiatives synced yet.</p>
		{:else}
			<div class="mt-4 space-y-3">
				{#each initiatives as row, i (row.id ?? i)}
					<details class="rounded-md border border-slate-200 dark:border-slate-700">
						<summary
							class="cursor-pointer px-4 py-2 text-sm font-medium text-slate-900 dark:text-white"
						>
							{String(row.name ?? row.id ?? `Row ${i + 1}`)}
						</summary>
						<pre
							class="overflow-x-auto border-t border-slate-200 bg-slate-50 px-4 py-3 text-xs text-slate-700 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-300">{JSON.stringify(
								row,
								null,
								2
							)}</pre>
					</details>
				{/each}
			</div>
		{/if}
	</div>
</div>
