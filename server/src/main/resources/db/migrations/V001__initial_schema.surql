-- ============================================================================
-- Altair Initial Database Schema
-- Migration V001: Create all core tables
-- ============================================================================

-- ============================================================================
-- SYSTEM TABLES
-- ============================================================================

-- User (server-side only, not synced to clients)
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD username ON user TYPE string;
DEFINE FIELD email ON user TYPE option<string>;
DEFINE FIELD password_hash ON user TYPE string;
DEFINE FIELD role ON user TYPE string DEFAULT 'member';
DEFINE FIELD status ON user TYPE string DEFAULT 'active';
DEFINE FIELD storage_used ON user TYPE int DEFAULT 0;
DEFINE FIELD storage_quota ON user TYPE option<int>;
DEFINE FIELD created_at ON user TYPE datetime DEFAULT time::now();
DEFINE FIELD last_login_at ON user TYPE option<datetime>;
DEFINE INDEX username_idx ON user FIELDS username UNIQUE;
DEFINE INDEX email_idx ON user FIELDS email UNIQUE;

-- Initiative (cross-module organization)
DEFINE TABLE initiative SCHEMAFULL;
DEFINE FIELD user_id ON initiative TYPE record<user>;
DEFINE FIELD name ON initiative TYPE string;
DEFINE FIELD description ON initiative TYPE option<string>;
DEFINE FIELD parent_id ON initiative TYPE option<record<initiative>>;
DEFINE FIELD ongoing ON initiative TYPE bool DEFAULT false;
DEFINE FIELD target_date ON initiative TYPE option<datetime>;
DEFINE FIELD status ON initiative TYPE string DEFAULT 'active';
DEFINE FIELD focused ON initiative TYPE bool DEFAULT false;
DEFINE FIELD created_at ON initiative TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON initiative TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON initiative TYPE option<datetime>;
DEFINE FIELD sync_version ON initiative TYPE int DEFAULT 0;
DEFINE INDEX user_idx ON initiative FIELDS user_id;
DEFINE INDEX status_idx ON initiative FIELDS user_id, status;

-- Inbox Item (universal capture)
DEFINE TABLE inbox_item SCHEMAFULL;
DEFINE FIELD user_id ON inbox_item TYPE record<user>;
DEFINE FIELD content ON inbox_item TYPE string;
DEFINE FIELD source ON inbox_item TYPE string;
DEFINE FIELD created_at ON inbox_item TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON inbox_item TYPE option<datetime>;
DEFINE FIELD sync_version ON inbox_item TYPE int DEFAULT 0;
DEFINE INDEX user_idx ON inbox_item FIELDS user_id;

-- Routine (recurring templates)
DEFINE TABLE routine SCHEMAFULL;
DEFINE FIELD user_id ON routine TYPE record<user>;
DEFINE FIELD name ON routine TYPE string;
DEFINE FIELD description ON routine TYPE option<string>;
DEFINE FIELD schedule ON routine TYPE string;
DEFINE FIELD time_of_day ON routine TYPE option<string>;
DEFINE FIELD energy_cost ON routine TYPE int DEFAULT 1;
DEFINE FIELD initiative_id ON routine TYPE option<record<initiative>>;
DEFINE FIELD active ON routine TYPE bool DEFAULT true;
DEFINE FIELD next_due ON routine TYPE option<datetime>;
DEFINE FIELD created_at ON routine TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON routine TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON routine TYPE option<datetime>;
DEFINE FIELD sync_version ON routine TYPE int DEFAULT 0;
DEFINE INDEX user_idx ON routine FIELDS user_id;
DEFINE INDEX active_idx ON routine FIELDS user_id, active;

-- SourceDocument (for Knowledge extraction)
DEFINE TABLE source_document SCHEMAFULL;
DEFINE FIELD user_id ON source_document TYPE record<user>;
DEFINE FIELD title ON source_document TYPE string;
DEFINE FIELD source_type ON source_document TYPE string;
DEFINE FIELD source_path ON source_document TYPE string;
DEFINE FIELD mime_type ON source_document TYPE string;
DEFINE FIELD content_hash ON source_document TYPE string;
DEFINE FIELD extracted_text ON source_document TYPE option<string>;
DEFINE FIELD embedding ON source_document TYPE option<array<float>>;
DEFINE FIELD status ON source_document TYPE string DEFAULT 'pending';
DEFINE FIELD error_message ON source_document TYPE option<string>;
DEFINE FIELD initiative_id ON source_document TYPE option<record<initiative>>;
DEFINE FIELD watched_folder_id ON source_document TYPE option<record<watched_folder>>;
DEFINE FIELD last_synced_at ON source_document TYPE option<datetime>;
DEFINE FIELD created_at ON source_document TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON source_document TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON source_document TYPE option<datetime>;
DEFINE FIELD sync_version ON source_document TYPE int DEFAULT 0;
DEFINE INDEX user_idx ON source_document FIELDS user_id;
DEFINE INDEX status_idx ON source_document FIELDS user_id, status;
DEFINE INDEX hash_idx ON source_document FIELDS user_id, content_hash;
DEFINE INDEX search_idx ON source_document FIELDS extracted_text SEARCH ANALYZER snowball(ENGLISH);
DEFINE INDEX embedding_idx ON source_document FIELDS embedding MTREE DIMENSION 384;

-- SourceAnnotation (highlights, comments on source documents)
DEFINE TABLE source_annotation SCHEMAFULL;
DEFINE FIELD user_id ON source_annotation TYPE record<user>;
DEFINE FIELD source_document_id ON source_annotation TYPE record<source_document>;
DEFINE FIELD anchor_type ON source_annotation TYPE string;
DEFINE FIELD anchor_value ON source_annotation TYPE option<string>;
DEFINE FIELD anchor_fingerprint ON source_annotation TYPE option<object>;
DEFINE FIELD content ON source_annotation TYPE string;
DEFINE FIELD created_at ON source_annotation TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON source_annotation TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON source_annotation TYPE option<datetime>;
DEFINE FIELD sync_version ON source_annotation TYPE int DEFAULT 0;
DEFINE INDEX user_idx ON source_annotation FIELDS user_id;
DEFINE INDEX document_idx ON source_annotation FIELDS user_id, source_document_id;

-- WatchedFolder (automatic document discovery)
DEFINE TABLE watched_folder SCHEMAFULL;
DEFINE FIELD user_id ON watched_folder TYPE record<user>;
DEFINE FIELD path ON watched_folder TYPE string;
DEFINE FIELD include_patterns ON watched_folder TYPE option<array<string>>;
DEFINE FIELD exclude_patterns ON watched_folder TYPE option<array<string>>;
DEFINE FIELD initiative_id ON watched_folder TYPE option<record<initiative>>;
DEFINE FIELD scan_interval ON watched_folder TYPE int DEFAULT 60;
DEFINE FIELD status ON watched_folder TYPE string DEFAULT 'active';
DEFINE FIELD last_scanned_at ON watched_folder TYPE option<datetime>;
DEFINE FIELD created_at ON watched_folder TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON watched_folder TYPE datetime DEFAULT time::now();
DEFINE INDEX user_idx ON watched_folder FIELDS user_id;
DEFINE INDEX path_idx ON watched_folder FIELDS user_id, path UNIQUE;

-- ExtractionJob (server-side only, background processing)
DEFINE TABLE extraction_job SCHEMAFULL;
DEFINE FIELD source_document_id ON extraction_job TYPE record<source_document>;
DEFINE FIELD status ON extraction_job TYPE string DEFAULT 'queued';
DEFINE FIELD error_message ON extraction_job TYPE option<string>;
DEFINE FIELD created_at ON extraction_job TYPE datetime DEFAULT time::now();
DEFINE FIELD started_at ON extraction_job TYPE option<datetime>;
DEFINE FIELD completed_at ON extraction_job TYPE option<datetime>;
DEFINE INDEX document_idx ON extraction_job FIELDS source_document_id;
DEFINE INDEX status_idx ON extraction_job FIELDS status;

-- ============================================================================
-- GUIDANCE MODULE
-- ============================================================================

-- Epic (larger goal containing multiple quests)
DEFINE TABLE epic SCHEMAFULL;
DEFINE FIELD user_id ON epic TYPE record<user>;
DEFINE FIELD title ON epic TYPE string;
DEFINE FIELD description ON epic TYPE option<string>;
DEFINE FIELD status ON epic TYPE string DEFAULT 'active';
DEFINE FIELD initiative_id ON epic TYPE option<record<initiative>>;
DEFINE FIELD created_at ON epic TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON epic TYPE datetime DEFAULT time::now();
DEFINE FIELD completed_at ON epic TYPE option<datetime>;
DEFINE FIELD deleted_at ON epic TYPE option<datetime>;
DEFINE FIELD sync_version ON epic TYPE int DEFAULT 0;
DEFINE INDEX user_idx ON epic FIELDS user_id;
DEFINE INDEX initiative_idx ON epic FIELDS user_id, initiative_id;

-- Quest (atomic task unit with energy cost)
DEFINE TABLE quest SCHEMAFULL;
DEFINE FIELD user_id ON quest TYPE record<user>;
DEFINE FIELD title ON quest TYPE string;
DEFINE FIELD description ON quest TYPE option<string>;
DEFINE FIELD status ON quest TYPE string DEFAULT 'backlog';
DEFINE FIELD energy_cost ON quest TYPE int DEFAULT 1;
DEFINE FIELD epic_id ON quest TYPE option<record<epic>>;
DEFINE FIELD routine_id ON quest TYPE option<record<routine>>;
DEFINE FIELD created_at ON quest TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON quest TYPE datetime DEFAULT time::now();
DEFINE FIELD started_at ON quest TYPE option<datetime>;
DEFINE FIELD completed_at ON quest TYPE option<datetime>;
DEFINE FIELD deleted_at ON quest TYPE option<datetime>;
DEFINE FIELD sync_version ON quest TYPE int DEFAULT 0;
DEFINE INDEX user_idx ON quest FIELDS user_id;
DEFINE INDEX status_idx ON quest FIELDS user_id, status;
DEFINE INDEX epic_idx ON quest FIELDS user_id, epic_id;
DEFINE INDEX routine_idx ON quest FIELDS user_id, routine_id;

-- Checkpoint (sub-steps within a quest)
DEFINE TABLE checkpoint SCHEMAFULL;
DEFINE FIELD user_id ON checkpoint TYPE record<user>;
DEFINE FIELD quest_id ON checkpoint TYPE record<quest>;
DEFINE FIELD title ON checkpoint TYPE string;
DEFINE FIELD completed ON checkpoint TYPE bool DEFAULT false;
DEFINE FIELD sort_order ON checkpoint TYPE int DEFAULT 0;
DEFINE FIELD created_at ON checkpoint TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON checkpoint TYPE datetime DEFAULT time::now();
DEFINE INDEX quest_idx ON checkpoint FIELDS quest_id;

-- Energy Budget (daily capacity tracking for ADHD-aware planning)
DEFINE TABLE energy_budget SCHEMAFULL;
DEFINE FIELD user_id ON energy_budget TYPE record<user>;
DEFINE FIELD date ON energy_budget TYPE datetime;
DEFINE FIELD budget ON energy_budget TYPE int DEFAULT 5;
DEFINE FIELD spent ON energy_budget TYPE int DEFAULT 0;
DEFINE INDEX user_date_idx ON energy_budget FIELDS user_id, date UNIQUE;

-- ============================================================================
-- KNOWLEDGE MODULE
-- ============================================================================

-- Folder (hierarchical organization for notes)
DEFINE TABLE folder SCHEMAFULL;
DEFINE FIELD user_id ON folder TYPE record<user>;
DEFINE FIELD name ON folder TYPE string;
DEFINE FIELD parent_id ON folder TYPE option<record<folder>>;
DEFINE FIELD created_at ON folder TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON folder TYPE datetime DEFAULT time::now();
DEFINE INDEX user_idx ON folder FIELDS user_id;
DEFINE INDEX parent_idx ON folder FIELDS parent_id;

-- Tag (categorical labels for notes)
DEFINE TABLE tag SCHEMAFULL;
DEFINE FIELD user_id ON tag TYPE record<user>;
DEFINE FIELD name ON tag TYPE string;
DEFINE FIELD color ON tag TYPE option<string>;
DEFINE FIELD created_at ON tag TYPE datetime DEFAULT time::now();
DEFINE INDEX user_idx ON tag FIELDS user_id;
DEFINE INDEX user_name_idx ON tag FIELDS user_id, name UNIQUE;

-- Note (PKM core entity with vector embeddings)
DEFINE TABLE note SCHEMAFULL;
DEFINE FIELD user_id ON note TYPE record<user>;
DEFINE FIELD title ON note TYPE string;
DEFINE FIELD content ON note TYPE option<string>;
DEFINE FIELD folder_id ON note TYPE option<record<folder>>;
DEFINE FIELD initiative_id ON note TYPE option<record<initiative>>;
DEFINE FIELD embedding ON note TYPE option<array<float>>;
DEFINE FIELD created_at ON note TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON note TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON note TYPE option<datetime>;
DEFINE FIELD sync_version ON note TYPE int DEFAULT 0;
DEFINE INDEX user_idx ON note FIELDS user_id;
DEFINE INDEX folder_idx ON note FIELDS user_id, folder_id;
DEFINE INDEX initiative_idx ON note FIELDS user_id, initiative_id;
DEFINE INDEX title_idx ON note FIELDS user_id, folder_id, title UNIQUE;
DEFINE INDEX search_idx ON note FIELDS content SEARCH ANALYZER snowball(ENGLISH);
DEFINE INDEX embedding_idx ON note FIELDS embedding MTREE DIMENSION 384;

-- Note Link (bidirectional wiki-style links between notes)
DEFINE TABLE note_link SCHEMAFULL;
DEFINE FIELD user_id ON note_link TYPE record<user>;
DEFINE FIELD source_note_id ON note_link TYPE record<note>;
DEFINE FIELD target_note_id ON note_link TYPE record<note>;
DEFINE FIELD link_text ON note_link TYPE option<string>;
DEFINE FIELD created_at ON note_link TYPE datetime DEFAULT time::now();
DEFINE INDEX source_idx ON note_link FIELDS source_note_id;
DEFINE INDEX target_idx ON note_link FIELDS target_note_id;

-- Note-Tag Junction (many-to-many relationship)
DEFINE TABLE note_tag SCHEMAFULL;
DEFINE FIELD note_id ON note_tag TYPE record<note>;
DEFINE FIELD tag_id ON note_tag TYPE record<tag>;
DEFINE INDEX note_idx ON note_tag FIELDS note_id;
DEFINE INDEX tag_idx ON note_tag FIELDS tag_id;
DEFINE INDEX unique_idx ON note_tag FIELDS note_id, tag_id UNIQUE;

-- Attachment (files attached to notes)
DEFINE TABLE attachment SCHEMAFULL;
DEFINE FIELD user_id ON attachment TYPE record<user>;
DEFINE FIELD note_id ON attachment TYPE record<note>;
DEFINE FIELD filename ON attachment TYPE string;
DEFINE FIELD mime_type ON attachment TYPE string;
DEFINE FIELD storage_path ON attachment TYPE string;
DEFINE FIELD file_size ON attachment TYPE int;
DEFINE FIELD created_at ON attachment TYPE datetime DEFAULT time::now();
DEFINE INDEX note_idx ON attachment FIELDS note_id;

-- ============================================================================
-- TRACKING MODULE
-- ============================================================================

-- Location (physical or logical places where items are stored)
DEFINE TABLE location SCHEMAFULL;
DEFINE FIELD user_id ON location TYPE record<user>;
DEFINE FIELD name ON location TYPE string;
DEFINE FIELD description ON location TYPE option<string>;
DEFINE FIELD parent_id ON location TYPE option<record<location>>;
DEFINE FIELD created_at ON location TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON location TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON location TYPE option<datetime>;
DEFINE INDEX user_idx ON location FIELDS user_id;
DEFINE INDEX parent_idx ON location FIELDS parent_id;

-- Container (boxes, bins, shelves within locations)
DEFINE TABLE container SCHEMAFULL;
DEFINE FIELD user_id ON container TYPE record<user>;
DEFINE FIELD name ON container TYPE string;
DEFINE FIELD description ON container TYPE option<string>;
DEFINE FIELD location_id ON container TYPE option<record<location>>;
DEFINE FIELD created_at ON container TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON container TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON container TYPE option<datetime>;
DEFINE INDEX user_idx ON container FIELDS user_id;
DEFINE INDEX location_idx ON container FIELDS location_id;

-- Item Template (reusable item definitions with custom fields)
DEFINE TABLE item_template SCHEMAFULL;
DEFINE FIELD user_id ON item_template TYPE record<user>;
DEFINE FIELD name ON item_template TYPE string;
DEFINE FIELD description ON item_template TYPE option<string>;
DEFINE FIELD default_quantity ON item_template TYPE int DEFAULT 1;
DEFINE FIELD created_at ON item_template TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON item_template TYPE datetime DEFAULT time::now();
DEFINE INDEX user_idx ON item_template FIELDS user_id;

-- Field Definition (schema for custom fields on item templates)
DEFINE TABLE field_definition SCHEMAFULL;
DEFINE FIELD user_id ON field_definition TYPE record<user>;
DEFINE FIELD template_id ON field_definition TYPE record<item_template>;
DEFINE FIELD name ON field_definition TYPE string;
DEFINE FIELD field_type ON field_definition TYPE string;
DEFINE FIELD required ON field_definition TYPE bool DEFAULT false;
DEFINE FIELD default_value ON field_definition TYPE option<string>;
DEFINE FIELD sort_order ON field_definition TYPE int DEFAULT 0;
DEFINE INDEX template_idx ON field_definition FIELDS template_id;

-- Item (tracked possessions with custom metadata)
DEFINE TABLE item SCHEMAFULL;
DEFINE FIELD user_id ON item TYPE record<user>;
DEFINE FIELD name ON item TYPE string;
DEFINE FIELD description ON item TYPE option<string>;
DEFINE FIELD quantity ON item TYPE int DEFAULT 1;
DEFINE FIELD template_id ON item TYPE option<record<item_template>>;
DEFINE FIELD location_id ON item TYPE option<record<location>>;
DEFINE FIELD container_id ON item TYPE option<record<container>>;
DEFINE FIELD initiative_id ON item TYPE option<record<initiative>>;
DEFINE FIELD created_at ON item TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON item TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON item TYPE option<datetime>;
DEFINE FIELD sync_version ON item TYPE int DEFAULT 0;
DEFINE INDEX user_idx ON item FIELDS user_id;
DEFINE INDEX location_idx ON item FIELDS user_id, location_id;
DEFINE INDEX container_idx ON item FIELDS user_id, container_id;
DEFINE INDEX initiative_idx ON item FIELDS user_id, initiative_id;

-- Custom Field (instance-specific field values for items)
DEFINE TABLE custom_field SCHEMAFULL;
DEFINE FIELD user_id ON custom_field TYPE record<user>;
DEFINE FIELD item_id ON custom_field TYPE record<item>;
DEFINE FIELD field_definition_id ON custom_field TYPE option<record<field_definition>>;
DEFINE FIELD name ON custom_field TYPE string;
DEFINE FIELD value ON custom_field TYPE string;
DEFINE INDEX item_idx ON custom_field FIELDS item_id;

-- ============================================================================
-- END OF SCHEMA
-- ============================================================================
