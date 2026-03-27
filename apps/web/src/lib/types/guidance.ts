/**
 * TypeScript type interfaces for the Guidance domain.
 *
 * These interfaces mirror the Rust model structs in `apps/server/src/guidance/`.
 * Enum literal types are defined here because the shared `packages/contracts`
 * package does not yet include guidance-specific enums.
 *
 * All ID fields are `string` (UUID text). All timestamps/dates are `string`
 * (ISO 8601 text from SQLite via PowerSync). Nullable Rust fields
 * (`Option<T>`) become `T | null`.
 */

// ---------------------------------------------------------------------------
// Enum literal types (matching Rust backend enums, except where noted)
// ---------------------------------------------------------------------------

export type QuestStatus = 'pending' | 'in_progress' | 'completed' | 'cancelled';

export type Priority = 'low' | 'medium' | 'high' | 'critical';

export type RoutineFrequency = 'daily' | 'weekly' | 'biweekly' | 'monthly';

export type RoutineStatus = 'active' | 'paused' | 'archived';

/**
 * Epics reuse `InitiativeStatus` in the Rust backend. Re-exported here under
 * a domain-specific alias so guidance consumers do not need to reach into
 * `core.ts`.
 */
export type EpicStatus = 'active' | 'paused' | 'completed' | 'archived';

export type EnergyLevel = 1 | 2 | 3 | 4 | 5;

// ---------------------------------------------------------------------------
// Entity interfaces
// ---------------------------------------------------------------------------

/** Mirrors `GuidanceQuest` in `apps/server/src/guidance/quests/models.rs`. */
export interface GuidanceQuest {
	id: string;
	epic_id: string | null;
	initiative_id: string | null;
	user_id: string;
	household_id: string | null;
	name: string;
	description: string | null;
	status: QuestStatus;
	priority: Priority;
	due_date: string | null;
	estimated_minutes: number | null;
	completed_at: string | null;
	created_at: string;
	updated_at: string;
}

/** Mirrors `GuidanceEpic` in `apps/server/src/guidance/epics/models.rs`. */
export interface GuidanceEpic {
	id: string;
	initiative_id: string | null;
	user_id: string;
	name: string;
	description: string | null;
	status: EpicStatus;
	priority: Priority;
	created_at: string;
	updated_at: string;
}

/** Mirrors `GuidanceRoutine` in `apps/server/src/guidance/routines/models.rs`. */
export interface GuidanceRoutine {
	id: string;
	user_id: string;
	household_id: string | null;
	name: string;
	description: string | null;
	frequency: RoutineFrequency;
	status: RoutineStatus;
	created_at: string;
	updated_at: string;
}

/** Mirrors `GuidanceFocusSession` in `apps/server/src/guidance/focus_sessions/models.rs`. */
export interface GuidanceFocusSession {
	id: string;
	quest_id: string;
	user_id: string;
	started_at: string;
	ended_at: string | null;
	duration_minutes: number | null;
	notes: string | null;
	created_at: string;
}

/** Mirrors `GuidanceDailyCheckin` in `apps/server/src/guidance/daily_checkins/models.rs`. */
export interface GuidanceDailyCheckin {
	id: string;
	user_id: string;
	date: string;
	energy_level: EnergyLevel | null;
	mood: string | null;
	notes: string | null;
	created_at: string;
}
