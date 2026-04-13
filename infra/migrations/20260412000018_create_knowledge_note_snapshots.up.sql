-- migrate:up
-- Snapshots are immutable (invariant E-6): no updated_at column, no deleted_at, no trigger.
CREATE TABLE knowledge_note_snapshots (
  id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  note_id     UUID        NOT NULL,
  content     TEXT        NOT NULL,
  captured_at TIMESTAMPTZ NOT NULL,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),

  CONSTRAINT fk_knowledge_note_snapshots_notes FOREIGN KEY (note_id) REFERENCES knowledge_notes(id) ON DELETE CASCADE
);

CREATE INDEX idx_snapshots_note ON knowledge_note_snapshots(note_id, captured_at);
