/**
 * TypeScript type interfaces for Entity Relations.
 *
 * These interfaces mirror the PowerSync local schema for the
 * `entity_relations` table. All ID fields are `string` (UUID text).
 * All timestamps are `string` (ISO 8601 text from SQLite via PowerSync).
 * Nullable fields become `T | null`.
 */

// ---------------------------------------------------------------------------
// Enum literal types
// ---------------------------------------------------------------------------

export type RelationDirection = 'outgoing' | 'incoming';

// ---------------------------------------------------------------------------
// Entity interfaces
// ---------------------------------------------------------------------------

export type EntityType =
	| 'knowledge_note'
	| 'tracking_item'
	| 'guidance_quest'
	| 'guidance_routine'
	| 'initiative'
	| 'household';

export type RelationType =
	| 'related_to'
	| 'depends_on'
	| 'references'
	| 'supports'
	| 'requires'
	| 'duplicates'
	| 'similar_to'
	| 'generated_from';

export type SourceType = 'user' | 'ai_suggested';

export type RelationStatus = 'accepted' | 'pending' | 'rejected';

export interface EntityRelation {
	id: string;
	from_entity_type: EntityType;
	from_entity_id: string;
	to_entity_type: EntityType;
	to_entity_id: string;
	relation_type: RelationType;
	source_type: SourceType;
	status: RelationStatus | null;
	confidence: number | null;
	evidence_json: string | null;
	created_by_user_id: string | null;
	created_by_process: string | null;
	created_at: string;
	updated_at: string;
	last_confirmed_at: string | null;
}
