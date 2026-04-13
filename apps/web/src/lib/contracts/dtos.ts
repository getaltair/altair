import type { EntityTypeValue } from './entityTypes.js';
import type { RelationTypeValue } from './relationTypes.js';
import type { SyncStreamValue } from './syncStreams.js';

/** A polymorphic reference to any entity by type and UUID. */
export interface EntityRef {
  entity_type: EntityTypeValue;
  /** UUID string */
  entity_id: string;
}

/** Mirrors the entity_relations table schema from docs/specs/05-erd.md. */
export interface RelationRecord {
  id: string;
  from_entity_type: EntityTypeValue;
  from_entity_id: string;
  to_entity_type: EntityTypeValue;
  to_entity_id: string;
  relation_type: RelationTypeValue;
  source_type: string;
  status: string;
  confidence: number | null;
  evidence: string | null;
  user_id: string;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

/** Mirrors the attachments table schema from docs/specs/05-erd.md. */
export interface AttachmentRecord {
  id: string;
  entity_type: EntityTypeValue;
  entity_id: string;
  file_name: string;
  content_type: string;
  size_bytes: number | null;
  state: string;
  storage_path: string | null;
  user_id: string;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

/** Parameters for subscribing to PowerSync sync streams. */
export interface SyncSubscriptionRequest {
  streams: SyncStreamValue[];
  user_id: string;
}
