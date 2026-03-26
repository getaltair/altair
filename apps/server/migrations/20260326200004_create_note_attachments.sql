CREATE TABLE IF NOT EXISTS note_attachments (
    note_id UUID NOT NULL REFERENCES knowledge_notes(id) ON DELETE CASCADE,
    attachment_id UUID NOT NULL REFERENCES attachments(id) ON DELETE CASCADE,
    PRIMARY KEY (note_id, attachment_id)
);
CREATE INDEX idx_note_attachments_attachment ON note_attachments(attachment_id);
