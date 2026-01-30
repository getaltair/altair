-- ============================================================================
-- Altair Desktop Embedded Database Schema
-- Migration V001: Create all core tables (single-user, no user_id scoping)
-- ============================================================================

-- ============================================================================
-- SYSTEM TABLES
-- ============================================================================

-- User (local user profile)
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD username ON user TYPE string;
DEFINE FIELD email ON user TYPE option<string>;
DEFINE FIELD created_at ON user TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON user TYPE datetime DEFAULT time::now();
DEFINE INDEX username_idx ON user FIELDS username UNIQUE;

-- Initiative (cross-module organization)
DEFINE TABLE initiative SCHEMAFULL;
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
DEFINE INDEX status_idx ON initiative FIELDS status;

-- Inbox Item (universal capture)
DEFINE TABLE inbox_item SCHEMAFULL;
DEFINE FIELD content ON inbox_item TYPE string;
DEFINE FIELD source ON inbox_item TYPE string;
DEFINE FIELD created_at ON inbox_item TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON inbox_item TYPE option<datetime>;
DEFINE FIELD sync_version ON inbox_item TYPE int DEFAULT 0;

-- Routine (recurring templates)
DEFINE TABLE routine SCHEMAFULL;
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
DEFINE INDEX active_idx ON routine FIELDS active;

-- ============================================================================
-- GUIDANCE MODULE
-- ============================================================================

-- Epic (larger goal containing multiple quests)
DEFINE TABLE epic SCHEMAFULL;
DEFINE FIELD title ON epic TYPE string;
DEFINE FIELD description ON epic TYPE option<string>;
DEFINE FIELD status ON epic TYPE string DEFAULT 'active';
DEFINE FIELD initiative_id ON epic TYPE option<record<initiative>>;
DEFINE FIELD created_at ON epic TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON epic TYPE datetime DEFAULT time::now();
DEFINE FIELD completed_at ON epic TYPE option<datetime>;
DEFINE FIELD deleted_at ON epic TYPE option<datetime>;
DEFINE FIELD sync_version ON epic TYPE int DEFAULT 0;
DEFINE INDEX initiative_idx ON epic FIELDS initiative_id;

-- Quest (atomic task unit with energy cost)
DEFINE TABLE quest SCHEMAFULL;
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
DEFINE INDEX status_idx ON quest FIELDS status;
DEFINE INDEX epic_idx ON quest FIELDS epic_id;

-- Checkpoint (sub-steps within a quest)
DEFINE TABLE checkpoint SCHEMAFULL;
DEFINE FIELD quest_id ON checkpoint TYPE record<quest>;
DEFINE FIELD title ON checkpoint TYPE string;
DEFINE FIELD completed ON checkpoint TYPE bool DEFAULT false;
DEFINE FIELD sort_order ON checkpoint TYPE int DEFAULT 0;
DEFINE FIELD created_at ON checkpoint TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON checkpoint TYPE datetime DEFAULT time::now();
DEFINE INDEX quest_idx ON checkpoint FIELDS quest_id;

-- Energy Budget (daily capacity tracking)
DEFINE TABLE energy_budget SCHEMAFULL;
DEFINE FIELD date ON energy_budget TYPE datetime;
DEFINE FIELD budget ON energy_budget TYPE int DEFAULT 5;
DEFINE FIELD spent ON energy_budget TYPE int DEFAULT 0;
DEFINE INDEX date_idx ON energy_budget FIELDS date UNIQUE;

-- ============================================================================
-- KNOWLEDGE MODULE
-- ============================================================================

-- Folder (hierarchical organization for notes)
DEFINE TABLE folder SCHEMAFULL;
DEFINE FIELD name ON folder TYPE string;
DEFINE FIELD parent_id ON folder TYPE option<record<folder>>;
DEFINE FIELD created_at ON folder TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON folder TYPE datetime DEFAULT time::now();
DEFINE INDEX parent_idx ON folder FIELDS parent_id;

-- Tag (categorical labels for notes)
DEFINE TABLE tag SCHEMAFULL;
DEFINE FIELD name ON tag TYPE string;
DEFINE FIELD color ON tag TYPE option<string>;
DEFINE FIELD created_at ON tag TYPE datetime DEFAULT time::now();
DEFINE INDEX name_idx ON tag FIELDS name UNIQUE;

-- Note (PKM core entity)
DEFINE TABLE note SCHEMAFULL;
DEFINE FIELD title ON note TYPE string;
DEFINE FIELD content ON note TYPE option<string>;
DEFINE FIELD folder_id ON note TYPE option<record<folder>>;
DEFINE FIELD initiative_id ON note TYPE option<record<initiative>>;
DEFINE FIELD created_at ON note TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON note TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON note TYPE option<datetime>;
DEFINE FIELD sync_version ON note TYPE int DEFAULT 0;
DEFINE INDEX folder_idx ON note FIELDS folder_id;
DEFINE INDEX initiative_idx ON note FIELDS initiative_id;
DEFINE INDEX search_idx ON note FIELDS content SEARCH ANALYZER snowball(ENGLISH);

-- Note Link (bidirectional wiki-style links)
DEFINE TABLE note_link SCHEMAFULL;
DEFINE FIELD source_note_id ON note_link TYPE record<note>;
DEFINE FIELD target_note_id ON note_link TYPE record<note>;
DEFINE FIELD link_text ON note_link TYPE option<string>;
DEFINE FIELD created_at ON note_link TYPE datetime DEFAULT time::now();
DEFINE INDEX source_idx ON note_link FIELDS source_note_id;
DEFINE INDEX target_idx ON note_link FIELDS target_note_id;

-- Note-Tag Junction
DEFINE TABLE note_tag SCHEMAFULL;
DEFINE FIELD note_id ON note_tag TYPE record<note>;
DEFINE FIELD tag_id ON note_tag TYPE record<tag>;
DEFINE INDEX note_idx ON note_tag FIELDS note_id;
DEFINE INDEX tag_idx ON note_tag FIELDS tag_id;
DEFINE INDEX unique_idx ON note_tag FIELDS note_id, tag_id UNIQUE;

-- ============================================================================
-- TRACKING MODULE
-- ============================================================================

-- Location (physical or logical places)
DEFINE TABLE location SCHEMAFULL;
DEFINE FIELD name ON location TYPE string;
DEFINE FIELD description ON location TYPE option<string>;
DEFINE FIELD parent_id ON location TYPE option<record<location>>;
DEFINE FIELD created_at ON location TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON location TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON location TYPE option<datetime>;
DEFINE INDEX parent_idx ON location FIELDS parent_id;

-- Container (boxes, bins within locations)
DEFINE TABLE container SCHEMAFULL;
DEFINE FIELD name ON container TYPE string;
DEFINE FIELD description ON container TYPE option<string>;
DEFINE FIELD location_id ON container TYPE option<record<location>>;
DEFINE FIELD created_at ON container TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON container TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON container TYPE option<datetime>;
DEFINE INDEX location_idx ON container FIELDS location_id;

-- Item (tracked possessions)
DEFINE TABLE item SCHEMAFULL;
DEFINE FIELD name ON item TYPE string;
DEFINE FIELD description ON item TYPE option<string>;
DEFINE FIELD quantity ON item TYPE int DEFAULT 1;
DEFINE FIELD location_id ON item TYPE option<record<location>>;
DEFINE FIELD container_id ON item TYPE option<record<container>>;
DEFINE FIELD initiative_id ON item TYPE option<record<initiative>>;
DEFINE FIELD created_at ON item TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON item TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON item TYPE option<datetime>;
DEFINE FIELD sync_version ON item TYPE int DEFAULT 0;
DEFINE INDEX location_idx ON item FIELDS location_id;
DEFINE INDEX container_idx ON item FIELDS container_id;
DEFINE INDEX initiative_idx ON item FIELDS initiative_id;

-- ============================================================================
-- END OF SCHEMA
-- ============================================================================
