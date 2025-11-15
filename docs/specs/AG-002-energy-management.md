# Feature AG-002: Energy Management System

## What it does

Tracks daily energy levels (1-5 scale) with spoon theory integration, filters tasks by energy requirements, and provides pattern recognition to optimize task scheduling based on personal energy patterns.

## User Journey

GIVEN user opens Altair Guidance for the day
WHEN user completes morning energy check-in
THEN system filters available quests to match current energy level and suggests optimal tasks

## Functional Requirements

- Daily energy check-in modal on first app open
- 1-5 scale energy rating with visual representations
- Spoon allocation system (default 12 spoons/day, customizable)
- Task energy requirement assignment (1-5 energy points)
- Energy pattern tracking over time (weekly/monthly views)
- Automatic task filtering based on current energy
- Energy prediction based on historical patterns
- Low energy mode activation (<2 energy)
- Energy boost tracking (coffee, medication, exercise)
- Rest period recommendations
- Energy debt/surplus tracking

## UI/UX Requirements

### Components

- `EnergyCheckInModal` - Daily check-in interface
- `SpoonCounter` - Visual spoon display
- `EnergyGraph` - Historical energy visualization
- `EnergyFilter` - Task filter component
- `EnergyBadge` - Energy requirement indicator
- `EnergyTrendCard` - Pattern insights display
- `LowEnergyBanner` - Warning/suggestion banner
- `EnergyBoostTracker` - Quick boost logging
- `RestTimer` - Break recommendation widget
- `EnergyCalendar` - Monthly energy heatmap

### Visual Design

- **Layout:** 
  - Check-in: Centered modal, 400px width
  - Dashboard: Energy widget in top-right, 200px
  - Graph: Full-width card, 300px height
- **Colors:**
  - Energy 1: `#FF5252` (Critical Red)
  - Energy 2: `#FF9800` (Low Orange) 
  - Energy 3: `#FFC107` (Medium Yellow)
  - Energy 4: `#8BC34A` (Good Green)
  - Energy 5: `#4CAF50` (Peak Green)
  - Spoons: `#9C27B0` (Purple)
- **Typography:**
  - Energy level: 48px bold
  - Spoon count: 24px medium
  - Insights: 14px regular
- **Iconography:**
  - Battery icons for energy (24x24)
  - Spoon icons for spoon theory (20x20)
  - Lightning bolt for energy boosts
  - Moon icon for rest periods
- **Borders/Shadows:**
  - Energy cards: 3px solid border matching energy color
  - Pulsing glow animation for low energy warnings

### User Interactions

- **Input Methods:**
  - Slider for energy selection
  - Click spoons to allocate/deallocate
  - Tap energy badges to filter
  - Voice input for check-in (optional)
- **Keyboard Shortcuts:**
  - `E`: Open energy check-in
  - `1-5`: Quick energy selection
  - `S`: Toggle spoon view
  - `R`: Start rest timer
- **Gestures:**
  - Swipe up/down to adjust energy
  - Long-press for energy history
  - Shake to randomize low-energy task
- **Feedback:**
  - Haptic feedback on energy selection
  - Sound effect for spoon allocation
  - Celebration animation for streak

### State Management

- **Local State:**
  - Current energy level
  - Today's spoon count
  - Active energy filter
  - Rest timer state
- **Global State:**
  ```dart
  final currentEnergyProvider = StateProvider<int>
  final spoonProvider = StateNotifierProvider<SpoonNotifier, SpoonState>
  final energyHistoryProvider = StateNotifierProvider<EnergyHistoryNotifier, List<EnergyRecord>>
  final energyPatternsProvider = FutureProvider<EnergyPatterns>
  final energyFilterProvider = StateProvider<EnergyFilter>
  ```
- **Persistence:**
  - Energy check-ins saved with timestamp
  - Spoon allocations tracked per task
  - Pattern analysis cached daily
  - Boost events logged

### Responsive Behavior

- **Desktop:** Side panel with full graph, detailed insights
- **Tablet:** Collapsible energy widget, simplified graph
- **Mobile:** Bottom sheet check-in, swipe for history
- **Breakpoint Strategy:** Adaptive layouts per platform

### Accessibility Requirements

- **Screen Reader:**
  - Announce energy level changes
  - Describe spoon allocation
  - Read pattern insights
- **Keyboard Navigation:**
  - Arrow keys for energy selection
  - Tab through spoon allocation
- **Color Contrast:** Icons/symbols supplement colors
- **Motion:** Optional animations for energy changes
- **Font Sizing:** Large touch targets for selection

### ADHD-Specific UI Requirements

- **Cognitive Load:**
  - Simple 1-5 scale, no decimals
  - Visual metaphors (battery, spoons)
  - One-click check-in option
- **Focus Management:**
  - Auto-dismiss after selection
  - Minimal required fields
  - Skip option available
- **Forgiveness:**
  - Edit today's check-in anytime
  - Retroactive check-ins allowed
  - No penalty for missing days
- **Visual Hierarchy:**
  - Current energy always visible
  - Color coding throughout app
  - Clear low-energy warnings
- **Immediate Feedback:**
  - Instant task filtering
  - Live spoon count updates
  - Quick celebration animations

## Non-Functional Requirements

### Performance Targets

- Check-in modal open <100ms
- Energy filtering <50ms
- Pattern analysis <500ms
- Graph rendering <200ms
- History load <300ms

### Technical Constraints

- Flutter 3.16+ with state persistence
- SurrealDB for time-series data
- Rust backend for pattern analysis
- gRPC for real-time updates
- Local caching for offline access

### Security Requirements

- Energy data encrypted at rest
- No PII in analytics
- Optional data export only
- User-controlled sharing

## Implementation Details

### Code Structure

```
lib/
├── features/
│   └── energy_management/
│       ├── presentation/
│       │   ├── widgets/
│       │   │   ├── energy_check_in_modal.dart
│       │   │   ├── spoon_counter.dart
│       │   │   ├── energy_graph.dart
│       │   │   ├── energy_filter.dart
│       │   │   └── energy_badge.dart
│       │   ├── providers/
│       │   │   ├── energy_provider.dart
│       │   │   ├── spoon_provider.dart
│       │   │   └── pattern_provider.dart
│       │   └── screens/
│       │       └── energy_dashboard.dart
│       ├── domain/
│       │   ├── entities/
│       │   │   ├── energy_record.dart
│       │   │   ├── spoon_allocation.dart
│       │   │   └── energy_pattern.dart
│       │   └── repositories/
│       │       └── energy_repository.dart
│       └── data/
│           ├── models/
│           │   └── energy_model.dart
│           └── datasources/
│               └── energy_datasource.dart

backend/
├── src/
│   └── services/
│       └── energy_service.rs
└── proto/
    └── energy.proto
```

### Key Files to Create

- `energy_check_in_modal.dart` - Check-in UI component
- `energy_provider.dart` - Energy state management
- `energy_service.rs` - Rust pattern analysis service
- `energy.proto` - gRPC service definitions
- `spoon_allocation_tracker.dart` - Spoon tracking logic

### Dependencies

```yaml
dependencies:
  flutter_riverpod: ^2.4.0
  fl_chart: ^0.63.0
  table_calendar: ^3.0.9
  flutter_local_notifications: ^15.1.0
  shared_preferences: ^2.2.0
  
dev_dependencies:
  mockito: ^5.4.0
  flutter_test:
    sdk: flutter
```

### Rust Dependencies

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
surrealdb = "2.0"
tonic = "0.12"
prost = "0.13"
chrono = "0.4"
serde = { version = "1.0", features = ["derive"] }
```

## Testing Requirements

### Unit Tests

- [ ] Energy level validation (1-5)
- [ ] Spoon allocation logic
- [ ] Pattern detection algorithms
- [ ] Filter application rules
- [ ] Energy prediction accuracy

### Widget Tests

- [ ] Check-in modal interaction
- [ ] Spoon counter updates
- [ ] Graph rendering with data
- [ ] Filter state changes
- [ ] Badge display logic

### Integration Tests

- [ ] Daily check-in → Task filtering flow
- [ ] Spoon allocation → Task completion
- [ ] Historical data → Pattern insights
- [ ] Energy tracking over week
- [ ] Low energy mode activation

### Accessibility Tests

- [ ] Screen reader check-in flow
- [ ] Keyboard-only operation
- [ ] Color-blind safe indicators
- [ ] Touch target sizes

## Definition of Done

- [ ] Daily check-in functional
- [ ] Spoon theory implemented
- [ ] Energy filtering working
- [ ] Pattern recognition operational
- [ ] Historical graphs displaying
- [ ] Low energy mode active
- [ ] All tests passing
- [ ] Performance metrics met
- [ ] Backend integration complete
- [ ] Data persistence verified
