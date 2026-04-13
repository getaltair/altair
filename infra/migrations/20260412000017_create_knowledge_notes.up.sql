-- migrate:up
CREATE TABLE knowledge_notes (
  id            UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
  title         VARCHAR(255) NOT NULL,
  content       TEXT,
  user_id       UUID         NOT NULL,
  initiative_id UUID,
  created_at    TIMESTAMPTZ  NOT NULL DEFAULT now(),
  updated_at    TIMESTAMPTZ  NOT NULL DEFAULT now(),
  deleted_at    TIMESTAMPTZ,

  CONSTRAINT fk_knowledge_notes_users       FOREIGN KEY (user_id)       REFERENCES users(id)       ON DELETE CASCADE,
  CONSTRAINT fk_knowledge_notes_initiatives FOREIGN KEY (initiative_id) REFERENCES initiatives(id) ON DELETE SET NULL
);

CREATE INDEX idx_notes_user       ON knowledge_notes(user_id);
CREATE INDEX idx_notes_initiative ON knowledge_notes(initiative_id);

CREATE TRIGGER knowledge_notes_updated_at
  BEFORE UPDATE ON knowledge_notes
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
