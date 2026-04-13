-- P4-008: refresh_tokens was missing updated_at despite receiving UPDATE operations
-- for token revocation (revoked_at). Convention: all tables with UPDATE ops require
-- updated_at + set_updated_at trigger (see .claude/rules/postgres.md).
ALTER TABLE refresh_tokens ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT now();

CREATE TRIGGER set_updated_at_refresh_tokens
    BEFORE UPDATE ON refresh_tokens
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();
