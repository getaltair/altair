-- migrate:up
CREATE TABLE tracking_locations (
  id           UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
  name         VARCHAR(100) NOT NULL,
  household_id UUID         NOT NULL,
  created_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
  updated_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
  deleted_at   TIMESTAMPTZ,

  CONSTRAINT fk_tracking_locations_households FOREIGN KEY (household_id) REFERENCES households(id) ON DELETE CASCADE
);

CREATE INDEX idx_locations_household ON tracking_locations(household_id);

CREATE TRIGGER tracking_locations_updated_at
  BEFORE UPDATE ON tracking_locations
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
