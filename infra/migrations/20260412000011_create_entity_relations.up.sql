-- migrate:up
CREATE TABLE entity_relations (
  id               UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  from_entity_type VARCHAR(50) NOT NULL,
  from_entity_id   UUID        NOT NULL,
  to_entity_type   VARCHAR(50) NOT NULL,
  to_entity_id     UUID        NOT NULL,
  relation_type    VARCHAR(30) NOT NULL,
  source_type      VARCHAR(20) NOT NULL,
  status           VARCHAR(20) NOT NULL DEFAULT 'accepted',
  confidence       FLOAT,
  evidence         TEXT,
  user_id          UUID        NOT NULL,
  created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
  deleted_at       TIMESTAMPTZ,

  CONSTRAINT fk_entity_relations_users FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_relations_from ON entity_relations(from_entity_type, from_entity_id);
CREATE INDEX idx_relations_to   ON entity_relations(to_entity_type, to_entity_id);
CREATE INDEX idx_relations_user  ON entity_relations(user_id);

CREATE TRIGGER entity_relations_updated_at
  BEFORE UPDATE ON entity_relations
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
