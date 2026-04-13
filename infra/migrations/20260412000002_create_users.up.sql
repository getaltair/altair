-- migrate:up
CREATE TABLE users (
  id           UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
  -- No password_hash: authentication is exclusively via OIDC. See ADR-006.
  oidc_sub     TEXT         NOT NULL UNIQUE,
  email        VARCHAR(255) NOT NULL UNIQUE,
  display_name VARCHAR(100) NOT NULL,
  created_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
  updated_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
  deleted_at   TIMESTAMPTZ
);

-- idx_users_oidc_sub and idx_users_email are created implicitly by UNIQUE constraints above.
CREATE INDEX idx_users_deleted_at ON users(deleted_at) WHERE deleted_at IS NOT NULL;

CREATE TRIGGER users_updated_at
  BEFORE UPDATE ON users
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
