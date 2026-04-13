-- Seed data for local development and CI smoke tests.
-- Idempotent: safe to run multiple times.

INSERT INTO users (id, oidc_sub, email, display_name)
VALUES (
  'a0000000-0000-0000-0000-000000000001',
  'dev-user-001',
  'dev@altair.local',
  'Dev User'
)
ON CONFLICT (oidc_sub) DO NOTHING;
