import { browser } from '$app/environment';
import { PowerSyncDatabase } from '@powersync/web';
import { AltairConnector } from './connector.js';
import { AppSchema } from './schema.js';

// ============================================================
// PowerSync singleton — browser-only
// ============================================================

let _client: PowerSyncDatabase | null = null;

export function getSyncClient(): PowerSyncDatabase {
  if (!browser) {
    throw new Error('PowerSync requires a browser environment');
  }
  if (!_client) {
    _client = new PowerSyncDatabase({
      schema: AppSchema,
      database: {
        dbFilename: 'altair.db',
      },
    });
    // Expose for integration test introspection only — no production use.
    (window as typeof window & { __altairSync?: PowerSyncDatabase }).__altairSync = _client;
  }
  return _client;
}

/**
 * Connect the sync client and subscribe to the five logical data streams.
 * PowerSync's sync rules on the server partition data into named buckets;
 * the client connects once and receives all buckets it is entitled to.
 */
export async function subscribeToStreams(client: PowerSyncDatabase): Promise<void> {
  try {
    await client.connect(new AltairConnector());
  } catch (err) {
    console.error('[sync] PowerSync connection failed:', err);
    throw err;
  }
}

export { AppSchema } from './schema.js';
export type { Database } from './schema.js';
export { AltairConnector } from './connector.js';
