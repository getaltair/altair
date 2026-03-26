-- Add cascade behavior to guidance FK relationships

ALTER TABLE guidance_quests DROP CONSTRAINT IF EXISTS guidance_quests_epic_id_fkey;
ALTER TABLE guidance_quests ADD CONSTRAINT guidance_quests_epic_id_fkey
    FOREIGN KEY (epic_id) REFERENCES guidance_epics(id) ON DELETE SET NULL;

ALTER TABLE guidance_focus_sessions DROP CONSTRAINT IF EXISTS guidance_focus_sessions_quest_id_fkey;
ALTER TABLE guidance_focus_sessions ADD CONSTRAINT guidance_focus_sessions_quest_id_fkey
    FOREIGN KEY (quest_id) REFERENCES guidance_quests(id) ON DELETE CASCADE;

ALTER TABLE guidance_quests DROP CONSTRAINT IF EXISTS guidance_quests_initiative_id_fkey;
ALTER TABLE guidance_quests ADD CONSTRAINT guidance_quests_initiative_id_fkey
    FOREIGN KEY (initiative_id) REFERENCES initiatives(id) ON DELETE SET NULL;

ALTER TABLE guidance_epics DROP CONSTRAINT IF EXISTS guidance_epics_initiative_id_fkey;
ALTER TABLE guidance_epics ADD CONSTRAINT guidance_epics_initiative_id_fkey
    FOREIGN KEY (initiative_id) REFERENCES initiatives(id) ON DELETE SET NULL;
