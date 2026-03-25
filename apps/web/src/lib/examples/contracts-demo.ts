/**
 * Altair Shared Contracts - Web App Usage Examples
 *
 * This file demonstrates how to import and use the shared contracts
 * in the web application.
 */

import {
	ENTITY_TYPES,
	RELATION_TYPES,
	RELATION_SOURCE_TYPES,
	RELATION_STATUS_TYPES,
	SYNC_STREAMS
} from '@altair/contracts';
import type {
	EntityRef,
	RelationRecord,
	EntityTypesValue,
	RelationTypesValue,
	SyncStreamsValue
} from '@altair/contracts';

/**
 * Example: Creating an entity reference
 */
export const householdRef: EntityRef = {
	entityType: ENTITY_TYPES.HOUSEHOLD,
	entityId: 'household-123'
};

export const userRef: EntityRef = {
	entityType: ENTITY_TYPES.USER,
	entityId: 'user-456'
};

/**
 * Example: Creating a relation record
 */
export const relatedToRelation: RelationRecord = {
	id: 'relation-abc',
	from: householdRef,
	to: userRef,
	relationType: RELATION_TYPES.RELATED_TO,
	sourceType: RELATION_SOURCE_TYPES.USER,
	status: RELATION_STATUS_TYPES.ACCEPTED,
	confidence: 0.9,
	evidence: { reason: 'User added to household' },
	createdAt: new Date().toISOString(),
	updatedAt: new Date().toISOString()
};

/**
 * Example: Sync stream configuration
 */
export const autoSubscribedStreams: SyncStreamsValue[] = [
	SYNC_STREAMS.MY_PROFILE,
	SYNC_STREAMS.MY_MEMBERSHIPS,
	SYNC_STREAMS.MY_PERSONAL_DATA,
	SYNC_STREAMS.MY_HOUSEHOLD_DATA,
	SYNC_STREAMS.MY_RELATIONS,
	SYNC_STREAMS.MY_ATTACHMENT_METADATA
];

export const onDemandStreams: SyncStreamsValue[] = [
	SYNC_STREAMS.INITIATIVE_DETAIL,
	SYNC_STREAMS.NOTE_DETAIL,
	SYNC_STREAMS.ITEM_HISTORY,
	SYNC_STREAMS.QUEST_DETAIL
];

/**
 * Example: Type-safe entity type handling
 */
export function getEntityTypeLabel(entityType: EntityTypesValue): string {
	switch (entityType) {
		case ENTITY_TYPES.USER:
			return 'User';
		case ENTITY_TYPES.HOUSEHOLD:
			return 'Household';
		case ENTITY_TYPES.INITIATIVE:
			return 'Initiative';
		case ENTITY_TYPES.TAG:
			return 'Tag';
		case ENTITY_TYPES.ATTACHMENT:
			return 'Attachment';
		default:
			return 'Unknown';
	}
}

/**
 * Example: Type-safe relation type handling
 */
export function getRelationTypeLabel(relationType: RelationTypesValue): string {
	switch (relationType) {
		case RELATION_TYPES.REFERENCES:
			return 'References';
		case RELATION_TYPES.SUPPORTS:
			return 'Supports';
		case RELATION_TYPES.REQUIRES:
			return 'Requires';
		case RELATION_TYPES.RELATED_TO:
			return 'Related To';
		case RELATION_TYPES.DEPENDS_ON:
			return 'Depends On';
		case RELATION_TYPES.DUPLICATES:
			return 'Duplicates';
		case RELATION_TYPES.SIMILAR_TO:
			return 'Similar To';
		case RELATION_TYPES.GENERATED_FROM:
			return 'Generated From';
		default:
			return 'Unknown';
	}
}

/**
 * Example: Validate sync stream is auto-subscribed
 */
export function isAutoSubscribed(stream: SyncStreamsValue): boolean {
	return autoSubscribedStreams.includes(stream);
}
