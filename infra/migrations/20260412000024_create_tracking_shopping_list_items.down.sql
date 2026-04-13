-- migrate:down
DROP TRIGGER IF EXISTS tracking_shopping_list_items_updated_at ON tracking_shopping_list_items;
DROP TABLE IF EXISTS tracking_shopping_list_items;
