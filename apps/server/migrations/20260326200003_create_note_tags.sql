CREATE TABLE IF NOT EXISTS note_tags (
    note_id UUID NOT NULL REFERENCES knowledge_notes(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (note_id, tag_id)
);
CREATE INDEX idx_note_tags_tag ON note_tags(tag_id);
