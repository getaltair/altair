# Feature SHARED-002: gRPC Cross-App Communication

## What it does

Enables real-time communication between Altair applications (Guidance, Knowledge, Tracking) via gRPC services, allowing data sharing, event notifications, and coordinated actions across the ecosystem.

## User Journey

GIVEN user has multiple Altair apps installed and running
WHEN user creates a quest in Guidance that references items from Tracking
THEN apps communicate via gRPC to link data and synchronize states automatically

## Functional Requirements

- gRPC server embedded in each application
- Service discovery via local ports
- Protocol buffer definitions for all messages
- Bidirectional streaming for real-time updates
- Request/response for queries
- Event broadcast system
- Service health checking
- Automatic reconnection logic
- Message queuing for offline apps
- Version compatibility checking
- Rate limiting and backpressure
- Encryption for local communication

## UI/UX Requirements

### Components

- `ServiceStatusBar` - Shows connected services
- `ConnectionToast` - Service discovery notifications
- `SyncIndicator` - Cross-app sync status
- `ServiceDebugPanel` - gRPC debug interface
- `PermissionDialog` - Cross-app data sharing consent
- `ConflictResolver` - Handle sync conflicts
- `MessageQueue` - Pending operations display
- `ServiceHealth` - Health check visualization
- `VersionMismatch` - Compatibility warnings
- `RateLimitIndicator` - Throttling feedback

### Visual Design

- **Layout:**
  - Status bar: Top-right, 150px width
  - Service dots: 8px diameter circles
  - Debug panel: Bottom drawer, 300px height
  - Permission modal: 500px centered
- **Colors:**
  - Connected: `#4CAF50` (Green)
  - Disconnected: `#9E9E9E` (Gray)
  - Syncing: `#2196F3` (Blue pulse)
  - Error: `#F44336` (Red)
  - Rate limited: `#FF9800` (Orange)
- **Typography:**
  - Service names: 12px medium
  - Status text: 11px regular
  - Debug logs: 10px monospace
- **Iconography:**
  - Link: Chain icon for connected
  - Broken link: Disconnected
  - Sync: Rotating arrows
  - Message: Envelope icon
- **Borders/Shadows:**
  - Service dots: 1px border
  - Active sync: Pulsing glow

### User Interactions

- **Input Methods:**
  - Hover for service details
  - Click to open debug panel
  - Toggle service connections
- **Keyboard Shortcuts:**
  - `Ctrl+Shift+G`: gRPC debug panel
  - `Ctrl+Shift+R`: Reconnect all
- **Gestures:**
  - Swipe up for debug panel
  - Long-press for service info
- **Feedback:**
  - Connection sound effects
  - Haptic on sync complete
  - Toast notifications

### State Management

- **Local State:**
  - Connection statuses
  - Message queue
  - Debug log buffer
- **Global State:**
  ```dart
  final grpcServerProvider = Provider<GrpcServer>
  final connectedServicesProvider = StateProvider<Map<String, ServiceConnection>>
  final messageQueueProvider = StateNotifierProvider<MessageQueueNotifier, List<PendingMessage>>
  final syncStatusProvider = StreamProvider<SyncStatus>
  final serviceHealthProvider = StreamProvider<Map<String, HealthStatus>>
  ```
- **Persistence:**
  - Service discovery cache
  - Message queue on disk
  - Connection preferences
  - Debug logs (optional)

### Responsive Behavior

- **Desktop:** Full debug panel, all indicators
- **Tablet:** Compact status, slide-out debug
- **Mobile:** Minimal indicator, no debug
- **Breakpoint Strategy:** Progressive feature reduction

### Accessibility Requirements

- **Screen Reader:**
  - Announce service connections
  - Describe sync operations
  - Read error messages
- **Keyboard Navigation:**
  - Tab through services
  - Enter to reconnect
- **Color Contrast:** Icons with text labels
- **Motion:** Static status option
- **Font Sizing:** Scalable debug text

### ADHD-Specific UI Requirements

- **Cognitive Load:**
  - Auto-connect by default
  - Hide unless problem
  - Simple status only
- **Focus Management:**
  - Non-intrusive notifications
  - Background operations
  - Silent reconnections
- **Forgiveness:**
  - Auto-retry failures
  - Queue when offline
  - No data loss
- **Visual Hierarchy:**
  - Green = working
  - Red = attention needed
  - Minimal UI presence
- **Immediate Feedback:**
  - Instant status updates
  - Quick sync confirmation
  - Fast reconnection

## Non-Functional Requirements

### Performance Targets

- Service discovery <100ms
- Message latency <10ms local
- Throughput >10,000 msg/s
- Reconnection <500ms
- Queue persistence <50ms

### Technical Constraints

- gRPC with Protocol Buffers 3
- Local ports 50051-50053
- TLS for encryption
- HTTP/2 transport
- Async/await patterns

### Security Requirements

- mTLS for local communication
- Service authentication tokens
- Message integrity checks
- Rate limiting per service
- Audit logging optional

## Implementation Details

### Code Structure

```
lib/
├── core/
│   └── grpc/
│       ├── grpc_server.dart
│       ├── grpc_client.dart
│       ├── service_discovery.dart
│       ├── message_queue.dart
│       ├── generated/
│       │   ├── guidance.pb.dart
│       │   ├── knowledge.pb.dart
│       │   ├── tracking.pb.dart
│       │   └── shared.pb.dart
│       ├── providers/
│       │   ├── grpc_provider.dart
│       │   ├── connection_provider.dart
│       │   └── sync_provider.dart
│       └── services/
│           ├── guidance_service.dart
│           ├── knowledge_service.dart
│           └── tracking_service.dart

backend/
├── src/
│   ├── grpc/
│   │   ├── mod.rs
│   │   ├── server.rs
│   │   ├── client.rs
│   │   └── discovery.rs
│   └── services/
│       ├── guidance_grpc.rs
│       ├── knowledge_grpc.rs
│       └── tracking_grpc.rs
└── proto/
    ├── guidance.proto
    ├── knowledge.proto
    ├── tracking.proto
    └── shared.proto
```

### Key Files to Create

- `grpc_server.dart` - gRPC server implementation
- `service_discovery.dart` - Auto-discovery logic
- `message_queue.dart` - Offline queue management
- `shared.proto` - Common message definitions
- `grpc_server.rs` - Rust gRPC server

### Protocol Buffer Definitions

```protobuf
// shared.proto
syntax = "proto3";

package altair.shared;

service CrossAppSync {
  rpc Subscribe(SubscribeRequest) returns (stream Event);
  rpc Publish(Event) returns (PublishResponse);
  rpc Query(QueryRequest) returns (QueryResponse);
  rpc HealthCheck(Empty) returns (HealthStatus);
}

message Event {
  string id = 1;
  string source_app = 2;
  string event_type = 3;
  google.protobuf.Any payload = 4;
  int64 timestamp = 5;
}

message SubscribeRequest {
  repeated string event_types = 1;
  string subscriber_id = 2;
}

message QueryRequest {
  string query_type = 1;
  map<string, string> parameters = 2;
}

// guidance.proto
syntax = "proto3";

package altair.guidance;
import "shared.proto";

service GuidanceService {
  rpc CreateQuest(CreateQuestRequest) returns (Quest);
  rpc UpdateQuestStatus(UpdateStatusRequest) returns (Quest);
  rpc GetActiveQuests(Empty) returns (QuestList);
  rpc LinkToInventory(LinkInventoryRequest) returns (LinkResponse);
}

message Quest {
  string id = 1;
  string title = 2;
  int32 energy_required = 3;
  string status = 4;
  repeated string linked_items = 5;
}
```

### Dependencies

```yaml
dependencies:
  grpc: ^3.2.0
  protobuf: ^3.0.0
  flutter_riverpod: ^2.4.0
  connectivity_plus: ^4.0.0
  
dev_dependencies:
  protoc_plugin: ^20.0.0
  build_runner: ^2.4.0
```

### Rust Dependencies

```toml
[dependencies]
tonic = "0.12"
prost = "0.13"
tokio = { version = "1", features = ["full"] }
tower = "0.4"
hyper = "1.0"
tonic-health = "0.12"
tracing = "0.1"

[build-dependencies]
tonic-build = "0.12"
```

## Testing Requirements

### Unit Tests

- [ ] Service discovery logic
- [ ] Message serialization
- [ ] Queue management
- [ ] Reconnection logic
- [ ] Rate limiting

### Widget Tests

- [ ] Status indicator updates
- [ ] Connection dialog
- [ ] Debug panel display
- [ ] Error notifications

### Integration Tests

- [ ] Multi-app communication
- [ ] Message delivery guarantee
- [ ] Offline queue sync
- [ ] Service health checks
- [ ] Version compatibility

### Accessibility Tests

- [ ] Status announcements
- [ ] Keyboard control
- [ ] Screen reader support

## Definition of Done

- [ ] gRPC servers running
- [ ] Service discovery working
- [ ] Message passing functional
- [ ] Event streaming active
- [ ] Health checks operational
- [ ] Queue persistence working
- [ ] Reconnection automatic
- [ ] All proto files generated
- [ ] Tests passing
- [ ] Performance metrics met
