CREATE TABLE IF NOT EXISTS guidance_quests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    epic_id UUID REFERENCES guidance_epics(id),
    initiative_id UUID REFERENCES initiatives(id),
    user_id UUID NOT NULL REFERENCES users(id),
    household_id UUID REFERENCES households(id),
    name TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'in_progress', 'completed', 'cancelled')),
    priority TEXT NOT NULL DEFAULT 'medium' CHECK (priority IN ('low', 'medium', 'high', 'critical')),
    due_date DATE,
    estimated_minutes INTEGER,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_guidance_quests_user ON guidance_quests(user_id);
CREATE INDEX idx_guidance_quests_epic ON guidance_quests(epic_id);
CREATE INDEX idx_guidance_quests_initiative ON guidance_quests(initiative_id);
CREATE INDEX idx_guidance_quests_household ON guidance_quests(household_id);
CREATE INDEX idx_guidance_quests_status ON guidance_quests(status);
CREATE INDEX idx_guidance_quests_due_date ON guidance_quests(due_date);
