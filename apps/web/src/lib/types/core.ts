/**
 * TypeScript type interfaces for core domain entities.
 *
 * These interfaces mirror the Rust model structs in `apps/server/src/core/`.
 * Enum literal types are defined here because the shared `packages/contracts`
 * package does not yet include these domain-specific enums.
 *
 * All ID fields are `string` (UUID text). All timestamps are `string`
 * (ISO 8601 text from SQLite via PowerSync). Nullable Rust fields
 * (`Option<T>`) become `T | null`.
 */

// ---------------------------------------------------------------------------
// Enum literal types (mirrored from apps/server/src/contracts.rs)
// ---------------------------------------------------------------------------

export type InitiativeStatus = 'active' | 'paused' | 'completed' | 'archived';

// ---------------------------------------------------------------------------
// Entity interfaces
// ---------------------------------------------------------------------------

/** Mirrors `Initiative` in `apps/server/src/core/initiatives/models.rs`. */
export interface Initiative {
	id: string;
	user_id: string;
	household_id: string | null;
	name: string;
	description: string | null;
	status: InitiativeStatus;
	created_at: string;
	updated_at: string;
}

/** Mirrors the `households` PowerSync table. */
export interface Household {
	id: string;
	name: string;
	created_by: string;
	created_at: string;
}

/** Mirrors the `tags` PowerSync table. */
export interface Tag {
	id: string;
	user_id: string;
	household_id: string | null;
	name: string;
	color: string | null;
	created_at: string;
}
