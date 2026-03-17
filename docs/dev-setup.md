# Developer Setup: Local PowerSync Environment

This guide covers the local development setup for Altair's offline-first synchronization using PowerSync and PostgreSQL.

## Architecture Overview

Altair uses an offline-first architecture where the client (Web/Mobile) interacts with a local SQLite database. PowerSync facilitates the bi-directional synchronization between the server's PostgreSQL database and the client's SQLite database.

- **PostgreSQL**: The source of truth for all data.
- **PowerSync Service**: Monitors PostgreSQL changes (via logical replication) and streams them to clients.
- **SQLite**: Local database on the client device.

## Prerequisites

Ensure you have the following tools installed:

- **Docker Desktop** or **Docker Engine + Docker Compose v2**
- **Bun** (v1.3+) - for the web application
- **Rust toolchain** - for the API server and worker
- **Git**

## Quick Start

Follow these steps to get your local environment running:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/getaltair/altair.git
   cd altair
   ```

2. **Configure environment variables**:
   Copy the example environment file to `.env` in the project root:
   ```bash
   cp .env.example .env
   ```

3. **Start infrastructure services**:
   Launch PostgreSQL and PowerSync using Docker Compose:
   ```bash
   docker compose up -d
   ```

4. **Verify service health**:
   Check that all containers are running and healthy:
   ```bash
   docker compose ps
   ```
   Expected output should show `postgres` and `powersync` as `Up (healthy)` or `Up`.

5. **Apply database schema**:
   Push the Drizzle schema to the PostgreSQL instance:
   ```bash
   cd apps/web
   bun run db:push
   ```

6. **Start the Web App**:
   ```bash
   bun run dev
   ```
   The web app will be available at [http://localhost:5173](http://localhost:5173).

7. **Start the API Server**:
   In a new terminal:
   ```bash
   cd apps/server
   cargo run
   ```
   The API server will be available at [http://localhost:3000](http://localhost:3000).

## Service Table

| Service | Port | URL | Health Check |
|---------|------|-----|--------------|
| PostgreSQL | 5432 | `localhost:5432` | `docker compose ps` |
| PowerSync | 8080 | `localhost:8080` | `curl http://localhost:8080/probes/liveness` |
| Web App | 5173 | [http://localhost:5173](http://localhost:5173) | Browser |
| API Server | 3000 | [http://localhost:3000](http://localhost:3000) | `curl http://localhost:3000/health` |

## PowerSync Specifics

### Diagnostics
You can check the internal state and health of the PowerSync service using the diagnostics endpoint:
```bash
curl -X POST http://localhost:8080/api/admin/v1/diagnostics \
  -H "Authorization: Bearer dev-admin-token-change-in-production"
```
*(Note: The token matches `PS_ADMIN_TOKEN` in your `.env` file.)*

### Configuration
- **Sync Rules**: Defined in `infra/powersync/sync-rules.yaml`. These rules determine which data is synced to which users.
- **Service Config**: Defined in `infra/powersync/powersync.yaml`.

### Restarting
If you modify the sync rules or configuration, restart the PowerSync service to apply changes:
```bash
docker compose restart powersync
```

### Authentication Note
Client authentication (JWT/JWKS) is not yet configured — this is planned for P4-003. The PowerSync service runs, but clients cannot authenticate until P4-003 is complete.

## Troubleshooting

### PowerSync won't start
- Ensure PostgreSQL is healthy first: `docker compose ps`.
- Check PowerSync logs: `docker compose logs powersync`.

### Replication not connecting
Verify that `wal_level` is set to `logical` in PostgreSQL:
```bash
docker compose exec postgres psql -U root -d local -c "SHOW wal_level;"
```
If it's not `logical`, ensure you are using the `compose.yaml` provided in the root, which sets this flag.

### Port conflicts
If ports 5432 or 8080 are already in use, you can change them in the `.env` file. Note that you will also need to update the connection strings (`PS_DATABASE_URI`, etc.) accordingly.

### Fresh Start / Reset
To completely reset the environment (including data):
```bash
docker compose down -v
docker compose up -d
```
*Note: The database initialization script (`infra/postgres/init.sql`) only runs when the volume is first created. Use `docker compose down -v` to trigger a re-initialization.
