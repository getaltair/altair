export { AppSchema, SYNCED_TABLE_NAMES, type Database } from './schema.js';
export {
	AltairConnector,
	initPowerSync,
	getPowerSyncDb,
	closePowerSync,
	createPowerSyncClient
} from './powersync-client.js';
