-- migrate:down
DROP TRIGGER IF EXISTS tracking_categories_updated_at ON tracking_categories;
DROP TABLE IF EXISTS tracking_categories;
