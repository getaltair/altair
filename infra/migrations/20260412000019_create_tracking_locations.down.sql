-- migrate:down
DROP TRIGGER IF EXISTS tracking_locations_updated_at ON tracking_locations;
DROP TABLE IF EXISTS tracking_locations;
