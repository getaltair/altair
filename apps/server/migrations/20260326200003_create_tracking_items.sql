CREATE TABLE tracking_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    household_id UUID NOT NULL REFERENCES households(id),
    category_id UUID REFERENCES tracking_categories(id),
    location_id UUID REFERENCES tracking_locations(id),
    name TEXT NOT NULL,
    description TEXT,
    quantity INTEGER NOT NULL DEFAULT 0,
    unit TEXT,
    min_quantity INTEGER,
    barcode TEXT,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'archived')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_tracking_items_household ON tracking_items(household_id);
CREATE INDEX idx_tracking_items_category ON tracking_items(category_id);
CREATE INDEX idx_tracking_items_location ON tracking_items(location_id);
CREATE INDEX idx_tracking_items_status ON tracking_items(status);
