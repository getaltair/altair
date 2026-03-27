ALTER TABLE entity_relations ADD COLUMN IF NOT EXISTS household_id UUID REFERENCES households(id);
ALTER TABLE entity_relations ADD COLUMN IF NOT EXISTS initiative_id UUID REFERENCES initiatives(id);
ALTER TABLE entity_relations ADD COLUMN IF NOT EXISTS owner_user_id UUID REFERENCES users(id);
ALTER TABLE entity_relations ADD COLUMN IF NOT EXISTS updated_by_user_id UUID REFERENCES users(id);
CREATE INDEX IF NOT EXISTS idx_relations_household ON entity_relations(household_id);
CREATE INDEX IF NOT EXISTS idx_relations_owner ON entity_relations(owner_user_id);
CREATE INDEX IF NOT EXISTS idx_relations_created_by ON entity_relations(created_by_user_id);
