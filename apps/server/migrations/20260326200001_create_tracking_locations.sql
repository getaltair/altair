CREATE TABLE tracking_locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    household_id UUID NOT NULL REFERENCES households(id),
    name TEXT NOT NULL,
    description TEXT,
    parent_location_id UUID REFERENCES tracking_locations(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_tracking_locations_household ON tracking_locations(household_id);
CREATE INDEX idx_tracking_locations_parent ON tracking_locations(parent_location_id);
