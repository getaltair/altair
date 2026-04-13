-- migrate:down
DROP TRIGGER IF EXISTS guidance_routines_updated_at ON guidance_routines;
DROP TABLE IF EXISTS guidance_routines;
