import { describe, it, expect } from 'vitest';
import { AppSchema, SYNCED_TABLE_NAMES } from './schema.js';

describe('AppSchema', () => {
	const expectedTables = [
		'users',
		'households',
		'household_memberships',
		'initiatives',
		'tags',
		'attachments',
		'entity_relations',
		'guidance_quests',
		'guidance_epics',
		'guidance_routines',
		'guidance_focus_sessions',
		'guidance_daily_checkins'
	] as const;

	it('contains exactly the expected tables', () => {
		const tableNames = Object.keys(AppSchema.props).sort();
		expect(tableNames).toEqual([...expectedTables].sort());
	});

	it('has no unexpected tables', () => {
		expect(Object.keys(AppSchema.props)).toHaveLength(expectedTables.length);
	});
});

describe('SYNCED_TABLE_NAMES', () => {
	it('matches the keys of AppSchema.props', () => {
		expect([...SYNCED_TABLE_NAMES].sort()).toEqual(Object.keys(AppSchema.props).sort());
	});

	it('is a readonly array', () => {
		// Verify it is an array (ReadonlyArray satisfies Array.isArray)
		expect(Array.isArray(SYNCED_TABLE_NAMES)).toBe(true);
	});
});

describe('table column definitions', () => {
	/** Extract column names from a Table instance via its .columns array. */
	function columnNames(tableName: string): string[] {
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		const props = (AppSchema as any).props as Record<string, { columns: any[] }>;
		const table = props[tableName];
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		return table.columns.map((c: any) => c.options.name as string);
	}

	it('users table has email, display_name, created_at, updated_at columns', () => {
		const cols = columnNames('users');
		expect(cols).toContain('email');
		expect(cols).toContain('display_name');
		expect(cols).toContain('created_at');
		expect(cols).toContain('updated_at');
	});

	it('households table has name, created_by, created_at columns', () => {
		const cols = columnNames('households');
		expect(cols).toContain('name');
		expect(cols).toContain('created_by');
		expect(cols).toContain('created_at');
	});

	it('household_memberships table has household_id, user_id, role, joined_at columns', () => {
		const cols = columnNames('household_memberships');
		expect(cols).toContain('household_id');
		expect(cols).toContain('user_id');
		expect(cols).toContain('role');
		expect(cols).toContain('joined_at');
	});

	it('initiatives table has user_id, household_id, name, description, status columns', () => {
		const cols = columnNames('initiatives');
		expect(cols).toContain('user_id');
		expect(cols).toContain('household_id');
		expect(cols).toContain('name');
		expect(cols).toContain('description');
		expect(cols).toContain('status');
	});

	it('tags table has user_id, household_id, name, color columns', () => {
		const cols = columnNames('tags');
		expect(cols).toContain('user_id');
		expect(cols).toContain('household_id');
		expect(cols).toContain('name');
		expect(cols).toContain('color');
	});

	it('attachments table has entity_type, entity_id, filename, size_bytes columns', () => {
		const cols = columnNames('attachments');
		expect(cols).toContain('entity_type');
		expect(cols).toContain('entity_id');
		expect(cols).toContain('filename');
		expect(cols).toContain('size_bytes');
	});

	it('entity_relations table has from/to entity fields and metadata columns', () => {
		const cols = columnNames('entity_relations');
		expect(cols).toContain('from_entity_type');
		expect(cols).toContain('from_entity_id');
		expect(cols).toContain('to_entity_type');
		expect(cols).toContain('to_entity_id');
		expect(cols).toContain('relation_type');
		expect(cols).toContain('confidence');
		expect(cols).toContain('evidence_json');
	});

	it('guidance_quests table has quest-specific columns', () => {
		const cols = columnNames('guidance_quests');
		expect(cols).toContain('epic_id');
		expect(cols).toContain('initiative_id');
		expect(cols).toContain('user_id');
		expect(cols).toContain('household_id');
		expect(cols).toContain('name');
		expect(cols).toContain('description');
		expect(cols).toContain('status');
		expect(cols).toContain('priority');
		expect(cols).toContain('due_date');
		expect(cols).toContain('estimated_minutes');
		expect(cols).toContain('completed_at');
	});

	it('guidance_epics table has epic-specific columns', () => {
		const cols = columnNames('guidance_epics');
		expect(cols).toContain('initiative_id');
		expect(cols).toContain('user_id');
		expect(cols).toContain('name');
		expect(cols).toContain('description');
		expect(cols).toContain('status');
		expect(cols).toContain('priority');
	});

	it('guidance_routines table has routine-specific columns', () => {
		const cols = columnNames('guidance_routines');
		expect(cols).toContain('user_id');
		expect(cols).toContain('household_id');
		expect(cols).toContain('name');
		expect(cols).toContain('description');
		expect(cols).toContain('frequency');
		expect(cols).toContain('status');
	});

	it('guidance_focus_sessions table has session-specific columns', () => {
		const cols = columnNames('guidance_focus_sessions');
		expect(cols).toContain('quest_id');
		expect(cols).toContain('user_id');
		expect(cols).toContain('started_at');
		expect(cols).toContain('ended_at');
		expect(cols).toContain('duration_minutes');
		expect(cols).toContain('notes');
	});

	it('guidance_daily_checkins table has checkin-specific columns', () => {
		const cols = columnNames('guidance_daily_checkins');
		expect(cols).toContain('user_id');
		expect(cols).toContain('date');
		expect(cols).toContain('energy_level');
		expect(cols).toContain('mood');
		expect(cols).toContain('notes');
	});
});
