# Altair Database Service

> **⚠️ EXPERIMENTAL:** This package is under active development and not yet production-ready. It provides infrastructure for future cross-app data integration but currently has known limitations. See [Limitations](&#x26A0;&#xFE0F; limitations) section below.

**TL;DR:** Shared local SurrealDB instance that all Altair apps connect to. One database, three apps, seamless integration.

## 🎯 What This Does

**Single SurrealDB process runs on user's device:**

- All Altair apps (Guidance, Knowledge, Tracking) connect to it
- Cross-app data integration works automatically
- User installs it once, all apps benefit

```
User's Device
    │
    ├─ SurrealDB Service (localhost:8000)
    │       ↑         ↑         ↑
    │       │         │         │
    ├─ Guidance   Knowledge  Tracking
    │
    └─ Shared data, seamless linking
```

---

## 📦 Package Structure

```
altair_db_service/
├── lib/
│   ├── altair_db_service.dart          # Main export
│   └── src/
│       ├── service_manager.dart         # Core service lifecycle
│       ├── connection.dart              # Connection handling
│       ├── config.dart                  # Configuration model
│       ├── models/
│       │   └── service_status.dart      # Status enum
│       └── platform/
│           ├── service_installer.dart   # Platform abstraction
│           ├── linux_installer.dart     # systemd service
│           ├── macos_installer.dart     # launchd service
│           └── windows_installer.dart   # Windows service
├── bin/
│   ├── linux/                           # SurrealDB binary for Linux
│   ├── macos/                           # SurrealDB binary for macOS
│   └── windows/                         # SurrealDB binary for Windows
├── systemd/
│   └── altair-db.service                # Linux service definition
└── test/
    └── service_manager_test.dart
```

---

## 🚀 Quick Start

### Installation in App

```dart
// In your app's main.dart
import 'package:altair_db_service/altair_db_service.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // Initialize database service
  final service = AltairDatabaseService();

  // Check if running, start if needed
  if (!await service.isRunning()) {
    await service.start();
  }

  // Get connection
  final db = await service.getConnection();

  runApp(MyApp(db: db));
}
```

### Cross-App Queries

```dart
// In Guidance: Create task
await db.create('task', {
  'id': 'task:setup-dev',
  'title': 'Setup dev environment',
  'status': 'in_progress',
});

// In Knowledge: Link note to task
await db.create('note', {
  'id': 'note:meeting-2025-01-15',
  'title': 'Sprint Planning',
  'linked_tasks': ['task:setup-dev'],
});

// In Tracking: Link item to task
await db.create('item', {
  'id': 'item:laptop-001',
  'name': 'Development Laptop',
  'needed_for': ['task:setup-dev'],
});

// Query across apps
final results = await db.query('''
  SELECT * FROM task, note, item
  WHERE task.id = $taskId
  AND note.linked_tasks CONTAINS task.id
  AND item.needed_for CONTAINS task.id
''');
```

---

## 🔧 How It Works

### Service Lifecycle

1. **First app installed** → Install database service
2. **Service starts** → Binds to localhost:8000
3. **Apps connect** → WebSocket to local service
4. **All apps share data** → One database, multiple clients
5. **Optional sync** → Cloud backup when online

### Platform Behavior

| Platform | Implementation | Auto-Start |
|----------|---------------|------------|
| **Linux** | systemd service | ✅ Boot |
| **macOS** | launchd service | ✅ Boot |
| **Windows** | Windows Service | ✅ Boot |
| **Android** | Background Service | ⚠️ On-demand |
| **iOS** | Fallback to SQLite | N/A |

### Data Location

```
Linux:   ~/.local/share/altair/database/
macOS:   ~/Library/Application Support/altair/database/
Windows: %APPDATA%\altair\database\
Android: /data/data/com.getaltair.*/files/database/
```

---

## 🛠️ Development

### Running Tests

```bash
flutter test
```

### Installing Service Manually

```bash
# Linux
./install-linux.sh

# macOS
./install-macos.sh

# Windows
./install-windows.bat
```

### Checking Service Status

```bash
# Linux
systemctl --user status altair-db

# macOS
launchctl list | grep altair

# Windows
sc query AltairDB
```

---

## 🔐 Security

**Local-only by default:**

- Binds to 127.0.0.1 (localhost only)
- No external network access
- User credentials stored securely in system keychain

**Optional cloud sync:**

- User must explicitly enable
- End-to-end encryption
- User controls sync frequency

---

## 📝 Configuration

Default config at `~/.altair/config.toml`:

```toml
[service]
port = 8000
bind_address = "127.0.0.1"
auto_start = true

[database]
namespace = "altair"
database = "local"
data_directory = "~/.local/share/altair/database"

[sync]
enabled = false
cloud_url = ""
sync_interval = 300  # seconds
```

---

## ⚠️ Troubleshooting

### Service won't start

```bash
# Check if port is in use
lsof -i :8000  # Linux/macOS
netstat -ano | findstr :8000  # Windows

# Check logs
journalctl --user -u altair-db  # Linux
~/Library/Logs/altair-db.log    # macOS
```

### Apps can't connect

1. Verify service is running: `curl http://localhost:8000/health`
2. Check firewall isn't blocking localhost
3. Ensure no other SurrealDB instance on port 8000

### Performance issues

- Check database size: `du -sh ~/.local/share/altair/database`
- Consider vacuum/optimize
- Increase memory limit in config

---

## 🤝 Contributing

See main [CONTRIBUTING.md](../../CONTRIBUTING.md) in repo root.

---

## 📄 License

MIT License - see [LICENSE](../../LICENSE)

---

**Last updated:** October 26, 2025

## ⚠️ Limitations

**Current Status: Experimental - Not Production Ready**

### Known Issues

1. **No Data Migration**: Existing SQLite data is not automatically migrated
2. **Security**: Password generation needs strengthening (tracked in issue #TBD)
3. **Platform Support**: Mobile platforms (Android/iOS) not fully implemented
4. **Error Handling**: Limited graceful degradation if service fails to start
5. **Testing**: Integration tests require manual SurrealDB setup

### Roadmap to Production

- [ ] Implement SQLite → SurrealDB migration tool
- [ ] Strengthen password generation with `Random.secure()`
- [ ] Add graceful fallback to SQLite if service unavailable
- [ ] Complete Android background service implementation
- [ ] Add comprehensive integration tests
- [ ] Implement automatic binary download/installation

### Use Cases

**✅ Recommended For:**

- Development and testing of cross-app features
- Prototype applications
- Internal tools

**❌ Not Recommended For:**

- Production deployments with existing user data
- Apps requiring 100% uptime without external dependencies
- Mobile-first applications (iOS limitations)

---
