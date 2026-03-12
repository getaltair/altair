-- Migration: 0010_entity_relations
-- Description: Create cross-domain entity relationship system

-- Entity Types: Enum of all entity types that can be related
create type entity_type as enum (
	'user',
	'household',
	'initiative',
	'epic',
	'quest',
	'routine',
	'focus_session',
	'note',
	'location',
	'category',
	'item',
	'tag',
	'attachment'
);

-- Relation Types: Types of relationships between entities
create type relation_type as enum (
	'parent_of',
	'child_of',
	'relates_to',
	'depends_on',
	'blocks',
	'blocked_by',
	'duplicates',
	'similar_to',
	'references',
	'contains',
	'owned_by',
	'assigned_to',
	'part_of',
	'precedes',
	'succeeds'
);

-- Source Types: How the relation was established
create type source_type as enum (
	'manual',
	'inferred',
	'imported',
	'system'
);

-- Relation Status: Status of the relation
create type relation_status as enum (
	'active',
	'suspended',
	'deleted'
);

-- Entity Relations: Generic relationship table
create table entity_relations (
	id uuid primary key default gen_random_uuid(),
	from_entity_type entity_type not null,
	from_entity_id uuid not null,
	to_entity_type entity_type not null,
	to_entity_id uuid not null,
	relation_type relation_type not null,
	source_type source_type not null default 'manual',
	status relation_status not null default 'active',
	confidence numeric check (confidence >= 0 and confidence <= 1),
	evidence jsonb,
	owner_user_id uuid references users(id) on delete cascade,
	household_id uuid references households(id) on delete cascade,
	notes text,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now(),
	deleted_at timestamptz,

	constraint entity_relations_unique unique (
		from_entity_type,
		from_entity_id,
		to_entity_type,
		to_entity_id,
		relation_type
	)
);

-- Indexes for entity relations
create index entity_relations_from_idx on entity_relations (from_entity_type, from_entity_id);
create index entity_relations_to_idx on entity_relations (to_entity_type, to_entity_id);
create index entity_relations_relation_type_idx on entity_relations (relation_type);
create index entity_relations_owner_user_id_idx on entity_relations (owner_user_id);
create index entity_relations_household_id_idx on entity_relations (household_id);
create index entity_relations_status_idx on entity_relations (status) where status = 'active';
create index entity_relations_source_type_idx on entity_relations (source_type);
create index entity_relations_confidence_idx on entity_relations (confidence) where source_type = 'inferred';
