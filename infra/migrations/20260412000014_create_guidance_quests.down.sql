-- migrate:down
DROP TRIGGER IF EXISTS guidance_quests_updated_at ON guidance_quests;
DROP TABLE IF EXISTS guidance_quests;
