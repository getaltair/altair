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

export interface EntityRelation {
	id: string;
	from_entity_id: string;
	from_entity_type: string;
	to_entity_id: string;
	to_entity_type: string;
	relation_type: string;
	source_type: string; // 'user' | 'ai_suggested'
	household_id: string | null;
	created_at: string;
	created_by: string | null;
	status: string | null;
}
