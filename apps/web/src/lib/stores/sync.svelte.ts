/**
 * Reactive sync store using Svelte 5 runes.
 *
 * Wraps the PowerSync local SQLite database with reactive state for
 * connection status, and provides query helpers with TypeScript type annotations
 * for the guidance and core domains.
 *
 * Usage:
 *   import { syncStore } from '$lib/stores/sync.svelte';
 *   await syncStore.initialize();
 *   const quests = await syncStore.queryTodayQuests();
 */

import { type PowerSyncDatabase } from '@powersync/web';
import { initPowerSync, getPowerSyncDb } from '$lib/sync/index.js';
import type {
	GuidanceQuest,
	GuidanceRoutine,
	GuidanceDailyCheckin,
	QuestStatus,
	RoutineStatus
} from '$lib/types/guidance.js';
import type { Initiative } from '$lib/types/core.js';

// ---------------------------------------------------------------------------
// Reactive module-level state (Svelte 5 runes)
// ---------------------------------------------------------------------------

export type SyncConnectionStatus = 'connected' | 'connecting' | 'disconnected' | 'error';

export type QuestFilter = { status?: QuestStatus; initiative_id?: string };
export type RoutineFilter = { status?: RoutineStatus };

let db = $state<PowerSyncDatabase | null>(null);
let syncStatus = $state<SyncConnectionStatus>('disconnected');
let lastSyncedAt = $state<string | null>(null);
let isInitialized = $state(false);
let initError = $state<string | null>(null);
let initPromise: Promise<void> | null = null;

/**
 * Dispose function returned by `db.registerListener()`.
 * Stored so we can clean up if re-initializing.
 */
let disposeListener: (() => void) | null = null;

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/**
 * Require an initialized database or throw a clear error.
 */
function requireDb(): PowerSyncDatabase {
	if (!db) {
		throw new Error('[sync] Database not initialized. Call syncStore.initialize() first.');
	}
	return db;
}

/**
 * Attach a `statusChanged` listener to the PowerSync database so that
 * reactive `syncStatus` and `lastSyncedAt` stay in sync.
 */
function attachStatusListener(database: PowerSyncDatabase): void {
	// Tear down any previous listener
	disposeListener?.();

	disposeListener = database.registerListener({
		statusChanged: (status) => {
			if (status.connected) {
				syncStatus = 'connected';
			} else if (status.connecting) {
				syncStatus = 'connecting';
			} else {
				// Not connected and not connecting -- could be idle or error.
				// Preserve 'error' state if it was set explicitly; otherwise
				// fall back to 'disconnected'.
				if (syncStatus !== 'error') {
					syncStatus = 'disconnected';
				}
			}

			if (status.lastSyncedAt) {
				lastSyncedAt = status.lastSyncedAt.toISOString();
			}
		}
	});
}

// ---------------------------------------------------------------------------
// Exported store
// ---------------------------------------------------------------------------

export const syncStore = {
	// -- Reactive getters -----------------------------------------------------

	get db() {
		return db;
	},
	get syncStatus() {
		return syncStatus;
	},
	get isOnline() {
		return syncStatus === 'connected';
	},
	get lastSyncedAt() {
		return lastSyncedAt;
	},
	get isInitialized() {
		return isInitialized;
	},
	get initError() {
		return initError;
	},

	// -- Lifecycle ------------------------------------------------------------

	/**
	 * Initialize the PowerSync database and begin syncing.
	 *
	 * Safe to call multiple times -- subsequent calls are no-ops when the
	 * database is already initialized. Concurrent calls share a single
	 * in-flight promise so initialization runs exactly once.
	 */
	async initialize(): Promise<void> {
		if (isInitialized && db) return;
		return (initPromise ??= this._doInitialize());
	},

	/**
	 * Internal initialization logic. Use `initialize()` externally.
	 */
	async _doInitialize(): Promise<void> {
		syncStatus = 'connecting';
		initError = null;

		try {
			// If an existing singleton exists (e.g. HMR reload), reuse it.
			const existing = getPowerSyncDb();
			if (existing) {
				db = existing;
			} else {
				db = await initPowerSync();
			}

			attachStatusListener(db);

			// Seed initial status from the database's current state.
			const current = db.currentStatus;
			if (current.connected) {
				syncStatus = 'connected';
			} else if (current.connecting) {
				syncStatus = 'connecting';
			} else {
				syncStatus = 'disconnected';
			}

			if (current.lastSyncedAt) {
				lastSyncedAt = current.lastSyncedAt.toISOString();
			}

			isInitialized = true;
		} catch (err) {
			syncStatus = 'error';
			initError = err instanceof Error ? err.message : String(err);
			initPromise = null;
			console.error('[sync] Initialization failed:', err);
		}
	},

	// -- Guidance quest queries -----------------------------------------------

	/**
	 * Query guidance quests with optional filters.
	 */
	async queryQuests(filter?: QuestFilter): Promise<GuidanceQuest[]> {
		const database = requireDb();
		const conditions: string[] = [];
		const params: unknown[] = [];

		if (filter?.status) {
			conditions.push('status = ?');
			params.push(filter.status);
		}
		if (filter?.initiative_id) {
			conditions.push('initiative_id = ?');
			params.push(filter.initiative_id);
		}

		const where = conditions.length > 0 ? ` WHERE ${conditions.join(' AND ')}` : '';
		return database.getAll<GuidanceQuest>(
			`SELECT * FROM guidance_quests${where} ORDER BY priority DESC, created_at ASC`,
			params
		);
	},

	/**
	 * Pending/in-progress quests that are due today, overdue, or have no due date set.
	 */
	async queryTodayQuests(): Promise<GuidanceQuest[]> {
		const database = requireDb();
		return database.getAll<GuidanceQuest>(
			`SELECT * FROM guidance_quests
			 WHERE status IN ('pending', 'in_progress')
			   AND (due_date IS NULL OR due_date <= date('now'))
			 ORDER BY priority DESC, created_at ASC`
		);
	},

	/**
	 * Mark a quest as completed with an optimistic local write.
	 * PowerSync will sync this mutation to the backend.
	 */
	async completeQuest(id: string): Promise<void> {
		const database = requireDb();
		await database.execute(
			`UPDATE guidance_quests SET status = 'completed', completed_at = datetime('now'), updated_at = datetime('now') WHERE id = ?`,
			[id]
		);
	},

	// -- Guidance routine queries ---------------------------------------------

	/**
	 * Query guidance routines with optional filters.
	 */
	async queryRoutines(filter?: RoutineFilter): Promise<GuidanceRoutine[]> {
		const database = requireDb();
		const conditions: string[] = [];
		const params: unknown[] = [];

		if (filter?.status) {
			conditions.push('status = ?');
			params.push(filter.status);
		}

		const where = conditions.length > 0 ? ` WHERE ${conditions.join(' AND ')}` : '';
		return database.getAll<GuidanceRoutine>(
			`SELECT * FROM guidance_routines${where} ORDER BY name ASC`,
			params
		);
	},

	/**
	 * Active routines for the Today View.
	 */
	async queryTodayRoutines(): Promise<GuidanceRoutine[]> {
		const database = requireDb();
		return database.getAll<GuidanceRoutine>(
			`SELECT * FROM guidance_routines
			 WHERE status = 'active'
			 ORDER BY name ASC`
		);
	},

	// -- Daily check-in queries -----------------------------------------------

	/**
	 * Get today's daily check-in, if one exists.
	 */
	async queryTodayCheckin(): Promise<GuidanceDailyCheckin | null> {
		const database = requireDb();
		return database.getOptional<GuidanceDailyCheckin>(
			`SELECT * FROM guidance_daily_checkins
			 WHERE date = date('now')
			 LIMIT 1`
		);
	},

	// -- Initiative queries ---------------------------------------------------

	/**
	 * Query initiatives with optional household filter.
	 */
	async queryInitiatives(filter?: { household_id?: string }): Promise<Initiative[]> {
		const database = requireDb();
		const conditions: string[] = [];
		const params: unknown[] = [];

		if (filter?.household_id) {
			conditions.push('household_id = ?');
			params.push(filter.household_id);
		}

		const where = conditions.length > 0 ? ` WHERE ${conditions.join(' AND ')}` : '';
		return database.getAll<Initiative>(
			`SELECT * FROM initiatives${where} ORDER BY name ASC`,
			params
		);
	}
};
