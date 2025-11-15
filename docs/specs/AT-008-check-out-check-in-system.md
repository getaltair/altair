# Feature AT-008: Check-Out/Check-In System

## What it does

Enables tracking of shared tools and components that are loaned out to projects or team members, with check-out/check-in timestamps, location tracking, and overdue notifications for workshop and makerspace environments.

## User Journey

GIVEN a user has items marked as "shared" or "loanable" in their inventory
WHEN they check out a tool to "ESP32 Project" or a team member "Alex"
THEN the system records who has it, when it was checked out, expected return date, and sends reminders if overdue; when checked back in, it updates availability and location

## Functional Requirements

### Check-Out Management
- Check out items to projects (internal) or people (external)
- Record check-out timestamp, expected return date
- Add notes for check-out (e.g., "For antenna testing")
- Support batch check-out (multiple items to same project)
- Assign check-out location (where it will be used)
- Optional: photo capture of item condition at check-out

### Check-In Management
- Quick check-in with barcode/QR scan
- Verify item condition on return
- Add return notes (e.g., "Working perfectly" or "Needs calibration")
- Automatic availability update
- Optional: photo capture of item condition at check-in
- Partial check-in support (some items from batch)

### Tracking Features
- View all currently checked-out items
- Filter by person, project, location, or overdue status
- Check-out history for each item
- Overdue notification system (email, push, in-app)
- Checkout duration analytics
- Most borrowed items report

### Integration Features
- Link check-outs to active quests in Guidance
- Automatically check out items when quest marked "In Progress"
- Suggest check-in when quest completed
- Show availability in BoM Intelligence (AT-006)
- Block deletion of checked-out items

### Business Rules
- Items must be marked as "loanable" to check out
- Cannot check out items with zero quantity
- Overdue items highlighted in red after due date
- Check-out history retained indefinitely
- Maximum check-out duration: 90 days (configurable)
- Send reminder 1 day before due date
- Send overdue notification on due date and every 3 days after

## UI/UX Requirements

### Components

**Existing Design System:** `SearchBar`, `DataTable`, `Card`, `Badge`, `Dialog`, `DatePicker`, `Avatar`, `Chip`

**Custom Components:**
- `CheckOutDialog` - Multi-step check-out wizard
- `CheckInScanner` - QR/barcode scanner widget
- `OverdueItemCard` - Highlighted overdue item
- `CheckOutHistory` - Timeline view of item history
- `LoanStatusBadge` - Visual status indicator

### Visual Design

**Layout:** Three tabs (Checked Out | Available | History), Grid cards (mobile) or table (desktop), Sidebar: Quick actions

**Colors:** Available: green, Checked out: yellow/orange, Overdue: red, Due soon: dark orange, 4px solid borders

**Typography:** Item: 18px bold, Borrower: 16px medium, Due date: 14px monospace, Days overdue: 20px bold red

**Borders/Shadows:** Cards: 3px solid black + 4px shadow, Overdue: 4px solid red + 6px shadow + pulsing animation

### State Management

**Global State (Riverpod):**
```dart
@riverpod
class CheckOutController extends _$CheckOutController {
  Future<void> checkOutItems(CheckOutRequest request) async { }
  Future<void> checkInItems(String checkOutId, CheckInRequest request) async { }
  Future<void> markOverdue() async { }
  Future<void> sendReminders() async { }
}

@riverpod
class CheckOutHistory extends _$CheckOutHistory {
  FutureOr<List<CheckOut>> build(String itemId) async { }
}

@riverpod
class OverdueNotifier extends _$OverdueNotifier {
  Future<void> scheduleReminder(String checkOutId) async { }
}
```

### ADHD-Specific UI Requirements

**Cognitive Load:** One-step check-out for simple cases, progressive disclosure for advanced options, default return date: 7 days

**Focus Management:** Autofocus on borrower field, large "Check In" button, scanner auto-activates

**Forgiveness:** Extend return date without penalty, edit after submission, undo within 5 min, no shame language ("Extended" not "Late")

**Visual Hierarchy:** Overdue always at top, check-in is primary action (largest), history is tertiary

**Immediate Feedback:** <100ms check-out confirmation, instant check-in with animation, real-time countdown

## Non-Functional Requirements

**Performance:** Check-out <200ms, Check-in <100ms, Scanner init <500ms, Overdue check every 5 min

**Technical:** Flutter 3.16+, packages: riverpod, drift, mobile_scanner, image_picker, flutter_local_notifications

**Security:** Verify ownership, log all actions, encrypt borrower PII, require auth, rate limit: 100/hr

## Implementation Details

### Code Structure

```
lib/features/check_out_in/
├── presentation/
│   ├── widgets/ (check_out_dialog.dart, check_in_scanner.dart, overdue_item_card.dart)
│   ├── providers/ (check_out_controller.dart, borrower_repository.dart)
│   └── screens/ (check_out_management_screen.dart, check_in_scanner_screen.dart)
├── domain/
│   ├── models/ (check_out.dart, check_in.dart, borrower.dart)
│   └── repositories/ (check_out_repository.dart)
└── data/
    └── repositories/ (check_out_repository_impl.dart)
```

### Rust Backend

```rust
// src/services/check_out_service.rs
pub struct CheckOutService {
    db: Arc<Surreal<Db>>,
}

impl CheckOutService {
    pub async fn check_out_item(&self, request: CheckOutRequest) -> Result<CheckOut> {
        // Verify item available and loanable
        // Create check-out record
        // Decrement item quantity
    }
    
    pub async fn check_in_item(&self, request: CheckInRequest) -> Result<CheckOut> {
        // Update check-out record
        // Increment item quantity
    }
    
    pub async fn get_overdue_check_outs(&self) -> Result<Vec<CheckOut>> {
        // Query overdue items
    }
}
```

### gRPC Proto

```protobuf
service CheckOutService {
  rpc CheckOutItem(CheckOutRequest) returns (CheckOutResponse);
  rpc CheckInItem(CheckInRequest) returns (CheckInResponse);
  rpc GetActiveCheckOuts(GetActiveCheckOutsRequest) returns (GetActiveCheckOutsResponse);
  rpc GetOverdueCheckOuts(GetOverdueCheckOutsRequest) returns (GetOverdueCheckOutsResponse);
  rpc ExtendReturnDate(ExtendReturnDateRequest) returns (ExtendReturnDateResponse);
}
```

### SurrealDB Schema

```sql
DEFINE TABLE check_outs SCHEMAFULL;
DEFINE FIELD item_id ON TABLE check_outs TYPE string;
DEFINE FIELD borrower_id ON TABLE check_outs TYPE option<string>;
DEFINE FIELD checked_out_at ON TABLE check_outs TYPE datetime DEFAULT time::now();
DEFINE FIELD expected_return_at ON TABLE check_outs TYPE datetime;
DEFINE FIELD checked_in_at ON TABLE check_outs TYPE option<datetime>;
DEFINE FIELD status ON TABLE check_outs TYPE string;
DEFINE INDEX check_outs_item_idx ON TABLE check_outs COLUMNS item_id;
DEFINE INDEX check_outs_overdue_idx ON TABLE check_outs COLUMNS expected_return_at, checked_in_at;

DEFINE TABLE borrowers SCHEMAFULL;
DEFINE FIELD name ON TABLE borrowers TYPE string;
DEFINE FIELD type ON TABLE borrowers TYPE string; // person, team, project
```

## Testing Requirements

- [ ] Unit: Check-out logic, check-in logic, overdue detection, reminder scheduling
- [ ] Widget: CheckOutDialog, CheckInScanner, OverdueItemCard, timeline
- [ ] Integration: End-to-end workflow, QR scan check-in, notifications, cross-app integration
- [ ] Performance: Check-out <200ms, Check-in <100ms, Scanner <500ms
- [ ] Accessibility: Screen reader, keyboard-only, color contrast

## Dependencies

**Depends on:** AT-001 (Item CRUD), SHARED-001 (SurrealDB), SHARED-002 (gRPC), SHARED-007 (QR Scanner), SHARED-010 (Notifications)

**Blocks:** AT-006 (BoM Intelligence - availability checking), Integration with Guidance quests
