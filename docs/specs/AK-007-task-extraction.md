# Feature AK-007: Task Extraction to Guidance

## What it does

Automatically detects tasks and action items within notes using AI, then exports them to Altair Guidance's quest system with proper energy levels, deadlines, and context preservation.

## User Journey

GIVEN a user writes "TODO: Review Flutter state management docs by Friday" in their notes
WHEN the task extraction runs
THEN a new quest appears in Guidance with title, due date, and link back to the source note

## Functional Requirements

- Real-time task detection in notes
- Natural language task parsing
- Checkbox syntax support (- [ ] task)
- TODO/FIXME/ACTION keyword detection
- Energy level inference from context
- Deadline extraction from text
- Batch task export to Guidance
- Task status sync back to notes
- Context preservation (note link)
- Duplicate task detection
- Task update tracking
- Manual task marking
- Bulk operations
- Task extraction history

## UI/UX Requirements

### Components

```dart
// Task extraction components
TaskDetectionIndicator
ExtractedTaskCard
TaskExportDialog
TaskSyncStatus
EnergyLevelSelector
DeadlineParser
TaskContextPreview
BatchExportPanel
TaskDuplicateWarning
ExtractionSettingsPanel
```

### Visual Design

- **Layout:**
  - Inline task indicators in editor
  - Side panel for detected tasks (280px)
  - Export modal: 600x400px
  - Floating sync status badge
  - Context preview: tooltip 300px
  
- **Colors:**
  ```dart
  detectedTask: Color(0xFFF59E0B), // Amber for detected
  exportedTask: Color(0xFF10B981), // Green for synced
  pendingTask: Color(0xFF3B82F6), // Blue for pending
  duplicateTask: Color(0xFFEF4444), // Red for duplicate
  taskCheckbox: Color(0xFF6B7280), // Gray unchecked
  ```
  
- **Typography:**
  - Task text: 14px regular
  - Energy badge: 12px semibold
  - Deadline: 13px italic
  - Context link: 12px underlined
  
- **Iconography:**
  - Task: checkbox icon 16px
  - Export: arrow-right icon 18px
  - Sync: refresh icon 16px
  - Energy: lightning icon 14px
  
- **Borders/Shadows:**
  - Task cards: 2px border
  - Hover state: 4px shadow
  - Active task: glow effect

### User Interactions

- **Input Methods:**
  - Type task with keywords
  - Click checkbox to mark
  - Select text and mark as task
  - Voice dictation of tasks
  
- **Keyboard Shortcuts:**
  - `Ctrl+Shift+Enter`: Mark as task
  - `Ctrl+E`: Export selected tasks
  - `Alt+T`: Toggle task view
  - `Tab`: Navigate tasks
  
- **Gestures:**
  - Swipe right to export
  - Long-press for options
  - Drag to reorder priority
  
- **Feedback:**
  - Task detected animation
  - Export success notification
  - Sync status indicator
  - Duplicate warning dialog

### State Management

```dart
// Riverpod providers
final detectedTasksProvider = StateNotifierProvider<DetectedTasksNotifier, List<DetectedTask>>(
  (ref) => DetectedTasksNotifier(),
);

final taskExtractionServiceProvider = Provider<TaskExtractionService>((ref) {
  return TaskExtractionService(
    aiService: ref.read(aiServiceProvider),
    guidanceClient: ref.read(guidanceGrpcClientProvider),
  );
});

final taskSyncStatusProvider = StreamProvider<SyncStatus>((ref) {
  return ref.read(taskExtractionServiceProvider).syncStatusStream;
});

final pendingExportsProvider = StateProvider<List<TaskExport>>((ref) => []);

final taskExtractionSettingsProvider = StateNotifierProvider<ExtractionSettingsNotifier, ExtractionSettings>(
  (ref) => ExtractionSettingsNotifier(),
);

final exportedTasksProvider = FutureProvider<List<ExportedTask>>((ref) async {
  return ref.read(taskExtractionServiceProvider).getExportedTasks();
});
```

### Responsive Behavior

- **Desktop (>1200px):**
  - Persistent task sidebar
  - Inline task indicators
  - Batch operations visible
  
- **Tablet (768-1199px):**
  - Collapsible task panel
  - Modal export dialog
  - Touch-optimized
  
- **Mobile (<768px):**
  - Bottom sheet for tasks
  - Simplified export flow
  - Swipe gestures

### Accessibility Requirements

- **Screen Reader:**
  - Task detection announced
  - Export status communicated
  - Context links described
  
- **Keyboard Navigation:**
  - Tab through tasks
  - Enter to export
  - Space to select
  
- **Color Contrast:**
  - Task indicators accessible
  - Alternative markers
  
- **Motion:**
  - Optional animations
  - Static indicators available
  
- **Font Sizing:**
  - Scalable task text
  - Clear checkbox targets

### ADHD-Specific UI Requirements

- **Cognitive Load:**
  - Auto-detect reduces manual work
  - Batch export in one action
  - Smart defaults for energy/deadline
  - Clear task indicators
  
- **Focus Management:**
  - Unobtrusive detection
  - Non-blocking export
  - Background sync
  
- **Forgiveness:**
  - Undo task export
  - Edit before sending
  - Re-sync capability
  - Ignore false positives
  
- **Visual Hierarchy:**
  - Color-coded by status
  - Energy level badges
  - Deadline prominence
  
- **Immediate Feedback:**
  - Instant detection (<100ms)
  - Quick export confirmation
  - Live sync status

## Non-Functional Requirements

### Performance Targets

- Task detection: <100ms per paragraph
- Export to Guidance: <500ms
- Sync status update: real-time
- Batch export: <2s for 20 tasks
- Duplicate check: <50ms

### Technical Constraints

- Flutter version: 3.16+
- gRPC communication with Guidance
- AI model for task detection
- Regex for pattern matching
- Cross-app state management

### Security Requirements

- Secure gRPC channel
- Task content encryption
- Access control validation
- Audit logging

## Implementation Details

### Code Structure

```
lib/features/task_extraction/
├── presentation/
│   ├── widgets/
│   │   ├── task_detection_indicator.dart
│   │   ├── extracted_task_card.dart
│   │   ├── task_export_dialog.dart
│   │   ├── energy_level_selector.dart
│   │   └── deadline_parser.dart
│   ├── providers/
│   │   ├── detected_tasks_provider.dart
│   │   ├── export_settings_provider.dart
│   │   └── sync_status_provider.dart
│   └── screens/
│       └── task_extraction_screen.dart
├── domain/
│   ├── models/
│   │   ├── detected_task.dart
│   │   ├── task_export.dart
│   │   └── extraction_settings.dart
│   ├── repositories/
│   │   └── task_extraction_repository.dart
│   └── use_cases/
│       ├── detect_tasks.dart
│       ├── export_to_guidance.dart
│       └── sync_task_status.dart
└── data/
    ├── services/
    │   ├── task_detection_service.dart
    │   ├── nlp_parser_service.dart
    │   └── guidance_sync_service.dart
    └── repositories/
        └── task_extraction_repository_impl.dart
```

### Rust Backend Service

```rust
// rust-backend/services/task_extraction/src/lib.rs
use axum::{Router, Json, Extension};
use tonic::transport::Channel;
use regex::Regex;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct DetectedTask {
    id: String,
    content: String,
    source_note_id: String,
    source_position: usize,
    detected_type: TaskType,
    energy_level: Option<i32>,
    deadline: Option<NaiveDateTime>,
    tags: Vec<String>,
    confidence_score: f32,
}

#[derive(Serialize, Deserialize)]
enum TaskType {
    Checkbox,      // - [ ] task
    TodoKeyword,   // TODO: task
    ActionKeyword, // ACTION: task
    AIDetected,    // ML-detected task
}

pub struct TaskExtractionService {
    ai_client: AIClient,
    guidance_client: GuidanceClient,
    task_patterns: Vec<Regex>,
}

impl TaskExtractionService {
    pub fn new() -> Self {
        let task_patterns = vec![
            Regex::new(r"(?m)^- \[ \] (.+)$").unwrap(),  // Checkbox
            Regex::new(r"(?i)\b(TODO|FIXME|ACTION|TASK):\s*(.+)").unwrap(),  // Keywords
            Regex::new(r"(?i)\b(need to|have to|must|should)\s+(.+)").unwrap(),  // Obligation
            Regex::new(r"(?i)\b(by|before|until|due)\s+(\d{1,2}/\d{1,2}|\w+ \d{1,2}|\w+)").unwrap(),  // Deadlines
        ];
        
        Self {
            ai_client: AIClient::new(),
            guidance_client: GuidanceClient::connect("http://localhost:50051").await?,
            task_patterns,
        }
    }
    
    pub async fn extract_tasks(&self, note_content: &str, note_id: &str) -> Result<Vec<DetectedTask>, Error> {
        let mut tasks = Vec::new();
        
        // Pattern-based extraction
        for pattern in &self.task_patterns {
            for cap in pattern.captures_iter(note_content) {
                let task_text = cap.get(cap.len() - 1).unwrap().as_str();
                let position = cap.get(0).unwrap().start();
                
                let task = DetectedTask {
                    id: generate_task_id(),
                    content: task_text.to_string(),
                    source_note_id: note_id.to_string(),
                    source_position: position,
                    detected_type: self.determine_type(&cap),
                    energy_level: None,
                    deadline: None,
                    tags: Vec::new(),
                    confidence_score: 0.9,
                };
                
                tasks.push(task);
            }
        }
        
        // AI-based extraction for complex tasks
        let ai_tasks = self.ai_extract_tasks(note_content, note_id).await?;
        tasks.extend(ai_tasks);
        
        // Enhance tasks with AI
        for task in &mut tasks {
            self.enhance_task_with_ai(task, note_content).await?;
        }
        
        // Deduplicate
        self.deduplicate_tasks(&mut tasks);
        
        Ok(tasks)
    }
    
    async fn ai_extract_tasks(&self, content: &str, note_id: &str) -> Result<Vec<DetectedTask>, Error> {
        let prompt = format!(
            "Extract actionable tasks from this text. Return JSON array of tasks with content and confidence score:\n\n{}",
            content
        );
        
        let response = self.ai_client.complete(&prompt).await?;
        let ai_tasks: Vec<AITask> = serde_json::from_str(&response)?;
        
        let mut tasks = Vec::new();
        for ai_task in ai_tasks {
            if ai_task.confidence > 0.7 {
                tasks.push(DetectedTask {
                    id: generate_task_id(),
                    content: ai_task.content,
                    source_note_id: note_id.to_string(),
                    source_position: 0,
                    detected_type: TaskType::AIDetected,
                    energy_level: None,
                    deadline: None,
                    tags: Vec::new(),
                    confidence_score: ai_task.confidence,
                });
            }
        }
        
        Ok(tasks)
    }
    
    async fn enhance_task_with_ai(&self, task: &mut DetectedTask, context: &str) -> Result<(), Error> {
        // Extract energy level
        task.energy_level = self.infer_energy_level(&task.content).await?;
        
        // Parse deadline
        task.deadline = self.parse_deadline(&task.content)?;
        
        // Extract tags
        task.tags = self.extract_tags(&task.content);
        
        Ok(())
    }
    
    pub async fn export_to_guidance(&self, tasks: Vec<DetectedTask>) -> Result<Vec<ExportResult>, Error> {
        let mut results = Vec::new();
        
        for task in tasks {
            let quest = self.convert_to_quest(&task)?;
            
            let request = CreateQuestRequest {
                title: quest.title,
                description: quest.description,
                energy_level: quest.energy_level,
                deadline: quest.deadline,
                source_link: format!("knowledge://{}", task.source_note_id),
                tags: task.tags,
            };
            
            match self.guidance_client.create_quest(request).await {
                Ok(response) => {
                    results.push(ExportResult {
                        task_id: task.id,
                        quest_id: response.quest_id,
                        success: true,
                        error: None,
                    });
                }
                Err(e) => {
                    results.push(ExportResult {
                        task_id: task.id,
                        quest_id: String::new(),
                        success: false,
                        error: Some(e.to_string()),
                    });
                }
            }
        }
        
        Ok(results)
    }
    
    fn convert_to_quest(&self, task: &DetectedTask) -> Result<Quest, Error> {
        Ok(Quest {
            title: self.clean_task_text(&task.content),
            description: format!("Extracted from: [[{}]]", task.source_note_id),
            energy_level: task.energy_level.unwrap_or(3),
            deadline: task.deadline,
            column: QuestColumn::QuestLog,
        })
    }
    
    async fn infer_energy_level(&self, task_text: &str) -> Result<Option<i32>, Error> {
        // Keywords for different energy levels
        let low_energy = ["quick", "simple", "easy", "minor", "small"];
        let high_energy = ["complex", "difficult", "major", "research", "analyze", "design"];
        
        let lower_text = task_text.to_lowercase();
        
        if low_energy.iter().any(|&word| lower_text.contains(word)) {
            Ok(Some(1))
        } else if high_energy.iter().any(|&word| lower_text.contains(word)) {
            Ok(Some(4))
        } else {
            Ok(Some(3)) // Default medium
        }
    }
    
    fn parse_deadline(&self, text: &str) -> Result<Option<NaiveDateTime>, Error> {
        // Common deadline patterns
        let patterns = vec![
            (r"by (\d{1,2}/\d{1,2}/\d{2,4})", "%m/%d/%Y"),
            (r"before (\w+ \d{1,2})", "%B %d"),
            (r"due (\w+)", "relative"),
        ];
        
        for (pattern, format) in patterns {
            let re = Regex::new(pattern)?;
            if let Some(cap) = re.captures(text) {
                let date_str = cap.get(1).unwrap().as_str();
                
                if format == "relative" {
                    return Ok(self.parse_relative_date(date_str));
                } else {
                    if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
                        return Ok(Some(date.and_hms(23, 59, 59)));
                    }
                }
            }
        }
        
        Ok(None)
    }
}
```

### gRPC Service Definition

```proto
// protos/task_extraction.proto
syntax = "proto3";
package altair.knowledge.tasks;

import "google/protobuf/timestamp.proto";

service TaskExtractionService {
  rpc ExtractTasks(ExtractTasksRequest) returns (TaskList);
  rpc ExportToGuidance(ExportTasksRequest) returns (ExportResultList);
  rpc SyncTaskStatus(SyncRequest) returns (SyncResult);
  rpc GetExportedTasks(GetExportedRequest) returns (ExportedTaskList);
  rpc UpdateTaskMapping(UpdateMappingRequest) returns (UpdateResult);
}

message ExtractTasksRequest {
  string note_content = 1;
  string note_id = 2;
  bool use_ai = 3;
  ExtractionSettings settings = 4;
}

message DetectedTask {
  string id = 1;
  string content = 2;
  string source_note_id = 3;
  int32 source_position = 4;
  TaskType detected_type = 5;
  int32 energy_level = 6;
  google.protobuf.Timestamp deadline = 7;
  repeated string tags = 8;
  float confidence_score = 9;
}

message ExportTasksRequest {
  repeated DetectedTask tasks = 1;
  bool auto_assign_energy = 2;
  bool preserve_context = 3;
}

message ExportResult {
  string task_id = 1;
  string quest_id = 2;
  bool success = 3;
  string error = 4;
}

message TaskMapping {
  string note_task_id = 1;
  string guidance_quest_id = 2;
  google.protobuf.Timestamp exported_at = 3;
  SyncStatus status = 4;
}

enum TaskType {
  CHECKBOX = 0;
  TODO_KEYWORD = 1;
  ACTION_KEYWORD = 2;
  AI_DETECTED = 3;
}

enum SyncStatus {
  PENDING = 0;
  SYNCED = 1;
  COMPLETED = 2;
  DELETED = 3;
}

// Cross-app communication with Guidance
message CreateQuestRequest {
  string title = 1;
  string description = 2;
  int32 energy_level = 3;
  google.protobuf.Timestamp deadline = 4;
  string source_link = 5;
  repeated string tags = 6;
  string column = 7;
}

message CreateQuestResponse {
  string quest_id = 1;
  bool success = 2;
}
```

### Cross-App Integration

```dart
// lib/features/task_extraction/data/services/guidance_sync_service.dart
import 'package:grpc/grpc.dart';

class GuidanceSyncService {
  late ClientChannel _channel;
  late TaskExtractionServiceClient _client;
  
  Future<void> connect() async {
    _channel = ClientChannel(
      'localhost',
      port: 50051,
      options: const ChannelOptions(
        credentials: ChannelCredentials.insecure(),
      ),
    );
    
    _client = TaskExtractionServiceClient(_channel);
  }
  
  Future<List<ExportResult>> exportTasks(List<DetectedTask> tasks) async {
    final request = ExportTasksRequest()
      ..tasks.addAll(tasks.map((t) => t.toProto()))
      ..autoAssignEnergy = true
      ..preserveContext = true;
    
    final response = await _client.exportToGuidance(request);
    return response.results.map((r) => ExportResult.fromProto(r)).toList();
  }
  
  Stream<SyncStatus> syncStatusStream(String noteId) async* {
    final request = SyncRequest()..noteId = noteId;
    
    await for (final response in _client.syncTaskStatus(request)) {
      yield response.status;
    }
  }
}

// Task detection in editor
class TaskDetectionService {
  final _taskRegex = RegExp(
    r'(?:^|\n)\s*[-*]\s*\[\s*\]\s*(.+)|'
    r'(?:TODO|FIXME|ACTION|TASK):\s*(.+)|'
    r'(?:need to|have to|must|should)\s+(.+?)(?:\.|$)',
    multiLine: true,
    caseSensitive: false,
  );
  
  List<DetectedTask> detectTasks(String content, String noteId) {
    final tasks = <DetectedTask>[];
    
    for (final match in _taskRegex.allMatches(content)) {
      final taskContent = match.group(1) ?? match.group(2) ?? match.group(3) ?? '';
      
      tasks.add(DetectedTask(
        id: generateId(),
        content: taskContent.trim(),
        sourceNoteId: noteId,
        sourcePosition: match.start,
        detectedType: _getTaskType(match),
        confidenceScore: 0.9,
      ));
    }
    
    return tasks;
  }
}
```

### SurrealDB Schema

```sql
-- Task extraction mappings
DEFINE TABLE task_mappings SCHEMAFULL;
DEFINE FIELD note_task_id ON task_mappings TYPE string;
DEFINE FIELD guidance_quest_id ON task_mappings TYPE string;
DEFINE FIELD source_note_id ON task_mappings TYPE record<notes>;
DEFINE FIELD exported_at ON task_mappings TYPE datetime DEFAULT time::now();
DEFINE FIELD sync_status ON task_mappings TYPE string DEFAULT 'pending';
DEFINE FIELD last_synced ON task_mappings TYPE datetime;

-- Task extraction settings
DEFINE TABLE extraction_settings SCHEMAFULL;
DEFINE FIELD auto_detect ON extraction_settings TYPE bool DEFAULT true;
DEFINE FIELD use_ai ON extraction_settings TYPE bool DEFAULT true;
DEFINE FIELD keywords ON extraction_settings TYPE array<string>;
DEFINE FIELD min_confidence ON extraction_settings TYPE float DEFAULT 0.7;
DEFINE FIELD auto_export ON extraction_settings TYPE bool DEFAULT false;

-- Function to get pending tasks
DEFINE FUNCTION fn::get_pending_tasks($note_id: string) {
  RETURN SELECT * FROM task_mappings 
  WHERE source_note_id = $note_id 
  AND sync_status = 'pending';
};

-- Function to check for duplicates
DEFINE FUNCTION fn::check_task_duplicate($content: string, $note_id: string) {
  RETURN SELECT * FROM task_mappings 
  WHERE source_note_id = $note_id 
  AND similarity(content, $content) > 0.8;
};
```

## Testing Requirements

### Unit Tests

```dart
// test/features/task_extraction/domain/use_cases/detect_tasks_test.dart
void main() {
  group('DetectTasks', () {
    test('detects checkbox tasks', () {
      final content = '- [ ] Review Flutter docs\n- [ ] Update dependencies';
      final tasks = detectTasks(content, 'note1');
      
      expect(tasks.length, equals(2));
      expect(tasks[0].content, equals('Review Flutter docs'));
      expect(tasks[0].detectedType, equals(TaskType.checkbox));
    });
    
    test('detects TODO keywords', () {
      final content = 'TODO: Implement search feature';
      final tasks = detectTasks(content, 'note1');
      
      expect(tasks.length, equals(1));
      expect(tasks[0].content, contains('Implement search feature'));
    });
    
    test('extracts deadlines', () {
      final content = '- [ ] Submit report by Friday';
      final tasks = detectTasks(content, 'note1');
      
      expect(tasks[0].deadline, isNotNull);
    });
  });
}
```

### Widget Tests

```dart
// test/features/task_extraction/presentation/widgets/task_export_dialog_test.dart
void main() {
  testWidgets('Export dialog shows detected tasks', (tester) async {
    final tasks = [
      DetectedTask(content: 'Task 1'),
      DetectedTask(content: 'Task 2'),
    ];
    
    await tester.pumpWidget(TaskExportDialog(tasks: tasks));
    
    expect(find.text('Task 1'), findsOneWidget);
    expect(find.text('Task 2'), findsOneWidget);
    expect(find.text('Export to Guidance'), findsOneWidget);
  });
}
```

### Integration Tests

```dart
// integration_test/task_extraction_flow_test.dart
void main() {
  testWidgets('Complete task extraction and export flow', (tester) async {
    await tester.pumpWidget(MyApp());
    
    // Type a task in note
    await tester.enterText(
      find.byType(NoteEditor),
      '- [ ] Review documentation\nTODO: Update tests',
    );
    await tester.pumpAndSettle();
    
    // Verify tasks detected
    expect(find.byType(TaskDetectionIndicator), findsNWidgets(2));
    
    // Open export dialog
    await tester.tap(find.byIcon(Icons.export));
    await tester.pumpAndSettle();
    
    // Export tasks
    await tester.tap(find.text('Export All'));
    await tester.pumpAndSettle();
    
    // Verify success
    expect(find.text('2 tasks exported'), findsOneWidget);
  });
}
```

### Accessibility Tests

- [ ] Task indicators visible to screen reader
- [ ] Export dialog keyboard navigable
- [ ] Status updates announced
- [ ] Color contrast meets standards

## Definition of Done

- [ ] Tasks detected from multiple formats
- [ ] AI extraction works accurately
- [ ] Export to Guidance successful
- [ ] Energy levels inferred correctly
- [ ] Deadlines parsed accurately
- [ ] Duplicate detection works
- [ ] Status sync bidirectional
- [ ] All tests passing (>80% coverage)
- [ ] Accessibility audit complete
- [ ] Performance targets met
- [ ] Documentation updated
- [ ] Code review approved
