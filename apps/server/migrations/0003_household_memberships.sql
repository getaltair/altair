-- Migration: 0003_household_memberships
-- Description: Create household_memberships for user-household relationships

create type household_role as enum ('owner', 'admin', 'member', 'viewer');

create table household_memberships (
	id uuid primary key default gen_random_uuid(),
	household_id uuid not null references households(id) on delete cascade,
	user_id uuid not null references users(id) on delete cascade,
	role household_role not null default 'member',
	is_active boolean default true,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now(),

	constraint household_memberships_unique unique (household_id, user_id)
);

-- Index for household membership lookups
create index household_memberships_household_id_idx on household_memberships (household_id);

-- Index for user's households
create index household_memberships_user_id_idx on household_memberships (user_id);
