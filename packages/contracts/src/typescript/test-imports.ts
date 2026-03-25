/**
 * Test that all generated bindings can be imported and used
 */

import {
  ENTITY_TYPES,
  RELATION_TYPES,
  RELATION_SOURCE_TYPES,
  RELATION_STATUS_TYPES,
  SYNC_STREAMS,
  EntityTypesValue,
  RelationTypesValue,
  RelationSourceTypesValue,
  RelationStatusTypesValue,
  SyncStreamsValue,
  EntityRef,
  RelationRecord,
} from '../../generated/typescript/contracts';

// Test entity types
const entityType: EntityTypesValue = ENTITY_TYPES.USER;

// Test relation types
const relationType: RelationTypesValue = RELATION_TYPES.REFERENCES;

// Test source types
const sourceType: RelationSourceTypesValue = RELATION_SOURCE_TYPES.AI;

// Test status types
const statusType: RelationStatusTypesValue = RELATION_STATUS_TYPES.ACCEPTED;

// Test sync streams
const stream: SyncStreamsValue = SYNC_STREAMS.MY_PROFILE;

// Test DTOs
const entityRef: EntityRef = {
  entityType: ENTITY_TYPES.HOUSEHOLD,
  entityId: "123",
};

const relationRecord: RelationRecord = {
  id: "abc",
  from: entityRef,
  to: {
    entityType: ENTITY_TYPES.USER,
    entityId: "456",
  },
  relationType: RELATION_TYPES.RELATED_TO,
  sourceType: RELATION_SOURCE_TYPES.USER,
  status: RELATION_STATUS_TYPES.ACCEPTED,
  confidence: 0.9,
  evidence: {},
  createdAt: new Date().toISOString(),
  updatedAt: new Date().toISOString(),
};

console.log("All imports work correctly!");
