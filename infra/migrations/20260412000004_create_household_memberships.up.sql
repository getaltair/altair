-- migrate:up
CREATE TABLE household_memberships (
  id           UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  household_id UUID        NOT NULL,
  user_id      UUID        NOT NULL,
  role         VARCHAR(50) NOT NULL DEFAULT 'member',
  created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
  deleted_at   TIMESTAMPTZ,

  CONSTRAINT fk_household_memberships_households FOREIGN KEY (household_id) REFERENCES households(id) ON DELETE CASCADE,
  CONSTRAINT fk_household_memberships_users FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT uq_household_memberships_pair UNIQUE (household_id, user_id)
);

CREATE INDEX idx_household_memberships_household_id ON household_memberships(household_id);
CREATE INDEX idx_household_memberships_user_id ON household_memberships(user_id);

CREATE TRIGGER household_memberships_updated_at
  BEFORE UPDATE ON household_memberships
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
