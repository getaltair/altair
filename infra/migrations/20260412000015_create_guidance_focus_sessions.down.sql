-- migrate:down
DROP TRIGGER IF EXISTS guidance_focus_sessions_updated_at ON guidance_focus_sessions;
DROP TABLE IF EXISTS guidance_focus_sessions;
