-- migrate:down
DROP TRIGGER IF EXISTS initiatives_updated_at ON initiatives;
DROP TABLE IF EXISTS initiatives;
