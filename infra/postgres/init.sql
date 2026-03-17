-- PostgreSQL initialization script for PowerSync replication
-- Runs once on first container init via docker-entrypoint-initdb.d/
-- Connected to POSTGRES_DB (local) by default

-- 1. Create powersync_storage database
SELECT 'CREATE DATABASE powersync_storage'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'powersync_storage')
\gexec

-- 2. Create powersync user with REPLICATION role
-- Dev-only password. Override via POSTGRES_POWERSYNC_PASSWORD env var in production.
DO $$ BEGIN
  CREATE USER powersync WITH PASSWORD 'powersync_password';
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

ALTER USER powersync WITH REPLICATION;

-- 3. Grant permissions
GRANT CONNECT ON DATABASE local TO powersync;
GRANT USAGE ON SCHEMA public TO powersync;
GRANT SELECT ON ALL TABLES IN SCHEMA public TO powersync;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT ON TABLES TO powersync;
-- Allow PowerSync to create its own schema in the storage database
GRANT CONNECT ON DATABASE powersync_storage TO powersync;
GRANT CREATE ON DATABASE powersync_storage TO powersync;

-- 4. Create publication for PowerSync replication (idempotent)
DO $$ BEGIN
  CREATE PUBLICATION powersync FOR ALL TABLES;
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;
