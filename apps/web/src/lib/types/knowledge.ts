/**
 * TypeScript type interfaces for the Knowledge domain.
 *
 * These interfaces mirror the PowerSync local schema tables for knowledge
 * notes and snapshots. All ID fields are `string` (UUID text). All timestamps
 * are `string` (ISO 8601 text from SQLite via PowerSync). Nullable fields
 * become `T | null`.
 */

// ---------------------------------------------------------------------------
// Entity interfaces
// ---------------------------------------------------------------------------

export interface KnowledgeNote {
	id: string;
	user_id: string;
	household_id: string | null;
	initiative_id: string | null;
	title: string;
	content: string | null;
	content_type: string;
	is_pinned: number; // 0/1 from SQLite
	created_at: string;
	updated_at: string;
}

export interface KnowledgeNoteSnapshot {
	id: string;
	note_id: string;
	content: string;
	created_at: string;
	created_by_process: string | null;
}
