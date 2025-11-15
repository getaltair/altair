# Feature AT-004: Maintenance and Warranties

## What it does

Tracks warranty information, maintenance schedules, and service history for inventory items. Automatically sends reminders for upcoming maintenance, warranty expirations, and calibration needs. Maintains complete audit trail of all service activities with cost tracking and vendor information.

## User Journey

**Scenario 1: Adding Warranty Information**
```
GIVEN user has a new oscilloscope in inventory
WHEN user adds warranty details (expiry date, vendor, documentation)
THEN system stores warranty info and schedules expiration reminder 30 days before
AND displays warranty status badge on item card
```

**Scenario 2: Scheduling Maintenance**
```
GIVEN user has a 3D printer requiring monthly maintenance
WHEN user creates recurring maintenance schedule
THEN system creates maintenance tasks with configurable intervals
AND sends notifications when maintenance is due
AND tracks completion history
```

**Scenario 3: Logging Service**
```
GIVEN user completed maintenance on a tool
WHEN user logs the service with notes, cost, and photos
THEN system records service history entry
AND updates next maintenance date automatically
AND calculates total cost of ownership
```

## Functional Requirements

### FR-1: Warranty Management
- Store warranty information: purchase date, expiry date, vendor, coverage details
- Support warranty documents (PDFs, images) stored via SHARED-005
- Calculate remaining warranty period dynamically
- Track warranty claims with status (submitted, approved, completed)
- Support extended warranties with separate tracking
- Bulk warranty import from CSV/spreadsheet

### FR-2: Maintenance Scheduling
- Create one-time maintenance tasks
- Define recurring maintenance schedules (daily, weekly, monthly, yearly, custom)
- Set maintenance categories (cleaning, calibration, inspection, repair)
- Assign priority levels (routine, important, critical)
- Track estimated time and actual time spent
- Support maintenance checklists with sub-tasks
- Snooze/postpone maintenance without shame (ADHD-friendly)

### FR-3: Service History
- Log completed maintenance with notes, cost, parts used
- Attach photos of work performed
- Track service provider/vendor information
- Record downtime periods
- Calculate mean time between failures (MTBF)
- Export service history as PDF report

### FR-4: Cost Tracking
- Track maintenance costs (labor, parts, outsourced)
- Calculate total cost of ownership (TCO)
- Budget alerts when costs exceed thresholds
- Generate cost reports by item, category, time period
- Track parts inventory consumption

### FR-5: Reminder System
- Warranty expiration reminders (30, 14, 7 days before)
- Maintenance due notifications
- Overdue maintenance escalation
- Configurable notification preferences (desktop, mobile, email)
- Respect user's notification preferences (ADHD-friendly)

### FR-6: Compliance & Calibration
- Track calibration schedules for measurement equipment
- Store calibration certificates
- Compliance status indicators
- Integration with quality management systems (future)

## UI/UX Requirements

### Components

**Flutter Widgets to Create:**
- `MaintenanceScheduleCard` - Display scheduled maintenance
- `WarrantyStatusBadge` - Visual warranty status indicator
- `ServiceHistoryTimeline` - Chronological service log
- `MaintenanceChecklistWidget` - Interactive checklist
- `CostTrackingChart` - Visual cost breakdown
- `ReminderSettingsPanel` - Notification configuration
- `CalibrationStatusIndicator` - Compliance badge
- `WarrantyDocumentViewer` - PDF/image viewer

**Design System Components:**
- `PrimaryButton` (neo-brutalist style)
- `SecondaryButton`
- `IconButton` with tooltips
- `DatePicker` with visual calendar
- `TimePicker`
- `Dropdown` for categories
- `TextField` with validation
- `Checkbox` for checklists
- `FileUploadButton`
- `TagChip` for categories

### Visual Design

**Layout:**
- Main view: 3-column layout (desktop) - Items list | Details panel | Timeline
- Tablet: 2-column layout - Items list | Details panel (timeline in tab)
- Mobile: Single column with bottom sheet details
- Grid spacing: 16px between elements, 24px between sections
- Card-based design with 2px solid borders (neo-brutalist)

**Colors:**
- Warranty status: Green (#10B981) active, Yellow (#F59E0B) expiring soon, Red (#EF4444) expired, Gray (#6B7280) no warranty
- Maintenance priority: Blue (#3B82F6) routine, Orange (#F97316) important, Red (#DC2626) critical
- Service type: Purple (#8B5CF6) repair, Teal (#14B8A6) maintenance, Indigo (#6366F1) calibration
- Background: White (#FFFFFF) cards, Light gray (#F3F4F6) page background
- Text: Dark gray (#111827) primary, Medium gray (#6B7280) secondary
- Borders: Black (#000000) for neo-brutalist borders

**Typography:**
- Headings: Inter Bold, 24px (h1), 20px (h2), 16px (h3)
- Body text: Inter Regular, 14px
- Labels: Inter Medium, 12px, uppercase
- Code/dates: JetBrains Mono, 14px
- Line height: 1.5 for body, 1.2 for headings

**Iconography:**
- Warranty: `shield_checkmark` (24px)
- Maintenance: `wrench` (24px)
- Calendar: `calendar` (20px)
- Cost: `currency_dollar` (20px)
- Alert: `bell` (20px)
- History: `clock_arrow_circlepath` (20px)
- Document: `doc_text` (20px)
- Photo: `camera` (20px)

**Borders/Shadows:**
- Neo-brutalist: 2px solid black borders on all cards
- Offset shadow: 4px right, 4px down, black
- No rounded corners (square corners for neo-brutalist aesthetic)
- Hover state: Translate card (-2px, -2px) with shadow adjustment

### User Interactions

**Input Methods:**
- **Click:** Select maintenance item, open details
- **Double-click:** Quick edit mode
- **Right-click:** Context menu (snooze, complete, edit, delete)
- **Drag-and-drop:** Reschedule maintenance by dragging to calendar
- **Voice:** Quick log maintenance via voice note (future)

**Keyboard Shortcuts:**
- `Ctrl+N` - New maintenance schedule
- `Ctrl+W` - Add warranty
- `Ctrl+L` - Log service
- `Space` - Toggle maintenance complete
- `Escape` - Close dialogs
- `F2` - Quick edit selected item
- `Ctrl+F` - Search maintenance/warranty

**Gestures (Mobile):**
- Swipe right: Mark maintenance complete
- Swipe left: Snooze maintenance
- Long-press: Open context menu
- Pull to refresh: Sync maintenance schedules

**Feedback:**
- Loading states: Skeleton loaders for maintenance cards
- Success: Green checkmark toast + haptic feedback (50ms)
- Error: Red banner with retry button + error sound
- Completion: Celebration animation for service logged (confetti, <1s)
- Reminder: Desktop notification + app badge count
- Animation timing: <300ms for all transitions

### State Management

**Local State (Widget-level):**
- Form input states (maintenance form, warranty form)
- Checklist item completion states
- File upload progress
- Photo gallery viewer state
- Calendar date selection

**Global State (Riverpod Providers):**
```dart
// State providers
final maintenanceSchedulesProvider = StateNotifierProvider<MaintenanceSchedulesNotifier, List<MaintenanceSchedule>>;
final warrantyProvider = StateNotifierProvider<WarrantyNotifier, List<Warranty>>;
final serviceHistoryProvider = StateNotifierProvider<ServiceHistoryNotifier, List<ServiceLog>>;
final maintenanceFiltersProvider = StateProvider<MaintenanceFilters>;
final reminderSettingsProvider = StateProvider<ReminderSettings>;

// Computed providers
final upcomingMaintenanceProvider = Provider<List<MaintenanceSchedule>>;
final overdueMaintenanceProvider = Provider<List<MaintenanceSchedule>>;
final expiringWarrantiesProvider = Provider<List<Warranty>>;
final totalMaintenanceCostProvider = Provider<double>;

// Async providers
final maintenanceScheduleByIdProvider = FutureProvider.family<MaintenanceSchedule, String>;
final serviceHistoryByItemProvider = FutureProvider.family<List<ServiceLog>, String>;
```

**Persistence:**
- Auto-save form drafts every 30 seconds to local SurrealDB
- Sync completed maintenance to backend immediately
- Cache warranty documents locally with background sync
- Store reminder preferences in user settings
- Offline queue for service logs (sync when online)

### Responsive Behavior

**Desktop (>1200px):**
- 3-column layout: Items sidebar (300px) | Details (flex) | Timeline (350px)
- Calendar view available with drag-and-drop rescheduling
- Hover effects on all interactive elements
- Context menus on right-click
- Keyboard shortcuts fully enabled

**Tablet (768-1199px):**
- 2-column layout: Items list (400px) | Details panel (flex)
- Timeline accessible via tabs within details panel
- Touch-optimized controls (larger tap targets, 48px minimum)
- Swipe gestures for quick actions

**Mobile (<768px):**
- Single column with card list
- Bottom sheet for item details
- Floating action button (FAB) for quick actions
- Simplified forms with progressive disclosure
- Tab navigation for different views (Scheduled, History, Warranties)

**Breakpoint Strategy:**
- Mobile-first approach with progressive enhancement
- Use Flutter's `LayoutBuilder` for responsive widgets
- Shared logic across breakpoints, different layouts only
- Test on 4 device sizes: Mobile (375px), Tablet (768px), Laptop (1366px), Desktop (1920px)

### Accessibility Requirements

**Screen Reader:**
- Semantic labels for all maintenance items: "Maintenance: Oscilloscope calibration, due in 3 days"
- Live regions for notifications and updates
- Table semantics for service history
- Progress indicators for multi-step forms
- Group related controls (ARIA grouping)

**Keyboard Navigation:**
- Full keyboard navigation without mouse
- Clear focus indicators (2px outline, high contrast)
- Tab order: List → Details → Actions → Timeline
- Skip links for power users ("Skip to maintenance due today")
- Focus management when modals open/close

**Color Contrast:**
- WCAG 2.1 AA compliance (4.5:1 for normal text, 3:1 for large text)
- Status colors combined with icons (not color alone)
- High contrast mode support
- Test with contrast checker tools

**Motion:**
- Respect `prefers-reduced-motion` media query
- Disable celebration animations if user preference set
- Fade transitions instead of slides when motion reduced
- Critical animations only (<200ms, no bounce/spring)

**Font Sizing:**
- Scalable text (use `em` and `rem` units)
- Minimum font size: 14px
- Support OS-level text scaling (up to 200%)
- Preserve layout integrity at 200% zoom

### ADHD-Specific UI Requirements

**Cognitive Load Reduction:**
- Show only upcoming and overdue maintenance by default
- Progressive disclosure: "Show completed" button for history
- Limit visible maintenance items to 10, load more on scroll
- Visual grouping: Today, This week, This month, Later
- Hide low-priority routine maintenance when overwhelmed (toggle)

**Focus Management:**
- Autofocus on most important overdue maintenance
- Highlight critical maintenance with pulsing animation (subtle, 2s interval)
- "Focus Mode": Show only critical items, hide everything else
- Visual timer for time-sensitive maintenance

**Forgiveness Features:**
- Snooze without guilt: "I'll do this later" button (snoozes to user-chosen time)
- No shame language: "Postponed" instead of "Overdue"
- Easy reset: "Start fresh" clears overdue count, reschedules intelligently
- Undo for any action (30 second window)
- Non-destructive operations: Archive instead of delete

**Visual Hierarchy:**
- Primary action always top-right: "Log Maintenance" or "Complete"
- Danger zone at bottom: Delete, archive
- Color coding for priority (with icons for accessibility)
- Bold for item names, regular for details
- Card elevation for active/selected items

**Immediate Feedback:**
- Instant visual confirmation (<100ms response time)
- Optimistic UI updates (assume success, rollback on error)
- Progress indicators for all async operations
- Haptic feedback on mobile (50ms vibration)
- Sound effects (optional, user-configurable): Success chime, error buzz

**Time Blindness Support:**
- "Due in X days" with visual progress bar
- Color-coded urgency: Green (>7 days), Yellow (3-7 days), Red (<3 days)
- Calendar integration with visual timeline
- Estimated time to complete each maintenance task
- Reminders at user-specified intervals (default: 3 days, 1 day, day-of)

## Non-Functional Requirements

### Performance Targets

- Maintenance list render time: <200ms for 100 items
- Service history load time: <500ms for 50 entries
- Calendar view render: <300ms for 30 days
- Form submission: <1s from click to confirmation
- Notification delivery: <5s from trigger
- Photo upload: <2s per image (with compression)
- Animation frame rate: 60fps for all transitions
- First paint: <1s on cold start

### Technical Constraints

**Flutter:**
- Flutter version: 3.16+
- Dart SDK: 3.2+
- Target platforms: Windows 10+, macOS 11+, Linux, Android 10+, iOS 14+

**Package Dependencies:**
```yaml
dependencies:
  flutter_riverpod: ^2.4.0
  riverpod_annotation: ^2.3.0
  drift: ^2.14.0
  grpc: ^3.2.0
  protobuf: ^3.1.0
  file_picker: ^6.1.0
  image_picker: ^1.0.4
  path_provider: ^2.1.1
  intl: ^0.18.1
  fl_chart: ^0.65.0  # For cost tracking charts
  table_calendar: ^3.0.9
  
dev_dependencies:
  riverpod_generator: ^2.3.0
  build_runner: ^2.4.0
  mockito: ^5.4.0
  flutter_test:
    sdk: flutter
```

**Rust Backend Dependencies:**
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1.35", features = ["full"] }
tonic = "0.11"
surrealdb = "2.0"
chrono = "0.4"
serde = { version = "1.0", features = ["derive"] }
uuid = { version = "1.6", features = ["v4"] }
```

### Security Requirements

**Data Protection:**
- Encrypt warranty documents at rest (AES-256)
- Sensitive vendor information redacted in logs
- User-specific access control for shared maintenance schedules
- Audit trail for all maintenance operations

**Input Validation:**
- Sanitize all user input (notes, vendor names)
- Validate dates (no dates in past for new warranties)
- File upload restrictions: Max 10MB per file, allowed types: PDF, JPG, PNG
- SQL injection prevention via parameterized queries

**Authentication:**
- Require authentication for all maintenance operations
- Session timeout after 30 minutes of inactivity
- Re-authentication for sensitive operations (delete, export)

## Implementation Details

### Code Structure

```
lib/
├── features/
│   └── maintenance/
│       ├── presentation/
│       │   ├── screens/
│       │   │   ├── maintenance_dashboard_screen.dart
│       │   │   ├── maintenance_schedule_screen.dart
│       │   │   ├── warranty_manager_screen.dart
│       │   │   └── service_history_screen.dart
│       │   ├── widgets/
│       │   │   ├── maintenance_schedule_card.dart
│       │   │   ├── warranty_status_badge.dart
│       │   │   ├── service_history_timeline.dart
│       │   │   ├── maintenance_checklist_widget.dart
│       │   │   ├── cost_tracking_chart.dart
│       │   │   ├── reminder_settings_panel.dart
│       │   │   └── calibration_status_indicator.dart
│       │   └── providers/
│       │       ├── maintenance_schedules_provider.dart
│       │       ├── warranty_provider.dart
│       │       ├── service_history_provider.dart
│       │       └── reminder_settings_provider.dart
│       ├── domain/
│       │   ├── models/
│       │   │   ├── maintenance_schedule.dart
│       │   │   ├── warranty.dart
│       │   │   ├── service_log.dart
│       │   │   └── reminder_settings.dart
│       │   ├── repositories/
│       │   │   ├── maintenance_repository.dart
│       │   │   └── warranty_repository.dart
│       │   └── use_cases/
│       │       ├── schedule_maintenance.dart
│       │       ├── log_service.dart
│       │       ├── send_reminders.dart
│       │       └── calculate_tco.dart
│       └── data/
│           ├── repositories/
│           │   ├── maintenance_repository_impl.dart
│           │   └── warranty_repository_impl.dart
│           ├── data_sources/
│           │   ├── maintenance_local_data_source.dart
│           │   └── maintenance_remote_data_source.dart
│           └── models/
│               ├── maintenance_schedule_dto.dart
│               └── warranty_dto.dart
```

### Key Files to Create

**Flutter Frontend:**
1. `maintenance_dashboard_screen.dart` - Main dashboard with upcoming maintenance
2. `maintenance_schedule_card.dart` - Reusable card component
3. `warranty_status_badge.dart` - Visual warranty indicator
4. `service_history_timeline.dart` - Chronological service log
5. `maintenance_schedules_provider.dart` - Riverpod state management
6. `maintenance_schedule.dart` - Domain model with freezed
7. `maintenance_repository.dart` - Repository interface
8. `maintenance_repository_impl.dart` - Repository implementation

**Rust Backend:**
9. `maintenance_service.rs` - Core business logic
10. `warranty_service.rs` - Warranty management
11. `reminder_service.rs` - Notification scheduling
12. `cost_tracking_service.rs` - TCO calculations

**gRPC Proto:**
13. `maintenance_service.proto` - Service definitions

### gRPC Service Definition

```protobuf
syntax = "proto3";

package altair.tracking.maintenance;

import "google/protobuf/timestamp.proto";

service MaintenanceService {
  rpc CreateMaintenanceSchedule(CreateMaintenanceScheduleRequest) returns (MaintenanceScheduleResponse);
  rpc UpdateMaintenanceSchedule(UpdateMaintenanceScheduleRequest) returns (MaintenanceScheduleResponse);
  rpc DeleteMaintenanceSchedule(DeleteMaintenanceScheduleRequest) returns (DeleteResponse);
  rpc GetMaintenanceSchedule(GetMaintenanceScheduleRequest) returns (MaintenanceScheduleResponse);
  rpc ListMaintenanceSchedules(ListMaintenanceSchedulesRequest) returns (ListMaintenanceSchedulesResponse);
  
  rpc LogService(LogServiceRequest) returns (ServiceLogResponse);
  rpc GetServiceHistory(GetServiceHistoryRequest) returns (GetServiceHistoryResponse);
  
  rpc CreateWarranty(CreateWarrantyRequest) returns (WarrantyResponse);
  rpc UpdateWarranty(UpdateWarrantyRequest) returns (WarrantyResponse);
  rpc GetWarranty(GetWarrantyRequest) returns (WarrantyResponse);
  rpc ListWarranties(ListWarrantiesRequest) returns (ListWarrantiesResponse);
  
  rpc CalculateTotalCostOfOwnership(CalculateTCORequest) returns (TCOResponse);
  rpc GetUpcomingMaintenance(GetUpcomingMaintenanceRequest) returns (ListMaintenanceSchedulesResponse);
  rpc GetOverdueMaintenance(GetOverdueMaintenanceRequest) returns (ListMaintenanceSchedulesResponse);
}

message MaintenanceSchedule {
  string id = 1;
  string item_id = 2;
  string name = 3;
  string description = 4;
  MaintenanceType type = 5;
  MaintenancePriority priority = 6;
  google.protobuf.Timestamp next_due_date = 7;
  RecurrenceRule recurrence = 8;
  int32 estimated_duration_minutes = 9;
  repeated string checklist_items = 10;
  bool is_active = 11;
  google.protobuf.Timestamp created_at = 12;
  google.protobuf.Timestamp updated_at = 13;
}

enum MaintenanceType {
  MAINTENANCE_TYPE_UNSPECIFIED = 0;
  MAINTENANCE_TYPE_CLEANING = 1;
  MAINTENANCE_TYPE_CALIBRATION = 2;
  MAINTENANCE_TYPE_INSPECTION = 3;
  MAINTENANCE_TYPE_REPAIR = 4;
  MAINTENANCE_TYPE_REPLACEMENT = 5;
}

enum MaintenancePriority {
  MAINTENANCE_PRIORITY_UNSPECIFIED = 0;
  MAINTENANCE_PRIORITY_ROUTINE = 1;
  MAINTENANCE_PRIORITY_IMPORTANT = 2;
  MAINTENANCE_PRIORITY_CRITICAL = 3;
}

message RecurrenceRule {
  RecurrenceFrequency frequency = 1;
  int32 interval = 2;  // e.g., every 2 weeks
  google.protobuf.Timestamp end_date = 3;  // optional
}

enum RecurrenceFrequency {
  RECURRENCE_FREQUENCY_UNSPECIFIED = 0;
  RECURRENCE_FREQUENCY_DAILY = 1;
  RECURRENCE_FREQUENCY_WEEKLY = 2;
  RECURRENCE_FREQUENCY_MONTHLY = 3;
  RECURRENCE_FREQUENCY_YEARLY = 4;
  RECURRENCE_FREQUENCY_CUSTOM = 5;
}

message ServiceLog {
  string id = 1;
  string item_id = 2;
  string maintenance_schedule_id = 3;  // optional
  google.protobuf.Timestamp service_date = 4;
  string notes = 5;
  double cost = 6;
  string currency = 7;
  repeated string photo_urls = 8;
  string service_provider = 9;
  int32 actual_duration_minutes = 10;
  repeated string parts_used = 11;
  google.protobuf.Timestamp created_at = 12;
}

message Warranty {
  string id = 1;
  string item_id = 2;
  google.protobuf.Timestamp purchase_date = 3;
  google.protobuf.Timestamp expiry_date = 4;
  string vendor = 5;
  string coverage_details = 6;
  repeated string document_urls = 7;
  WarrantyStatus status = 8;
  bool is_extended = 9;
  google.protobuf.Timestamp created_at = 10;
  google.protobuf.Timestamp updated_at = 11;
}

enum WarrantyStatus {
  WARRANTY_STATUS_UNSPECIFIED = 0;
  WARRANTY_STATUS_ACTIVE = 1;
  WARRANTY_STATUS_EXPIRING_SOON = 2;  // <30 days
  WARRANTY_STATUS_EXPIRED = 3;
  WARRANTY_STATUS_CLAIMED = 4;
}

message CreateMaintenanceScheduleRequest {
  MaintenanceSchedule schedule = 1;
}

message UpdateMaintenanceScheduleRequest {
  MaintenanceSchedule schedule = 1;
}

message DeleteMaintenanceScheduleRequest {
  string id = 1;
}

message GetMaintenanceScheduleRequest {
  string id = 1;
}

message ListMaintenanceSchedulesRequest {
  string item_id = 1;  // optional filter
  repeated MaintenanceType types = 2;  // optional filter
  repeated MaintenancePriority priorities = 3;  // optional filter
  bool include_inactive = 4;
  int32 page_size = 5;
  string page_token = 6;
}

message MaintenanceScheduleResponse {
  MaintenanceSchedule schedule = 1;
}

message ListMaintenanceSchedulesResponse {
  repeated MaintenanceSchedule schedules = 1;
  string next_page_token = 2;
  int32 total_count = 3;
}

message LogServiceRequest {
  ServiceLog log = 1;
}

message ServiceLogResponse {
  ServiceLog log = 1;
}

message GetServiceHistoryRequest {
  string item_id = 1;
  google.protobuf.Timestamp start_date = 2;  // optional
  google.protobuf.Timestamp end_date = 3;  // optional
  int32 page_size = 4;
  string page_token = 5;
}

message GetServiceHistoryResponse {
  repeated ServiceLog logs = 1;
  string next_page_token = 2;
  int32 total_count = 3;
}

message CreateWarrantyRequest {
  Warranty warranty = 1;
}

message UpdateWarrantyRequest {
  Warranty warranty = 1;
}

message GetWarrantyRequest {
  string id = 1;
}

message ListWarrantiesRequest {
  string item_id = 1;  // optional filter
  repeated WarrantyStatus statuses = 2;  // optional filter
  int32 page_size = 3;
  string page_token = 4;
}

message WarrantyResponse {
  Warranty warranty = 1;
}

message ListWarrantiesResponse {
  repeated Warranty warranties = 1;
  string next_page_token = 2;
  int32 total_count = 3;
}

message CalculateTCORequest {
  string item_id = 1;
  google.protobuf.Timestamp start_date = 2;
  google.protobuf.Timestamp end_date = 3;
}

message TCOResponse {
  double total_cost = 1;
  string currency = 2;
  CostBreakdown breakdown = 3;
}

message CostBreakdown {
  double labor_cost = 1;
  double parts_cost = 2;
  double outsourced_cost = 3;
  map<string, double> cost_by_type = 4;
}

message GetUpcomingMaintenanceRequest {
  int32 days_ahead = 1;  // default 30
  int32 page_size = 2;
  string page_token = 3;
}

message GetOverdueMaintenanceRequest {
  int32 page_size = 1;
  string page_token = 2;
}

message DeleteResponse {
  bool success = 1;
  string message = 2;
}
```

### SurrealDB Schema

```sql
-- Maintenance schedules table
DEFINE TABLE maintenance_schedules SCHEMAFULL;
DEFINE FIELD item_id ON maintenance_schedules TYPE record<items>;
DEFINE FIELD name ON maintenance_schedules TYPE string;
DEFINE FIELD description ON maintenance_schedules TYPE string;
DEFINE FIELD type ON maintenance_schedules TYPE string 
  ASSERT $value IN ['cleaning', 'calibration', 'inspection', 'repair', 'replacement'];
DEFINE FIELD priority ON maintenance_schedules TYPE string
  ASSERT $value IN ['routine', 'important', 'critical'];
DEFINE FIELD next_due_date ON maintenance_schedules TYPE datetime;
DEFINE FIELD recurrence ON maintenance_schedules TYPE object;
DEFINE FIELD recurrence.frequency ON maintenance_schedules TYPE string;
DEFINE FIELD recurrence.interval ON maintenance_schedules TYPE int;
DEFINE FIELD recurrence.end_date ON maintenance_schedules TYPE option<datetime>;
DEFINE FIELD estimated_duration_minutes ON maintenance_schedules TYPE int;
DEFINE FIELD checklist_items ON maintenance_schedules TYPE array<string>;
DEFINE FIELD is_active ON maintenance_schedules TYPE bool DEFAULT true;
DEFINE FIELD created_at ON maintenance_schedules TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON maintenance_schedules TYPE datetime DEFAULT time::now();

DEFINE INDEX maintenance_item_idx ON maintenance_schedules FIELDS item_id;
DEFINE INDEX maintenance_due_date_idx ON maintenance_schedules FIELDS next_due_date;
DEFINE INDEX maintenance_type_idx ON maintenance_schedules FIELDS type;

-- Service logs table
DEFINE TABLE service_logs SCHEMAFULL;
DEFINE FIELD item_id ON service_logs TYPE record<items>;
DEFINE FIELD maintenance_schedule_id ON service_logs TYPE option<record<maintenance_schedules>>;
DEFINE FIELD service_date ON service_logs TYPE datetime;
DEFINE FIELD notes ON service_logs TYPE string;
DEFINE FIELD cost ON service_logs TYPE decimal;
DEFINE FIELD currency ON service_logs TYPE string DEFAULT 'USD';
DEFINE FIELD photo_urls ON service_logs TYPE array<string>;
DEFINE FIELD service_provider ON service_logs TYPE string;
DEFINE FIELD actual_duration_minutes ON service_logs TYPE int;
DEFINE FIELD parts_used ON service_logs TYPE array<string>;
DEFINE FIELD created_at ON service_logs TYPE datetime DEFAULT time::now();

DEFINE INDEX service_item_idx ON service_logs FIELDS item_id;
DEFINE INDEX service_date_idx ON service_logs FIELDS service_date;

-- Warranties table
DEFINE TABLE warranties SCHEMAFULL;
DEFINE FIELD item_id ON warranties TYPE record<items>;
DEFINE FIELD purchase_date ON warranties TYPE datetime;
DEFINE FIELD expiry_date ON warranties TYPE datetime;
DEFINE FIELD vendor ON warranties TYPE string;
DEFINE FIELD coverage_details ON warranties TYPE string;
DEFINE FIELD document_urls ON warranties TYPE array<string>;
DEFINE FIELD status ON warranties TYPE string
  ASSERT $value IN ['active', 'expiring_soon', 'expired', 'claimed'];
DEFINE FIELD is_extended ON warranties TYPE bool DEFAULT false;
DEFINE FIELD created_at ON warranties TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON warranties TYPE datetime DEFAULT time::now();

DEFINE INDEX warranty_item_idx ON warranties FIELDS item_id;
DEFINE INDEX warranty_expiry_idx ON warranties FIELDS expiry_date;
DEFINE INDEX warranty_status_idx ON warranties FIELDS status;

-- Computed fields for warranty status
DEFINE FIELD status ON warranties 
  VALUE {
    LET expiry = expiry_date;
    LET now = time::now();
    LET days_until = time::diff(expiry, now, 'days');
    
    RETURN IF days_until < 0 THEN 'expired'
           ELSE IF days_until < 30 THEN 'expiring_soon'
           ELSE 'active';
  };
```

### Rust Backend Implementation

```rust
// src/services/maintenance_service.rs
use crate::models::{MaintenanceSchedule, ServiceLog, Warranty, RecurrenceRule};
use crate::repositories::MaintenanceRepository;
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;

pub struct MaintenanceService {
    repository: MaintenanceRepository,
}

impl MaintenanceService {
    pub fn new(repository: MaintenanceRepository) -> Self {
        Self { repository }
    }
    
    /// Create a new maintenance schedule with automatic recurrence calculation
    pub async fn create_schedule(
        &self,
        schedule: MaintenanceSchedule,
    ) -> Result<MaintenanceSchedule, ServiceError> {
        let mut schedule = schedule;
        schedule.id = Uuid::new_v4().to_string();
        schedule.created_at = Utc::now();
        schedule.updated_at = Utc::now();
        
        // Calculate first due date if recurrence is set
        if let Some(recurrence) = &schedule.recurrence {
            schedule.next_due_date = self.calculate_next_due_date(&recurrence, Utc::now());
        }
        
        self.repository.create_schedule(schedule).await
    }
    
    /// Log a service completion and update next maintenance date
    pub async fn log_service(
        &self,
        log: ServiceLog,
    ) -> Result<ServiceLog, ServiceError> {
        let mut log = log;
        log.id = Uuid::new_v4().to_string();
        log.created_at = Utc::now();
        
        // If this service is for a scheduled maintenance, update the schedule
        if let Some(schedule_id) = &log.maintenance_schedule_id {
            self.update_schedule_after_service(schedule_id, &log).await?;
        }
        
        self.repository.create_service_log(log).await
    }
    
    /// Update maintenance schedule's next due date after service completion
    async fn update_schedule_after_service(
        &self,
        schedule_id: &str,
        log: &ServiceLog,
    ) -> Result<(), ServiceError> {
        let mut schedule = self.repository.get_schedule(schedule_id).await?;
        
        if let Some(recurrence) = &schedule.recurrence {
            schedule.next_due_date = self.calculate_next_due_date(recurrence, log.service_date);
            schedule.updated_at = Utc::now();
            self.repository.update_schedule(schedule).await?;
        }
        
        Ok(())
    }
    
    /// Calculate next due date based on recurrence rule
    fn calculate_next_due_date(
        &self,
        recurrence: &RecurrenceRule,
        from_date: DateTime<Utc>,
    ) -> DateTime<Utc> {
        match recurrence.frequency.as_str() {
            "daily" => from_date + Duration::days(recurrence.interval as i64),
            "weekly" => from_date + Duration::weeks(recurrence.interval as i64),
            "monthly" => {
                // Approximate month as 30 days (use chrono's month addition for precision)
                from_date + Duration::days(30 * recurrence.interval as i64)
            }
            "yearly" => from_date + Duration::days(365 * recurrence.interval as i64),
            _ => from_date,
        }
    }
    
    /// Get upcoming maintenance within specified days
    pub async fn get_upcoming_maintenance(
        &self,
        days_ahead: i32,
    ) -> Result<Vec<MaintenanceSchedule>, ServiceError> {
        let now = Utc::now();
        let future = now + Duration::days(days_ahead as i64);
        
        self.repository.get_schedules_by_date_range(now, future).await
    }
    
    /// Get overdue maintenance
    pub async fn get_overdue_maintenance(
        &self,
    ) -> Result<Vec<MaintenanceSchedule>, ServiceError> {
        let now = Utc::now();
        self.repository.get_schedules_before_date(now).await
    }
    
    /// Calculate total cost of ownership for an item
    pub async fn calculate_tco(
        &self,
        item_id: &str,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
    ) -> Result<CostBreakdown, ServiceError> {
        let logs = self.repository.get_service_history(
            item_id,
            start_date,
            end_date,
        ).await?;
        
        let mut breakdown = CostBreakdown {
            total_cost: 0.0,
            labor_cost: 0.0,
            parts_cost: 0.0,
            outsourced_cost: 0.0,
            cost_by_type: HashMap::new(),
        };
        
        for log in logs {
            breakdown.total_cost += log.cost;
            
            // Categorize costs (simplified logic, can be enhanced)
            if !log.service_provider.is_empty() {
                breakdown.outsourced_cost += log.cost;
            } else if !log.parts_used.is_empty() {
                breakdown.parts_cost += log.cost;
            } else {
                breakdown.labor_cost += log.cost;
            }
            
            // Track by maintenance type if available
            if let Some(schedule_id) = &log.maintenance_schedule_id {
                if let Ok(schedule) = self.repository.get_schedule(schedule_id).await {
                    *breakdown.cost_by_type.entry(schedule.type_).or_insert(0.0) += log.cost;
                }
            }
        }
        
        Ok(breakdown)
    }
    
    /// Update warranty status based on expiry date
    pub async fn update_warranty_statuses(&self) -> Result<(), ServiceError> {
        let warranties = self.repository.list_warranties(None, None).await?;
        let now = Utc::now();
        
        for mut warranty in warranties {
            let days_until_expiry = (warranty.expiry_date - now).num_days();
            
            let new_status = if days_until_expiry < 0 {
                "expired"
            } else if days_until_expiry < 30 {
                "expiring_soon"
            } else {
                "active"
            };
            
            if warranty.status != new_status {
                warranty.status = new_status.to_string();
                warranty.updated_at = Utc::now();
                self.repository.update_warranty(warranty).await?;
            }
        }
        
        Ok(())
    }
}

#[derive(Debug)]
pub struct CostBreakdown {
    pub total_cost: f64,
    pub labor_cost: f64,
    pub parts_cost: f64,
    pub outsourced_cost: f64,
    pub cost_by_type: HashMap<String, f64>,
}
```

## Testing Requirements

### Unit Tests

**State Management:**
- [ ] MaintenanceSchedulesProvider correctly loads schedules
- [ ] MaintenanceSchedulesProvider filters by priority
- [ ] WarrantyProvider calculates expiry status correctly
- [ ] ServiceHistoryProvider sorts logs chronologically
- [ ] ReminderSettingsProvider persists user preferences

**Business Logic:**
- [ ] Recurrence calculation for daily, weekly, monthly, yearly
- [ ] Next due date updates correctly after service completion
- [ ] TCO calculation sums costs accurately
- [ ] Warranty status transitions (active → expiring_soon → expired)
- [ ] Maintenance priority assignment based on due date

**Data Validation:**
- [ ] Reject maintenance schedules with past due dates
- [ ] Validate file upload size and type
- [ ] Sanitize user input in notes fields
- [ ] Ensure positive cost values only
- [ ] Validate date ranges (start < end)

### Widget Tests

**Component Rendering:**
- [ ] MaintenanceScheduleCard displays all schedule details
- [ ] WarrantyStatusBadge shows correct color for each status
- [ ] ServiceHistoryTimeline renders chronologically
- [ ] CostTrackingChart displays breakdown correctly
- [ ] ReminderSettingsPanel saves preferences on change

**User Interactions:**
- [ ] Click on schedule card opens detail view
- [ ] Swipe right on mobile marks maintenance complete
- [ ] Drag-and-drop reschedules maintenance
- [ ] File picker opens on upload button click
- [ ] Keyboard shortcuts trigger correct actions

**State Updates:**
- [ ] Completing maintenance updates UI immediately
- [ ] Snoozing maintenance changes due date
- [ ] Adding service log updates history timeline
- [ ] Deleting schedule removes from list
- [ ] Notification preferences reflect in UI

### Integration Tests

**End-to-End Flows:**
- [ ] Create maintenance schedule → Receive reminder → Log service → Update next due date
- [ ] Add warranty → Expiry notification triggers 30 days before → Status updates
- [ ] Schedule recurring maintenance → Multiple occurrences created correctly
- [ ] Log service with photo → Photo uploads to storage → Appears in history
- [ ] Calculate TCO → Fetches all service logs → Returns accurate total

**Cross-App Integration:**
- [ ] Maintenance scheduled for item → Item detail shows maintenance badge
- [ ] Service uses parts from inventory → Inventory quantity decreases
- [ ] Warranty document stored via SHARED-005 → Retrievable from warranty details
- [ ] Maintenance notification sent via SHARED-003 → Desktop notification appears

**Database Operations:**
- [ ] Create schedule persists to SurrealDB
- [ ] Query upcoming maintenance returns correct results
- [ ] Update warranty status via computed field works
- [ ] Service history pagination loads correct pages
- [ ] Cascade delete: Deleting item deletes associated maintenance/warranties

### Accessibility Tests

**Screen Reader:**
- [ ] Maintenance items announced with full context
- [ ] Status badges have proper ARIA labels
- [ ] Forms have associated labels
- [ ] Error messages announced to screen readers
- [ ] Table navigation works with screen readers

**Keyboard-Only Navigation:**
- [ ] Can navigate entire maintenance dashboard with keyboard
- [ ] Focus visible on all interactive elements
- [ ] Modal dialogs trap focus appropriately
- [ ] Tab order is logical and efficient
- [ ] Escape key closes dialogs

**Color Contrast:**
- [ ] All text meets WCAG AA standards (4.5:1 ratio)
- [ ] Status colors combined with icons
- [ ] High contrast mode supported
- [ ] Focus indicators have sufficient contrast

**Motion & Animation:**
- [ ] Celebration animations respect prefers-reduced-motion
- [ ] Transitions fade instead of slide when motion reduced
- [ ] No auto-playing animations longer than 5 seconds
- [ ] Pulsing animations can be disabled

### Performance Tests

**Load Testing:**
- [ ] Render 100 maintenance schedules in <200ms
- [ ] Load 50 service log entries in <500ms
- [ ] Calendar view with 30 days renders in <300ms
- [ ] Photo upload completes in <2s per image

**Stress Testing:**
- [ ] Handle 1000+ maintenance schedules without lag
- [ ] Concurrent service log submissions process correctly
- [ ] Multiple file uploads (10+) complete successfully
- [ ] Notification queue processes 100+ reminders efficiently

**Network Resilience:**
- [ ] Offline mode: Can view cached maintenance schedules
- [ ] Offline mode: Service logs queued for sync when online
- [ ] Retry logic for failed gRPC calls (3 retries with exponential backoff)
- [ ] Graceful degradation when backend unreachable

## Definition of Done

**Feature Complete:**
- [ ] All functional requirements implemented
- [ ] UI matches design specifications
- [ ] Keyboard shortcuts functional
- [ ] Mobile gestures working

**Testing:**
- [ ] All unit tests passing (>90% coverage)
- [ ] All widget tests passing
- [ ] Integration tests covering critical paths
- [ ] Accessibility tests passing (WCAG 2.1 AA)
- [ ] Performance metrics met

**Documentation:**
- [ ] Code documented with inline comments
- [ ] API documentation generated (Dart doc)
- [ ] User guide updated with maintenance features
- [ ] README with setup instructions

**Code Quality:**
- [ ] Code review approved by 2+ reviewers
- [ ] No critical/high severity linting errors
- [ ] Rust code passes clippy checks
- [ ] Flutter code passes dart analyze

**Deployment:**
- [ ] Deployed to staging environment
- [ ] Smoke tests passed in staging
- [ ] Performance tested in production-like environment
- [ ] Rollback plan documented

**Accessibility:**
- [ ] Screen reader tested (NVDA/JAWS on Windows, VoiceOver on macOS/iOS)
- [ ] Keyboard-only navigation verified
- [ ] Color contrast validated with tools
- [ ] Motion preferences respected

**ADHD-Friendly:**
- [ ] Cognitive load assessment completed
- [ ] Forgiveness features tested (snooze, undo)
- [ ] Immediate feedback verified (<300ms)
- [ ] Visual hierarchy clear and scannable

---

## Dependencies

**Required Features:**
- AT-001: Item CRUD Operations (for item references)
- SHARED-001: Authentication Service
- SHARED-002: Database Service (SurrealDB)
- SHARED-003: Notification Service (for reminders)
- SHARED-005: File Storage Service (for warranty documents and photos)

**Optional Features:**
- AT-003: BoM Intelligence (for tracking parts used in maintenance)
- SHARED-004: AI/LLM Provider (for smart maintenance suggestions, future)

---

## Future Enhancements

**Phase 2 Features:**
- Predictive maintenance using ML (analyze service history to predict failures)
- Integration with manufacturer maintenance schedules (API integration)
- Barcode scanning for parts tracking during service
- Voice-based service logging
- Collaborative maintenance schedules (for shared workshops)
- Export service history for tax/business purposes
- Integration with external calendars (Google Calendar, Outlook)

**Phase 3 Features:**
- Mobile AR for maintenance instructions overlay
- Video tutorials linked to maintenance tasks
- Community-sourced maintenance guides
- Parts ordering integration with distributors
- Warranty claim submission automation
- ISO compliance tracking (for professional workshops)

---

*This specification is optimized for AI-assisted development with Cursor IDE and follows Altair's ADHD-friendly design principles.*
