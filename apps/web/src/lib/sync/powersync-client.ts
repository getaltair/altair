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

/**
 * Base URL for the Rust backend API.
 * In development this defaults to the local server; in production it should
 * be set via the VITE_API_URL environment variable.
 */
const BACKEND_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

export class AltairConnector implements PowerSyncBackendConnector {
	/**
	 * Fetch a fresh JWT and the PowerSync service endpoint from the backend.
	 *
	 * TODO: Auth integration gap -- the Rust backend requires Authorization: Bearer <token>,
	 * but this app uses Better Auth cookies for SvelteKit routes. The auth bridge between
	 * Better Auth sessions and Rust backend Bearer tokens is not yet implemented.
	 * Once the integration is wired (e.g., via a SvelteKit proxy endpoint or shared auth),
	 * this connector will need to include the Bearer token in the Authorization header.
	 * For now, credentials: 'include' sends cookies which will work once the proxy is in place.
	 */
	async fetchCredentials() {
		const response = await fetch(`${BACKEND_URL}/auth/powersync-token`, {
			method: 'POST',
			credentials: 'include'
		});

		if (!response.ok) {
			const body = await response.text().catch(() => '');
			throw new Error(`Failed to fetch PowerSync token (${response.status}): ${body}`);
		}

		let data: unknown;
		try {
			data = await response.json();
		} catch {
			throw new Error('Failed to fetch PowerSync token: invalid JSON response');
		}

		if (
			typeof data !== 'object' ||
			data === null ||
			typeof (data as Record<string, unknown>).token !== 'string' ||
			typeof (data as Record<string, unknown>).powersync_url !== 'string'
		) {
			throw new Error(
				'Failed to fetch PowerSync token: missing required fields (token, powersync_url)'
			);
		}

		const { token, powersync_url } = data as { token: string; powersync_url: string };

		return {
			endpoint: powersync_url,
			token
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
				let resp: Response;

				switch (op.op) {
					case UpdateType.PUT:
						resp = await fetch(`${BACKEND_URL}/core/${table}`, {
							method: 'POST',
							headers: { 'Content-Type': 'application/json' },
							credentials: 'include',
							body: JSON.stringify({ id, ...opData })
						});
						if (!resp.ok) {
							const body = await resp.text().catch(() => '');
							throw new Error(`${op.op} ${table}/${id} failed (${resp.status}): ${body}`);
						}
						break;

					case UpdateType.PATCH:
						resp = await fetch(`${BACKEND_URL}/core/${table}/${id}`, {
							method: 'PATCH',
							headers: { 'Content-Type': 'application/json' },
							credentials: 'include',
							body: JSON.stringify(opData)
						});
						if (!resp.ok) {
							const body = await resp.text().catch(() => '');
							throw new Error(`${op.op} ${table}/${id} failed (${resp.status}): ${body}`);
						}
						break;

					case UpdateType.DELETE:
						resp = await fetch(`${BACKEND_URL}/core/${table}/${id}`, {
							method: 'DELETE',
							credentials: 'include'
						});
						if (!resp.ok) {
							const body = await resp.text().catch(() => '');
							throw new Error(`${op.op} ${table}/${id} failed (${resp.status}): ${body}`);
						}
						break;

					default:
						throw new Error(`Unhandled CRUD operation type: ${op.op}`);
				}
			}

			await transaction.complete();
		} catch (error) {
			console.error('[powersync] Failed to upload data:', error);
			throw error;
		}
	}
}

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
 * Subsequent calls return the same instance. The singleton is only assigned
 * after init and connect succeed, so a failed attempt does not leave a
 * partially-initialized instance in place.
 */
export async function initPowerSync(): Promise<PowerSyncDatabase> {
	if (_db) return _db;

	const client = createPowerSyncClient();
	try {
		await client.init();
		await client.connect(new AltairConnector());
		_db = client;
		return _db;
	} catch (err) {
		try {
			await client.close();
		} catch {
			/* ignore close errors */
		}
		throw err;
	}
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
	const db = _db;
	_db = null;
	if (db) {
		try {
			await db.close();
		} catch (err) {
			console.error('[powersync] Error closing database:', err);
		}
	}
}
