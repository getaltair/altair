-- migrate:down
DROP TRIGGER IF EXISTS tracking_shopping_lists_updated_at ON tracking_shopping_lists;
DROP TABLE IF EXISTS tracking_shopping_lists;
