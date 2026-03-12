-- Migration: 0002_households
-- Description: Create the households table for group/family management

create table households (
	id uuid primary key default gen_random_uuid(),
	owner_user_id uuid not null references users(id) on delete cascade,
	name text not null,
	slug text,
	description text,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now(),
	deleted_at timestamptz,

	constraint households_slug_unique unique (slug)
);

-- Index for owner lookups
create index households_owner_user_id_idx on households (owner_user_id);

-- Index for slug lookups
create index households_slug_idx on households (slug);
