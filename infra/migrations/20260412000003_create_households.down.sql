-- migrate:down
DROP TRIGGER IF EXISTS households_updated_at ON households;
DROP TABLE IF EXISTS households;
