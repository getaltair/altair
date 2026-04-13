-- migrate:up
-- Item events are append-only (invariant D-5): no updated_at, no deleted_at, no trigger.
CREATE TABLE tracking_item_events (
  id               UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  item_id          UUID        NOT NULL,
  event_type       VARCHAR(20) NOT NULL,
  quantity_change  NUMERIC     NOT NULL,
  from_location_id UUID,
  to_location_id   UUID,
  notes            TEXT,
  occurred_at      TIMESTAMPTZ NOT NULL,
  created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),

  CONSTRAINT fk_tracking_item_events_items          FOREIGN KEY (item_id)          REFERENCES tracking_items(id)     ON DELETE CASCADE,
  CONSTRAINT fk_tracking_item_events_from_locations FOREIGN KEY (from_location_id) REFERENCES tracking_locations(id) ON DELETE SET NULL,
  CONSTRAINT fk_tracking_item_events_to_locations   FOREIGN KEY (to_location_id)   REFERENCES tracking_locations(id) ON DELETE SET NULL
);

CREATE INDEX idx_item_events_item ON tracking_item_events(item_id, occurred_at);
