CREATE TABLE IF NOT EXISTS quest_tags (
    quest_id UUID NOT NULL REFERENCES guidance_quests(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (quest_id, tag_id)
);
