-- migrate:down
DROP TRIGGER IF EXISTS household_memberships_updated_at ON household_memberships;
DROP TABLE IF EXISTS household_memberships;
