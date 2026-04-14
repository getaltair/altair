-- migrate:up
CREATE TABLE sync_mutations (
  mutation_id  UUID        PRIMARY KEY,
  device_id    UUID        NOT NULL,
  user_id      UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  entity_type  TEXT        NOT NULL,
  entity_id    UUID        NOT NULL,
  operation    TEXT        NOT NULL,
  applied_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_sync_mutations_entity_id ON sync_mutations(entity_id);
CREATE INDEX idx_sync_mutations_user_id   ON sync_mutations(user_id);
