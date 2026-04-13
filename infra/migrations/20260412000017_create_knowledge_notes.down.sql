-- migrate:down
DROP TRIGGER IF EXISTS knowledge_notes_updated_at ON knowledge_notes;
DROP TABLE IF EXISTS knowledge_notes;
