
create extension if not exists pgcrypto;

create table users (
  id uuid primary key default gen_random_uuid(),
  email text not null unique,
  display_name text not null,
  created_at timestamptz default now(),
  updated_at timestamptz default now()
);

create table households (
  id uuid primary key default gen_random_uuid(),
  owner_user_id uuid references users(id),
  name text not null,
  created_at timestamptz default now()
);

create table initiatives (
  id uuid primary key default gen_random_uuid(),
  owner_user_id uuid references users(id),
  household_id uuid references households(id),
  title text not null,
  status text default 'active',
  created_at timestamptz default now()
);

create table guidance_quests (
  id uuid primary key default gen_random_uuid(),
  initiative_id uuid references initiatives(id),
  title text not null,
  status text default 'todo',
  created_at timestamptz default now()
);

create table knowledge_notes (
  id uuid primary key default gen_random_uuid(),
  owner_user_id uuid references users(id),
  title text not null,
  content_markdown text,
  created_at timestamptz default now()
);

create table tracking_items (
  id uuid primary key default gen_random_uuid(),
  household_id uuid references households(id),
  name text not null,
  quantity numeric default 0,
  created_at timestamptz default now()
);

create table entity_relations (
  id uuid primary key default gen_random_uuid(),
  from_entity_type text not null,
  from_entity_id uuid not null,
  to_entity_type text not null,
  to_entity_id uuid not null,
  relation_type text not null,
  confidence numeric
);
