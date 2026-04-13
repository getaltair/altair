-- migrate:up
CREATE TABLE initiatives (
  id           UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
  title        VARCHAR(255) NOT NULL,
  description  TEXT,
  status       VARCHAR(20)  NOT NULL DEFAULT 'draft',
  user_id      UUID         NOT NULL,
  household_id UUID,
  created_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
  updated_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
  deleted_at   TIMESTAMPTZ,

  CONSTRAINT fk_initiatives_users      FOREIGN KEY (user_id)      REFERENCES users(id)      ON DELETE CASCADE,
  CONSTRAINT fk_initiatives_households FOREIGN KEY (household_id) REFERENCES households(id) ON DELETE SET NULL
);

CREATE INDEX idx_initiatives_user      ON initiatives(user_id, status);
CREATE INDEX idx_initiatives_household ON initiatives(household_id, status);

CREATE TRIGGER initiatives_updated_at
  BEFORE UPDATE ON initiatives
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
