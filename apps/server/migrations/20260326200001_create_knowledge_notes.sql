CREATE TABLE IF NOT EXISTS knowledge_notes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    household_id UUID REFERENCES households(id),
    initiative_id UUID REFERENCES initiatives(id),
    title TEXT NOT NULL,
    content TEXT,
    content_type TEXT NOT NULL DEFAULT 'markdown' CHECK (content_type IN ('markdown', 'plain')),
    is_pinned BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_knowledge_notes_user ON knowledge_notes(user_id);
CREATE INDEX idx_knowledge_notes_household ON knowledge_notes(household_id);
CREATE INDEX idx_knowledge_notes_initiative ON knowledge_notes(initiative_id);
CREATE INDEX idx_knowledge_notes_pinned ON knowledge_notes(user_id, is_pinned) WHERE is_pinned = true;
