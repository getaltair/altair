-- Migration: 0004_initiatives
-- Description: Create initiatives table for goals/projects

create type initiative_status as enum ('draft', 'active', 'paused', 'completed', 'archived');

create table initiatives (
	id uuid primary key default gen_random_uuid(),
	owner_user_id uuid not null references users(id) on delete cascade,
	household_id uuid references households(id) on delete set null,
	title text not null,
	slug text,
	description text,
	status initiative_status not null default 'active',
	start_date date,
	target_date date,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now(),
	deleted_at timestamptz,

	constraint initiatives_slug_unique unique (slug)
);

-- Index for owner's initiatives
create index initiatives_owner_user_id_idx on initiatives (owner_user_id);

-- Index for household initiatives
create index initiatives_household_id_idx on initiatives (household_id);

-- Index for status filtering
create index initiatives_status_idx on initiatives (status);

-- Index for slug lookups
create index initiatives_slug_idx on initiatives (slug);
