-- ============================================================================
-- Altair Database Migration V002
-- Migration: Refresh Token Storage
-- Description: Add refresh_token table for JWT token management
-- ============================================================================

-- Refresh token storage for JWT token management
-- Tokens are stored hashed (SHA-256) for security
-- Supports revocation and expiration tracking
DEFINE TABLE refresh_token SCHEMAFULL;

-- User who owns this refresh token
DEFINE FIELD user_id ON refresh_token TYPE record<user>;

-- SHA-256 hash of the refresh token (never store plaintext)
DEFINE FIELD token_hash ON refresh_token TYPE string;

-- When this token expires
DEFINE FIELD expires_at ON refresh_token TYPE datetime;

-- When this token was created
DEFINE FIELD created_at ON refresh_token TYPE datetime DEFAULT time::now();

-- When this token was revoked (null if active)
DEFINE FIELD revoked_at ON refresh_token TYPE option<datetime>;

-- Index on user_id for efficient lookup of all tokens for a user
-- Used for revokeAllForUser operations
DEFINE INDEX user_idx ON refresh_token FIELDS user_id;

-- Unique index on token_hash for fast token lookups during refresh
-- Ensures no duplicate tokens can be stored
DEFINE INDEX token_idx ON refresh_token FIELDS token_hash UNIQUE;

-- Index on expires_at for efficient cleanup of expired tokens
-- Used by deleteExpired periodic maintenance
DEFINE INDEX expires_idx ON refresh_token FIELDS expires_at;

-- ============================================================================
-- END OF MIGRATION
-- ============================================================================
