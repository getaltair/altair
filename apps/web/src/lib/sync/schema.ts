import { column, Schema, Table } from '@powersync/web';

// ============================================================
// PowerSync AppSchema
// Column names must exactly match Postgres migration columns.
// UUID columns  → column.text
// Timestamps    → column.text
// Booleans      → column.integer
// Numerics      → column.real
// Integers      → column.integer
// ============================================================

const users = new Table({
  password_hash: column.text,
  email: column.text,
  display_name: column.text,
  is_admin: column.integer,
  status: column.text,
  created_at: column.text,
  updated_at: column.text,
  deleted_at: column.text,
});

const households = new Table({
  owner_id: column.text,
  name: column.text,
  created_at: column.text,
  updated_at: column.text,
  deleted_at: column.text,
});

const household_memberships = new Table({
  household_id: column.text,
  user_id: column.text,
  role: column.text,
  created_at: column.text,
  updated_at: column.text,
  deleted_at: column.text,
});

const quests = new Table({
  title: column.text,
  description: column.text,
  status: column.text,
  priority: column.text,
  due_date: column.text,
  epic_id: column.text,
  initiative_id: column.text,
  routine_id: column.text,
  user_id: column.text,
  created_at: column.text,
  updated_at: column.text,
  deleted_at: column.text,
});

const routines = new Table({
  title: column.text,
  description: column.text,
  frequency_type: column.text,
  frequency_config: column.text,
  status: column.text,
  user_id: column.text,
  created_at: column.text,
  updated_at: column.text,
  deleted_at: column.text,
});

const epics = new Table({
  initiative_id: column.text,
  title: column.text,
  description: column.text,
  status: column.text,
  sort_order: column.integer,
  user_id: column.text,
  created_at: column.text,
  updated_at: column.text,
  deleted_at: column.text,
});

const initiatives = new Table({
  title: column.text,
  description: column.text,
  status: column.text,
  user_id: column.text,
  household_id: column.text,
  created_at: column.text,
  updated_at: column.text,
  deleted_at: column.text,
});

const focus_sessions = new Table({
  quest_id: column.text,
  started_at: column.text,
  ended_at: column.text,
  duration_minutes: column.integer,
  user_id: column.text,
  created_at: column.text,
  updated_at: column.text,
  deleted_at: column.text,
});

const daily_checkins = new Table({
  user_id: column.text,
  checkin_date: column.text,
  energy_level: column.integer,
  mood: column.text,
  notes: column.text,
  created_at: column.text,
  updated_at: column.text,
  deleted_at: column.text,
});

const notes = new Table({
  title: column.text,
  content: column.text,
  user_id: column.text,
  initiative_id: column.text,
  created_at: column.text,
  updated_at: column.text,
  deleted_at: column.text,
});

// note_snapshots is append-only: no updated_at, no deleted_at
const note_snapshots = new Table({
  note_id: column.text,
  content: column.text,
  captured_at: column.text,
  created_at: column.text,
});

const entity_relations = new Table({
  from_entity_type: column.text,
  from_entity_id: column.text,
  to_entity_type: column.text,
  to_entity_id: column.text,
  relation_type: column.text,
  source_type: column.text,
  status: column.text,
  confidence: column.real,
  evidence: column.text,
  user_id: column.text,
  created_at: column.text,
  updated_at: column.text,
  deleted_at: column.text,
});

const tags = new Table({
  name: column.text,
  user_id: column.text,
  created_at: column.text,
  updated_at: column.text,
});

// entity_tags is a generic union table for PowerSync sync purposes
const entity_tags = new Table({
  entity_id: column.text,
  entity_type: column.text,
  tag_id: column.text,
  created_at: column.text,
});

const tracking_items = new Table({
  name: column.text,
  description: column.text,
  quantity: column.real,
  barcode: column.text,
  location_id: column.text,
  category_id: column.text,
  user_id: column.text,
  household_id: column.text,
  initiative_id: column.text,
  expires_at: column.text,
  created_at: column.text,
  updated_at: column.text,
  deleted_at: column.text,
});

// tracking_item_events is append-only: no updated_at, no deleted_at
const tracking_item_events = new Table({
  item_id: column.text,
  event_type: column.text,
  quantity_change: column.real,
  from_location_id: column.text,
  to_location_id: column.text,
  notes: column.text,
  occurred_at: column.text,
  created_at: column.text,
});

const tracking_locations = new Table({
  name: column.text,
  household_id: column.text,
  created_at: column.text,
  updated_at: column.text,
  deleted_at: column.text,
});

const tracking_categories = new Table({
  name: column.text,
  household_id: column.text,
  created_at: column.text,
  updated_at: column.text,
  deleted_at: column.text,
});

const shopping_lists = new Table({
  name: column.text,
  household_id: column.text,
  created_at: column.text,
  updated_at: column.text,
  deleted_at: column.text,
});

const shopping_list_items = new Table({
  shopping_list_id: column.text,
  item_id: column.text,
  name: column.text,
  quantity: column.real,
  status: column.text,
  created_at: column.text,
  updated_at: column.text,
  deleted_at: column.text,
});

export const AppSchema = new Schema({
  users,
  households,
  household_memberships,
  quests,
  routines,
  epics,
  initiatives,
  focus_sessions,
  daily_checkins,
  notes,
  note_snapshots,
  entity_relations,
  tags,
  entity_tags,
  tracking_items,
  tracking_item_events,
  tracking_locations,
  tracking_categories,
  shopping_lists,
  shopping_list_items,
});

export type Database = (typeof AppSchema)['types'];
