-- V2: Authentication Tables
-- Adds password hash to users and creates refresh token / invite code tables

-- ============================================================
-- USER TABLE UPDATES
-- ============================================================

-- Add password_hash field to user table (nullable for existing users)
DEFINE FIELD password_hash ON user TYPE option<string>;

-- ============================================================
-- REFRESH TOKEN TABLE
-- ============================================================

-- Refresh tokens for session management
DEFINE TABLE refresh_token SCHEMAFULL;
DEFINE FIELD user_id ON refresh_token TYPE record<user>;
DEFINE FIELD token_hash ON refresh_token TYPE string;
DEFINE FIELD device_name ON refresh_token TYPE option<string>;
DEFINE FIELD expires_at ON refresh_token TYPE datetime;
DEFINE FIELD created_at ON refresh_token TYPE datetime DEFAULT time::now();
DEFINE FIELD revoked_at ON refresh_token TYPE option<datetime>;
DEFINE INDEX idx_refresh_token_hash ON refresh_token FIELDS token_hash UNIQUE;
DEFINE INDEX idx_refresh_token_user ON refresh_token FIELDS user_id;
DEFINE INDEX idx_refresh_token_expires ON refresh_token FIELDS expires_at;

-- ============================================================
-- INVITE CODE TABLE
-- ============================================================

-- Invite codes for controlled registration
DEFINE TABLE invite_code SCHEMAFULL;
DEFINE FIELD code ON invite_code TYPE string;
DEFINE FIELD created_by ON invite_code TYPE record<user>;
DEFINE FIELD used_by ON invite_code TYPE option<record<user>>;
DEFINE FIELD expires_at ON invite_code TYPE datetime;
DEFINE FIELD created_at ON invite_code TYPE datetime DEFAULT time::now();
DEFINE FIELD used_at ON invite_code TYPE option<datetime>;
DEFINE INDEX idx_invite_code ON invite_code FIELDS code UNIQUE;
DEFINE INDEX idx_invite_created_by ON invite_code FIELDS created_by;
