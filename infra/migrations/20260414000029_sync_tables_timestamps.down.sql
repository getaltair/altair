-- migrate:down
DROP TRIGGER IF EXISTS sync_conflicts_updated_at ON sync_conflicts;
ALTER TABLE sync_conflicts DROP COLUMN IF EXISTS updated_at;

DROP TRIGGER IF EXISTS sync_mutations_updated_at ON sync_mutations;
ALTER TABLE sync_mutations DROP COLUMN IF EXISTS updated_at;
ALTER TABLE sync_mutations DROP COLUMN IF EXISTS created_at;
