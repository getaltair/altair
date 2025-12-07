-- Migration 001: Initial Schema - Core Entity Tables
-- Creates namespace, database, and all 15+ entity tables with SCHEMAFULL and CHANGEFEED

-- Define namespace and database
DEFINE NAMESPACE altair;
DEFINE DATABASE main;
USE NS altair DB main;

-- ============================================================================
-- User Table
-- ============================================================================

DEFINE TABLE user SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE user TYPE record<user>;
DEFINE FIELD email ON TABLE user TYPE string;
DEFINE FIELD display_name ON TABLE user TYPE string;
DEFINE FIELD avatar_url ON TABLE user TYPE option<string>;
DEFINE FIELD role ON TABLE user TYPE string ASSERT $value IN ['owner', 'viewer'];
DEFINE FIELD preferences ON TABLE user TYPE object;
DEFINE FIELD preferences.theme ON TABLE user TYPE string;
DEFINE FIELD preferences.energy_filter_default ON TABLE user TYPE option<string>;
DEFINE FIELD preferences.gamification_enabled ON TABLE user TYPE bool;
DEFINE FIELD preferences.weekly_harvest_day ON TABLE user TYPE option<int>;
DEFINE FIELD preferences.weekly_harvest_time ON TABLE user TYPE option<string>;
DEFINE FIELD preferences.focus_session_duration ON TABLE user TYPE option<int>;
DEFINE FIELD preferences.pomodoro_break_duration ON TABLE user TYPE option<int>;
DEFINE FIELD device_id ON TABLE user TYPE string;
DEFINE FIELD created_at ON TABLE user TYPE datetime VALUE time::now();
DEFINE FIELD updated_at ON TABLE user TYPE datetime VALUE time::now();

-- ============================================================================
-- Quest Domain Tables
-- ============================================================================

-- Campaign Table
DEFINE TABLE campaign SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE campaign TYPE record<campaign>;
DEFINE FIELD title ON TABLE campaign TYPE string;
DEFINE FIELD description ON TABLE campaign TYPE option<string>;
DEFINE FIELD status ON TABLE campaign TYPE string ASSERT $value IN ['active', 'archived'];
DEFINE FIELD color ON TABLE campaign TYPE option<string>;
DEFINE FIELD owner ON TABLE campaign TYPE record<user>;
DEFINE FIELD device_id ON TABLE campaign TYPE string;
DEFINE FIELD created_at ON TABLE campaign TYPE datetime VALUE time::now();
DEFINE FIELD updated_at ON TABLE campaign TYPE datetime VALUE time::now();

-- Quest Table
DEFINE TABLE quest SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE quest TYPE record<quest>;
DEFINE FIELD title ON TABLE quest TYPE string;
DEFINE FIELD description ON TABLE quest TYPE option<string>;
DEFINE FIELD column ON TABLE quest TYPE string ASSERT $value IN ['idea_greenhouse', 'quest_log', 'this_cycle', 'next_up', 'in_progress', 'harvested', 'archived'];
DEFINE FIELD energy_cost ON TABLE quest TYPE string ASSERT $value IN ['tiny', 'small', 'medium', 'large', 'huge'];
DEFINE FIELD estimated_minutes ON TABLE quest TYPE option<int>;
DEFINE FIELD actual_minutes ON TABLE quest TYPE option<int>;
DEFINE FIELD xp_value ON TABLE quest TYPE option<int>;
DEFINE FIELD due_date ON TABLE quest TYPE option<datetime>;
DEFINE FIELD completed_at ON TABLE quest TYPE option<datetime>;
DEFINE FIELD status ON TABLE quest TYPE string ASSERT $value IN ['active', 'archived'];
DEFINE FIELD owner ON TABLE quest TYPE record<user>;
DEFINE FIELD device_id ON TABLE quest TYPE string;
DEFINE FIELD created_at ON TABLE quest TYPE datetime VALUE time::now();
DEFINE FIELD updated_at ON TABLE quest TYPE datetime VALUE time::now();

-- Focus Session Table
DEFINE TABLE focus_session SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE focus_session TYPE record<focus_session>;
DEFINE FIELD started_at ON TABLE focus_session TYPE datetime;
DEFINE FIELD planned_duration ON TABLE focus_session TYPE int;
DEFINE FIELD actual_duration ON TABLE focus_session TYPE option<int>;
DEFINE FIELD completed_steps ON TABLE focus_session TYPE option<int>;
DEFINE FIELD status ON TABLE focus_session TYPE string ASSERT $value IN ['active', 'completed', 'abandoned'];
DEFINE FIELD notes ON TABLE focus_session TYPE option<string>;
DEFINE FIELD owner ON TABLE focus_session TYPE record<user>;
DEFINE FIELD device_id ON TABLE focus_session TYPE string;
DEFINE FIELD created_at ON TABLE focus_session TYPE datetime VALUE time::now();
DEFINE FIELD updated_at ON TABLE focus_session TYPE datetime VALUE time::now();

-- Energy Check-in Table
DEFINE TABLE energy_checkin SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE energy_checkin TYPE record<energy_checkin>;
DEFINE FIELD date ON TABLE energy_checkin TYPE datetime;
DEFINE FIELD energy_level ON TABLE energy_checkin TYPE int ASSERT $value IN [1, 2, 3, 4, 5];
DEFINE FIELD notes ON TABLE energy_checkin TYPE option<string>;
DEFINE FIELD owner ON TABLE energy_checkin TYPE record<user>;
DEFINE FIELD device_id ON TABLE energy_checkin TYPE string;
DEFINE FIELD created_at ON TABLE energy_checkin TYPE datetime VALUE time::now();
DEFINE FIELD updated_at ON TABLE energy_checkin TYPE datetime VALUE time::now();

-- ============================================================================
-- Knowledge Domain Tables
-- ============================================================================

-- Note Table
DEFINE TABLE note SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE note TYPE record<note>;
DEFINE FIELD title ON TABLE note TYPE string;
DEFINE FIELD content ON TABLE note TYPE string;
DEFINE FIELD embedding ON TABLE note TYPE option<array<float, 384>>;
DEFINE FIELD is_daily ON TABLE note TYPE bool;
DEFINE FIELD version ON TABLE note TYPE int;
DEFINE FIELD status ON TABLE note TYPE string ASSERT $value IN ['active', 'archived'];
DEFINE FIELD owner ON TABLE note TYPE record<user>;
DEFINE FIELD device_id ON TABLE note TYPE string;
DEFINE FIELD created_at ON TABLE note TYPE datetime VALUE time::now();
DEFINE FIELD updated_at ON TABLE note TYPE datetime VALUE time::now();

-- Folder Table
DEFINE TABLE folder SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE folder TYPE record<folder>;
DEFINE FIELD name ON TABLE folder TYPE string;
DEFINE FIELD color ON TABLE folder TYPE option<string>;
DEFINE FIELD status ON TABLE folder TYPE string ASSERT $value IN ['active', 'archived'];
DEFINE FIELD owner ON TABLE folder TYPE record<user>;
DEFINE FIELD device_id ON TABLE folder TYPE string;
DEFINE FIELD created_at ON TABLE folder TYPE datetime VALUE time::now();
DEFINE FIELD updated_at ON TABLE folder TYPE datetime VALUE time::now();

-- Daily Note Table
DEFINE TABLE daily_note SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE daily_note TYPE record<daily_note>;
DEFINE FIELD date ON TABLE daily_note TYPE datetime;
DEFINE FIELD note_id ON TABLE daily_note TYPE record<note>;
DEFINE FIELD auto_created ON TABLE daily_note TYPE bool;
DEFINE FIELD owner ON TABLE daily_note TYPE record<user>;
DEFINE FIELD device_id ON TABLE daily_note TYPE string;
DEFINE FIELD created_at ON TABLE daily_note TYPE datetime VALUE time::now();
DEFINE FIELD updated_at ON TABLE daily_note TYPE datetime VALUE time::now();

-- ============================================================================
-- Inventory Domain Tables
-- ============================================================================

-- Item Table
DEFINE TABLE item SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE item TYPE record<item>;
DEFINE FIELD name ON TABLE item TYPE string;
DEFINE FIELD description ON TABLE item TYPE option<string>;
DEFINE FIELD quantity ON TABLE item TYPE int;
DEFINE FIELD status ON TABLE item TYPE string ASSERT $value IN ['available', 'reserved', 'in_use', 'depleted', 'archived'];
DEFINE FIELD category ON TABLE item TYPE option<string>;
DEFINE FIELD custom_fields ON TABLE item TYPE option<object>;
DEFINE FIELD owner ON TABLE item TYPE record<user>;
DEFINE FIELD device_id ON TABLE item TYPE string;
DEFINE FIELD created_at ON TABLE item TYPE datetime VALUE time::now();
DEFINE FIELD updated_at ON TABLE item TYPE datetime VALUE time::now();

-- Location Table
DEFINE TABLE location SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE location TYPE record<location>;
DEFINE FIELD name ON TABLE location TYPE string;
DEFINE FIELD description ON TABLE location TYPE option<string>;
DEFINE FIELD geo ON TABLE location TYPE option<geometry<point>>;
DEFINE FIELD status ON TABLE location TYPE string ASSERT $value IN ['active', 'archived'];
DEFINE FIELD owner ON TABLE location TYPE record<user>;
DEFINE FIELD device_id ON TABLE location TYPE string;
DEFINE FIELD created_at ON TABLE location TYPE datetime VALUE time::now();
DEFINE FIELD updated_at ON TABLE location TYPE datetime VALUE time::now();

-- Reservation Table
DEFINE TABLE reservation SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE reservation TYPE record<reservation>;
DEFINE FIELD quantity ON TABLE reservation TYPE int;
DEFINE FIELD status ON TABLE reservation TYPE string ASSERT $value IN ['pending', 'in_use', 'released'];
DEFINE FIELD reserved_at ON TABLE reservation TYPE datetime VALUE time::now();
DEFINE FIELD released_at ON TABLE reservation TYPE option<datetime>;
DEFINE FIELD owner ON TABLE reservation TYPE record<user>;
DEFINE FIELD device_id ON TABLE reservation TYPE string;
DEFINE FIELD created_at ON TABLE reservation TYPE datetime VALUE time::now();
DEFINE FIELD updated_at ON TABLE reservation TYPE datetime VALUE time::now();

-- Maintenance Schedule Table
DEFINE TABLE maintenance_schedule SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE maintenance_schedule TYPE record<maintenance_schedule>;
DEFINE FIELD task_name ON TABLE maintenance_schedule TYPE string;
DEFINE FIELD interval ON TABLE maintenance_schedule TYPE duration;
DEFINE FIELD last_performed ON TABLE maintenance_schedule TYPE option<datetime>;
DEFINE FIELD next_due ON TABLE maintenance_schedule TYPE datetime;
DEFINE FIELD notes ON TABLE maintenance_schedule TYPE option<string>;
DEFINE FIELD notify_days_before ON TABLE maintenance_schedule TYPE option<int>;
DEFINE FIELD owner ON TABLE maintenance_schedule TYPE record<user>;
DEFINE FIELD device_id ON TABLE maintenance_schedule TYPE string;
DEFINE FIELD created_at ON TABLE maintenance_schedule TYPE datetime VALUE time::now();
DEFINE FIELD updated_at ON TABLE maintenance_schedule TYPE datetime VALUE time::now();

-- ============================================================================
-- Capture Table (Multi-Modal Input)
-- ============================================================================

DEFINE TABLE capture SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE capture TYPE record<capture>;
DEFINE FIELD text_content ON TABLE capture TYPE option<string>;
DEFINE FIELD capture_type ON TABLE capture TYPE string ASSERT $value IN ['text', 'voice', 'photo', 'video', 'mixed'];
DEFINE FIELD source ON TABLE capture TYPE string ASSERT $value IN ['desktop', 'mobile', 'widget', 'voice_assistant'];
DEFINE FIELD status ON TABLE capture TYPE string ASSERT $value IN ['pending', 'processed', 'discarded'];
DEFINE FIELD processed_to ON TABLE capture TYPE option<record>;
DEFINE FIELD ai_suggestion ON TABLE capture TYPE option<string>;
DEFINE FIELD ai_confidence ON TABLE capture TYPE option<float>;
DEFINE FIELD location ON TABLE capture TYPE option<geometry<point>>;
DEFINE FIELD owner ON TABLE capture TYPE record<user>;
DEFINE FIELD device_id ON TABLE capture TYPE string;
DEFINE FIELD captured_at ON TABLE capture TYPE datetime VALUE time::now();
DEFINE FIELD created_at ON TABLE capture TYPE datetime VALUE time::now();
DEFINE FIELD updated_at ON TABLE capture TYPE datetime VALUE time::now();

-- ============================================================================
-- Gamification Tables
-- ============================================================================

-- User Progress Table
DEFINE TABLE user_progress SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE user_progress TYPE record<user_progress>;
DEFINE FIELD xp_total ON TABLE user_progress TYPE int;
DEFINE FIELD level ON TABLE user_progress TYPE int;
DEFINE FIELD title ON TABLE user_progress TYPE string;
DEFINE FIELD owner ON TABLE user_progress TYPE record<user>;
DEFINE FIELD device_id ON TABLE user_progress TYPE string;
DEFINE FIELD created_at ON TABLE user_progress TYPE datetime VALUE time::now();
DEFINE FIELD updated_at ON TABLE user_progress TYPE datetime VALUE time::now();

-- Achievement Table
DEFINE TABLE achievement SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE achievement TYPE record<achievement>;
DEFINE FIELD name ON TABLE achievement TYPE string;
DEFINE FIELD description ON TABLE achievement TYPE string;
DEFINE FIELD icon ON TABLE achievement TYPE string;
DEFINE FIELD unlocked_at ON TABLE achievement TYPE option<datetime>;
DEFINE FIELD owner ON TABLE achievement TYPE record<user>;
DEFINE FIELD device_id ON TABLE achievement TYPE string;
DEFINE FIELD created_at ON TABLE achievement TYPE datetime VALUE time::now();
DEFINE FIELD updated_at ON TABLE achievement TYPE datetime VALUE time::now();

-- Streak Table
DEFINE TABLE streak SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE streak TYPE record<streak>;
DEFINE FIELD type ON TABLE streak TYPE string ASSERT $value IN ['daily_checkin', 'quest_completion', 'focus_session'];
DEFINE FIELD current_count ON TABLE streak TYPE int;
DEFINE FIELD longest_count ON TABLE streak TYPE int;
DEFINE FIELD last_activity ON TABLE streak TYPE datetime;
DEFINE FIELD started_at ON TABLE streak TYPE datetime;
DEFINE FIELD owner ON TABLE streak TYPE record<user>;
DEFINE FIELD device_id ON TABLE streak TYPE string;
DEFINE FIELD created_at ON TABLE streak TYPE datetime VALUE time::now();
DEFINE FIELD updated_at ON TABLE streak TYPE datetime VALUE time::now();

-- ============================================================================
-- Shared Tables
-- ============================================================================

-- Attachment Table
DEFINE TABLE attachment SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE attachment TYPE record<attachment>;
DEFINE FIELD filename ON TABLE attachment TYPE string;
DEFINE FIELD mime_type ON TABLE attachment TYPE string;
DEFINE FIELD size_bytes ON TABLE attachment TYPE int;
DEFINE FIELD storage_key ON TABLE attachment TYPE string;
DEFINE FIELD checksum ON TABLE attachment TYPE string;
DEFINE FIELD media_type ON TABLE attachment TYPE string ASSERT $value IN ['photo', 'audio', 'video', 'document'];
DEFINE FIELD duration ON TABLE attachment TYPE option<duration>;
DEFINE FIELD thumbnail_key ON TABLE attachment TYPE option<string>;
DEFINE FIELD transcription ON TABLE attachment TYPE option<string>;
DEFINE FIELD owner ON TABLE attachment TYPE record<user>;
DEFINE FIELD device_id ON TABLE attachment TYPE string;
DEFINE FIELD created_at ON TABLE attachment TYPE datetime VALUE time::now();
DEFINE FIELD updated_at ON TABLE attachment TYPE datetime VALUE time::now();

-- Tag Table
DEFINE TABLE tag SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE tag TYPE record<tag>;
DEFINE FIELD name ON TABLE tag TYPE string;
DEFINE FIELD namespace ON TABLE tag TYPE option<string>;
DEFINE FIELD color ON TABLE tag TYPE option<string>;
DEFINE FIELD owner ON TABLE tag TYPE record<user>;
DEFINE FIELD device_id ON TABLE tag TYPE string;
DEFINE FIELD created_at ON TABLE tag TYPE datetime VALUE time::now();
DEFINE FIELD updated_at ON TABLE tag TYPE datetime VALUE time::now();
