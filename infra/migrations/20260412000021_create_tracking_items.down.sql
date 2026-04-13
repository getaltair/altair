-- migrate:down
DROP TRIGGER IF EXISTS tracking_items_updated_at ON tracking_items;
DROP TABLE IF EXISTS tracking_items;
