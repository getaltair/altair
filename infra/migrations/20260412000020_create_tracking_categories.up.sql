-- migrate:up
CREATE TABLE tracking_categories (
  id           UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
  name         VARCHAR(100) NOT NULL,
  household_id UUID         NOT NULL,
  created_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
  updated_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
  deleted_at   TIMESTAMPTZ,

  CONSTRAINT fk_tracking_categories_households FOREIGN KEY (household_id) REFERENCES households(id) ON DELETE CASCADE
);

CREATE INDEX idx_categories_household ON tracking_categories(household_id);

CREATE TRIGGER tracking_categories_updated_at
  BEFORE UPDATE ON tracking_categories
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
