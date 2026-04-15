-- migrate:up
-- Add created_at / updated_at columns and trigger to sync_mutations (postgres.md convention).
-- sync_mutations has applied_at only; add created_at as an alias and updated_at for mutations.
ALTER TABLE sync_mutations
  ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT now();

CREATE TRIGGER sync_mutations_updated_at
  BEFORE UPDATE ON sync_mutations
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- Add updated_at column and trigger to sync_conflicts.
-- sync_conflicts already has created_at but is missing updated_at despite being mutated
-- (resolution and resolved_at are updated on resolve).
ALTER TABLE sync_conflicts
  ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT now();

CREATE TRIGGER sync_conflicts_updated_at
  BEFORE UPDATE ON sync_conflicts
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
