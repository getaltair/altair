CREATE TABLE IF NOT EXISTS knowledge_note_snapshots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    note_id UUID NOT NULL REFERENCES knowledge_notes(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_by_process TEXT
);
CREATE INDEX idx_note_snapshots_note ON knowledge_note_snapshots(note_id, created_at DESC);
