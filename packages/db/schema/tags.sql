-- Tags schema: tags and entity tag associations
-- Reference file (not executable - use migrations/)

create table tags (
	id uuid primary key default gen_random_uuid(),
	owner_user_id uuid not null references users(id) on delete cascade,
	household_id uuid references households(id) on delete cascade,
	name text not null,
	slug text,
	description text,
	color text,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now(),
	deleted_at timestamptz,
	constraint tags_name_owner_unique unique (name, owner_user_id)
);

create table initiative_tags (
	id uuid primary key default gen_random_uuid(),
	initiative_id uuid not null references initiatives(id) on delete cascade,
	tag_id uuid not null references tags(id) on delete cascade,
	created_at timestamptz not null default now(),
	constraint initiative_tags_unique unique (initiative_id, tag_id)
);

create table quest_tags (
	id uuid primary key default gen_random_uuid(),
	quest_id uuid not null references guidance_quests(id) on delete cascade,
	tag_id uuid not null references tags(id) on delete cascade,
	created_at timestamptz not null default now(),
	constraint quest_tags_unique unique (quest_id, tag_id)
);

create table note_tags (
	id uuid primary key default gen_random_uuid(),
	note_id uuid not null references knowledge_notes(id) on delete cascade,
	tag_id uuid not null references tags(id) on delete cascade,
	created_at timestamptz not null default now(),
	constraint note_tags_unique unique (note_id, tag_id)
);

create table item_tags (
	id uuid primary key default gen_random_uuid(),
	item_id uuid not null references tracking_items(id) on delete cascade,
	tag_id uuid not null references tags(id) on delete cascade,
	created_at timestamptz not null default now(),
	constraint item_tags_unique unique (item_id, tag_id)
);
