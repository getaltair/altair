-- migrate:down
DROP TRIGGER IF EXISTS attachments_updated_at ON attachments;
DROP TABLE IF EXISTS attachments;
