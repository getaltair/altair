-- migrate:down
DROP TRIGGER IF EXISTS entity_relations_updated_at ON entity_relations;
DROP TABLE IF EXISTS entity_relations;
