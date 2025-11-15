# Feature SHARED-001: SurrealDB Local Database Setup

## What it does

Establishes local-first SurrealDB instance with multi-model support (graph + document + vector), handles schema management, data persistence, and provides unified data access layer for all Altair applications.

## User Journey

GIVEN user installs any Altair application
WHEN application first launches
THEN local SurrealDB instance is initialized with proper schema and ready for data operations

## Functional Requirements

- Automatic SurrealDB embedded instance creation
- Schema migration system with versioning
- Multi-namespace support (guidance, knowledge, tracking)
- Graph relationships between entities
- Vector embeddings storage for RAG
- Document storage for flexible data
- Real-time change notifications
- Backup/restore functionality
- Data export in multiple formats
- Query performance optimization
- Connection pooling management
- Transaction support with rollback

## UI/UX Requirements

### Components

- `DatabaseInitializer` - First-run setup component
- `MigrationProgress` - Schema update indicator
- `BackupManager` - Manual backup interface
- `DataExporter` - Export format selector
- `ConnectionIndicator` - DB status widget
- `StorageMonitor` - Disk usage display
- `QueryDebugger` - Development tool (debug only)
- `SchemaViewer` - Database structure browser
- `HealthCheck` - Connection status monitor
- `RepairWizard` - Database recovery tool

### Visual Design

- **Layout:**
  - Status bar indicator: 24px square
  - Migration modal: 400px width centered
  - Backup dialog: 500px with progress bar
  - Debug panel: Collapsible drawer
- **Colors:**
  - Connected: `#4CAF50` (Green)
  - Syncing: `#2196F3` (Blue)
  - Error: `#F44336` (Red)
  - Warning: `#FF9800` (Orange)
- **Typography:**
  - Status: 12px regular
  - Progress: 14px medium
  - Errors: 14px bold
- **Iconography:**
  - Database: Cylinder icon (16x16)
  - Sync: Rotating arrows
  - Backup: Download arrow
  - Error: Warning triangle
- **Borders/Shadows:**
  - Status indicator: 1px border
  - Modal: 4px solid black shadow

### User Interactions

- **Input Methods:**
  - Click for backup dialog
  - Select export format
  - Choose backup location
- **Keyboard Shortcuts:**
  - `Ctrl+Shift+B`: Backup now
  - `Ctrl+Shift+D`: Open debug panel
- **Gestures:**
  - Long-press status for details
  - Swipe to dismiss notifications
- **Feedback:**
  - Progress bars for operations
  - Success/error toasts
  - Sound on backup complete

### State Management

- **Local State:**
  - Connection status
  - Current operation
  - Progress percentage
- **Global State:**
  ```dart
  final databaseProvider = Provider<SurrealDB>((ref) => SurrealDB.instance)
  final connectionStatusProvider = StreamProvider<ConnectionStatus>
  final migrationProvider = StateNotifierProvider<MigrationNotifier, MigrationState>
  final backupProvider = StateNotifierProvider<BackupNotifier, BackupState>
  final storageProvider = StreamProvider<StorageStats>
  ```
- **Persistence:**
  - Database files in app directory
  - Migration history tracked
  - Backup schedule saved
  - Connection settings cached

### Responsive Behavior

- **Desktop:** Full debug panel available
- **Tablet:** Simplified status display
- **Mobile:** Minimal indicator only
- **Breakpoint Strategy:** Hide advanced features on mobile

### Accessibility Requirements

- **Screen Reader:**
  - Announce connection changes
  - Describe migration progress
  - Read backup status
- **Keyboard Navigation:**
  - Tab through backup options
  - Enter to confirm actions
- **Color Contrast:** Status icons with text labels
- **Motion:** Static indicators option
- **Font Sizing:** Adjustable debug text

### ADHD-Specific UI Requirements

- **Cognitive Load:**
  - Automatic everything by default
  - Hide technical details
  - One-click backup
- **Focus Management:**
  - Non-blocking operations
  - Background migrations
  - Silent reconnection
- **Forgiveness:**
  - Auto-recovery from errors
  - Backup reminders only
  - No data loss on crash
- **Visual Hierarchy:**
  - Green = good, ignore
  - Red = needs attention
  - Simple status only
- **Immediate Feedback:**
  - Instant connection status
  - Live progress updates
  - Quick operation completion

## Non-Functional Requirements

### Performance Targets

- Database init <2s
- Query response <50ms (indexed)
- Migration per table <500ms
- Backup speed >10MB/s
- Connection pool: 10 concurrent

### Technical Constraints

- SurrealDB 2.0+ embedded mode
- RocksDB storage engine
- 100GB max database size
- UTF-8 encoding throughout
- ACID compliance required

### Security Requirements

- Local data encryption at rest
- No network access by default
- Query parameterization enforced
- Backup encryption optional
- User data isolation

## Implementation Details

### Code Structure

```
lib/
├── core/
│   └── database/
│       ├── surreal_db_setup.dart
│       ├── connection_manager.dart
│       ├── migration_system.dart
│       ├── backup_service.dart
│       ├── schema/
│       │   ├── guidance_schema.dart
│       │   ├── knowledge_schema.dart
│       │   └── tracking_schema.dart
│       ├── providers/
│       │   ├── database_provider.dart
│       │   ├── connection_provider.dart
│       │   └── migration_provider.dart
│       └── repositories/
│           └── base_repository.dart

backend/
├── src/
│   ├── database/
│   │   ├── mod.rs
│   │   ├── connection.rs
│   │   ├── migrations.rs
│   │   └── schema.rs
│   └── services/
│       └── database_service.rs
└── migrations/
    ├── 001_initial_schema.surql
    ├── 002_add_vectors.surql
    └── 003_add_indexes.surql
```

### Key Files to Create

- `surreal_db_setup.dart` - Database initialization
- `connection_manager.dart` - Connection pooling
- `migration_system.dart` - Schema migrations
- `database_service.rs` - Rust database service
- `schema.surql` - SurrealQL schema definitions

### Dependencies

```yaml
dependencies:
  surrealdb_flutter: ^0.8.0
  path_provider: ^2.1.0
  path: ^1.8.0
  drift: ^2.12.0
  drift_surrealdb: ^0.1.0
  
dev_dependencies:
  drift_dev: ^2.12.0
  build_runner: ^2.4.0
```

### Rust Dependencies

```toml
[dependencies]
surrealdb = { version = "2.0", features = ["embedded", "rocksdb"] }
tokio = { version = "1", features = ["full"] }
axum = "0.7"
tonic = "0.12"
serde = { version = "1.0", features = ["derive"] }
anyhow = "1.0"
tracing = "0.1"
```

### SurrealQL Schema

```sql
-- Namespaces
DEFINE NAMESPACE guidance;
DEFINE NAMESPACE knowledge;
DEFINE NAMESPACE tracking;

-- Guidance tables
DEFINE TABLE guidance:epic SCHEMAFULL;
DEFINE FIELD title ON guidance:epic TYPE string;
DEFINE FIELD description ON guidance:epic TYPE option<string>;
DEFINE FIELD energy ON guidance:epic TYPE int DEFAULT 3;
DEFINE FIELD status ON guidance:epic TYPE string DEFAULT 'idea_greenhouse';
DEFINE FIELD created_at ON guidance:epic TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON guidance:epic TYPE datetime DEFAULT time::now();
DEFINE INDEX idx_epic_status ON guidance:epic COLUMNS status;

DEFINE TABLE guidance:quest SCHEMAFULL;
DEFINE FIELD title ON guidance:quest TYPE string;
DEFINE FIELD epic ON guidance:quest TYPE option<record<guidance:epic>>;
DEFINE FIELD energy ON guidance:quest TYPE int DEFAULT 3;
DEFINE FIELD column ON guidance:quest TYPE string;
DEFINE FIELD position ON guidance:quest TYPE int;
DEFINE FIELD xp_value ON guidance:quest TYPE int DEFAULT 10;

-- Relationships
DEFINE TABLE contains SCHEMAFULL;
DEFINE FIELD in ON contains TYPE record<guidance:epic>;
DEFINE FIELD out ON contains TYPE record<guidance:quest>;
DEFINE INDEX idx_contains ON contains COLUMNS in, out UNIQUE;
```

## Testing Requirements

### Unit Tests

- [ ] Connection establishment
- [ ] Schema migration logic
- [ ] Query building
- [ ] Transaction rollback
- [ ] Backup/restore cycle

### Widget Tests

- [ ] Status indicator states
- [ ] Migration progress display
- [ ] Backup dialog interaction
- [ ] Error message display

### Integration Tests

- [ ] Full database initialization
- [ ] Multi-namespace operations
- [ ] Concurrent connections
- [ ] Large data operations
- [ ] Recovery from corruption

### Accessibility Tests

- [ ] Status announcements
- [ ] Keyboard navigation
- [ ] Screen reader compatibility

## Definition of Done

- [ ] SurrealDB embedded working
- [ ] Schema migrations functional
- [ ] All namespaces created
- [ ] Backup/restore operational
- [ ] Connection pooling active
- [ ] Performance targets met
- [ ] Error recovery working
- [ ] Tests passing
- [ ] Documentation complete
