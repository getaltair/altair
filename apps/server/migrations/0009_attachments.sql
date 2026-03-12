-- Migration: 0009_attachments
-- Description: Create attachment system for file management

-- Attachments: File metadata storage
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

-- Quest Attachments: Link attachments to quests
create table quest_attachments (
	id uuid primary key default gen_random_uuid(),
	quest_id uuid not null references guidance_quests(id) on delete cascade,
	attachment_id uuid not null references attachments(id) on delete cascade,
	created_at timestamptz not null default now(),

	constraint quest_attachments_unique unique (quest_id, attachment_id)
);

-- Note Attachments: Link attachments to notes
create table note_attachments (
	id uuid primary key default gen_random_uuid(),
	note_id uuid not null references knowledge_notes(id) on delete cascade,
	attachment_id uuid not null references attachments(id) on delete cascade,
	created_at timestamptz not null default now(),

	constraint note_attachments_unique unique (note_id, attachment_id)
);

-- Item Attachments: Link attachments to tracking items
create table item_attachments (
	id uuid primary key default gen_random_uuid(),
	item_id uuid not null references tracking_items(id) on delete cascade,
	attachment_id uuid not null references attachments(id) on delete cascade,
	created_at timestamptz not null default now(),

	constraint item_attachments_unique unique (item_id, attachment_id)
);

-- Indexes for attachments
create index attachments_owner_user_id_idx on attachments (owner_user_id);
create index attachments_mime_type_idx on attachments (mime_type);
create index attachments_created_at_idx on attachments (created_at);
create index attachments_checksum_idx on attachments (checksum);

create index quest_attachments_quest_id_idx on quest_attachments (quest_id);
create index quest_attachments_attachment_id_idx on quest_attachments (attachment_id);

create index note_attachments_note_id_idx on note_attachments (note_id);
create index note_attachments_attachment_id_idx on note_attachments (attachment_id);

create index item_attachments_item_id_idx on item_attachments (item_id);
create index item_attachments_attachment_id_idx on item_attachments (attachment_id);
