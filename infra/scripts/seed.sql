-- Seed data for local development and CI smoke tests.
-- Idempotent: safe to run multiple times.

INSERT INTO users (id, email, display_name, password_hash, is_admin, status)
VALUES (
  'a0000000-0000-0000-0000-000000000001',
  'dev@altair.local',
  'Dev User',
  '$argon2id$v=19$m=65536,t=3,p=4$Y2ktc2VlZC11c2VyLTAwMQ$placeholder-ci-only-not-a-real-hash',
  false,
  'active'
)
ON CONFLICT (email) DO NOTHING;
