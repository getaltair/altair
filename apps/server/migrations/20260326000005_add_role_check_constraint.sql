ALTER TABLE household_memberships
ADD CONSTRAINT chk_membership_role CHECK (role IN ('owner', 'member'));
