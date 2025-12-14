-- Migration 006: Storage Quota Table
-- Creates per-user storage quota tracking for attachment management

USE NS altair DB main;

-- ============================================================================
-- Storage Quota Table
-- ============================================================================

DEFINE TABLE storage_quota SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE storage_quota TYPE record<storage_quota>;
DEFINE FIELD owner ON TABLE storage_quota TYPE record<user>;
DEFINE FIELD bytes_used ON TABLE storage_quota TYPE int DEFAULT 0;
DEFINE FIELD bytes_limit ON TABLE storage_quota TYPE int DEFAULT 5368709120; -- 5GB in bytes
DEFINE FIELD last_reconciled ON TABLE storage_quota TYPE option<datetime>;
DEFINE FIELD device_id ON TABLE storage_quota TYPE string;
DEFINE FIELD created_at ON TABLE storage_quota TYPE datetime VALUE time::now();
DEFINE FIELD updated_at ON TABLE storage_quota TYPE datetime VALUE time::now();

-- Index on owner for fast lookup (one quota per user)
DEFINE INDEX idx_storage_quota_owner ON TABLE storage_quota COLUMNS owner UNIQUE;
