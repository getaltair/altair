-- Attachments schema: file metadata and entity associations
-- Reference file (not executable - use migrations/)

create table attachments (
	id uuid primary key default gen_random_uuid(),
	owner_user_id uuid not null references users(id) on delete cascade,
	filename text not null,
	original_filename text not null,
	mime_type text not null,
	file_size bigint not null,
	storage_path text not null,
	checksum text,
	description text,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now(),
	deleted_at timestamptz
);

create table quest_attachments (
	id uuid primary key default gen_random_uuid(),
	quest_id uuid not null references guidance_quests(id) on delete cascade,
	attachment_id uuid not null references attachments(id) on delete cascade,
	created_at timestamptz not null default now(),
	constraint quest_attachments_unique unique (quest_id, attachment_id)
);

create table note_attachments (
	id uuid primary key default gen_random_uuid(),
	note_id uuid not null references knowledge_notes(id) on delete cascade,
	attachment_id uuid not null references attachments(id) on delete cascade,
	created_at timestamptz not null default now(),
	constraint note_attachments_unique unique (note_id, attachment_id)
);

create table item_attachments (
	id uuid primary key default gen_random_uuid(),
	item_id uuid not null references tracking_items(id) on delete cascade,
	attachment_id uuid not null references attachments(id) on delete cascade,
	created_at timestamptz not null default now(),
	constraint item_attachments_unique unique (item_id, attachment_id)
);
