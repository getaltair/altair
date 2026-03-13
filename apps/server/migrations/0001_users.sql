-- Migration: 0001_users
-- Description: Create the users table with core user data

-- Enable pgcrypto for gen_random_uuid()
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

-- Index for email lookups
create index users_email_idx on users (email);

-- Index for active user queries
create index users_is_active_idx on users (is_active) where is_active = true;
