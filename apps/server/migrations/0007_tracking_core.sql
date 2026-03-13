-- Migration: 0007_tracking_core
-- Description: Create tracking tables for inventory, categories, and events

-- Locations: Physical locations for items
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

-- Categories: Item categorization
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

-- Items: Tracked inventory items
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

-- Item Events: Track changes to items over time
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

-- Shopping Lists: Manage shopping needs
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

-- Shopping List Items: Items within shopping lists
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

-- Indexes for tracking tables
create index tracking_locations_household_id_idx on tracking_locations (household_id);
create index tracking_locations_parent_location_id_idx on tracking_locations (parent_location_id);

create index tracking_categories_household_id_idx on tracking_categories (household_id);
create index tracking_categories_parent_category_id_idx on tracking_categories (parent_category_id);

create index tracking_items_household_id_idx on tracking_items (household_id);
create index tracking_items_category_id_idx on tracking_items (category_id);
create index tracking_items_location_id_idx on tracking_items (location_id);
create index tracking_items_barcode_idx on tracking_items (barcode);
create index tracking_items_expiry_date_idx on tracking_items (expiry_date) where expiry_date is not null;

create index tracking_item_events_item_id_idx on tracking_item_events (item_id);
create index tracking_item_events_created_at_idx on tracking_item_events (created_at);
create index tracking_item_events_event_type_idx on tracking_item_events (event_type);

create index tracking_shopping_lists_household_id_idx on tracking_shopping_lists (household_id);
create index tracking_shopping_lists_is_completed_idx on tracking_shopping_lists (is_completed) where is_completed = false;

create index tracking_shopping_list_items_shopping_list_id_idx on tracking_shopping_list_items (shopping_list_id);
create index tracking_shopping_list_items_item_id_idx on tracking_shopping_list_items (item_id);
create index tracking_shopping_list_items_status_idx on tracking_shopping_list_items (status);
