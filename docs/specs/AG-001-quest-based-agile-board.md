# Feature AG-001: Quest-Based Agile Board

## What it does

Provides a 6-column kanban-style board implementing Quest-Based Agile methodology with Epic→Quest→Subquest hierarchy, enabling ADHD-friendly project management through energy-based planning instead of time-based estimation.

## User Journey

GIVEN user has created epics, quests, and subquests in the system
WHEN user opens the Quest Board view
THEN user sees tasks organized in 6 columns with WIP=1 enforcement on In-Progress column

## Functional Requirements

- 6-column workflow: Idea Greenhouse → Quest Log → This Cycle's Quest → Next Up → In-Progress (WIP=1) → Harvested
- Hierarchical task structure: Epic → Quest → Subquest
- Energy points (1-5) instead of time estimates
- Drag-and-drop between columns with validation rules
- WIP=1 hard enforcement on In-Progress column
- Auto-archive completed items after configurable period
- Filter by energy level, tags, epic, assignee
- Bulk operations for column management
- Undo/redo for last 10 actions
- Board state persistence across sessions

## UI/UX Requirements

### Components

- `QuestBoard` - Main container widget
- `QuestColumn` - Individual column component
- `QuestCard` - Draggable task card
- `EnergyIndicator` - Visual energy level display
- `WipLimitBadge` - Shows WIP limit status
- `QuickAddButton` - Floating action button for task creation
- `FilterBar` - Top bar with filter controls
- `BoardHeader` - Title and view switcher
- `DragPreview` - Ghost card during drag
- `DropZone` - Visual drop indicators

### Visual Design

- **Layout:** 6-column grid with 16px gaps, 280px min column width
- **Colors:**
  - Idea Greenhouse: `#E8F5E9`
  - Quest Log: `#E3F2FD`
  - This Cycle: `#FFF3E0`
  - Next Up: `#FCE4EC`
  - In-Progress: `#F3E5F5` with 4px border
  - Harvested: `#F5F5F5` with 50% opacity
- **Typography:**
  - Column headers: 16px semi-bold
  - Card titles: 14px medium
  - Metadata: 12px regular
- **Iconography:**
  - Energy: Lightning bolt icons (16x16)
  - Epic: Mountain icon
  - Quest: Flag icon
  - Subquest: Checkbox icon
- **Borders/Shadows:**
  - Cards: 2px solid black border, 4px offset shadow
  - Hover: 6px offset shadow
  - Dragging: 8px blur shadow

### User Interactions

- **Input Methods:**
  - Drag-and-drop with mouse/touch
  - Keyboard navigation (Arrow keys + Space to grab/drop)
  - Right-click context menu
  - Double-click to open detail view
- **Keyboard Shortcuts:**
  - `Ctrl+N`: New quest
  - `Ctrl+F`: Focus filter
  - `Tab`: Navigate columns
  - `1-6`: Jump to column
  - `Escape`: Cancel drag
- **Gestures:**
  - Long-press on mobile to initiate drag
  - Swipe left/right to scroll columns
  - Pinch to zoom board scale
- **Feedback:**
  - Drop zones highlight on drag start
  - Invalid drops show red border
  - Success animation on column change
  - Toast notification for WIP violations

### State Management

- **Local State:**
  - Current drag item
  - Hover states
  - Expanded/collapsed cards
  - Selected filters
- **Global State:**

  ```dart
  final questBoardProvider = StateNotifierProvider<QuestBoardNotifier, QuestBoardState>
  final activeFiltersProvider = StateProvider<BoardFilters>
  final boardLayoutProvider = StateProvider<BoardLayout>
  final dragStateProvider = StateProvider<DragState?>
  ```

- **Persistence:**
  - Board state saved on every change
  - Filter preferences saved per user
  - Column widths saved
  - Collapsed state saved

### Responsive Behavior

- **Desktop (>1200px):** All 6 columns visible, 280px width each
- **Tablet (768-1199px):** 3 columns visible, horizontal scroll for others
- **Mobile (<768px):** Single column view with column switcher
- **Breakpoint Strategy:** Container queries for column sizing

### Accessibility Requirements

- **Screen Reader:**
  - Announce column names and item count
  - Describe drag operations
  - Read card metadata on focus
- **Keyboard Navigation:**
  - Full keyboard control for all operations
  - Focus trap within board
  - Skip links to columns
- **Color Contrast:** 4.5:1 minimum for all text
- **Motion:** Respect prefers-reduced-motion for animations
- **Font Sizing:** Minimum 12px, scalable to 200%

### ADHD-Specific UI Requirements

- **Cognitive Load:**
  - Hide completed items after 24 hours
  - Collapse card details by default
  - Maximum 5 items visible per column (scroll for more)
- **Focus Management:**
  - Red border pulse on WIP violation
  - Auto-focus on newly created cards
  - Single item focus mode available
- **Forgiveness:**
  - Undo last 10 actions
  - Restore archived items within 30 days
  - Easy column reset option
- **Visual Hierarchy:**
  - In-Progress column always centered
  - Energy indicators prominent
  - Clear "Start Working" CTA
- **Immediate Feedback:**
  - Card movement <150ms
  - Column highlight instant
  - Success haptic on mobile

## Non-Functional Requirements

### Performance Targets

- Board initial render <500ms
- Drag operation <16ms per frame (60fps)
- Column reorder <100ms
- Filter application <200ms
- Board with 500 items renders <1s

### Technical Constraints

- Flutter 3.16+ with desktop support
- Riverpod 2.4+ for state management
- drift for local persistence
- flutter_animate for animations
- super_drag_and_drop for drag operations

### Security Requirements

- Input sanitization for card content
- XSS prevention in markdown rendering
- Rate limiting on board operations
- Audit log for all changes

## Implementation Details

### Code Structure

```text
lib/
├── features/
│   └── quest_board/
│       ├── presentation/
│       │   ├── widgets/
│       │   │   ├── quest_board.dart
│       │   │   ├── quest_column.dart
│       │   │   ├── quest_card.dart
│       │   │   ├── energy_indicator.dart
│       │   │   └── filter_bar.dart
│       │   ├── providers/
│       │   │   ├── board_state_provider.dart
│       │   │   ├── drag_provider.dart
│       │   │   └── filter_provider.dart
│       │   └── screens/
│       │       └── board_screen.dart
│       ├── domain/
│       │   ├── entities/
│       │   │   ├── quest.dart
│       │   │   ├── epic.dart
│       │   │   └── subquest.dart
│       │   └── repositories/
│       │       └── quest_repository.dart
│       └── data/
│           ├── models/
│           │   └── quest_model.dart
│           ├── datasources/
│           │   └── surrealdb_datasource.dart
│           └── repositories/
│               └── quest_repository_impl.dart
```

### Key Files to Create

- `quest_board.dart` - Main board container widget
- `quest_board_provider.dart` - State management for board
- `quest_model.dart` - Data model for quests
- `board_service.rs` - Rust backend service
- `quest.proto` - gRPC definitions

### Dependencies

```yaml
dependencies:
  flutter_riverpod: ^2.4.0
  drift: ^2.12.0
  super_drag_and_drop: ^0.2.0
  flutter_animate: ^4.2.0
  collection: ^1.17.0
  
dev_dependencies:
  build_runner: ^2.4.0
  drift_dev: ^2.12.0
```

## Testing Requirements

### Unit Tests

- [x] WIP=1 enforcement logic
- [x] Column transition rules
- [x] Energy calculation
- [ ] Filter combinations
- [ ] Drag validation rules

### Widget Tests

- [ ] Board rendering with different item counts
- [ ] Drag and drop operations
- [ ] Filter application
- [ ] Responsive layout changes
- [x] Keyboard navigation

### Integration Tests

- [ ] Create quest → Move through columns → Archive
- [ ] Apply filters → Drag filtered item
- [ ] Bulk operations on multiple items
- [ ] Undo/redo sequence
- [ ] Board state persistence

### Accessibility Tests

- [ ] Screen reader navigation
- [x] Keyboard-only operation
- [ ] Color contrast validation
- [ ] Focus management

## Definition of Done

- [x] All 6 columns functional with drag-and-drop
- [x] WIP=1 enforcement working
- [x] Energy-based filtering operational
- [x] Responsive design implemented
- [x] Keyboard navigation complete
- [ ] Performance targets met
- [ ] All tests passing
- [ ] Accessibility audit passed
- [x] Backend gRPC endpoints integrated
- [x] SurrealDB persistence working
