/**
 * PowerSync local SQLite schema definition.
 *
 * Defines the client-side schema for offline-first data access. Each table
 * mirrors the corresponding Postgres table that PowerSync replicates.
 *
 * NOTE: PowerSync automatically manages the `id` column (TEXT PRIMARY KEY)
 * on every table, so it is NOT included in the column definitions below.
 *
 * Column definitions here match the current Postgres migration columns. The
 * PowerSync YAML sync rules and the design spec (docs/schema/altair-schema-design-spec.md)
 * may reference additional columns from the target schema that do not yet exist.
 */
import { column, Schema, Table } from '@powersync/web';

const users = new Table({
	email: column.text,
	display_name: column.text,
	created_at: column.text,
	updated_at: column.text
});

const households = new Table({
	name: column.text,
	created_by: column.text,
	created_at: column.text
});

const household_memberships = new Table(
	{
		household_id: column.text,
		user_id: column.text,
		role: column.text,
		joined_at: column.text
	},
	{ indexes: { by_household: ['household_id'], by_user: ['user_id'] } }
);

const initiatives = new Table(
	{
		user_id: column.text,
		household_id: column.text,
		name: column.text,
		description: column.text,
		status: column.text,
		created_at: column.text,
		updated_at: column.text
	},
	{ indexes: { by_user: ['user_id'], by_household: ['household_id'] } }
);

const tags = new Table(
	{
		user_id: column.text,
		household_id: column.text,
		name: column.text,
		color: column.text,
		created_at: column.text
	},
	{ indexes: { by_user: ['user_id'] } }
);

const attachments = new Table(
	{
		entity_type: column.text,
		entity_id: column.text,
		filename: column.text,
		content_type: column.text,
		storage_key: column.text,
		size_bytes: column.integer,
		processing_state: column.text,
		created_at: column.text
	},
	{ indexes: { by_entity: ['entity_type', 'entity_id'] } }
);

const entity_relations = new Table(
	{
		from_entity_type: column.text,
		from_entity_id: column.text,
		to_entity_type: column.text,
		to_entity_id: column.text,
		relation_type: column.text,
		source_type: column.text,
		status: column.text,
		confidence: column.real,
		evidence_json: column.text,
		created_by_user_id: column.text,
		created_by_process: column.text,
		created_at: column.text,
		updated_at: column.text,
		last_confirmed_at: column.text
	},
	{
		indexes: {
			by_from: ['from_entity_type', 'from_entity_id'],
			by_to: ['to_entity_type', 'to_entity_id'],
			by_status: ['status']
		}
	}
);

const guidance_quests = new Table(
	{
		epic_id: column.text,
		initiative_id: column.text,
		user_id: column.text,
		household_id: column.text,
		name: column.text,
		description: column.text,
		status: column.text,
		priority: column.text,
		due_date: column.text,
		estimated_minutes: column.integer,
		completed_at: column.text,
		created_at: column.text,
		updated_at: column.text
	},
	{ indexes: { by_user: ['user_id'], by_initiative: ['initiative_id'], by_status: ['status'] } }
);

const guidance_epics = new Table(
	{
		initiative_id: column.text,
		user_id: column.text,
		name: column.text,
		description: column.text,
		status: column.text,
		priority: column.text,
		created_at: column.text,
		updated_at: column.text
	},
	{ indexes: { by_user: ['user_id'], by_initiative: ['initiative_id'] } }
);

const guidance_routines = new Table(
	{
		user_id: column.text,
		household_id: column.text,
		name: column.text,
		description: column.text,
		frequency: column.text,
		status: column.text,
		created_at: column.text,
		updated_at: column.text
	},
	{ indexes: { by_user: ['user_id'], by_status: ['status'] } }
);

const guidance_focus_sessions = new Table(
	{
		quest_id: column.text,
		user_id: column.text,
		started_at: column.text,
		ended_at: column.text,
		duration_minutes: column.integer,
		notes: column.text,
		created_at: column.text
	},
	{ indexes: { by_quest: ['quest_id'], by_user: ['user_id'] } }
);

const guidance_daily_checkins = new Table(
	{
		user_id: column.text,
		date: column.text,
		energy_level: column.integer,
		mood: column.text,
		notes: column.text,
		created_at: column.text
	},
	{ indexes: { by_user: ['user_id'], by_date: ['date'] } }
);

// ---------------------------------------------------------------------------
// Knowledge domain tables
// ---------------------------------------------------------------------------

const knowledge_notes = new Table(
	{
		user_id: column.text,
		household_id: column.text,
		initiative_id: column.text,
		title: column.text,
		content: column.text,
		content_type: column.text,
		is_pinned: column.integer,
		created_at: column.text,
		updated_at: column.text
	},
	{ indexes: { by_user: ['user_id'], by_household: ['household_id'] } }
);

const knowledge_note_snapshots = new Table(
	{
		note_id: column.text,
		content: column.text,
		created_at: column.text,
		created_by_process: column.text
	},
	{ indexes: { by_note: ['note_id'] } }
);

// ---------------------------------------------------------------------------
// Tracking domain tables
// ---------------------------------------------------------------------------

const tracking_items = new Table(
	{
		user_id: column.text,
		household_id: column.text,
		category_id: column.text,
		location_id: column.text,
		name: column.text,
		description: column.text,
		quantity: column.integer,
		unit: column.text,
		min_quantity: column.integer,
		barcode: column.text,
		status: column.text,
		created_at: column.text,
		updated_at: column.text
	},
	{
		indexes: {
			by_household: ['household_id'],
			by_category: ['category_id'],
			by_location: ['location_id']
		}
	}
);

const tracking_item_events = new Table(
	{
		item_id: column.text,
		user_id: column.text,
		event_type: column.text,
		quantity_change: column.integer,
		notes: column.text,
		created_at: column.text
	},
	{ indexes: { by_item: ['item_id'] } }
);

const tracking_locations = new Table(
	{
		user_id: column.text,
		household_id: column.text,
		name: column.text,
		description: column.text,
		parent_location_id: column.text,
		created_at: column.text,
		updated_at: column.text
	},
	{ indexes: { by_household: ['household_id'], by_parent: ['parent_location_id'] } }
);

const tracking_categories = new Table(
	{
		user_id: column.text,
		household_id: column.text,
		name: column.text,
		description: column.text,
		parent_category_id: column.text,
		created_at: column.text,
		updated_at: column.text
	},
	{ indexes: { by_household: ['household_id'], by_parent: ['parent_category_id'] } }
);

const tracking_shopping_lists = new Table(
	{
		user_id: column.text,
		household_id: column.text,
		name: column.text,
		status: column.text,
		created_at: column.text,
		updated_at: column.text
	},
	{ indexes: { by_household: ['household_id'] } }
);

const tracking_shopping_list_items = new Table(
	{
		shopping_list_id: column.text,
		item_id: column.text,
		name: column.text,
		quantity: column.integer,
		unit: column.text,
		is_checked: column.integer,
		created_at: column.text
	},
	{ indexes: { by_list: ['shopping_list_id'] } }
);

export const AppSchema = new Schema({
	users,
	households,
	household_memberships,
	initiatives,
	tags,
	attachments,
	entity_relations,
	guidance_quests,
	guidance_epics,
	guidance_routines,
	guidance_focus_sessions,
	guidance_daily_checkins,
	knowledge_notes,
	knowledge_note_snapshots,
	tracking_items,
	tracking_item_events,
	tracking_locations,
	tracking_categories,
	tracking_shopping_lists,
	tracking_shopping_list_items
});

/** Table names included in the schema, useful for iteration (e.g. row counts). */
export const SYNCED_TABLE_NAMES = Object.keys(AppSchema.props) as ReadonlyArray<
	keyof typeof AppSchema.props
>;

export type Database = (typeof AppSchema)['types'];
