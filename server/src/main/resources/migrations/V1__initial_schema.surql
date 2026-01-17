-- V1: Initial Schema for Altair
-- Creates all core tables with user isolation

-- ============================================================
-- SYSTEM MODULE
-- ============================================================

-- Users table (system-wide, not user-scoped)
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD email ON user TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD display_name ON user TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD role ON user TYPE string ASSERT $value IN ['admin', 'member'];
DEFINE FIELD status ON user TYPE string ASSERT $value IN ['active', 'suspended', 'pending'];
DEFINE FIELD storage_used_bytes ON user TYPE int DEFAULT 0;
DEFINE FIELD storage_quota_bytes ON user TYPE int DEFAULT 10737418240; -- 10GB
DEFINE FIELD created_at ON user TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON user TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON user TYPE option<datetime>;
DEFINE INDEX idx_user_email ON user FIELDS email UNIQUE;
DEFINE INDEX idx_user_status ON user FIELDS status;

-- Initiatives (cross-cutting organizational units)
DEFINE TABLE initiative SCHEMAFULL;
DEFINE FIELD user_id ON initiative TYPE record<user>;
DEFINE FIELD name ON initiative TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD description ON initiative TYPE option<string>;
DEFINE FIELD color ON initiative TYPE option<string>;
DEFINE FIELD icon ON initiative TYPE option<string>;
DEFINE FIELD status ON initiative TYPE string ASSERT $value IN ['active', 'archived', 'on_hold'];
DEFINE FIELD created_at ON initiative TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON initiative TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON initiative TYPE option<datetime>;
DEFINE INDEX idx_initiative_user ON initiative FIELDS user_id;
DEFINE INDEX idx_initiative_status ON initiative FIELDS user_id, status;

-- Inbox Items (quick capture)
DEFINE TABLE inbox_item SCHEMAFULL;
DEFINE FIELD user_id ON inbox_item TYPE record<user>;
DEFINE FIELD content ON inbox_item TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD source ON inbox_item TYPE string ASSERT $value IN ['manual', 'voice', 'share', 'email', 'api'];
DEFINE FIELD attachment_ids ON inbox_item TYPE array DEFAULT [];
DEFINE FIELD created_at ON inbox_item TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON inbox_item TYPE datetime DEFAULT time::now();
DEFINE INDEX idx_inbox_user ON inbox_item FIELDS user_id;
DEFINE INDEX idx_inbox_created ON inbox_item FIELDS user_id, created_at;

-- Routines (recurring templates)
DEFINE TABLE routine SCHEMAFULL;
DEFINE FIELD user_id ON routine TYPE record<user>;
DEFINE FIELD title ON routine TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD description ON routine TYPE option<string>;
DEFINE FIELD energy_cost ON routine TYPE int ASSERT $value >= 1 AND $value <= 5;
DEFINE FIELD schedule ON routine TYPE object;
DEFINE FIELD scheduled_time ON routine TYPE option<string>;
DEFINE FIELD initiative_id ON routine TYPE option<record<initiative>>;
DEFINE FIELD is_active ON routine TYPE bool DEFAULT true;
DEFINE FIELD last_spawned_at ON routine TYPE option<datetime>;
DEFINE FIELD created_at ON routine TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON routine TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON routine TYPE option<datetime>;
DEFINE INDEX idx_routine_user ON routine FIELDS user_id;
DEFINE INDEX idx_routine_active ON routine FIELDS user_id, is_active;

-- ============================================================
-- GUIDANCE MODULE
-- ============================================================

-- Epics (large goals)
DEFINE TABLE epic SCHEMAFULL;
DEFINE FIELD user_id ON epic TYPE record<user>;
DEFINE FIELD title ON epic TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD description ON epic TYPE option<string>;
DEFINE FIELD status ON epic TYPE string ASSERT $value IN ['active', 'completed', 'abandoned'];
DEFINE FIELD initiative_id ON epic TYPE option<record<initiative>>;
DEFINE FIELD target_date ON epic TYPE option<string>;
DEFINE FIELD created_at ON epic TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON epic TYPE datetime DEFAULT time::now();
DEFINE FIELD completed_at ON epic TYPE option<datetime>;
DEFINE FIELD deleted_at ON epic TYPE option<datetime>;
DEFINE INDEX idx_epic_user ON epic FIELDS user_id;
DEFINE INDEX idx_epic_status ON epic FIELDS user_id, status;
DEFINE INDEX idx_epic_initiative ON epic FIELDS initiative_id;

-- Quests (actionable tasks)
DEFINE TABLE quest SCHEMAFULL;
DEFINE FIELD user_id ON quest TYPE record<user>;
DEFINE FIELD title ON quest TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD description ON quest TYPE option<string>;
DEFINE FIELD energy_cost ON quest TYPE int ASSERT $value >= 1 AND $value <= 5;
DEFINE FIELD status ON quest TYPE string ASSERT $value IN ['backlog', 'active', 'completed', 'abandoned'];
DEFINE FIELD epic_id ON quest TYPE option<record<epic>>;
DEFINE FIELD routine_id ON quest TYPE option<record<routine>>;
DEFINE FIELD initiative_id ON quest TYPE option<record<initiative>>;
DEFINE FIELD due_date ON quest TYPE option<string>;
DEFINE FIELD scheduled_date ON quest TYPE option<string>;
DEFINE FIELD created_at ON quest TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON quest TYPE datetime DEFAULT time::now();
DEFINE FIELD started_at ON quest TYPE option<datetime>;
DEFINE FIELD completed_at ON quest TYPE option<datetime>;
DEFINE FIELD deleted_at ON quest TYPE option<datetime>;
DEFINE INDEX idx_quest_user ON quest FIELDS user_id;
DEFINE INDEX idx_quest_status ON quest FIELDS user_id, status;
DEFINE INDEX idx_quest_epic ON quest FIELDS epic_id;
DEFINE INDEX idx_quest_routine ON quest FIELDS routine_id;
DEFINE INDEX idx_quest_initiative ON quest FIELDS initiative_id;
DEFINE INDEX idx_quest_scheduled ON quest FIELDS user_id, scheduled_date;
DEFINE INDEX idx_quest_due ON quest FIELDS user_id, due_date;

-- Checkpoints (quest sub-steps)
DEFINE TABLE checkpoint SCHEMAFULL;
DEFINE FIELD user_id ON checkpoint TYPE record<user>;
DEFINE FIELD quest_id ON checkpoint TYPE record<quest>;
DEFINE FIELD title ON checkpoint TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD sort_order ON checkpoint TYPE int DEFAULT 0;
DEFINE FIELD is_completed ON checkpoint TYPE bool DEFAULT false;
DEFINE FIELD completed_at ON checkpoint TYPE option<datetime>;
DEFINE FIELD created_at ON checkpoint TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON checkpoint TYPE datetime DEFAULT time::now();
DEFINE INDEX idx_checkpoint_quest ON checkpoint FIELDS quest_id;
DEFINE INDEX idx_checkpoint_user ON checkpoint FIELDS user_id;

-- Energy Budgets (daily energy tracking)
DEFINE TABLE energy_budget SCHEMAFULL;
DEFINE FIELD user_id ON energy_budget TYPE record<user>;
DEFINE FIELD date ON energy_budget TYPE string; -- LocalDate as YYYY-MM-DD
DEFINE FIELD total_budget ON energy_budget TYPE int DEFAULT 10;
DEFINE FIELD spent_energy ON energy_budget TYPE int DEFAULT 0;
DEFINE FIELD created_at ON energy_budget TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON energy_budget TYPE datetime DEFAULT time::now();
DEFINE INDEX idx_energy_budget_user_date ON energy_budget FIELDS user_id, date UNIQUE;

-- ============================================================
-- KNOWLEDGE MODULE
-- ============================================================

-- Folders (hierarchical organization)
DEFINE TABLE folder SCHEMAFULL;
DEFINE FIELD user_id ON folder TYPE record<user>;
DEFINE FIELD name ON folder TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD parent_id ON folder TYPE option<record<folder>>;
DEFINE FIELD sort_order ON folder TYPE int DEFAULT 0;
DEFINE FIELD created_at ON folder TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON folder TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON folder TYPE option<datetime>;
DEFINE INDEX idx_folder_user ON folder FIELDS user_id;
DEFINE INDEX idx_folder_parent ON folder FIELDS parent_id;

-- Notes (markdown content)
DEFINE TABLE note SCHEMAFULL;
DEFINE FIELD user_id ON note TYPE record<user>;
DEFINE FIELD title ON note TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD content ON note TYPE string DEFAULT '';
DEFINE FIELD folder_id ON note TYPE option<record<folder>>;
DEFINE FIELD initiative_id ON note TYPE option<record<initiative>>;
DEFINE FIELD is_pinned ON note TYPE bool DEFAULT false;
DEFINE FIELD created_at ON note TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON note TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON note TYPE option<datetime>;
DEFINE INDEX idx_note_user ON note FIELDS user_id;
DEFINE INDEX idx_note_folder ON note FIELDS folder_id;
DEFINE INDEX idx_note_initiative ON note FIELDS initiative_id;
DEFINE INDEX idx_note_pinned ON note FIELDS user_id, is_pinned;
-- Full-text search index for notes
DEFINE ANALYZER note_analyzer TOKENIZERS blank, class FILTERS lowercase, ascii, snowball(english);
DEFINE INDEX idx_note_search ON note FIELDS title, content SEARCH ANALYZER note_analyzer;

-- Note Links (bi-directional connections)
DEFINE TABLE note_link SCHEMAFULL;
DEFINE FIELD user_id ON note_link TYPE record<user>;
DEFINE FIELD source_note_id ON note_link TYPE record<note>;
DEFINE FIELD target_note_id ON note_link TYPE record<note>;
DEFINE FIELD context ON note_link TYPE option<string>;
DEFINE FIELD created_at ON note_link TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON note_link TYPE datetime DEFAULT time::now();
DEFINE INDEX idx_note_link_source ON note_link FIELDS source_note_id;
DEFINE INDEX idx_note_link_target ON note_link FIELDS target_note_id;
DEFINE INDEX idx_note_link_unique ON note_link FIELDS source_note_id, target_note_id UNIQUE;

-- Tags (flat-namespace labels)
DEFINE TABLE tag SCHEMAFULL;
DEFINE FIELD user_id ON tag TYPE record<user>;
DEFINE FIELD name ON tag TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD color ON tag TYPE option<string>;
DEFINE FIELD created_at ON tag TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON tag TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON tag TYPE option<datetime>;
DEFINE INDEX idx_tag_user ON tag FIELDS user_id;
DEFINE INDEX idx_tag_name ON tag FIELDS user_id, name UNIQUE;

-- Note-Tag junction table
DEFINE TABLE note_tag SCHEMAFULL;
DEFINE FIELD note_id ON note_tag TYPE record<note>;
DEFINE FIELD tag_id ON note_tag TYPE record<tag>;
DEFINE FIELD created_at ON note_tag TYPE datetime DEFAULT time::now();
DEFINE INDEX idx_note_tag_note ON note_tag FIELDS note_id;
DEFINE INDEX idx_note_tag_tag ON note_tag FIELDS tag_id;
DEFINE INDEX idx_note_tag_unique ON note_tag FIELDS note_id, tag_id UNIQUE;

-- Attachments (file metadata)
DEFINE TABLE attachment SCHEMAFULL;
DEFINE FIELD user_id ON attachment TYPE record<user>;
DEFINE FIELD note_id ON attachment TYPE option<record<note>>;
DEFINE FIELD inbox_item_id ON attachment TYPE option<record<inbox_item>>;
DEFINE FIELD filename ON attachment TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD mime_type ON attachment TYPE string;
DEFINE FIELD size_bytes ON attachment TYPE int;
DEFINE FIELD storage_path ON attachment TYPE string;
DEFINE FIELD created_at ON attachment TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON attachment TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON attachment TYPE option<datetime>;
DEFINE INDEX idx_attachment_user ON attachment FIELDS user_id;
DEFINE INDEX idx_attachment_note ON attachment FIELDS note_id;
DEFINE INDEX idx_attachment_inbox ON attachment FIELDS inbox_item_id;

-- Source Documents (imported external documents)
DEFINE TABLE source_document SCHEMAFULL;
DEFINE FIELD user_id ON source_document TYPE record<user>;
DEFINE FIELD title ON source_document TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD source_type ON source_document TYPE string ASSERT $value IN ['file', 'url', 'watched_folder'];
DEFINE FIELD source_path ON source_document TYPE string;
DEFINE FIELD mime_type ON source_document TYPE option<string>;
DEFINE FIELD file_size_bytes ON source_document TYPE option<int>;
DEFINE FIELD page_count ON source_document TYPE option<int>;
DEFINE FIELD extraction_status ON source_document TYPE string ASSERT $value IN ['pending', 'processing', 'completed', 'failed'];
DEFINE FIELD extracted_text ON source_document TYPE option<string>;
DEFINE FIELD watched_folder_id ON source_document TYPE option<string>;
DEFINE FIELD initiative_id ON source_document TYPE option<record<initiative>>;
DEFINE FIELD created_at ON source_document TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON source_document TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON source_document TYPE option<datetime>;
DEFINE INDEX idx_source_doc_user ON source_document FIELDS user_id;
DEFINE INDEX idx_source_doc_status ON source_document FIELDS extraction_status;
DEFINE INDEX idx_source_doc_initiative ON source_document FIELDS initiative_id;

-- Source Annotations
DEFINE TABLE source_annotation SCHEMAFULL;
DEFINE FIELD user_id ON source_annotation TYPE record<user>;
DEFINE FIELD source_document_id ON source_annotation TYPE record<source_document>;
DEFINE FIELD anchor_type ON source_annotation TYPE string ASSERT $value IN ['page', 'text_range', 'time_range'];
DEFINE FIELD anchor_data ON source_annotation TYPE object;
DEFINE FIELD content ON source_annotation TYPE string;
DEFINE FIELD color ON source_annotation TYPE option<string>;
DEFINE FIELD linked_note_id ON source_annotation TYPE option<record<note>>;
DEFINE FIELD created_at ON source_annotation TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON source_annotation TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON source_annotation TYPE option<datetime>;
DEFINE INDEX idx_annotation_doc ON source_annotation FIELDS source_document_id;
DEFINE INDEX idx_annotation_user ON source_annotation FIELDS user_id;

-- ============================================================
-- TRACKING MODULE
-- ============================================================

-- Locations (physical places)
DEFINE TABLE location SCHEMAFULL;
DEFINE FIELD user_id ON location TYPE record<user>;
DEFINE FIELD name ON location TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD description ON location TYPE option<string>;
DEFINE FIELD parent_id ON location TYPE option<record<location>>;
DEFINE FIELD address ON location TYPE option<string>;
DEFINE FIELD created_at ON location TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON location TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON location TYPE option<datetime>;
DEFINE INDEX idx_location_user ON location FIELDS user_id;
DEFINE INDEX idx_location_parent ON location FIELDS parent_id;

-- Containers (movable storage)
DEFINE TABLE container SCHEMAFULL;
DEFINE FIELD user_id ON container TYPE record<user>;
DEFINE FIELD name ON container TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD description ON container TYPE option<string>;
DEFINE FIELD location_id ON container TYPE option<record<location>>;
DEFINE FIELD parent_container_id ON container TYPE option<record<container>>;
DEFINE FIELD label ON container TYPE option<string>;
DEFINE FIELD created_at ON container TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON container TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON container TYPE option<datetime>;
DEFINE INDEX idx_container_user ON container FIELDS user_id;
DEFINE INDEX idx_container_location ON container FIELDS location_id;
DEFINE INDEX idx_container_parent ON container FIELDS parent_container_id;

-- Item Templates (field schemas)
DEFINE TABLE item_template SCHEMAFULL;
DEFINE FIELD user_id ON item_template TYPE record<user>;
DEFINE FIELD name ON item_template TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD description ON item_template TYPE option<string>;
DEFINE FIELD icon ON item_template TYPE option<string>;
DEFINE FIELD created_at ON item_template TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON item_template TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON item_template TYPE option<datetime>;
DEFINE INDEX idx_item_template_user ON item_template FIELDS user_id;

-- Field Definitions (template fields)
DEFINE TABLE field_definition SCHEMAFULL;
DEFINE FIELD user_id ON field_definition TYPE record<user>;
DEFINE FIELD template_id ON field_definition TYPE record<item_template>;
DEFINE FIELD name ON field_definition TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD field_type ON field_definition TYPE string ASSERT $value IN ['text', 'number', 'date', 'boolean', 'enum', 'url'];
DEFINE FIELD is_required ON field_definition TYPE bool DEFAULT false;
DEFINE FIELD default_value ON field_definition TYPE option<string>;
DEFINE FIELD enum_options ON field_definition TYPE option<array>;
DEFINE FIELD sort_order ON field_definition TYPE int DEFAULT 0;
DEFINE FIELD created_at ON field_definition TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON field_definition TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON field_definition TYPE option<datetime>;
DEFINE INDEX idx_field_def_template ON field_definition FIELDS template_id;

-- Items (physical objects)
DEFINE TABLE item SCHEMAFULL;
DEFINE FIELD user_id ON item TYPE record<user>;
DEFINE FIELD name ON item TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD description ON item TYPE option<string>;
DEFINE FIELD template_id ON item TYPE option<record<item_template>>;
DEFINE FIELD location_id ON item TYPE option<record<location>>;
DEFINE FIELD container_id ON item TYPE option<record<container>>;
DEFINE FIELD quantity ON item TYPE int DEFAULT 1;
DEFINE FIELD photo_attachment_id ON item TYPE option<record<attachment>>;
DEFINE FIELD initiative_id ON item TYPE option<record<initiative>>;
DEFINE FIELD created_at ON item TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON item TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON item TYPE option<datetime>;
DEFINE INDEX idx_item_user ON item FIELDS user_id;
DEFINE INDEX idx_item_location ON item FIELDS location_id;
DEFINE INDEX idx_item_container ON item FIELDS container_id;
DEFINE INDEX idx_item_template ON item FIELDS template_id;
DEFINE INDEX idx_item_initiative ON item FIELDS initiative_id;

-- Custom Fields (item field values)
DEFINE TABLE custom_field SCHEMAFULL;
DEFINE FIELD item_id ON custom_field TYPE record<item>;
DEFINE FIELD field_definition_id ON custom_field TYPE record<field_definition>;
DEFINE FIELD value ON custom_field TYPE string;
DEFINE FIELD created_at ON custom_field TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON custom_field TYPE datetime DEFAULT time::now();
DEFINE INDEX idx_custom_field_item ON custom_field FIELDS item_id;
DEFINE INDEX idx_custom_field_def ON custom_field FIELDS field_definition_id;
DEFINE INDEX idx_custom_field_unique ON custom_field FIELDS item_id, field_definition_id UNIQUE;
