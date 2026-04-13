-- migrate:down
DROP TRIGGER IF EXISTS guidance_daily_checkins_updated_at ON guidance_daily_checkins;
DROP TABLE IF EXISTS guidance_daily_checkins;
