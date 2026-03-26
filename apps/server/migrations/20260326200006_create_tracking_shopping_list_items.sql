CREATE TABLE tracking_shopping_list_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    shopping_list_id UUID NOT NULL REFERENCES tracking_shopping_lists(id) ON DELETE CASCADE,
    item_id UUID REFERENCES tracking_items(id),
    name TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    unit TEXT,
    is_checked BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_tracking_shopping_list_items_list ON tracking_shopping_list_items(shopping_list_id);
