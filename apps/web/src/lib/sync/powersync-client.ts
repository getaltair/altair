/**
 * PowerSync client setup and backend connector for the Altair web app.
 *
 * The connector handles two responsibilities:
 * 1. fetchCredentials -- obtains a short-lived JWT from the Rust backend
 *    that PowerSync uses to authenticate and scope sync streams.
 * 2. uploadData -- replays local CRUD mutations back to the backend API
 *    so that offline writes eventually reach Postgres.
 */
import {
	PowerSyncDatabase,
	UpdateType,
	type AbstractPowerSyncDatabase,
	type PowerSyncBackendConnector
} from '@powersync/web';

import { AppSchema } from './schema.js';

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

/**
 * Base URL for the Rust backend API.
 * In development this defaults to the local server; in production it should
 * be set via the VITE_API_URL environment variable.
 */
const BACKEND_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

// ---------------------------------------------------------------------------
// Backend connector
// ---------------------------------------------------------------------------

export class AltairConnector implements PowerSyncBackendConnector {
	/**
	 * Fetch a fresh JWT and the PowerSync service endpoint from the backend.
	 * The backend validates the session cookie and returns a signed token
	 * scoped to the current user and their household memberships.
	 */
	async fetchCredentials() {
		const response = await fetch(`${BACKEND_URL}/auth/powersync-token`, {
			method: 'POST',
			credentials: 'include'
		});

		if (!response.ok) {
			throw new Error(`Failed to fetch PowerSync token: ${response.status}`);
		}

		const data: { token: string; powersync_url: string } = await response.json();

		return {
			endpoint: data.powersync_url,
			token: data.token
		};
	}

	/**
	 * Upload queued local mutations to the backend REST API.
	 *
	 * Each CRUD entry is dispatched to the appropriate endpoint based on
	 * table name and operation type. On success the transaction is marked
	 * complete so PowerSync removes it from the upload queue.
	 *
	 * Any thrown error causes PowerSync to retry after a backoff period.
	 */
	async uploadData(database: AbstractPowerSyncDatabase): Promise<void> {
		const transaction = await database.getNextCrudTransaction();
		if (!transaction) return;

		try {
			for (const op of transaction.crud) {
				const { table, id, opData } = op;

				switch (op.op) {
					case UpdateType.PUT:
						await fetch(`${BACKEND_URL}/api/${table}`, {
							method: 'POST',
							headers: { 'Content-Type': 'application/json' },
							credentials: 'include',
							body: JSON.stringify({ id, ...opData })
						});
						break;

					case UpdateType.PATCH:
						await fetch(`${BACKEND_URL}/api/${table}/${id}`, {
							method: 'PATCH',
							headers: { 'Content-Type': 'application/json' },
							credentials: 'include',
							body: JSON.stringify(opData)
						});
						break;

					case UpdateType.DELETE:
						await fetch(`${BACKEND_URL}/api/${table}/${id}`, {
							method: 'DELETE',
							credentials: 'include'
						});
						break;
				}
			}

			await transaction.complete();
		} catch (error) {
			console.error('[powersync] Failed to upload data:', error);
			throw error;
		}
	}
}

// ---------------------------------------------------------------------------
// Database singleton
// ---------------------------------------------------------------------------

let _db: PowerSyncDatabase | null = null;

/**
 * Create a new PowerSyncDatabase instance without connecting.
 * Prefer `initPowerSync()` which also handles connection.
 */
export function createPowerSyncClient(): PowerSyncDatabase {
	return new PowerSyncDatabase({
		schema: AppSchema,
		database: {
			dbFilename: 'altair.db'
		}
	});
}

/**
 * Initialize the PowerSync database singleton.
 *
 * - Creates the local SQLite database (if not already created).
 * - Calls `db.init()` to run schema migrations on the local database.
 * - Connects to the PowerSync service via the AltairConnector.
 *
 * Subsequent calls return the same instance.
 */
export async function initPowerSync(): Promise<PowerSyncDatabase> {
	if (!_db) {
		_db = createPowerSyncClient();
		await _db.init();
		await _db.connect(new AltairConnector());
	}
	return _db;
}

/**
 * Return the existing PowerSync database instance (or null before init).
 * Useful for checking state without triggering initialization.
 */
export function getPowerSyncDb(): PowerSyncDatabase | null {
	return _db;
}

/**
 * Disconnect and tear down the current PowerSync instance.
 * After calling this, `initPowerSync()` will create a fresh connection.
 */
export async function closePowerSync(): Promise<void> {
	if (_db) {
		await _db.close();
		_db = null;
	}
}
