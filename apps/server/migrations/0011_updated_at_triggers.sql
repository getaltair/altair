-- Migration: 0011_updated_at_triggers
-- Description: Add automatic updated_at timestamp triggers for all mutable tables

-- Create the trigger function
create or replace function set_updated_at()
returns trigger as $$
begin
	new.updated_at = now();
	return new;
end;
$$ language plpgsql;

-- Apply triggers to all tables with updated_at column

-- Core tables
create trigger users_updated_at
	before update on users
	for each row
	execute function set_updated_at();

create trigger households_updated_at
	before update on households
	for each row
	execute function set_updated_at();

create trigger household_memberships_updated_at
	before update on household_memberships
	for each row
	execute function set_updated_at();

create trigger initiatives_updated_at
	before update on initiatives
	for each row
	execute function set_updated_at();

-- Guidance tables
create trigger guidance_epics_updated_at
	before update on guidance_epics
	for each row
	execute function set_updated_at();

create trigger guidance_quests_updated_at
	before update on guidance_quests
	for each row
	execute function set_updated_at();

create trigger guidance_routines_updated_at
	before update on guidance_routines
	for each row
	execute function set_updated_at();

create trigger guidance_focus_sessions_updated_at
	before update on guidance_focus_sessions
	for each row
	execute function set_updated_at();

create trigger guidance_daily_checkins_updated_at
	before update on guidance_daily_checkins
	for each row
	execute function set_updated_at();

-- Knowledge tables
create trigger knowledge_notes_updated_at
	before update on knowledge_notes
	for each row
	execute function set_updated_at();

-- Tracking tables
create trigger tracking_locations_updated_at
	before update on tracking_locations
	for each row
	execute function set_updated_at();

create trigger tracking_categories_updated_at
	before update on tracking_categories
	for each row
	execute function set_updated_at();

create trigger tracking_items_updated_at
	before update on tracking_items
	for each row
	execute function set_updated_at();

create trigger tracking_shopping_lists_updated_at
	before update on tracking_shopping_lists
	for each row
	execute function set_updated_at();

create trigger tracking_shopping_list_items_updated_at
	before update on tracking_shopping_list_items
	for each row
	execute function set_updated_at();

-- Tagging
create trigger tags_updated_at
	before update on tags
	for each row
	execute function set_updated_at();

-- Attachments
create trigger attachments_updated_at
	before update on attachments
	for each row
	execute function set_updated_at();

-- Entity relations
create trigger entity_relations_updated_at
	before update on entity_relations
	for each row
	execute function set_updated_at();
