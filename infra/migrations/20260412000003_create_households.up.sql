-- migrate:up
CREATE TABLE households (
  id         UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
  owner_id   UUID         NOT NULL,
  name       VARCHAR(255) NOT NULL,
  created_at TIMESTAMPTZ  NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ  NOT NULL DEFAULT now(),
  deleted_at TIMESTAMPTZ,

  CONSTRAINT fk_households_users FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE RESTRICT
);

CREATE INDEX idx_households_owner_id ON households(owner_id);
CREATE INDEX idx_households_deleted_at ON households(deleted_at) WHERE deleted_at IS NOT NULL;

CREATE TRIGGER households_updated_at
  BEFORE UPDATE ON households
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
