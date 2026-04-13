-- migrate:up
CREATE TABLE guidance_focus_sessions (
  id               UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  quest_id         UUID        NOT NULL,
  started_at       TIMESTAMPTZ NOT NULL,
  ended_at         TIMESTAMPTZ,
  duration_minutes INTEGER,
  user_id          UUID        NOT NULL,
  created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
  deleted_at       TIMESTAMPTZ,

  CONSTRAINT fk_guidance_focus_sessions_quests FOREIGN KEY (quest_id) REFERENCES guidance_quests(id) ON DELETE CASCADE,
  CONSTRAINT fk_guidance_focus_sessions_users  FOREIGN KEY (user_id)  REFERENCES users(id)           ON DELETE CASCADE
);

CREATE INDEX idx_focus_quest ON guidance_focus_sessions(quest_id);
CREATE INDEX idx_focus_user  ON guidance_focus_sessions(user_id);

CREATE TRIGGER guidance_focus_sessions_updated_at
  BEFORE UPDATE ON guidance_focus_sessions
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
