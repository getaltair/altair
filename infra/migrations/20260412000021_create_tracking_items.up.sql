-- migrate:up
CREATE TABLE tracking_items (
  id            UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
  name          VARCHAR(255) NOT NULL,
  description   TEXT,
  quantity      NUMERIC      NOT NULL DEFAULT 0 CHECK (quantity >= 0),
  barcode       VARCHAR(50),
  location_id   UUID,
  category_id   UUID,
  user_id       UUID         NOT NULL,
  household_id  UUID,
  initiative_id UUID,
  expires_at    TIMESTAMPTZ,
  created_at    TIMESTAMPTZ  NOT NULL DEFAULT now(),
  updated_at    TIMESTAMPTZ  NOT NULL DEFAULT now(),
  deleted_at    TIMESTAMPTZ,

  CONSTRAINT fk_tracking_items_locations   FOREIGN KEY (location_id)   REFERENCES tracking_locations(id)   ON DELETE SET NULL,
  CONSTRAINT fk_tracking_items_categories  FOREIGN KEY (category_id)   REFERENCES tracking_categories(id)  ON DELETE SET NULL,
  CONSTRAINT fk_tracking_items_users       FOREIGN KEY (user_id)       REFERENCES users(id)                ON DELETE CASCADE,
  CONSTRAINT fk_tracking_items_households  FOREIGN KEY (household_id)  REFERENCES households(id)           ON DELETE SET NULL,
  CONSTRAINT fk_tracking_items_initiatives FOREIGN KEY (initiative_id) REFERENCES initiatives(id)          ON DELETE SET NULL
);

CREATE INDEX idx_items_household_location_category ON tracking_items(household_id, location_id, category_id);
CREATE INDEX idx_items_user    ON tracking_items(user_id);
CREATE INDEX idx_items_barcode ON tracking_items(barcode);
CREATE INDEX idx_items_expires ON tracking_items(expires_at);

CREATE TRIGGER tracking_items_updated_at
  BEFORE UPDATE ON tracking_items
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
