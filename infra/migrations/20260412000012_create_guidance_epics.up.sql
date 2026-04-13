-- migrate:up
CREATE TABLE guidance_epics (
  id            UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
  initiative_id UUID         NOT NULL,
  title         VARCHAR(255) NOT NULL,
  description   TEXT,
  status        VARCHAR(20)  NOT NULL DEFAULT 'not_started',
  sort_order    INTEGER      NOT NULL DEFAULT 0,
  user_id       UUID         NOT NULL,
  created_at    TIMESTAMPTZ  NOT NULL DEFAULT now(),
  updated_at    TIMESTAMPTZ  NOT NULL DEFAULT now(),
  deleted_at    TIMESTAMPTZ,

  CONSTRAINT fk_guidance_epics_initiatives FOREIGN KEY (initiative_id) REFERENCES initiatives(id) ON DELETE CASCADE,
  CONSTRAINT fk_guidance_epics_users       FOREIGN KEY (user_id)       REFERENCES users(id)       ON DELETE CASCADE
);

CREATE INDEX idx_epics_initiative ON guidance_epics(initiative_id);
CREATE INDEX idx_epics_user       ON guidance_epics(user_id);

CREATE TRIGGER guidance_epics_updated_at
  BEFORE UPDATE ON guidance_epics
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
