-- Migration 003: Database Indexes for Performance Optimization
-- Creates indexes on commonly queried fields across all entity tables
-- Performance targets: owner/status queries < 100ms on 10k+ records

USE NS altair DB main;

-- ============================================================================
-- OWNER INDEXES (Record-Level Authorization)
-- ============================================================================
-- These indexes optimize the primary access pattern: filtering by owner
-- All queries filter by owner for record-level security

DEFINE INDEX idx_user_owner ON TABLE user FIELDS owner;
DEFINE INDEX idx_campaign_owner ON TABLE campaign FIELDS owner;
DEFINE INDEX idx_quest_owner ON TABLE quest FIELDS owner;
DEFINE INDEX idx_focus_session_owner ON TABLE focus_session FIELDS owner;
DEFINE INDEX idx_energy_checkin_owner ON TABLE energy_checkin FIELDS owner;
DEFINE INDEX idx_note_owner ON TABLE note FIELDS owner;
DEFINE INDEX idx_folder_owner ON TABLE folder FIELDS owner;
DEFINE INDEX idx_daily_note_owner ON TABLE daily_note FIELDS owner;
DEFINE INDEX idx_item_owner ON TABLE item FIELDS owner;
DEFINE INDEX idx_location_owner ON TABLE location FIELDS owner;
DEFINE INDEX idx_reservation_owner ON TABLE reservation FIELDS owner;
DEFINE INDEX idx_maintenance_schedule_owner ON TABLE maintenance_schedule FIELDS owner;
DEFINE INDEX idx_capture_owner ON TABLE capture FIELDS owner;
DEFINE INDEX idx_user_progress_owner ON TABLE user_progress FIELDS owner;
DEFINE INDEX idx_achievement_owner ON TABLE achievement FIELDS owner;
DEFINE INDEX idx_streak_owner ON TABLE streak FIELDS owner;
DEFINE INDEX idx_attachment_owner ON TABLE attachment FIELDS owner;
DEFINE INDEX idx_tag_owner ON TABLE tag FIELDS owner;

-- ============================================================================
-- STATUS INDEXES (Filtering Active vs Archived)
-- ============================================================================
-- Optimize filtering by status (active/archived) - common pattern in UI

DEFINE INDEX idx_campaign_status ON TABLE campaign FIELDS status;
DEFINE INDEX idx_quest_status ON TABLE quest FIELDS status;
DEFINE INDEX idx_note_status ON TABLE note FIELDS status;
DEFINE INDEX idx_folder_status ON TABLE folder FIELDS status;
DEFINE INDEX idx_item_status ON TABLE item FIELDS status;
DEFINE INDEX idx_location_status ON TABLE location FIELDS status;
DEFINE INDEX idx_reservation_status ON TABLE reservation FIELDS status;
DEFINE INDEX idx_capture_status ON TABLE capture FIELDS status;

-- ============================================================================
-- QUEST-SPECIFIC INDEXES
-- ============================================================================
-- Optimize Quest-Based Agile workflows

-- Filter by column (idea_greenhouse, quest_log, this_cycle, etc.)
DEFINE INDEX idx_quest_column ON TABLE quest FIELDS column;

-- Full-text search on quest titles
DEFINE INDEX idx_quest_title_search ON TABLE quest FIELDS title SEARCH ANALYZER ascii BM25;

-- Filter quests by energy cost for filtering UI
DEFINE INDEX idx_quest_energy_cost ON TABLE quest FIELDS energy_cost;

-- Composite index for common query: owner + status + column
DEFINE INDEX idx_quest_owner_status_column ON TABLE quest FIELDS owner, status, column;

-- ============================================================================
-- NOTE-SPECIFIC INDEXES
-- ============================================================================
-- Optimize PKM and semantic search workflows

-- Full-text search on note content (markdown)
DEFINE INDEX idx_note_content_search ON TABLE note FIELDS content SEARCH ANALYZER ascii BM25;

-- Filter daily notes
DEFINE INDEX idx_note_is_daily ON TABLE note FIELDS is_daily;

-- Composite index for common query: owner + status
DEFINE INDEX idx_note_owner_status ON TABLE note FIELDS owner, status;

-- VECTOR INDEX PLACEHOLDER (Reserved for core-012: Semantic Search)
-- The following index will be enabled when semantic search is implemented:
-- DEFINE INDEX idx_note_embedding ON TABLE note FIELDS embedding MTREE DIMENSION 384;
--
-- Notes:
-- - MTREE index type optimized for vector similarity search
-- - DIMENSION 384 matches all-MiniLM-L6-v2 embedding model
-- - Will enable efficient kNN queries: SELECT * FROM note WHERE embedding <|2|> $query_vector LIMIT 10
-- - Currently commented to avoid overhead until semantic search implementation

-- ============================================================================
-- ITEM-SPECIFIC INDEXES
-- ============================================================================
-- Optimize inventory management queries

-- Filter items by category
DEFINE INDEX idx_item_category ON TABLE item FIELDS category;

-- Composite index: owner + status for inventory views
DEFINE INDEX idx_item_owner_status ON TABLE item FIELDS owner, status;

-- ============================================================================
-- USER-SPECIFIC INDEXES
-- ============================================================================
-- Authentication and user lookup

-- Unique constraint on email (prevents duplicate accounts)
DEFINE INDEX idx_user_email_unique ON TABLE user FIELDS email UNIQUE;

-- Device ID for multi-device sync
DEFINE INDEX idx_user_device_id ON TABLE user FIELDS device_id;

-- ============================================================================
-- DAILY NOTE INDEX
-- ============================================================================
-- Optimize daily note retrieval by date

-- Unique composite index: owner + date (one daily note per user per day)
DEFINE INDEX idx_daily_note_owner_date ON TABLE daily_note FIELDS owner, date UNIQUE;

-- ============================================================================
-- CAPTURE-SPECIFIC INDEXES
-- ============================================================================
-- Optimize Quick Capture triage workflow

-- Composite index: owner + status for pending captures view
DEFINE INDEX idx_capture_owner_status ON TABLE capture FIELDS owner, status;

-- Filter by capture type (text, voice, photo, video, mixed)
DEFINE INDEX idx_capture_type ON TABLE capture FIELDS capture_type;

-- ============================================================================
-- GAMIFICATION INDEXES
-- ============================================================================
-- Optimize XP, achievements, and streak queries

-- Composite index: owner + type for streak queries
DEFINE INDEX idx_streak_owner_type ON TABLE streak FIELDS owner, type;

-- ============================================================================
-- TEMPORAL INDEXES
-- ============================================================================
-- Optimize time-based queries

-- Created/updated timestamps for sorting and filtering
DEFINE INDEX idx_quest_created_at ON TABLE quest FIELDS created_at;
DEFINE INDEX idx_quest_updated_at ON TABLE quest FIELDS updated_at;
DEFINE INDEX idx_note_created_at ON TABLE note FIELDS created_at;
DEFINE INDEX idx_note_updated_at ON TABLE note FIELDS updated_at;
DEFINE INDEX idx_capture_captured_at ON TABLE capture FIELDS captured_at;

-- Quest due dates for filtering upcoming quests
DEFINE INDEX idx_quest_due_date ON TABLE quest FIELDS due_date;

-- ============================================================================
-- TAG INDEXES
-- ============================================================================
-- Optimize tag lookups

-- Composite index: owner + namespace + name for tag uniqueness
DEFINE INDEX idx_tag_owner_namespace_name ON TABLE tag FIELDS owner, namespace, name;
