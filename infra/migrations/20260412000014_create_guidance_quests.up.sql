-- migrate:up
CREATE TABLE guidance_quests (
  id            UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
  title         VARCHAR(255) NOT NULL,
  description   TEXT,
  status        VARCHAR(20)  NOT NULL DEFAULT 'not_started',
  priority      VARCHAR(10)  NOT NULL DEFAULT 'medium',
  due_date      DATE,
  epic_id       UUID,
  initiative_id UUID,
  routine_id    UUID,
  user_id       UUID         NOT NULL,
  created_at    TIMESTAMPTZ  NOT NULL DEFAULT now(),
  updated_at    TIMESTAMPTZ  NOT NULL DEFAULT now(),
  deleted_at    TIMESTAMPTZ,

  CONSTRAINT fk_guidance_quests_epics       FOREIGN KEY (epic_id)       REFERENCES guidance_epics(id)    ON DELETE SET NULL,
  CONSTRAINT fk_guidance_quests_initiatives FOREIGN KEY (initiative_id) REFERENCES initiatives(id)        ON DELETE SET NULL,
  CONSTRAINT fk_guidance_quests_routines    FOREIGN KEY (routine_id)    REFERENCES guidance_routines(id)  ON DELETE SET NULL,
  CONSTRAINT fk_guidance_quests_users       FOREIGN KEY (user_id)       REFERENCES users(id)              ON DELETE CASCADE
);

CREATE INDEX idx_quests_user_due_status ON guidance_quests(user_id, due_date, status);
CREATE INDEX idx_quests_epic            ON guidance_quests(epic_id);
CREATE INDEX idx_quests_initiative      ON guidance_quests(initiative_id);
CREATE INDEX idx_quests_routine         ON guidance_quests(routine_id);

CREATE TRIGGER guidance_quests_updated_at
  BEFORE UPDATE ON guidance_quests
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
