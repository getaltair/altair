-- PostgreSQL initialization script for PowerSync replication
-- This script runs once when the Postgres Docker container first initializes
-- It is idempotent and safe to run multiple times

-- ============================================================================
-- 1. Create powersync_storage database (for PowerSync bucket storage)
-- ============================================================================
SELECT 'CREATE DATABASE powersync_storage'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'powersync_storage')
\gexec

-- ============================================================================
-- 2. Create powersync user with REPLICATION role
-- ============================================================================
DO $$ BEGIN
  CREATE USER powersync WITH PASSWORD 'powersync_password';
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

-- Grant REPLICATION role to powersync user
ALTER USER powersync WITH REPLICATION;

-- ============================================================================
-- 3. Grant permissions on both databases
-- ============================================================================
-- Grant CONNECT on local database
GRANT CONNECT ON DATABASE local TO powersync;

-- Grant CONNECT on powersync_storage database
GRANT CONNECT ON DATABASE powersync_storage TO powersync;

-- Grant usage on public schema in local database
GRANT USAGE ON SCHEMA public TO powersync;

-- Grant SELECT on all tables in local database (for replication)
GRANT SELECT ON ALL TABLES IN SCHEMA public TO powersync;

-- Set default privileges for future tables
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT ON TABLES TO powersync;

-- ============================================================================
-- 4. Create publication on local database for PowerSync replication
-- ============================================================================
-- Switch to local database
\c local

-- Create publication for all tables (idempotent with IF NOT EXISTS)
CREATE PUBLICATION IF NOT EXISTS powersync FOR ALL TABLES;

-- Grant permissions on powersync_storage database
-- (Must be done in the context of that database)
\c powersync_storage

-- Grant all privileges on public schema to powersync user
GRANT ALL PRIVILEGES ON SCHEMA public TO powersync;

-- Grant all privileges on all tables to powersync user
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO powersync;

-- Set default privileges for future tables
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL PRIVILEGES ON TABLES TO powersync;

-- Grant all privileges on all sequences
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO powersync;

-- Set default privileges for future sequences
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL PRIVILEGES ON SEQUENCES TO powersync;
