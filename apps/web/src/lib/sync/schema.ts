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
 *
 * Only tables that currently exist in the Postgres migrations are included.
 * Future tables (tracking_locations, tracking_categories, tracking_items,
 * tracking_shopping_lists, tracking_shopping_list_items, knowledge_notes,
 * guidance_quests, guidance_routines, guidance_epics) will be added here
 * as their backend migrations land.
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

export const AppSchema = new Schema({
	users,
	households,
	household_memberships,
	initiatives,
	tags,
	attachments,
	entity_relations
});

/** Table names included in the schema, useful for iteration (e.g. row counts). */
export const SYNCED_TABLE_NAMES = Object.keys(AppSchema.props) as ReadonlyArray<
	keyof typeof AppSchema.props
>;

export type Database = (typeof AppSchema)['types'];
