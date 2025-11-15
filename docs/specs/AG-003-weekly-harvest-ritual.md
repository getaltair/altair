# Feature AG-003: Weekly Harvest Ritual

## What it does

Facilitates scheduled weekly reflection sessions where users review completed quests, celebrate progress, archive finished work, and adaptively plan the next cycle without shame or judgment.

## User Journey

GIVEN it's the user's scheduled harvest day (default Sunday evening)
WHEN user receives the harvest ritual notification
THEN user is guided through a structured reflection process that reviews achievements, archives completed work, and plans the next cycle

## Functional Requirements

- Configurable weekly schedule (day/time)
- Multi-step guided reflection wizard
- Achievement celebration animations
- Automatic quest archival system
- Next cycle planning interface
- Progress metrics visualization
- Failure recovery suggestions
- Adaptive replanning without penalties
- Harvest history tracking
- Export harvest summaries
- Skip/postpone options with no guilt
- Integration with energy patterns

## UI/UX Requirements

### Components

- `HarvestWizard` - Multi-step ritual interface
- `AchievementCelebration` - Success animation component
- `QuestReviewCard` - Individual quest reflection
- `CycleMetrics` - Progress visualization
- `ArchiveConfirmation` - Batch archive interface
- `NextCyclePlanner` - Planning board
- `FailureRecoveryPanel` - Adaptive suggestions
- `HarvestSummary` - Exportable summary view
- `ScheduleSettings` - Ritual timing configuration
- `HarvestHistory` - Previous harvests browser

### Visual Design

- **Layout:**
  - Wizard: Full-screen modal, 800px max-width
  - Steps: Horizontal progress bar at top
  - Cards: Grid layout, 3 columns desktop
  - Metrics: Dashboard style, 2-column split
- **Colors:**
  - Success: `#4CAF50` (Green)
  - In-progress: `#2196F3` (Blue)
  - Incomplete: `#FFC107` (Yellow, not red)
  - Celebration: Rainbow gradient animation
  - Background: `#FAFAFA` with grain texture
- **Typography:**
  - Headers: 24px bold
  - Celebration text: 32px with animation
  - Body: 16px regular
  - Metrics: 20px medium
- **Iconography:**
  - Harvest: Wheat/grain icon (32x32)
  - Trophy: Achievement celebration
  - Archive: Box with arrow
  - Calendar: Next cycle planning
  - Heart: Self-compassion reminders
- **Borders/Shadows:**
  - Wizard container: 4px solid black
  - Card hover: 6px offset shadow
  - Celebration: Particle effects

### User Interactions

- **Input Methods:**
  - Click through wizard steps
  - Drag to reorder next cycle
  - Type reflection notes
  - Voice notes option
- **Keyboard Shortcuts:**
  - `H`: Open harvest ritual
  - `Enter`: Next step
  - `Escape`: Save and exit
  - `Tab`: Navigate cards
  - `Space`: Toggle selection
- **Gestures:**
  - Swipe between steps (mobile)
  - Pull-to-refresh metrics
  - Long-press to archive
  - Pinch to zoom timeline
- **Feedback:**
  - Confetti on achievements
  - Gentle chime sounds
  - Progress saved indicator
  - Encouraging messages

### State Management

- **Local State:**
  - Current wizard step
  - Selected quests for archive
  - Reflection notes
  - Next cycle selections
- **Global State:**
  ```dart
  final harvestWizardProvider = StateNotifierProvider<HarvestWizardNotifier, HarvestState>
  final harvestHistoryProvider = StateNotifierProvider<HarvestHistoryNotifier, List<HarvestRecord>>
  final cycleMetricsProvider = FutureProvider<CycleMetrics>
  final nextCycleProvider = StateProvider<List<Quest>>
  final harvestScheduleProvider = StateProvider<HarvestSchedule>
  ```
- **Persistence:**
  - Harvest records saved permanently
  - Reflection notes encrypted
  - Metrics cached for performance
  - Schedule preferences saved

### Responsive Behavior

- **Desktop:** Full wizard with side panels
- **Tablet:** Stacked layout, swipe navigation
- **Mobile:** Single card per screen, vertical flow
- **Breakpoint Strategy:** Progressive simplification

### Accessibility Requirements

- **Screen Reader:**
  - Step announcements
  - Achievement descriptions
  - Metric summaries spoken
- **Keyboard Navigation:**
  - Full wizard keyboard control
  - Skip links between sections
- **Color Contrast:** Non-color achievement indicators
- **Motion:** Reduced motion option for celebrations
- **Font Sizing:** Adjustable text size preference

### ADHD-Specific UI Requirements

- **Cognitive Load:**
  - One step at a time
  - Optional steps clearly marked
  - Auto-save at each step
  - Maximum 5 items per screen
- **Focus Management:**
  - Clear current step indicator
  - Disable outside clicks
  - Auto-focus next action
- **Forgiveness:**
  - Skip entire ritual option
  - Reschedule with one click
  - No streak breaking
  - Partial completion saved
- **Visual Hierarchy:**
  - Primary action always green
  - Secondary actions muted
  - Progress clearly shown
- **Immediate Feedback:**
  - Step completion animation
  - Auto-save confirmation
  - Instant metric updates

## Non-Functional Requirements

### Performance Targets

- Wizard load <300ms
- Step transition <150ms
- Metrics calculation <500ms
- Archive operation <1s for 50 items
- Export generation <2s

### Technical Constraints

- Flutter 3.16+ with animations
- SurrealDB for harvest records
- Rust backend for metrics calculation
- gRPC for real-time sync
- PDF generation for exports

### Security Requirements

- Reflection notes encrypted
- Export authentication required
- Audit trail for archives
- Privacy mode for sensitive content

## Implementation Details

### Code Structure

```
lib/
├── features/
│   └── harvest_ritual/
│       ├── presentation/
│       │   ├── widgets/
│       │   │   ├── harvest_wizard.dart
│       │   │   ├── achievement_celebration.dart
│       │   │   ├── quest_review_card.dart
│       │   │   ├── cycle_metrics.dart
│       │   │   ├── archive_confirmation.dart
│       │   │   └── next_cycle_planner.dart
│       │   ├── providers/
│       │   │   ├── harvest_wizard_provider.dart
│       │   │   ├── metrics_provider.dart
│       │   │   └── schedule_provider.dart
│       │   └── screens/
│       │       ├── harvest_screen.dart
│       │       └── harvest_history_screen.dart
│       ├── domain/
│       │   ├── entities/
│       │   │   ├── harvest_record.dart
│       │   │   ├── cycle_metrics.dart
│       │   │   └── reflection_note.dart
│       │   └── repositories/
│       │       └── harvest_repository.dart
│       └── data/
│           ├── models/
│           │   └── harvest_model.dart
│           └── datasources/
│               └── harvest_datasource.dart

backend/
├── src/
│   └── services/
│       ├── harvest_service.rs
│       └── metrics_calculator.rs
└── proto/
    └── harvest.proto
```

### Key Files to Create

- `harvest_wizard.dart` - Main wizard component
- `harvest_wizard_provider.dart` - Wizard state management
- `harvest_service.rs` - Rust backend service
- `harvest.proto` - gRPC definitions
- `metrics_calculator.rs` - Analytics engine

### Dependencies

```yaml
dependencies:
  flutter_riverpod: ^2.4.0
  step_progress_indicator: ^1.0.2
  confetti: ^0.7.0
  pdf: ^3.10.0
  flutter_local_notifications: ^15.1.0
  lottie: ^2.6.0
  
dev_dependencies:
  golden_toolkit: ^0.15.0
  patrol_test: ^3.0.0
```

### Rust Dependencies

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
surrealdb = "2.0"
tonic = "0.12"
chrono = "0.4"
serde = { version = "1.0", features = ["derive"] }
printpdf = "0.7"
```

## Testing Requirements

### Unit Tests

- [ ] Wizard step navigation logic
- [ ] Archive selection rules
- [ ] Metrics calculation accuracy
- [ ] Schedule configuration
- [ ] Export generation

### Widget Tests

- [ ] Wizard flow completion
- [ ] Card selection states
- [ ] Animation triggers
- [ ] Form validation
- [ ] Responsive layouts

### Integration Tests

- [ ] Complete harvest ritual flow
- [ ] Archive → Confirmation → Success
- [ ] Metrics → Export PDF
- [ ] Schedule → Notification → Open
- [ ] Skip → Resume later

### Accessibility Tests

- [ ] Screen reader wizard navigation
- [ ] Keyboard-only completion
- [ ] Reduced motion mode
- [ ] High contrast verification

## Definition of Done

- [ ] Wizard fully functional
- [ ] Celebration animations working
- [ ] Archive system operational
- [ ] Metrics calculating correctly
- [ ] Export to PDF working
- [ ] Schedule notifications active
- [ ] History view complete
- [ ] All tests passing
- [ ] Accessibility verified
- [ ] Backend integration complete
