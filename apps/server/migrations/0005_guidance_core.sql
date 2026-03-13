-- Migration: 0005_guidance_core
-- Description: Create core guidance tables for goals, quests, routines, and focus sessions

-- Epic: Large, long-term goals that span multiple quests
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

-- Quest: Actionable tasks that contribute to epics
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

-- Routine: Recurring tasks/habits
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

-- Focus Session: Time-blocked work sessions
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

-- Daily Check-in: Track progress and reflections
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

-- Indexes for guidance tables
create index guidance_epics_owner_user_id_idx on guidance_epics (owner_user_id);
create index guidance_epics_initiative_id_idx on guidance_epics (initiative_id);
create index guidance_epics_status_idx on guidance_epics (status);

create index guidance_quests_owner_user_id_idx on guidance_quests (owner_user_id);
create index guidance_quests_epic_id_idx on guidance_quests (epic_id);
create index guidance_quests_initiative_id_idx on guidance_quests (initiative_id);
create index guidance_quests_status_idx on guidance_quests (status);
create index guidance_quests_due_date_idx on guidance_quests (due_date);

create index guidance_routines_owner_user_id_idx on guidance_routines (owner_user_id);
create index guidance_routines_household_id_idx on guidance_routines (household_id);
create index guidance_routines_status_idx on guidance_routines (status);

create index guidance_focus_sessions_user_id_idx on guidance_focus_sessions (user_id);
create index guidance_focus_sessions_quest_id_idx on guidance_focus_sessions (quest_id);
create index guidance_focus_sessions_scheduled_start_idx on guidance_focus_sessions (scheduled_start);
create index guidance_focus_sessions_status_idx on guidance_focus_sessions (status);

create index guidance_daily_checkins_user_id_idx on guidance_daily_checkins (user_id);
create index guidance_daily_checkins_checkin_date_idx on guidance_daily_checkins (checkin_date);
