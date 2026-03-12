-- Tracking schema: locations, categories, items, events, shopping lists
-- Reference file (not executable - use migrations/)

create table tracking_locations (
	id uuid primary key default gen_random_uuid(),
	household_id uuid references households(id) on delete cascade,
	name text not null,
	description text,
	parent_location_id uuid references tracking_locations(id) on delete set null,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now(),
	deleted_at timestamptz
);

create table tracking_categories (
	id uuid primary key default gen_random_uuid(),
	household_id uuid references households(id) on delete cascade,
	name text not null,
	description text,
	color text,
	parent_category_id uuid references tracking_categories(id) on delete set null,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now(),
	deleted_at timestamptz
);

create table tracking_items (
	id uuid primary key default gen_random_uuid(),
	household_id uuid references households(id) on delete cascade,
	category_id uuid references tracking_categories(id) on delete set null,
	location_id uuid references tracking_locations(id) on delete set null,
	name text not null,
	description text,
	quantity numeric not null default 0,
	unit text,
	min_quantity numeric,
	max_quantity numeric,
	sku text,
	barcode text,
	acquired_date date,
	expiry_date date,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now(),
	deleted_at timestamptz
);

create type item_event_type as enum ('created', 'updated', 'quantity_changed', 'location_changed', 'consumed', 'restocked', 'expired', 'disposed');

create table tracking_item_events (
	id uuid primary key default gen_random_uuid(),
	item_id uuid not null references tracking_items(id) on delete cascade,
	event_type item_event_type not null,
	quantity_delta numeric,
	previous_value jsonb,
	new_value jsonb,
	notes text,
	created_at timestamptz not null default now()
);

create table tracking_shopping_lists (
	id uuid primary key default gen_random_uuid(),
	household_id uuid references households(id) on delete cascade,
	name text not null,
	description text,
	is_completed boolean default false,
	completed_at timestamptz,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now(),
	deleted_at timestamptz
);

create type shopping_item_status as enum ('needed', 'purchased', 'unavailable');

create table tracking_shopping_list_items (
	id uuid primary key default gen_random_uuid(),
	shopping_list_id uuid not null references tracking_shopping_lists(id) on delete cascade,
	item_id uuid references tracking_items(id) on delete set null,
	name text not null,
	quantity numeric not null default 1,
	unit text,
	status shopping_item_status not null default 'needed',
	notes text,
	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now()
);
