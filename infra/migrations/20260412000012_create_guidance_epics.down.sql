-- migrate:down
DROP TRIGGER IF EXISTS guidance_epics_updated_at ON guidance_epics;
DROP TABLE IF EXISTS guidance_epics;
