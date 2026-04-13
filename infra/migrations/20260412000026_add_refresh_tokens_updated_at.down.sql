DROP TRIGGER IF EXISTS set_updated_at_refresh_tokens ON refresh_tokens;
ALTER TABLE refresh_tokens DROP COLUMN IF EXISTS updated_at;
