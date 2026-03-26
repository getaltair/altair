CREATE TABLE IF NOT EXISTS guidance_focus_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    quest_id UUID NOT NULL REFERENCES guidance_quests(id),
    user_id UUID NOT NULL REFERENCES users(id),
    started_at TIMESTAMPTZ NOT NULL,
    ended_at TIMESTAMPTZ,
    duration_minutes INTEGER,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_guidance_focus_sessions_quest ON guidance_focus_sessions(quest_id);
CREATE INDEX idx_guidance_focus_sessions_user ON guidance_focus_sessions(user_id);
