-- migrate:up
CREATE TABLE guidance_daily_checkins (
  id           UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id      UUID        NOT NULL,
  checkin_date DATE        NOT NULL,
  energy_level INTEGER     NOT NULL,
  mood         VARCHAR(30),
  notes        TEXT,
  created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
  deleted_at   TIMESTAMPTZ,

  CONSTRAINT fk_guidance_daily_checkins_users FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT uq_guidance_daily_checkins_user_date UNIQUE (user_id, checkin_date)
);

CREATE INDEX idx_checkins_user_date ON guidance_daily_checkins(user_id, checkin_date);

CREATE TRIGGER guidance_daily_checkins_updated_at
  BEFORE UPDATE ON guidance_daily_checkins
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
