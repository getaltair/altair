CREATE TABLE IF NOT EXISTS entity_relations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_entity_type TEXT NOT NULL,
    from_entity_id UUID NOT NULL,
    to_entity_type TEXT NOT NULL,
    to_entity_id UUID NOT NULL,
    relation_type TEXT NOT NULL,
    source_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'accepted',
    confidence NUMERIC CHECK (confidence >= 0.0 AND confidence <= 1.0),
    evidence_json JSONB,
    created_by_user_id UUID REFERENCES users(id),
    created_by_process TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_confirmed_at TIMESTAMPTZ
);
CREATE INDEX idx_relations_from ON entity_relations(from_entity_type, from_entity_id);
CREATE INDEX idx_relations_to ON entity_relations(to_entity_type, to_entity_id);
CREATE INDEX idx_relations_status ON entity_relations(status);
