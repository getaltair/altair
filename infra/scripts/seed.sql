-- Seed data for local development and CI smoke tests.
-- Idempotent: safe to run multiple times.

-- Note: oidc_sub must match the dev identity created by infra/scripts/zitadel-setup.sh.
-- If Zitadel is reconfigured, update this value to match the generated subject identifier.
INSERT INTO users (id, oidc_sub, email, display_name)
VALUES (
  'a0000000-0000-0000-0000-000000000001',
  'dev-user-001',
  'dev@altair.local',
  'Dev User'
)
ON CONFLICT (oidc_sub) DO NOTHING;
