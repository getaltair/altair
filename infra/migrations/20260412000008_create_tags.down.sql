-- migrate:down
DROP TRIGGER IF EXISTS tags_updated_at ON tags;
DROP TABLE IF EXISTS tags;
