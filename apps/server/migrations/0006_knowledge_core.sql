-- Migration: 0006_knowledge_core
-- Description: Create knowledge tables for notes and snapshots

-- Notes: Core knowledge management
create table knowledge_notes (
	id uuid primary key default gen_random_uuid(),
	owner_user_id uuid not null references users(id) on delete cascade,
	parent_note_id uuid references knowledge_notes(id) on delete set null,
	title text not null,
	content_markdown text,
	is_archived boolean default false,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now(),
	deleted_at timestamptz
);

-- Note Snapshots: Version history for notes
create table knowledge_note_snapshots (
	id uuid primary key default gen_random_uuid(),
	note_id uuid not null references knowledge_notes(id) on delete cascade,
	title text not null,
	content_markdown text,
	snapshot_reason text default 'auto',
	created_at timestamptz not null default now()
);

-- Indexes for knowledge tables
create index knowledge_notes_owner_user_id_idx on knowledge_notes (owner_user_id);
create index knowledge_notes_parent_note_id_idx on knowledge_notes (parent_note_id);
create index knowledge_notes_is_archived_idx on knowledge_notes (is_archived) where is_archived = false;
create index knowledge_notes_created_at_idx on knowledge_notes (created_at);

create index knowledge_note_snapshots_note_id_idx on knowledge_note_snapshots (note_id);
create index knowledge_note_snapshots_created_at_idx on knowledge_note_snapshots (created_at);
