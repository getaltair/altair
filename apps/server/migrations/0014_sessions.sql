-- Migration: 0014_sessions
-- Description: Create session table for Better-Auth authentication
--
-- This table stores user sessions for Better-Auth authentication.
-- Sessions allow users to remain authenticated across requests.

CREATE TABLE session (
	id UUID PRIMARY KEY,
	user_id UUID NOT NULL,
	token TEXT NOT NULL,
	expires_at TIMESTAMPTZ NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
	ip_address TEXT,
	user_agent TEXT,
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Index for session token lookups (most common query)
CREATE INDEX session_token_idx ON session(token);

-- Index for user sessions lookup
CREATE INDEX session_user_id_idx ON session(user_id);

-- Index for expired session cleanup
CREATE INDEX session_expires_at_idx ON session(expires_at);
