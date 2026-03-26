CREATE TABLE tracking_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    household_id UUID NOT NULL REFERENCES households(id),
    name TEXT NOT NULL,
    description TEXT,
    parent_category_id UUID REFERENCES tracking_categories(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_tracking_categories_household ON tracking_categories(household_id);
CREATE INDEX idx_tracking_categories_parent ON tracking_categories(parent_category_id);
