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
import type { KnowledgeNote, KnowledgeNoteSnapshot } from '$lib/types/knowledge.js';
import type {
	TrackingItem,
	TrackingItemEvent,
	TrackingLocation,
	TrackingCategory,
	TrackingShoppingList,
	TrackingShoppingListItem
} from '$lib/types/tracking.js';

// ---------------------------------------------------------------------------
// Reactive module-level state (Svelte 5 runes)
// ---------------------------------------------------------------------------

export type SyncConnectionStatus = 'connected' | 'connecting' | 'disconnected' | 'error';

export type QuestFilter = { status?: QuestStatus; initiative_id?: string };
export type RoutineFilter = { status?: RoutineStatus };
export type NoteFilter = {
	household_id?: string;
	initiative_id?: string;
	is_pinned?: number;
	search?: string;
};
export type ItemFilter = { household_id?: string; category_id?: string; location_id?: string };
export type ShoppingListFilter = { household_id?: string; status?: string };

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
	},

	// -- Knowledge note queries -----------------------------------------------

	/**
	 * Query knowledge notes with optional filters.
	 */
	async queryNotes(filter?: NoteFilter): Promise<KnowledgeNote[]> {
		const database = requireDb();
		const conditions: string[] = [];
		const params: unknown[] = [];

		if (filter?.household_id) {
			conditions.push('household_id = ?');
			params.push(filter.household_id);
		}
		if (filter?.initiative_id) {
			conditions.push('initiative_id = ?');
			params.push(filter.initiative_id);
		}
		if (filter?.is_pinned !== undefined) {
			conditions.push('is_pinned = ?');
			params.push(filter.is_pinned);
		}
		if (filter?.search) {
			conditions.push("title LIKE '%' || ? || '%'");
			params.push(filter.search);
		}

		const where = conditions.length > 0 ? ` WHERE ${conditions.join(' AND ')}` : '';
		return database.getAll<KnowledgeNote>(
			`SELECT * FROM knowledge_notes${where} ORDER BY is_pinned DESC, updated_at DESC`,
			params
		);
	},

	/**
	 * Get a single knowledge note by ID.
	 */
	async queryNote(id: string): Promise<KnowledgeNote | null> {
		const database = requireDb();
		return database.getOptional<KnowledgeNote>('SELECT * FROM knowledge_notes WHERE id = ?', [id]);
	},

	/**
	 * Get all snapshots for a note, ordered newest first.
	 */
	async queryNoteSnapshots(noteId: string): Promise<KnowledgeNoteSnapshot[]> {
		const database = requireDb();
		return database.getAll<KnowledgeNoteSnapshot>(
			'SELECT * FROM knowledge_note_snapshots WHERE note_id = ? ORDER BY created_at DESC',
			[noteId]
		);
	},

	// -- Tracking item queries ------------------------------------------------

	/**
	 * Query tracking items with optional filters.
	 */
	async queryItems(filter?: ItemFilter): Promise<TrackingItem[]> {
		const database = requireDb();
		const conditions: string[] = [];
		const params: unknown[] = [];

		if (filter?.household_id) {
			conditions.push('household_id = ?');
			params.push(filter.household_id);
		}
		if (filter?.category_id) {
			conditions.push('category_id = ?');
			params.push(filter.category_id);
		}
		if (filter?.location_id) {
			conditions.push('location_id = ?');
			params.push(filter.location_id);
		}

		const where = conditions.length > 0 ? ` WHERE ${conditions.join(' AND ')}` : '';
		return database.getAll<TrackingItem>(
			`SELECT * FROM tracking_items${where} ORDER BY name ASC`,
			params
		);
	},

	/**
	 * Get a single tracking item by ID.
	 */
	async queryItem(id: string): Promise<TrackingItem | null> {
		const database = requireDb();
		return database.getOptional<TrackingItem>('SELECT * FROM tracking_items WHERE id = ?', [id]);
	},

	/**
	 * Get all events for a tracking item, ordered newest first.
	 */
	async queryItemEvents(itemId: string): Promise<TrackingItemEvent[]> {
		const database = requireDb();
		return database.getAll<TrackingItemEvent>(
			'SELECT * FROM tracking_item_events WHERE item_id = ? ORDER BY created_at DESC',
			[itemId]
		);
	},

	/**
	 * Items where quantity is below min_quantity threshold.
	 */
	async queryLowStockItems(householdId: string): Promise<TrackingItem[]> {
		const database = requireDb();
		return database.getAll<TrackingItem>(
			`SELECT * FROM tracking_items
			 WHERE household_id = ?
			   AND min_quantity IS NOT NULL
			   AND quantity < min_quantity
			 ORDER BY name ASC`,
			[householdId]
		);
	},

	/**
	 * Query all locations for a household.
	 */
	async queryLocations(householdId: string): Promise<TrackingLocation[]> {
		const database = requireDb();
		return database.getAll<TrackingLocation>(
			'SELECT * FROM tracking_locations WHERE household_id = ? ORDER BY name ASC',
			[householdId]
		);
	},

	/**
	 * Query all categories for a household.
	 */
	async queryCategories(householdId: string): Promise<TrackingCategory[]> {
		const database = requireDb();
		return database.getAll<TrackingCategory>(
			'SELECT * FROM tracking_categories WHERE household_id = ? ORDER BY name ASC',
			[householdId]
		);
	},

	/**
	 * Query shopping lists with optional filters.
	 */
	async queryShoppingLists(filter?: ShoppingListFilter): Promise<TrackingShoppingList[]> {
		const database = requireDb();
		const conditions: string[] = [];
		const params: unknown[] = [];

		if (filter?.household_id) {
			conditions.push('household_id = ?');
			params.push(filter.household_id);
		}
		if (filter?.status) {
			conditions.push('status = ?');
			params.push(filter.status);
		}

		const where = conditions.length > 0 ? ` WHERE ${conditions.join(' AND ')}` : '';
		return database.getAll<TrackingShoppingList>(
			`SELECT * FROM tracking_shopping_lists${where} ORDER BY updated_at DESC`,
			params
		);
	},

	/**
	 * Get all items in a shopping list.
	 */
	async queryShoppingListItems(listId: string): Promise<TrackingShoppingListItem[]> {
		const database = requireDb();
		return database.getAll<TrackingShoppingListItem>(
			'SELECT * FROM tracking_shopping_list_items WHERE shopping_list_id = ? ORDER BY created_at ASC',
			[listId]
		);
	},

	// -- Mutation helpers ------------------------------------------------------

	/**
	 * Adjust an item's quantity and record the event.
	 * Uses optimistic local writes via PowerSync's db.execute.
	 */
	async adjustItemQuantity(
		itemId: string,
		change: number,
		eventType: string,
		notes?: string
	): Promise<void> {
		const database = requireDb();
		const item = await this.queryItem(itemId);
		if (!item) return;

		// eslint-disable-next-line svelte/prefer-svelte-reactivity
		const now = new Date().toISOString();
		await database.execute('UPDATE tracking_items SET quantity = ?, updated_at = ? WHERE id = ?', [
			item.quantity + change,
			now,
			itemId
		]);
		await database.execute(
			'INSERT INTO tracking_item_events (id, item_id, user_id, event_type, quantity_change, notes, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)',
			[crypto.randomUUID(), itemId, item.user_id, eventType, change, notes ?? null, now]
		);
	},

	/**
	 * Toggle the checked state of a shopping list item.
	 */
	async toggleShoppingListItemCheck(itemId: string): Promise<void> {
		const database = requireDb();
		const current = await database.getOptional<{ is_checked: number }>(
			'SELECT is_checked FROM tracking_shopping_list_items WHERE id = ?',
			[itemId]
		);
		if (current) {
			await database.execute(
				'UPDATE tracking_shopping_list_items SET is_checked = ? WHERE id = ?',
				[current.is_checked ? 0 : 1, itemId]
			);
		}
	}
};
