-- migrate:up
-- NOTE: Routines are created before quests (000014) so that guidance_quests.routine_id
-- can reference this table. Task specification listed quests as 000013 and routines as 000014;
-- ordering is swapped here to satisfy FK dependency.
CREATE TABLE guidance_routines (
  id               UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
  title            VARCHAR(255) NOT NULL,
  description      TEXT,
  frequency_type   VARCHAR(20)  NOT NULL,
  frequency_config JSONB        NOT NULL,
  status           VARCHAR(20)  NOT NULL DEFAULT 'active',
  user_id          UUID         NOT NULL,
  created_at       TIMESTAMPTZ  NOT NULL DEFAULT now(),
  updated_at       TIMESTAMPTZ  NOT NULL DEFAULT now(),
  deleted_at       TIMESTAMPTZ,

  CONSTRAINT fk_guidance_routines_users FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_routines_user_status ON guidance_routines(user_id, status);

CREATE TRIGGER guidance_routines_updated_at
  BEFORE UPDATE ON guidance_routines
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
