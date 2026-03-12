-- Guidance schema: epics, quests, routines, focus sessions, daily checkins
-- Reference file (not executable - use migrations/)

create type epic_status as enum ('draft', 'active', 'paused', 'completed', 'archived');

create table guidance_epics (
	id uuid primary key default gen_random_uuid(),
	initiative_id uuid references initiatives(id) on delete set null,
	owner_user_id uuid not null references users(id) on delete cascade,
	title text not null,
	description text,
	status epic_status not null default 'active',
	start_date date,
	target_date date,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now(),
	deleted_at timestamptz
);

create type quest_status as enum ('todo', 'in_progress', 'blocked', 'completed', 'cancelled');

create table guidance_quests (
	id uuid primary key default gen_random_uuid(),
	epic_id uuid references guidance_epics(id) on delete set null,
	initiative_id uuid references initiatives(id) on delete set null,
	owner_user_id uuid not null references users(id) on delete cascade,
	title text not null,
	description text,
	status quest_status not null default 'todo',
	priority integer default 0,
	due_date date,
	completed_at timestamptz,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now(),
	deleted_at timestamptz
);

create type routine_frequency as enum ('daily', 'weekly', 'monthly', 'custom');
create type routine_status as enum ('active', 'paused', 'archived');

create table guidance_routines (
	id uuid primary key default gen_random_uuid(),
	owner_user_id uuid not null references users(id) on delete cascade,
	household_id uuid references households(id) on delete set null,
	title text not null,
	description text,
	frequency routine_frequency not null default 'daily',
	custom_cron text,
	status routine_status not null default 'active',
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now(),
	deleted_at timestamptz
);

create type focus_session_status as enum ('scheduled', 'active', 'completed', 'cancelled');

create table guidance_focus_sessions (
	id uuid primary key default gen_random_uuid(),
	user_id uuid not null references users(id) on delete cascade,
	quest_id uuid references guidance_quests(id) on delete set null,
	title text not null,
	description text,
	scheduled_start timestamptz not null,
	scheduled_end timestamptz not null,
	actual_start timestamptz,
	actual_end timestamptz,
	status focus_session_status not null default 'scheduled',
	notes text,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now()
);

create table guidance_daily_checkins (
	id uuid primary key default gen_random_uuid(),
	user_id uuid not null references users(id) on delete cascade,
	checkin_date date not null,
	mood integer check (mood >= 1 and mood <= 5),
	energy_level integer check (energy_level >= 1 and energy_level <= 5),
	reflection text,
	gratitude text,
	tomorrow_intent text,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now(),
	constraint daily_checkins_user_date_unique unique (user_id, checkin_date)
);
