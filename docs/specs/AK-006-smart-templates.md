# Feature AK-006: Smart Templates

## What it does

Provides AI-powered note templates with dynamic fields, context-aware suggestions, and automatic content generation to reduce blank page anxiety and accelerate structured note creation.

## User Journey

GIVEN a user wants to create a meeting notes document
WHEN they select the "Meeting Notes" template
THEN a pre-structured note is created with AI-suggested attendees, agenda items from calendar, and action item tracking

## Functional Requirements

- Template library with categories
- AI field suggestions based on context
- Dynamic variable replacement
- Custom template creation
- Template sharing/import
- Conditional sections
- Auto-fill from calendar/email
- Template inheritance
- Version control for templates
- Template analytics (usage stats)
- Quick template switcher
- Template preview before apply
- Batch template application

## UI/UX Requirements

### Components

```dart
// Smart template components
TemplateGallery
TemplateCard
TemplateEditor
TemplatePreview
DynamicFieldInput
TemplateVariablePanel
AIFieldSuggestion
TemplateWizard
TemplateVersionHistory
TemplateCategorySelector
```

### Visual Design

- **Layout:**
  - Template gallery: Grid 3 columns
  - Template editor: Split view (code/preview)
  - Variable panel: Right sidebar 320px
  - Preview modal: 800x600px centered
  - Quick switcher: Floating dropdown
  
- **Colors:**
  ```dart
  templateCard: Color(0xFFF3F4F6), // Light gray background
  templateActive: Color(0xFF6366F1), // Indigo for selected
  variableHighlight: Color(0xFFFBBF24), // Yellow for variables
  aiSuggestion: Color(0xFF10B981), // Green for AI content
  customTemplate: Color(0xFF8B5CF6), // Purple for custom
  ```
  
- **Typography:**
  - Template title: 16px semibold
  - Description: 14px regular
  - Variable names: 14px monospace
  - Preview text: 16px regular
  
- **Iconography:**
  - Template: document-text icon 24px
  - AI suggestion: sparkles icon 18px
  - Variable: code icon 16px
  - Category: folder icon 20px
  
- **Borders/Shadows:**
  - Template cards: 3px border, hover shadow
  - Active template: 6px shadow offset
  - Editor panels: 2px divider

### User Interactions

- **Input Methods:**
  - Click to select template
  - Type in variable fields
  - Drag to reorder sections
  - Voice input for fields
  
- **Keyboard Shortcuts:**
  - `Ctrl+Shift+T`: Open template gallery
  - `Ctrl+Alt+T`: Quick template switch
  - `Tab`: Navigate fields
  - `Ctrl+Space`: AI suggestions
  - `Ctrl+Enter`: Apply template
  
- **Gestures:**
  - Swipe to browse templates
  - Long-press for preview
  - Pinch to zoom preview
  
- **Feedback:**
  - Template applied toast
  - AI generation spinner
  - Field validation indicators
  - Usage count badges

### State Management

```dart
// Riverpod providers
final templateLibraryProvider = FutureProvider<List<Template>>((ref) async {
  return ref.read(templateServiceProvider).getAllTemplates();
});

final selectedTemplateProvider = StateProvider<Template?>((ref) => null);

final templateVariablesProvider = StateNotifierProvider<TemplateVariablesNotifier, Map<String, dynamic>>(
  (ref) => TemplateVariablesNotifier(),
);

final aiSuggestionsProvider = FutureProvider.family<Map<String, String>, Template>((ref, template) async {
  return ref.read(aiServiceProvider).generateSuggestions(template);
});

final customTemplatesProvider = StateNotifierProvider<CustomTemplatesNotifier, List<Template>>(
  (ref) => CustomTemplatesNotifier(),
);

final templateUsageStatsProvider = StreamProvider<TemplateStats>((ref) {
  return ref.read(templateServiceProvider).usageStatsStream;
});
```

### Responsive Behavior

- **Desktop (>1200px):**
  - Full gallery with previews
  - Side-by-side editor
  - Multiple template comparison
  
- **Tablet (768-1199px):**
  - 2-column gallery
  - Stacked editor views
  - Touch-optimized
  
- **Mobile (<768px):**
  - Single column list
  - Full-screen editor
  - Bottom sheet variables

### Accessibility Requirements

- **Screen Reader:**
  - Template descriptions read
  - Variable fields announced
  - AI suggestions explained
  
- **Keyboard Navigation:**
  - Tab through all fields
  - Arrow keys in gallery
  - Enter to apply
  
- **Color Contrast:**
  - Variable highlighting accessible
  - Alternative indicators
  
- **Motion:**
  - Optional transitions
  - Static preview option
  
- **Font Sizing:**
  - Scalable template text
  - Readable variable names

### ADHD-Specific UI Requirements

- **Cognitive Load:**
  - Favorites/recent at top
  - Max 9 templates visible
  - Progressive field disclosure
  - Smart defaults for all fields
  
- **Focus Management:**
  - Auto-focus first field
  - Clear field progression
  - Skip optional fields option
  
- **Forgiveness:**
  - Undo template application
  - Reset to defaults
  - Save as draft anytime
  - Template history
  
- **Visual Hierarchy:**
  - Required fields highlighted
  - Optional fields dimmed
  - AI suggestions prominent
  
- **Immediate Feedback:**
  - Instant preview update
  - Live variable replacement
  - Quick template switch
  - One-click application

## Non-Functional Requirements

### Performance Targets

- Template loading: <200ms
- AI suggestions: <2 seconds
- Variable replacement: instant
- Preview generation: <100ms
- Template search: <50ms

### Technical Constraints

- Flutter version: 3.16+
- Handlebars-style templating
- Local AI model support
- Markdown/YAML templates
- Git-based version control

### Security Requirements

- Template input sanitization
- Secure variable evaluation
- No code execution in templates
- Access control for shared templates

## Implementation Details

### Code Structure

```
lib/features/smart_templates/
├── presentation/
│   ├── widgets/
│   │   ├── template_gallery.dart
│   │   ├── template_card.dart
│   │   ├── template_editor.dart
│   │   ├── dynamic_field_input.dart
│   │   └── ai_suggestion_chip.dart
│   ├── providers/
│   │   ├── template_provider.dart
│   │   ├── ai_suggestions_provider.dart
│   │   └── template_variables_provider.dart
│   └── screens/
│       ├── template_gallery_screen.dart
│       └── template_editor_screen.dart
├── domain/
│   ├── models/
│   │   ├── template.dart
│   │   ├── template_variable.dart
│   │   └── template_category.dart
│   ├── repositories/
│   │   └── template_repository.dart
│   └── use_cases/
│       ├── apply_template.dart
│       ├── generate_suggestions.dart
│       └── create_custom_template.dart
└── data/
    ├── services/
    │   ├── template_engine_service.dart
    │   └── ai_template_service.dart
    └── repositories/
        └── template_repository_impl.dart
```

### Rust Backend Service

```rust
// rust-backend/services/templates/src/lib.rs
use axum::{Router, Json, Extension};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Local};

#[derive(Serialize, Deserialize)]
struct Template {
    id: String,
    name: String,
    description: String,
    category: TemplateCategory,
    content: String,
    variables: Vec<TemplateVariable>,
    metadata: TemplateMetadata,
}

#[derive(Serialize, Deserialize)]
struct TemplateVariable {
    name: String,
    var_type: VariableType,
    required: bool,
    default_value: Option<String>,
    ai_generated: bool,
    description: String,
}

#[derive(Serialize, Deserialize)]
enum VariableType {
    Text,
    Date,
    List,
    Boolean,
    Number,
    Select(Vec<String>),
}

pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
    ai_service: AIService,
}

impl TemplateEngine {
    pub async fn apply_template(
        &self,
        template: &Template,
        variables: HashMap<String, Value>,
        context: &NoteContext,
    ) -> Result<String, Error> {
        // Merge provided variables with AI suggestions
        let mut final_vars = variables.clone();
        
        // Generate AI suggestions for missing variables
        for var in &template.variables {
            if !final_vars.contains_key(&var.name) && var.ai_generated {
                let suggestion = self.ai_service
                    .generate_field_content(&var, context)
                    .await?;
                final_vars.insert(var.name.clone(), suggestion);
            }
        }
        
        // Add system variables
        final_vars.insert("date".to_string(), Value::String(Local::now().format("%Y-%m-%d").to_string()));
        final_vars.insert("time".to_string(), Value::String(Local::now().format("%H:%M").to_string()));
        final_vars.insert("day_of_week".to_string(), Value::String(Local::now().format("%A").to_string()));
        
        // Render template
        let rendered = self.handlebars.render_template(&template.content, &final_vars)?;
        
        // Post-process with AI if needed
        if template.metadata.ai_enhancement {
            let enhanced = self.ai_service.enhance_content(&rendered, context).await?;
            Ok(enhanced)
        } else {
            Ok(rendered)
        }
    }
    
    pub async fn generate_suggestions(
        &self,
        template: &Template,
        context: &NoteContext,
    ) -> Result<HashMap<String, String>, Error> {
        let mut suggestions = HashMap::new();
        
        // Analyze context (calendar, recent notes, email)
        let calendar_events = context.get_calendar_events().await?;
        let recent_notes = context.get_recent_notes(7).await?;
        
        for variable in &template.variables {
            let suggestion = match variable.var_type {
                VariableType::List => {
                    // Generate list items based on context
                    self.ai_service.generate_list_items(&variable.name, context).await?
                }
                VariableType::Text => {
                    // Generate contextual text
                    self.ai_service.generate_text_content(&variable.name, context).await?
                }
                VariableType::Date => {
                    // Suggest relevant date
                    self.suggest_date_from_context(&variable.name, &calendar_events)?
                }
                _ => variable.default_value.clone().unwrap_or_default(),
            };
            
            suggestions.insert(variable.name.clone(), suggestion);
        }
        
        Ok(suggestions)
    }
    
    pub async fn create_custom_template(
        &self,
        name: String,
        content: String,
        example_note: Option<String>,
    ) -> Result<Template, Error> {
        // Parse content to extract variables
        let variables = self.extract_variables(&content)?;
        
        // If example provided, learn patterns
        let enhanced_template = if let Some(example) = example_note {
            self.ai_service.learn_from_example(&content, &example).await?
        } else {
            content
        };
        
        // Create template
        let template = Template {
            id: generate_id(),
            name,
            description: self.ai_service.generate_description(&enhanced_template).await?,
            category: TemplateCategory::Custom,
            content: enhanced_template,
            variables,
            metadata: TemplateMetadata::default(),
        };
        
        Ok(template)
    }
    
    fn extract_variables(&self, content: &str) -> Result<Vec<TemplateVariable>, Error> {
        let var_regex = Regex::new(r"\{\{([^}]+)\}\}").unwrap();
        let mut variables = Vec::new();
        
        for cap in var_regex.captures_iter(content) {
            let var_name = cap.get(1).unwrap().as_str();
            
            // Infer type from name patterns
            let var_type = self.infer_variable_type(var_name);
            
            variables.push(TemplateVariable {
                name: var_name.to_string(),
                var_type,
                required: !var_name.contains("optional"),
                default_value: None,
                ai_generated: true,
                description: format!("Field for {}", var_name),
            });
        }
        
        Ok(variables)
    }
}
```

### gRPC Service Definition

```proto
// protos/smart_templates.proto
syntax = "proto3";
package altair.knowledge.templates;

service TemplateService {
  rpc GetTemplates(GetTemplatesRequest) returns (TemplateList);
  rpc ApplyTemplate(ApplyTemplateRequest) returns (AppliedTemplate);
  rpc GenerateSuggestions(GenerateSuggestionsRequest) returns (SuggestionMap);
  rpc CreateCustomTemplate(CreateTemplateRequest) returns (Template);
  rpc GetTemplateUsage(GetUsageRequest) returns (TemplateStats);
  rpc PreviewTemplate(PreviewRequest) returns (PreviewResult);
  rpc ShareTemplate(ShareTemplateRequest) returns (ShareResult);
}

message Template {
  string id = 1;
  string name = 2;
  string description = 3;
  TemplateCategory category = 4;
  string content = 5;
  repeated TemplateVariable variables = 6;
  TemplateMetadata metadata = 7;
}

message TemplateVariable {
  string name = 1;
  VariableType type = 2;
  bool required = 3;
  string default_value = 4;
  bool ai_generated = 5;
  string description = 6;
  repeated string options = 7; // For select type
}

message ApplyTemplateRequest {
  string template_id = 1;
  map<string, string> variables = 2;
  NoteContext context = 3;
  bool use_ai_suggestions = 4;
}

message NoteContext {
  string current_note_id = 1;
  repeated string recent_note_ids = 2;
  repeated CalendarEvent upcoming_events = 3;
  repeated string recent_tags = 4;
  string user_timezone = 5;
}

enum TemplateCategory {
  MEETING = 0;
  PROJECT = 1;
  DAILY = 2;
  WEEKLY = 3;
  RESEARCH = 4;
  PERSONAL = 5;
  CUSTOM = 6;
}

enum VariableType {
  TEXT = 0;
  DATE = 1;
  LIST = 2;
  BOOLEAN = 3;
  NUMBER = 4;
  SELECT = 5;
  MULTILINE = 6;
}
```

### Template Examples

```yaml
# Meeting Notes Template
name: Meeting Notes
category: meeting
variables:
  - name: meeting_title
    type: text
    required: true
    ai_generated: true
  - name: attendees
    type: list
    ai_generated: true
  - name: agenda_items
    type: list
    ai_generated: true
  - name: action_items
    type: list
    default: "- [ ] "
content: |
  # {{meeting_title}}
  **Date:** {{date}}
  **Time:** {{time}}
  
  ## Attendees
  {{#each attendees}}
  - {{this}}
  {{/each}}
  
  ## Agenda
  {{#each agenda_items}}
  1. {{this}}
  {{/each}}
  
  ## Discussion Notes
  
  
  ## Action Items
  {{#each action_items}}
  {{this}}
  {{/each}}
  
  ## Next Steps
  
  
  ---
  *Meeting notes for [[{{meeting_title}}]]*
```

### SurrealDB Schema

```sql
-- Templates table
DEFINE TABLE templates SCHEMAFULL;
DEFINE FIELD name ON templates TYPE string;
DEFINE FIELD description ON templates TYPE string;
DEFINE FIELD category ON templates TYPE string;
DEFINE FIELD content ON templates TYPE string;
DEFINE FIELD variables ON templates TYPE array;
DEFINE FIELD usage_count ON templates TYPE int DEFAULT 0;
DEFINE FIELD created_at ON templates TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON templates TYPE datetime DEFAULT time::now();
DEFINE FIELD is_custom ON templates TYPE bool DEFAULT false;
DEFINE FIELD owner ON templates TYPE string;

-- Template usage tracking
DEFINE TABLE template_usage SCHEMAFULL;
DEFINE FIELD template_id ON template_usage TYPE record<templates>;
DEFINE FIELD used_at ON template_usage TYPE datetime DEFAULT time::now();
DEFINE FIELD variables_used ON template_usage TYPE object;
DEFINE FIELD ai_suggestions_accepted ON template_usage TYPE int;

-- Function to get popular templates
DEFINE FUNCTION fn::get_popular_templates($limit: int) {
  RETURN SELECT *, usage_count 
  FROM templates 
  ORDER BY usage_count DESC 
  LIMIT $limit;
};

-- Function to get user's custom templates
DEFINE FUNCTION fn::get_custom_templates($user_id: string) {
  RETURN SELECT * FROM templates 
  WHERE owner = $user_id AND is_custom = true
  ORDER BY updated_at DESC;
};
```

## Testing Requirements

### Unit Tests

```dart
// test/features/smart_templates/domain/use_cases/apply_template_test.dart
void main() {
  group('ApplyTemplate', () {
    test('applies template with variables', () async {
      final template = Template(
        content: 'Hello {{name}}, today is {{date}}',
        variables: [
          TemplateVariable(name: 'name', type: VariableType.text),
        ],
      );
      
      final result = await applyTemplate(template, {'name': 'John'});
      expect(result, contains('Hello John'));
      expect(result, contains(DateFormat('yyyy-MM-dd').format(DateTime.now())));
    });
    
    test('generates AI suggestions for missing variables', () async {
      // Test AI generation
    });
  });
}
```

### Widget Tests

```dart
// test/features/smart_templates/presentation/widgets/template_gallery_test.dart
void main() {
  testWidgets('Template gallery displays categories', (tester) async {
    await tester.pumpWidget(TemplateGallery());
    
    expect(find.text('Meeting'), findsOneWidget);
    expect(find.text('Project'), findsOneWidget);
    expect(find.text('Daily'), findsOneWidget);
  });
}
```

### Integration Tests

```dart
// integration_test/template_flow_test.dart
void main() {
  testWidgets('Complete template application flow', (tester) async {
    await tester.pumpWidget(MyApp());
    
    // Open template gallery
    await tester.tap(find.byIcon(Icons.description));
    await tester.pumpAndSettle();
    
    // Select meeting template
    await tester.tap(find.text('Meeting Notes'));
    await tester.pumpAndSettle();
    
    // Fill in variables
    await tester.enterText(find.byKey(Key('meeting_title')), 'Sprint Planning');
    
    // Apply template
    await tester.tap(find.text('Apply'));
    await tester.pumpAndSettle();
    
    // Verify note created
    expect(find.text('Sprint Planning'), findsOneWidget);
  });
}
```

### Accessibility Tests

- [ ] Template gallery keyboard navigable
- [ ] Variable fields properly labeled
- [ ] AI suggestions announced
- [ ] Color contrast meets standards

## Definition of Done

- [ ] Template gallery loads with categories
- [ ] Variables extracted and highlighted
- [ ] AI suggestions generated accurately
- [ ] Custom templates can be created
- [ ] Template preview works
- [ ] Usage statistics tracked
- [ ] All tests passing (>80% coverage)
- [ ] Accessibility audit complete
- [ ] Performance targets met
- [ ] Documentation updated
- [ ] Code review approved
