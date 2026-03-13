-- Core schema: users, households, memberships, initiatives
-- Reference file (not executable - use migrations/)

create extension if not exists pgcrypto;

create table users (
	id uuid primary key default gen_random_uuid(),
	email text not null,
	display_name text not null,
	timezone text default 'America/Chicago',
	is_active boolean default true,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now(),
	deleted_at timestamptz,
	constraint users_email_unique unique (email)
);

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
