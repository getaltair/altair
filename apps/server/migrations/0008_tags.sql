-- Migration: 0008_tags
-- Description: Create tagging system for all entity types

-- Tags: Reusable tags for categorization
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

-- Initiative Tags: Link tags to initiatives
create table initiative_tags (
	id uuid primary key default gen_random_uuid(),
	initiative_id uuid not null references initiatives(id) on delete cascade,
	tag_id uuid not null references tags(id) on delete cascade,
	created_at timestamptz not null default now(),

	constraint initiative_tags_unique unique (initiative_id, tag_id)
);

-- Quest Tags: Link tags to quests
create table quest_tags (
	id uuid primary key default gen_random_uuid(),
	quest_id uuid not null references guidance_quests(id) on delete cascade,
	tag_id uuid not null references tags(id) on delete cascade,
	created_at timestamptz not null default now(),

	constraint quest_tags_unique unique (quest_id, tag_id)
);

-- Note Tags: Link tags to notes
create table note_tags (
	id uuid primary key default gen_random_uuid(),
	note_id uuid not null references knowledge_notes(id) on delete cascade,
	tag_id uuid not null references tags(id) on delete cascade,
	created_at timestamptz not null default now(),

	constraint note_tags_unique unique (note_id, tag_id)
);

-- Item Tags: Link tags to tracking items
create table item_tags (
	id uuid primary key default gen_random_uuid(),
	item_id uuid not null references tracking_items(id) on delete cascade,
	tag_id uuid not null references tags(id) on delete cascade,
	created_at timestamptz not null default now(),

	constraint item_tags_unique unique (item_id, tag_id)
);

-- Indexes for tags
create index tags_owner_user_id_idx on tags (owner_user_id);
create index tags_household_id_idx on tags (household_id);
create index tags_slug_idx on tags (slug);

create index initiative_tags_initiative_id_idx on initiative_tags (initiative_id);
create index initiative_tags_tag_id_idx on initiative_tags (tag_id);

create index quest_tags_quest_id_idx on quest_tags (quest_id);
create index quest_tags_tag_id_idx on quest_tags (tag_id);

create index note_tags_note_id_idx on note_tags (note_id);
create index note_tags_tag_id_idx on note_tags (tag_id);

create index item_tags_item_id_idx on item_tags (item_id);
create index item_tags_tag_id_idx on item_tags (tag_id);
