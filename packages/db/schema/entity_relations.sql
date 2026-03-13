-- Entity Relations schema: cross-domain relationships
-- Reference file (not executable - use migrations/)

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

create type source_type as enum ('manual', 'inferred', 'imported', 'system');

create type relation_status as enum ('active', 'suspended', 'deleted');

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
