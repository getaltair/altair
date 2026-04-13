-- migrate:up
CREATE TABLE attachments (
  id           UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
  entity_type  VARCHAR(50)  NOT NULL,
  entity_id    UUID         NOT NULL,
  file_name    VARCHAR(255) NOT NULL,
  content_type VARCHAR(100) NOT NULL,
  size_bytes   BIGINT,
  state        VARCHAR(20)  NOT NULL DEFAULT 'pending',
  storage_path TEXT,
  user_id      UUID         NOT NULL,
  created_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
  updated_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
  deleted_at   TIMESTAMPTZ,

  CONSTRAINT fk_attachments_users FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_attachments_entity ON attachments(entity_type, entity_id);
CREATE INDEX idx_attachments_user   ON attachments(user_id);

CREATE TRIGGER attachments_updated_at
  BEFORE UPDATE ON attachments
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
