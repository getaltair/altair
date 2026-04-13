-- Verified: applies cleanly on empty users table; drops oidc_sub, adds password_hash/is_admin/status
-- Requires empty users table; safe for dev/CI. See ADR-012.
ALTER TABLE users DROP COLUMN IF EXISTS oidc_sub;
ALTER TABLE users ADD COLUMN password_hash TEXT;
ALTER TABLE users ALTER COLUMN password_hash SET NOT NULL;
ALTER TABLE users ADD COLUMN is_admin BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE users ADD COLUMN status TEXT NOT NULL DEFAULT 'active'
  CHECK (status IN ('active', 'pending'));
