CREATE TABLE tracking_item_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    item_id UUID NOT NULL REFERENCES tracking_items(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id),
    event_type TEXT NOT NULL CHECK (event_type IN ('consumed', 'restocked', 'moved', 'adjusted', 'expired', 'donated')),
    quantity_change INTEGER NOT NULL,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_tracking_item_events_item ON tracking_item_events(item_id);
CREATE INDEX idx_tracking_item_events_created ON tracking_item_events(created_at);
