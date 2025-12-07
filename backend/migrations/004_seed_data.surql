-- Migration: 004_seed_data.surql
-- Description: Optional seed data for development and testing
-- Created: 2025-12-07
--
-- NOTE: This migration is optional and intended for development only.
-- In production, the migration runner should skip this file or the
-- application should be configured to exclude seed data.

USE NS altair DB main;

-- =============================================================================
-- Sample User with Preferences
-- =============================================================================

CREATE user:sample SET
    email = "sample@altair.dev",
    display_name = "Sample User",
    avatar_url = NONE,
    role = "owner",
    preferences = {
        theme: "dark",
        energy_filter_default: "medium",
        gamification_enabled: true,
        weekly_harvest_day: 5,          -- Friday
        weekly_harvest_time: "17:00",   -- 5 PM
        focus_session_duration: 25,     -- 25 minutes (Pomodoro)
        pomodoro_break_duration: 5      -- 5 minute breaks
    },
    device_id = "dev-seed-001";

-- =============================================================================
-- Sample User Progress (Gamification)
-- =============================================================================

CREATE user_progress:sample SET
    xp_total = 1250,
    level = 5,
    title = "Novice Adventurer",
    owner = user:sample,
    device_id = "dev-seed-001";

-- =============================================================================
-- Sample Campaign with Quests
-- =============================================================================

-- Create a sample campaign
CREATE campaign:sample_campaign SET
    title = "Learn SurrealDB",
    description = "A campaign to master SurrealDB for the Altair project",
    status = "active",
    color = "#6366f1",  -- Indigo
    owner = user:sample,
    device_id = "dev-seed-001";

-- Create 3 quests with different columns and energy costs
CREATE quest:quest_setup SET
    title = "Set up SurrealDB development environment",
    description = "Install SurrealDB locally and configure for development",
    column = "harvested",           -- Already completed
    energy_cost = "small",
    estimated_minutes = 30,
    actual_minutes = 25,
    xp_value = 50,
    completed_at = time::now() - 2d,
    status = "active",
    owner = user:sample,
    device_id = "dev-seed-001";

CREATE quest:quest_schema SET
    title = "Design database schema",
    description = "Create SCHEMAFULL tables with proper constraints",
    column = "in_progress",
    energy_cost = "medium",
    estimated_minutes = 120,
    xp_value = 100,
    status = "active",
    owner = user:sample,
    device_id = "dev-seed-001";

CREATE quest:quest_queries SET
    title = "Learn graph queries",
    description = "Master SurrealDB graph traversal syntax",
    column = "next_up",
    energy_cost = "large",
    estimated_minutes = 180,
    xp_value = 150,
    status = "active",
    owner = user:sample,
    device_id = "dev-seed-001";

-- Link quests to campaign via contains edge
RELATE campaign:sample_campaign->contains->quest:quest_setup;
RELATE campaign:sample_campaign->contains->quest:quest_schema;
RELATE campaign:sample_campaign->contains->quest:quest_queries;

-- =============================================================================
-- Sample Folder and Notes with Wiki-Links
-- =============================================================================

-- Create a folder
CREATE folder:sample_folder SET
    name = "SurrealDB Notes",
    color = "#10b981",  -- Emerald
    status = "active",
    owner = user:sample,
    device_id = "dev-seed-001";

-- Create 3 notes with wiki-link relationships
CREATE note:note_intro SET
    title = "SurrealDB Introduction",
    content = "# SurrealDB Introduction\n\nSurrealDB is a multi-model database that combines the best features of traditional and modern databases.\n\n## Key Features\n- Graph queries\n- SCHEMAFULL tables\n- CHANGEFEED for sync\n\nSee also: [[SurrealDB Schema Design]], [[Graph Queries]]",
    is_daily = false,
    version = 1,
    status = "active",
    owner = user:sample,
    device_id = "dev-seed-001";

CREATE note:note_schema SET
    title = "SurrealDB Schema Design",
    content = "# Schema Design in SurrealDB\n\nUse SCHEMAFULL tables for strict type enforcement.\n\n```surql\nDEFINE TABLE user SCHEMAFULL;\nDEFINE FIELD email ON user TYPE string;\n```\n\nRelated: [[SurrealDB Introduction]], [[Graph Queries]]",
    is_daily = false,
    version = 1,
    status = "active",
    owner = user:sample,
    device_id = "dev-seed-001";

CREATE note:note_graph SET
    title = "Graph Queries",
    content = "# Graph Queries in SurrealDB\n\nTraverse relationships with arrow syntax:\n\n```surql\nSELECT ->contains->quest FROM campaign:xyz\n```\n\nBacklinks: [[SurrealDB Introduction]], [[SurrealDB Schema Design]]",
    is_daily = false,
    version = 1,
    status = "active",
    owner = user:sample,
    device_id = "dev-seed-001";

-- Link notes to folder
RELATE folder:sample_folder->contains->note:note_intro;
RELATE folder:sample_folder->contains->note:note_schema;
RELATE folder:sample_folder->contains->note:note_graph;

-- Create bidirectional wiki-links between notes
-- Note 1 links to Notes 2 and 3
RELATE note:note_intro->links_to->note:note_schema;
RELATE note:note_intro->links_to->note:note_graph;

-- Note 2 links back to Notes 1 and 3
RELATE note:note_schema->links_to->note:note_intro;
RELATE note:note_schema->links_to->note:note_graph;

-- Note 3 links back to Notes 1 and 2
RELATE note:note_graph->links_to->note:note_intro;
RELATE note:note_graph->links_to->note:note_schema;

-- Link quest to reference note
RELATE quest:quest_schema->references->note:note_schema;
RELATE quest:quest_queries->references->note:note_graph;

-- =============================================================================
-- Sample Tags
-- =============================================================================

CREATE tag:learning SET
    name = "learning",
    namespace = "default",
    color = "#f59e0b",  -- Amber
    owner = user:sample,
    device_id = "dev-seed-001";

CREATE tag:database SET
    name = "database",
    namespace = "tech",
    color = "#3b82f6",  -- Blue
    owner = user:sample,
    device_id = "dev-seed-001";

-- Tag some entities
RELATE note:note_intro->tagged->tag:learning;
RELATE note:note_intro->tagged->tag:database;
RELATE quest:quest_setup->tagged->tag:learning;
RELATE campaign:sample_campaign->tagged->tag:learning;
