# Development Setup for altair-db-service

This guide covers setting up your development environment to work with the Altair database service.

## Prerequisites

### Install SurrealDB

The database service requires SurrealDB to be installed:

**Linux/macOS:**

```bash
curl -sSf https://install.surrealdb.com | sh
```

**Windows:**

```powershell
iwr https://windows.surrealdb.com -useb | iex
```

Verify installation:

```bash
surreal version
```

### Install Flutter

Make sure you have Flutter 3.5+ installed:

```bash
flutter --version
```

## Quick Start

### 1. Install Dependencies

```bash
cd packages/altair-db-service
flutter pub get
```

### 2. Start SurrealDB for Development

Use the provided script (from repo root):

```bash
./scripts/start-db.sh
```

This starts SurrealDB with:

- Port: 8000
- User: altair
- Password: altair-local-dev
- Database file: `./altair.db`

Or start manually:

```bash
surreal start \
  --bind 127.0.0.1:8000 \
  --user altair \
  --pass altair-local-dev \
  file://./altair.db
```

### 3. Run Tests

```bash
flutter test
```

## Development Workflow

### Running the Service Programmatically

```dart
import 'package:altair_db_service/altair_db_service.dart';

void main() async {
  final service = AltairDatabaseService();

  // Start the service
  await service.start();

  // Get a connection
  final db = await service.getConnection();

  // Use the database
  await db.create('task', {
    'title': 'Test task',
    'status': 'todo',
  });

  // Stop when done
  await service.stop();
}
```

### Testing Integration with Apps

1. Start the database service
2. Run your app (Guidance, Knowledge, or Tracking)
3. The app will automatically connect to the running service

```bash
# Terminal 1: Start database
./scripts/start-db.sh

# Terminal 2: Run app
cd apps/altair_guidance
flutter run
```

## Testing

### Unit Tests

Test individual components without requiring a running database:

```bash
flutter test
```

### Integration Tests

Test with actual SurrealDB connection:

```bash
# Start SurrealDB first
./scripts/start-db.sh

# Run integration tests
cd packages/altair-core
flutter test test/repositories/task_repository_integration_test.dart
```

## Platform-Specific Development

### Linux (systemd)

Install as user service for testing:

```bash
dart run bin/install_service.dart
systemctl --user status altair-db
```

Logs:

```bash
journalctl --user -u altair-db -f
```

### macOS (launchd)

Install as launch agent:

```bash
dart run bin/install_service.dart
launchctl list | grep altair
```

Logs:

```bash
tail -f ~/Library/Application\ Support/altair/logs/stdout.log
```

### Windows

Run as process (service installation requires admin):

```bash
dart run bin/start_service.dart
```

## Troubleshooting

### Port Already in Use

If port 8000 is taken:

```bash
# Find process using port
lsof -i :8000  # Linux/macOS
netstat -ano | findstr :8000  # Windows

# Kill process
pkill surreal  # Linux/macOS
```

### Connection Refused

1. Check if SurrealDB is running:

   ```bash
   curl http://localhost:8000/health
   ```

2. Check logs for errors
3. Verify credentials match (default: altair/altair-local-dev)

### Schema Initialization Fails

The connection manager automatically initializes the schema. If it fails:

1. Check SurrealDB logs
2. Verify you have write permissions to data directory
3. Try manual schema initialization:

   ```bash
   surreal sql --conn http://localhost:8000 \
     --user altair --pass altair-local-dev \
     --ns altair --db local

   # Then paste schema from connection_manager.dart
   ```

### Tests Failing

1. Ensure SurrealDB is running
2. Check no other services are using port 8000
3. Verify database is clean:

   ```bash
   rm -rf altair.db
   ./scripts/start-db.sh
   ```

## Project Structure

```
packages/altair-db-service/
├── lib/
│   ├── altair_db_service.dart          # Main exports
│   └── src/
│       ├── service_manager.dart         # Service lifecycle
│       ├── connection_manager.dart      # DB connections
│       ├── config.dart                  # Configuration
│       ├── queries.dart                 # Cross-app queries
│       ├── models/
│       │   └── service_status.dart
│       └── platform/
│           ├── service_installer.dart   # Platform abstraction
│           ├── linux_installer.dart
│           ├── macos_installer.dart
│           └── windows_installer.dart
├── test/
│   ├── config_test.dart
│   ├── service_status_test.dart
│   └── connection_manager_test.dart
└── scripts/
    └── start-db.sh                      # Dev database script
```

## Contributing

When adding features to altair-db-service:

1. **Add tests** - All new code must have tests
2. **Update docs** - Update this file and the main README
3. **Test on multiple platforms** - Verify Linux, macOS, Windows if possible
4. **Check schema** - Update schema initialization if adding new tables
5. **Verify migrations** - Ensure changes don't break existing data

## Additional Resources

- **SurrealDB Docs**: <https://surrealdb.com/docs>
- **Design Docs**: See `docs/database/` in repo root
- **Schema Design**: `docs/database/altair-db-schema-and-development.md`
- **Integration Examples**: `docs/database/altair-guidance-integration-example.md`

## Getting Help

- Check existing issues: <https://github.com/getaltair/altair/issues>
- SurrealDB Discord: <https://discord.gg/surrealdb>
- Project discussions: <https://github.com/getaltair/altair/discussions>
