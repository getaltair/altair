-- migrate:up
CREATE TABLE sync_conflicts (
  id               UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  mutation_id      UUID,
  entity_type      TEXT        NOT NULL,
  entity_id        UUID        NOT NULL,
  base_version     TIMESTAMPTZ,
  current_version  TIMESTAMPTZ,
  incoming_payload JSONB,
  current_payload  JSONB,
  resolution       TEXT        NOT NULL DEFAULT 'pending',
  resolved_at      TIMESTAMPTZ,
  user_id          UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  created_at       TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_sync_conflicts_entity_id ON sync_conflicts(entity_id);
CREATE INDEX idx_sync_conflicts_user_id   ON sync_conflicts(user_id);
