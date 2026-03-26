/**
 * Re-export all shared contract types from @altair/contracts.
 *
 * Import from '$lib/contracts' throughout the web app so that
 * the workspace dependency is referenced in a single place.
 */
export {
	EntityType,
	ALL_ENTITY_TYPES,
	RelationType,
	RelationSource,
	RelationStatus,
	AttachmentState,
	SyncStream,
	AUTO_SUBSCRIBED_STREAMS,
	ON_DEMAND_STREAMS
} from '@altair/contracts';
