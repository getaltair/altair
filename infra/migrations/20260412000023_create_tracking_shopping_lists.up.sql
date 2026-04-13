-- migrate:up
CREATE TABLE tracking_shopping_lists (
  id           UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
  name         VARCHAR(255) NOT NULL,
  household_id UUID         NOT NULL,
  created_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
  updated_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
  deleted_at   TIMESTAMPTZ,

  CONSTRAINT fk_tracking_shopping_lists_households FOREIGN KEY (household_id) REFERENCES households(id) ON DELETE CASCADE
);

CREATE INDEX idx_shopping_lists_household ON tracking_shopping_lists(household_id);

CREATE TRIGGER tracking_shopping_lists_updated_at
  BEFORE UPDATE ON tracking_shopping_lists
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
