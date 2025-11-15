# Feature AK-002: Daily Notes System

## What it does

Provides an auto-dating daily notes system as the default entry point for knowledge capture, with automatic creation, templating, and seamless navigation between days.

## User Journey

GIVEN a user opens Altair Knowledge
WHEN they start typing or click "Today's Note"
THEN a new daily note is automatically created with today's date and they can immediately start capturing thoughts

## Functional Requirements

- Automatic daily note creation on first access each day
- Custom date format support (YYYY-MM-DD default)
- Daily note templates with dynamic variables
- Calendar navigation for past/future notes
- Quick navigation (previous/next day shortcuts)
- Auto-linking to previous/next daily notes
- Daily summary generation
- Task rollover from previous days
- Weather/context injection (optional)
- Archive old daily notes after X days
- Search within daily notes only
- Export daily notes to markdown/PDF

## UI/UX Requirements

### Components

```dart
// Daily notes components
DailyNoteEditor
CalendarNavigator
DailyNoteHeader
QuickCaptureBar
DailySummaryCard
TaskRolloverWidget
DailyNoteTemplate
DateNavigationBar
DailyNoteSidebar
ContextInjectionPanel
```

### Visual Design

- **Layout:**
  - Full-width editor with 80ch max width
  - Fixed header with date navigation (64px)
  - Optional sidebar for calendar (280px)
  - Bottom quick-capture bar (56px)
  - Central editor with 24px padding
  
- **Colors:**
  ```dart
  todayHighlight: Color(0xFF10B981), // Green for today
  pastDate: Color(0xFF6B7280), // Gray for past
  futureDate: Color(0xFF93C5FD), // Light blue for future
  weekendDate: Color(0xFFFBBF24), // Yellow for weekends
  taskIndicator: Color(0xFFF59E0B), // Amber for tasks
  ```
  
- **Typography:**
  - Date header: 24px bold
  - Day of week: 14px uppercase
  - Note content: 16px regular, 1.6 line height
  - Quick capture: 14px regular
  
- **Iconography:**
  - Calendar: calendar icon 24px
  - Previous/Next: chevron icons 20px
  - Today: home icon 20px
  - Template: document icon 18px
  
- **Borders/Shadows:**
  - Neo-brutalist date cards: 3px border
  - Active date: 6px shadow offset
  - Editor border: 2px solid black

### User Interactions

- **Input Methods:**
  - Keyboard: Direct typing in editor
  - Voice: Transcription button
  - Paste: Rich text preservation
  - Drag-drop: File/image support
  
- **Keyboard Shortcuts:**
  - `Ctrl+T`: Jump to today
  - `Ctrl+J`: Previous day
  - `Ctrl+K`: Next day
  - `Ctrl+D`: Open date picker
  - `Ctrl+Enter`: Quick capture
  - `Alt+T`: Insert timestamp
  
- **Gestures:**
  - Swipe left/right: Navigate days
  - Pull down: Refresh template
  - Two-finger swipe: Calendar view
  
- **Feedback:**
  - Auto-save indicator
  - "Note created" toast
  - Sync status badge
  - Character/word count

### State Management

```dart
// Riverpod providers
final currentDateProvider = StateProvider<DateTime>((ref) => DateTime.now());

final dailyNoteProvider = FutureProvider.family<DailyNote, DateTime>((ref, date) async {
  final service = ref.read(dailyNoteServiceProvider);
  return service.getOrCreateDailyNote(date);
});

final dailyNoteContentProvider = StateNotifierProvider.family<DailyNoteContentNotifier, String, DateTime>(
  (ref, date) => DailyNoteContentNotifier(date),
);

final dailyTemplateProvider = StateNotifierProvider<DailyTemplateNotifier, DailyTemplate>(
  (ref) => DailyTemplateNotifier(),
);

final taskRolloverProvider = FutureProvider<List<Task>>((ref) async {
  final date = ref.watch(currentDateProvider);
  return ref.read(taskServiceProvider).getUncompletedTasksBefore(date);
});

final calendarNotesProvider = FutureProvider<Map<DateTime, NoteSummary>>((ref) async {
  final service = ref.read(dailyNoteServiceProvider);
  return service.getMonthSummary(DateTime.now());
});
```

### Responsive Behavior

- **Desktop (>1200px):**
  - Three-panel layout: calendar, editor, outline
  - Full calendar widget visible
  - Hover previews for other dates
  
- **Tablet (768-1199px):**
  - Two-panel: editor with collapsible calendar
  - Week view in calendar
  - Touch-optimized date selection
  
- **Mobile (<768px):**
  - Single panel with bottom navigation
  - Minimal date bar
  - Swipe navigation primary

### Accessibility Requirements

- **Screen Reader:**
  - Date announced on navigation
  - Content changes announced
  - Template variables explained
  
- **Keyboard Navigation:**
  - Date picker keyboard accessible
  - Editor shortcuts documented
  - Tab through all controls
  
- **Color Contrast:**
  - Date indicators have patterns
  - High contrast mode support
  
- **Motion:**
  - Optional page transitions
  - Reduced motion respects system
  
- **Font Sizing:**
  - Editor font adjustable
  - UI scales with system

### ADHD-Specific UI Requirements

- **Cognitive Load:**
  - Today's note always one click away
  - Minimal UI when writing
  - Template reduces blank page anxiety
  - Clear visual date indicators
  
- **Focus Management:**
  - Auto-focus editor on load
  - Distraction-free writing mode
  - Collapsible UI elements
  
- **Forgiveness:**
  - Automatic daily note creation
  - No penalty for missing days
  - Easy date navigation
  - Undo/redo support
  
- **Visual Hierarchy:**
  - Today prominently highlighted
  - Clear past/present/future distinction
  - Task rollover visible but not intrusive
  
- **Immediate Feedback:**
  - Instant note creation
  - Live word count
  - Auto-save every 3 seconds
  - Quick capture confirmation

## Non-Functional Requirements

### Performance Targets

- Note creation: <100ms
- Template application: <50ms
- Date navigation: <200ms
- Auto-save: <100ms
- Search within notes: <500ms

### Technical Constraints

- Flutter version: 3.16+
- Markdown support required
- Local storage with SurrealDB
- Template engine (Handlebars-style)
- Date handling with timezone support

### Security Requirements

- Notes encrypted at rest
- Secure template variable handling
- Input sanitization for XSS prevention
- Safe file attachment handling

## Implementation Details

### Code Structure

```
lib/features/daily_notes/
├── presentation/
│   ├── widgets/
│   │   ├── daily_note_editor.dart
│   │   ├── calendar_navigator.dart
│   │   ├── date_navigation_bar.dart
│   │   ├── quick_capture_bar.dart
│   │   └── task_rollover_widget.dart
│   ├── providers/
│   │   ├── daily_note_provider.dart
│   │   ├── template_provider.dart
│   │   └── calendar_provider.dart
│   └── screens/
│       └── daily_note_screen.dart
├── domain/
│   ├── models/
│   │   ├── daily_note.dart
│   │   ├── daily_template.dart
│   │   └── note_summary.dart
│   ├── repositories/
│   │   └── daily_note_repository.dart
│   └── use_cases/
│       ├── create_daily_note.dart
│       ├── get_daily_note.dart
│       └── rollover_tasks.dart
└── data/
    ├── services/
    │   └── daily_note_service.dart
    └── repositories/
        └── daily_note_repository_impl.dart
```

### Rust Backend Service

```rust
// rust-backend/services/daily_notes/src/lib.rs
use axum::{Router, Json, Extension};
use surrealdb::Surreal;
use chrono::{DateTime, Utc, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct DailyNote {
    id: String,
    date: DateTime<Utc>,
    content: String,
    template_used: Option<String>,
    tasks: Vec<TaskItem>,
    metadata: NoteMetadata,
}

#[derive(Deserialize)]
struct GetDailyNoteRequest {
    date: DateTime<Utc>,
    create_if_missing: bool,
}

pub async fn get_or_create_daily_note(
    Json(req): Json<GetDailyNoteRequest>,
    db: Extension<Surreal<Client>>,
    template_service: Extension<TemplateService>,
) -> Result<Json<DailyNote>, Error> {
    let note_id = format!("daily_note:{}", req.date.format("%Y-%m-%d"));
    
    // Try to get existing note
    let existing: Option<DailyNote> = db
        .select(&note_id)
        .await?;
    
    match existing {
        Some(note) => Ok(Json(note)),
        None if req.create_if_missing => {
            // Create new note with template
            let template = template_service.get_daily_template().await?;
            let content = template.render(&req.date)?;
            
            let new_note = DailyNote {
                id: note_id,
                date: req.date,
                content,
                template_used: Some(template.id),
                tasks: Vec::new(),
                metadata: NoteMetadata::default(),
            };
            
            db.create(&note_id)
                .content(&new_note)
                .await?;
            
            Ok(Json(new_note))
        }
        None => Err(Error::NotFound),
    }
}

pub async fn rollover_tasks(
    Json(date): Json<DateTime<Utc>>,
    db: Extension<Surreal<Client>>,
) -> Result<Json<Vec<TaskItem>>, Error> {
    // Get uncompleted tasks from previous days
    let tasks: Vec<TaskItem> = db
        .query("SELECT * FROM tasks WHERE completed = false AND date < $date")
        .bind(("date", date))
        .await?;
    
    // Update tasks to new date
    for task in &tasks {
        db.query("UPDATE tasks SET date = $date WHERE id = $id")
            .bind(("date", date))
            .bind(("id", &task.id))
            .await?;
    }
    
    Ok(Json(tasks))
}
```

### gRPC Service Definition

```proto
// protos/daily_notes.proto
syntax = "proto3";
package altair.knowledge.daily_notes;

import "google/protobuf/timestamp.proto";

service DailyNoteService {
  rpc GetOrCreateDailyNote(GetDailyNoteRequest) returns (DailyNote);
  rpc UpdateDailyNote(UpdateDailyNoteRequest) returns (DailyNote);
  rpc GetDailyNoteRange(GetRangeRequest) returns (DailyNoteList);
  rpc RolloverTasks(RolloverTasksRequest) returns (TaskList);
  rpc GetCalendarSummary(CalendarRequest) returns (CalendarSummary);
  rpc ApplyTemplate(ApplyTemplateRequest) returns (DailyNote);
}

message DailyNote {
  string id = 1;
  google.protobuf.Timestamp date = 2;
  string content = 3;
  string template_used = 4;
  repeated Task tasks = 5;
  NoteMetadata metadata = 6;
}

message GetDailyNoteRequest {
  google.protobuf.Timestamp date = 1;
  bool create_if_missing = 2;
  bool include_context = 3;
}

message Task {
  string id = 1;
  string content = 2;
  bool completed = 3;
  int32 energy_level = 4;
  google.protobuf.Timestamp due_date = 5;
}

message NoteMetadata {
  int32 word_count = 1;
  repeated string tags = 2;
  string weather = 3;
  string location = 4;
  int32 mood_rating = 5;
}
```

### SurrealDB Schema

```sql
-- Daily notes schema
DEFINE TABLE daily_notes SCHEMAFULL;
DEFINE FIELD date ON daily_notes TYPE datetime ASSERT $value != NONE;
DEFINE FIELD content ON daily_notes TYPE string;
DEFINE FIELD template_used ON daily_notes TYPE string;
DEFINE FIELD word_count ON daily_notes TYPE int;
DEFINE FIELD created_at ON daily_notes TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON daily_notes TYPE datetime DEFAULT time::now();
DEFINE FIELD tasks ON daily_notes TYPE array<record<tasks>>;
DEFINE FIELD tags ON daily_notes TYPE array<string>;

-- Unique constraint on date
DEFINE INDEX daily_notes_date_idx ON daily_notes FIELDS date UNIQUE;

-- Daily templates
DEFINE TABLE daily_templates SCHEMAFULL;
DEFINE FIELD name ON daily_templates TYPE string;
DEFINE FIELD content ON daily_templates TYPE string;
DEFINE FIELD variables ON daily_templates TYPE array<string>;
DEFINE FIELD is_default ON daily_templates TYPE bool DEFAULT false;

-- Function to get notes for date range
DEFINE FUNCTION fn::get_daily_notes_range($start_date: datetime, $end_date: datetime) {
  RETURN SELECT * FROM daily_notes 
  WHERE date >= $start_date AND date <= $end_date
  ORDER BY date DESC;
};
```

### Template Engine Integration

```dart
// lib/features/daily_notes/domain/services/template_service.dart
class TemplateService {
  String renderTemplate(String template, DateTime date) {
    final variables = {
      'date': DateFormat('yyyy-MM-dd').format(date),
      'day': DateFormat('EEEE').format(date),
      'month': DateFormat('MMMM').format(date),
      'year': date.year.toString(),
      'week_number': _getWeekNumber(date),
      'yesterday': _linkToDate(date.subtract(Duration(days: 1))),
      'tomorrow': _linkToDate(date.add(Duration(days: 1))),
    };
    
    return _replaceVariables(template, variables);
  }
}
```

## Testing Requirements

### Unit Tests

```dart
// test/features/daily_notes/domain/use_cases/create_daily_note_test.dart
void main() {
  group('CreateDailyNote', () {
    test('creates note with today\'s date', () async {
      final useCase = CreateDailyNote(mockRepo);
      final note = await useCase(DateTime.now());
      expect(note.date.day, equals(DateTime.now().day));
    });
    
    test('applies template variables correctly', () async {
      // Test template rendering
    });
  });
}
```

### Widget Tests

```dart
// test/features/daily_notes/presentation/widgets/daily_note_editor_test.dart
void main() {
  testWidgets('Editor auto-focuses on load', (tester) async {
    await tester.pumpWidget(DailyNoteEditor());
    final textField = find.byType(TextField);
    expect(textField, findsOneWidget);
    expect(tester.widget<TextField>(textField).autofocus, isTrue);
  });
}
```

### Integration Tests

```dart
// integration_test/daily_note_flow_test.dart
void main() {
  testWidgets('Daily note creation and navigation', (tester) async {
    await tester.pumpWidget(MyApp());
    
    // Navigate to today
    await tester.tap(find.byIcon(Icons.today));
    await tester.pumpAndSettle();
    
    // Verify note created
    expect(find.text(DateFormat('yyyy-MM-dd').format(DateTime.now())), findsOneWidget);
    
    // Navigate to previous day
    await tester.tap(find.byIcon(Icons.chevron_left));
    await tester.pumpAndSettle();
    
    // Verify date changed
    final yesterday = DateTime.now().subtract(Duration(days: 1));
    expect(find.text(DateFormat('yyyy-MM-dd').format(yesterday)), findsOneWidget);
  });
}
```

### Accessibility Tests

- [ ] Date navigation with screen reader
- [ ] Calendar keyboard navigation
- [ ] Template variable announcements
- [ ] Focus management on date change

## Definition of Done

- [ ] Daily notes auto-create on access
- [ ] Date navigation works smoothly
- [ ] Templates apply correctly with variables
- [ ] Task rollover functions properly
- [ ] Calendar view shows note summaries
- [ ] Auto-save works reliably
- [ ] All tests passing (>80% coverage)
- [ ] Accessibility audit complete
- [ ] Performance targets met
- [ ] Documentation updated
- [ ] Code review approved
