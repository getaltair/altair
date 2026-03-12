-- Knowledge schema: notes and snapshots
-- Reference file (not executable - use migrations/)

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

create table knowledge_note_snapshots (
	id uuid primary key default gen_random_uuid(),
	note_id uuid not null references knowledge_notes(id) on delete cascade,
	title text not null,
	content_markdown text,
	snapshot_reason text default 'auto',
	created_at timestamptz not null default now()
);
