-- Migration 005: Authentication Tables
-- Creates session and user_credential tables for local authentication

USE NS altair DB main;

-- ============================================================================
-- Session Table
-- ============================================================================

DEFINE TABLE session SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE session TYPE record<session>;
DEFINE FIELD token ON TABLE session TYPE string;
DEFINE FIELD user ON TABLE session TYPE record<user>;
DEFINE FIELD expires_at ON TABLE session TYPE datetime;
DEFINE FIELD device_id ON TABLE session TYPE string;
DEFINE FIELD created_at ON TABLE session TYPE datetime VALUE time::now();

-- Index on token for fast lookup during validation
DEFINE INDEX token_idx ON TABLE session COLUMNS token UNIQUE;

-- ============================================================================
-- User Credential Table
-- ============================================================================

DEFINE TABLE user_credential SCHEMAFULL CHANGEFEED 7d;

DEFINE FIELD id ON TABLE user_credential TYPE record<user_credential>;
DEFINE FIELD user ON TABLE user_credential TYPE record<user>;
DEFINE FIELD password_hash ON TABLE user_credential TYPE string;
DEFINE FIELD updated_at ON TABLE user_credential TYPE datetime VALUE time::now();

-- Index on user for fast credential lookup
DEFINE INDEX user_cred_idx ON TABLE user_credential COLUMNS user UNIQUE;
