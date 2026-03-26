ALTER TABLE tracking_items ADD CONSTRAINT chk_tracking_items_quantity_non_negative CHECK (quantity >= 0);
