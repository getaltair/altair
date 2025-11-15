# Feature SHARED-003: AI Task Breakdown Service

## What it does

Provides intelligent task decomposition using local or cloud LLMs to break down complex quests into manageable subquests, estimate energy requirements, and suggest optimal task sequences for ADHD-friendly execution.

## User Journey

GIVEN user creates a complex quest like "Build a personal website"
WHEN user clicks "Break Down with AI" button
THEN system generates structured subquests with energy estimates and logical ordering

## Functional Requirements

- Natural language quest parsing
- Multi-level breakdown (Epic → Quest → Subquest)
- Energy estimation per task (1-5 scale)
- Dependency detection and ordering
- Time estimation (optional, not default)
- Context awareness from previous tasks
- Multiple breakdown strategies (detailed/simple)
- Batch processing for multiple quests
- Template learning from user patterns
- Local LLM fallback option
- Provider abstraction (Claude, GPT, Local)
- Prompt optimization for ADHD

## UI/UX Requirements

### Components

- `BreakdownButton` - Trigger AI analysis
- `BreakdownPreview` - Show proposed subtasks
- `TaskTree` - Hierarchical visualization
- `EnergyEstimator` - AI-suggested energy levels
- `BreakdownEditor` - Modify AI suggestions
- `StrategySelector` - Choose breakdown style
- `ProviderSettings` - LLM configuration
- `BreakdownHistory` - Previous breakdowns
- `LoadingAnimation` - Processing indicator
- `ErrorRecovery` - Fallback options

### Visual Design

- **Layout:**
  - Button: Inline with quest, 120px width
  - Preview modal: 600px width, scrollable
  - Task tree: Indented hierarchy
  - Loading: Overlay with spinner
- **Colors:**
  - AI indicator: `#9C27B0` (Purple)
  - Processing: `#2196F3` (Blue pulse)
  - Success: `#4CAF50` (Green)
  - Energy colors: Match energy system
  - Error: `#F44336` (Red)
- **Typography:**
  - Button: 14px medium
  - Task titles: 16px regular
  - Energy: 12px with icon
  - Descriptions: 14px light
- **Iconography:**
  - AI: Sparkle/magic wand (16x16)
  - Breakdown: Tree structure icon
  - Energy: Lightning bolts
  - Dependencies: Arrow connectors
- **Borders/Shadows:**
  - Preview: 4px solid black border
  - Selected tasks: 2px highlight
  - Hover: Subtle shadow increase

### User Interactions

- **Input Methods:**
  - Click button to trigger
  - Checkbox to accept/reject tasks
  - Drag to reorder suggestions
  - Edit inline task names
- **Keyboard Shortcuts:**
  - `Ctrl+B`: Break down current
  - `Enter`: Accept all
  - `Escape`: Cancel breakdown
  - `Tab`: Navigate suggestions
- **Gestures:**
  - Swipe to dismiss suggestion
  - Long-press for alternatives
  - Pinch to zoom tree view
- **Feedback:**
  - Progressive task appearance
  - Success checkmarks
  - Energy visual indicators
  - Sound on completion

### State Management

- **Local State:**
  - Current breakdown
  - Selected suggestions
  - Edit mode states
  - Loading status
- **Global State:**
  ```dart
  final aiProviderProvider = StateProvider<AIProvider>
  final breakdownServiceProvider = Provider<BreakdownService>
  final activeBreakdownProvider = StateNotifierProvider<BreakdownNotifier, BreakdownState>
  final breakdownHistoryProvider = StateNotifierProvider<HistoryNotifier, List<Breakdown>>
  final aiSettingsProvider = StateProvider<AISettings>
  ```
- **Persistence:**
  - API keys encrypted
  - Breakdown history saved
  - User preferences stored
  - Templates cached

### Responsive Behavior

- **Desktop:** Side-by-side preview
- **Tablet:** Full-screen modal
- **Mobile:** Bottom sheet preview
- **Breakpoint Strategy:** Adaptive layout shifts

### Accessibility Requirements

- **Screen Reader:**
  - Announce breakdown progress
  - Read task hierarchy
  - Describe energy levels
- **Keyboard Navigation:**
  - Full keyboard control
  - Clear focus indicators
- **Color Contrast:** Text readable on all backgrounds
- **Motion:** Respect reduced motion
- **Font Sizing:** Minimum 12px

### ADHD-Specific UI Requirements

- **Cognitive Load:**
  - Default to simple breakdown
  - Maximum 7 subtasks shown
  - Hide complex dependencies
  - One-click acceptance
- **Focus Management:**
  - Auto-focus on results
  - Clear accept/reject
  - Minimal decisions required
- **Forgiveness:**
  - Easy to modify after
  - Undo breakdown option
  - Re-breakdown anytime
  - No API call limits shown
- **Visual Hierarchy:**
  - Most important tasks first
  - Energy clearly visible
  - Simple tree structure
- **Immediate Feedback:**
  - Stream results as available
  - Show progress percentage
  - Quick animations

## Non-Functional Requirements

### Performance Targets

- Local LLM response <2s
- Cloud LLM response <5s
- UI remains responsive
- Batch processing <10s for 10 items
- Cache hit response <100ms

### Technical Constraints

- Multiple LLM provider support
- 4K token context minimum
- Streaming response support
- Ollama integration for local
- Rate limiting compliance

### Security Requirements

- API keys encrypted
- No task data in logs
- Local processing option
- Secure key storage
- User consent for cloud

## Implementation Details

### Code Structure

```
lib/
├── core/
│   └── ai/
│       ├── breakdown_service.dart
│       ├── providers/
│       │   ├── ai_provider.dart
│       │   ├── claude_provider.dart
│       │   ├── openai_provider.dart
│       │   └── ollama_provider.dart
│       ├── models/
│       │   ├── breakdown_request.dart
│       │   ├── breakdown_response.dart
│       │   └── task_suggestion.dart
│       ├── prompts/
│       │   ├── breakdown_prompt.dart
│       │   └── energy_prompt.dart
│       └── widgets/
│           ├── breakdown_button.dart
│           ├── breakdown_preview.dart
│           └── task_tree.dart

backend/
├── src/
│   └── ai/
│       ├── mod.rs
│       ├── breakdown_service.rs
│       ├── providers/
│       │   ├── anthropic.rs
│       │   ├── openai.rs
│       │   └── ollama.rs
│       └── prompts.rs
└── proto/
    └── ai_breakdown.proto
```

### Key Files to Create

- `breakdown_service.dart` - Main service class
- `ai_provider.dart` - Provider abstraction
- `breakdown_prompt.dart` - ADHD-optimized prompts
- `breakdown_service.rs` - Rust AI service
- `ai_breakdown.proto` - gRPC definitions

### Dependencies

```yaml
dependencies:
  dart_openai: ^5.0.0
  anthropic_sdk_dart: ^0.0.3
  http: ^1.1.0
  flutter_riverpod: ^2.4.0
  
dev_dependencies:
  mockito: ^5.4.0
```

### Rust Dependencies

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
async-openai = "0.23"
ollama-rs = "0.1"
anyhow = "1.0"
```

### Prompt Template

```dart
const String breakdownPrompt = '''
You are an ADHD-friendly task breakdown assistant. Break down this task into 
manageable subtasks following these rules:

1. Maximum 7 subtasks (cognitive load management)
2. Each subtask should be completable in one focused session
3. Assign energy levels 1-5 (1=low energy, 5=high energy)
4. Order by logical sequence and dependencies
5. Use clear, action-oriented language
6. Avoid time estimates (causes anxiety)

Task: {task_description}
Context: {user_context}

Output format:
- Subtask title (Energy: N)
  - Optional brief description

Be encouraging and break complex tasks into very small, concrete steps.
''';
```

## Testing Requirements

### Unit Tests

- [ ] Prompt generation
- [ ] Response parsing
- [ ] Energy estimation logic
- [ ] Provider switching
- [ ] Error handling

### Widget Tests

- [ ] Breakdown button states
- [ ] Preview modal interaction
- [ ] Task selection
- [ ] Loading states
- [ ] Error display

### Integration Tests

- [ ] Full breakdown flow
- [ ] Multiple provider testing
- [ ] Offline fallback
- [ ] Batch processing
- [ ] Template learning

### Accessibility Tests

- [ ] Screen reader flow
- [ ] Keyboard navigation
- [ ] Focus management
- [ ] Motion preferences

## Definition of Done

- [ ] AI breakdown functional
- [ ] Multiple providers working
- [ ] Energy estimation accurate
- [ ] ADHD-optimized prompts
- [ ] Local LLM option available
- [ ] Streaming responses working
- [ ] Error handling robust
- [ ] Cache system operational
- [ ] All tests passing
- [ ] Performance targets met
