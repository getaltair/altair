CREATE TABLE IF NOT EXISTS routine_tags (
    routine_id UUID NOT NULL REFERENCES guidance_routines(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (routine_id, tag_id)
);
