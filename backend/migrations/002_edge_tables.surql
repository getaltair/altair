-- Migration: 002_edge_tables.surql
-- Description: Define graph edge tables for relationships between entities
-- Created: 2025-12-06

USE NS altair DB main;

-- =============================================================================
-- EDGE TABLE: contains
-- Description: Hierarchical containment relationships
-- Relationships:
--   - Campaign → Quest (campaign contains quests)
--   - Folder → Note (folder contains notes)
--   - Folder → Folder (folder contains subfolders, nesting)
--   - Location → Location (location contains sublocations, hierarchy)
-- =============================================================================
DEFINE TABLE contains SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD in ON TABLE contains TYPE record;
DEFINE FIELD out ON TABLE contains TYPE record;
DEFINE FIELD created_at ON TABLE contains TYPE datetime DEFAULT time::now();

-- =============================================================================
-- EDGE TABLE: references
-- Description: Quest references to notes (documentation, research, context)
-- Relationships:
--   - Quest → Note (quest references note for context/documentation)
-- =============================================================================
DEFINE TABLE references SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD in ON TABLE references TYPE record<quest>;
DEFINE FIELD out ON TABLE references TYPE record<note>;
DEFINE FIELD created_at ON TABLE references TYPE datetime DEFAULT time::now();

-- =============================================================================
-- EDGE TABLE: requires
-- Description: Quest item requirements (inventory dependencies)
-- Relationships:
--   - Quest → Item (quest requires specific item with quantity)
-- =============================================================================
DEFINE TABLE requires SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD in ON TABLE requires TYPE record<quest>;
DEFINE FIELD out ON TABLE requires TYPE record<item>;
DEFINE FIELD quantity ON TABLE requires TYPE int DEFAULT 1;
DEFINE FIELD created_at ON TABLE requires TYPE datetime DEFAULT time::now();

-- Quantity must be positive
DEFINE FIELD quantity ON TABLE requires ASSERT $value > 0;

-- =============================================================================
-- EDGE TABLE: links_to
-- Description: Bidirectional wiki-links between notes (knowledge graph)
-- Relationships:
--   - Note ↔ Note (bidirectional linking for knowledge graph traversal)
-- =============================================================================
DEFINE TABLE links_to SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD in ON TABLE links_to TYPE record<note>;
DEFINE FIELD out ON TABLE links_to TYPE record<note>;
DEFINE FIELD created_at ON TABLE links_to TYPE datetime DEFAULT time::now();

-- =============================================================================
-- EDGE TABLE: stored_in
-- Description: Item storage location
-- Relationships:
--   - Item → Location (item stored in specific location)
-- =============================================================================
DEFINE TABLE stored_in SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD in ON TABLE stored_in TYPE record<item>;
DEFINE FIELD out ON TABLE stored_in TYPE record<location>;
DEFINE FIELD created_at ON TABLE stored_in TYPE datetime DEFAULT time::now();

-- =============================================================================
-- EDGE TABLE: documents
-- Description: Note documentation of items (cross-domain linking)
-- Relationships:
--   - Note → Item (note documents/describes item)
-- =============================================================================
DEFINE TABLE documents SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD in ON TABLE documents TYPE record<note>;
DEFINE FIELD out ON TABLE documents TYPE record<item>;
DEFINE FIELD created_at ON TABLE documents TYPE datetime DEFAULT time::now();

-- =============================================================================
-- EDGE TABLE: reserved_for
-- Description: Reservation to quest relationship
-- Relationships:
--   - Reservation → Quest (reservation is for a specific quest)
-- =============================================================================
DEFINE TABLE reserved_for SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD in ON TABLE reserved_for TYPE record<reservation>;
DEFINE FIELD out ON TABLE reserved_for TYPE record<quest>;
DEFINE FIELD created_at ON TABLE reserved_for TYPE datetime DEFAULT time::now();

-- =============================================================================
-- EDGE TABLE: reserves
-- Description: Reservation to item relationship
-- Relationships:
--   - Reservation → Item (reservation reserves specific item)
-- =============================================================================
DEFINE TABLE reserves SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD in ON TABLE reserves TYPE record<reservation>;
DEFINE FIELD out ON TABLE reserves TYPE record<item>;
DEFINE FIELD created_at ON TABLE reserves TYPE datetime DEFAULT time::now();

-- =============================================================================
-- EDGE TABLE: blocks
-- Description: Quest dependency blocking (one quest blocks another)
-- Relationships:
--   - Quest → Quest (blocking quest prevents blocked quest from starting)
-- =============================================================================
DEFINE TABLE blocks SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD in ON TABLE blocks TYPE record<quest>;
DEFINE FIELD out ON TABLE blocks TYPE record<quest>;
DEFINE FIELD created_at ON TABLE blocks TYPE datetime DEFAULT time::now();

-- =============================================================================
-- EDGE TABLE: has_attachment
-- Description: Polymorphic attachment relationships (any entity can have attachments)
-- Relationships:
--   - Quest → Attachment
--   - Note → Attachment
--   - Capture → Attachment
--   - Item → Attachment
--   - (any entity) → Attachment
-- =============================================================================
DEFINE TABLE has_attachment SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD in ON TABLE has_attachment TYPE record;
DEFINE FIELD out ON TABLE has_attachment TYPE record<attachment>;
DEFINE FIELD created_at ON TABLE has_attachment TYPE datetime DEFAULT time::now();

-- =============================================================================
-- EDGE TABLE: tagged
-- Description: Polymorphic tagging relationships (any entity can be tagged)
-- Relationships:
--   - Quest → Tag
--   - Note → Tag
--   - Item → Tag
--   - Capture → Tag
--   - (any entity) → Tag
-- =============================================================================
DEFINE TABLE tagged SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD in ON TABLE tagged TYPE record;
DEFINE FIELD out ON TABLE tagged TYPE record<tag>;
DEFINE FIELD created_at ON TABLE tagged TYPE datetime DEFAULT time::now();

-- =============================================================================
-- EDGE TABLE: has_session
-- Description: Quest to focus session relationship
-- Relationships:
--   - Quest → FocusSession (quest has associated focus sessions for time tracking)
-- =============================================================================
DEFINE TABLE has_session SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD in ON TABLE has_session TYPE record<quest>;
DEFINE FIELD out ON TABLE has_session TYPE record<focus_session>;
DEFINE FIELD created_at ON TABLE has_session TYPE datetime DEFAULT time::now();

-- =============================================================================
-- EDGE TABLE: has_maintenance
-- Description: Item to maintenance schedule relationship
-- Relationships:
--   - Item → MaintenanceSchedule (item has maintenance schedules)
-- =============================================================================
DEFINE TABLE has_maintenance SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD in ON TABLE has_maintenance TYPE record<item>;
DEFINE FIELD out ON TABLE has_maintenance TYPE record<maintenance_schedule>;
DEFINE FIELD created_at ON TABLE has_maintenance TYPE datetime DEFAULT time::now();
