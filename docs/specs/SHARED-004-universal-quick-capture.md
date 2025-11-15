# Feature SHARED-004: Universal Quick Capture

## What it does

Provides system-wide quick capture capability with intelligent routing to appropriate Altair app (Guidance for tasks, Knowledge for notes, Tracking for items) using natural language processing and voice input support.

## User Journey

GIVEN user has a thought/task/item to capture quickly
WHEN user triggers quick capture via hotkey, widget, or voice
THEN system captures input and intelligently routes to the correct app with zero friction

## Functional Requirements

- Global hotkey activation (configurable)
- Voice input with Whisper transcription
- Natural language processing for routing
- Multi-line text support
- Image/screenshot capture
- File attachment support
- Smart routing to apps
- Fallback to daily notes
- Offline capture queue
- Mobile widgets
- Desktop system tray
- Command palette integration
- Capture history
- Undo last capture

## UI/UX Requirements

### Components

- `QuickCaptureOverlay` - Floating input window
- `VoiceButton` - Voice recording trigger
- `RouterIndicator` - Shows destination app
- `CaptureWidget` - Mobile home screen widget
- `SystemTrayIcon` - Desktop quick access
- `CommandPalette` - Searchable capture
- `CaptureHistory` - Recent captures list
- `AttachmentPreview` - File/image preview
- `OfflineQueue` - Pending captures
- `HotkeyConfig` - Shortcut settings

### Visual Design

- **Layout:**
  - Overlay: 400px width, centered top
  - Input field: Multiline, 100px initial height
  - Mobile widget: 2x2 grid size
  - System tray: 24x24 icon
- **Colors:**
  - Background: `#FFFFFF` with 95% opacity
  - Border: `#000000` 3px solid
  - Voice recording: `#F44336` (Red pulse)
  - Routing indicator: App-specific colors
  - Success: `#4CAF50` flash
- **Typography:**
  - Input text: 16px regular
  - Placeholder: 14px italic gray
  - Router hint: 12px colored
  - Character count: 11px light
- **Iconography:**
  - Capture: Lightning bolt (24x24)
  - Voice: Microphone icon
  - Guidance: Flag icon
  - Knowledge: Book icon
  - Tracking: Box icon
- **Borders/Shadows:**
  - Overlay: 8px shadow blur
  - Focus: 4px colored border
  - Success: Green flash border

### User Interactions

- **Input Methods:**
  - Type directly in field
  - Paste text/images
  - Drag and drop files
  - Voice dictation
  - Screenshot tool
- **Keyboard Shortcuts:**
  - `Ctrl+Shift+Space`: Open capture
  - `Ctrl+Enter`: Submit capture
  - `Escape`: Cancel capture
  - `Ctrl+V`: Paste content
  - `Tab`: Cycle routing options
- **Gestures:**
  - Swipe down to dismiss
  - Long-press for voice
  - Shake to undo (mobile)
  - Edge swipe to open
- **Feedback:**
  - Routing preview in real-time
  - Voice waveform display
  - Success animation
  - Error shake effect

### State Management

- **Local State:**
  - Current input text
  - Recording status
  - Routing prediction
  - Attachment list
- **Global State:**
  ```dart
  final quickCaptureProvider = StateNotifierProvider<QuickCaptureNotifier, QuickCaptureState>
  final captureHistoryProvider = StateNotifierProvider<HistoryNotifier, List<CaptureRecord>>
  final routingEngineProvider = Provider<RoutingEngine>
  final voiceRecorderProvider = StateNotifierProvider<VoiceRecorderNotifier, RecorderState>
  final offlineQueueProvider = StateNotifierProvider<QueueNotifier, List<PendingCapture>>
  final hotkeyProvider = StateProvider<HotkeyConfig>
  ```
- **Persistence:**
  - Capture history (30 days)
  - Offline queue on disk
  - Hotkey preferences
  - Voice settings

### Responsive Behavior

- **Desktop:** Floating overlay, system tray
- **Tablet:** Full-width top sheet
- **Mobile:** Bottom sheet, home widget
- **Breakpoint Strategy:** Platform-specific UX

### Accessibility Requirements

- **Screen Reader:**
  - Announce capture mode
  - Read routing destination
  - Describe attachments
- **Keyboard Navigation:**
  - Full keyboard control
  - Tab order logical
- **Color Contrast:** High contrast mode
- **Motion:** Minimal animations
- **Font Sizing:** Respects system size

### ADHD-Specific UI Requirements

- **Cognitive Load:**
  - Single input field
  - Auto-routing (no choice)
  - Minimal UI elements
  - Clear success feedback
- **Focus Management:**
  - Auto-focus on open
  - Escape to cancel
  - No distractions
- **Forgiveness:**
  - Undo last capture
  - Edit after routing
  - Queue when offline
  - No timeout pressure
- **Visual Hierarchy:**
  - Input field dominant
  - Routing hint subtle
  - Success very clear
- **Immediate Feedback:**
  - Instant open (<100ms)
  - Real-time routing hint
  - Quick close on submit

## Non-Functional Requirements

### Performance Targets

- Hotkey response <50ms
- Window open <100ms
- Voice transcription <2s
- Routing decision <100ms
- Submit and close <200ms

### Technical Constraints

- System-wide hotkey registration
- Whisper model size <100MB
- Cross-platform compatibility
- Background service required
- Memory footprint <50MB

### Security Requirements

- No keylogging
- Voice data local only
- Secure attachment handling
- Permission prompts
- Sandboxed execution

## Implementation Details

### Code Structure

```
lib/
├── core/
│   └── quick_capture/
│       ├── capture_service.dart
│       ├── routing_engine.dart
│       ├── voice_recorder.dart
│       ├── transcription_service.dart
│       ├── models/
│       │   ├── capture_record.dart
│       │   ├── routing_result.dart
│       │   └── capture_type.dart
│       ├── providers/
│       │   ├── capture_provider.dart
│       │   ├── routing_provider.dart
│       │   └── voice_provider.dart
│       └── widgets/
│           ├── capture_overlay.dart
│           ├── voice_button.dart
│           └── routing_indicator.dart

backend/
├── src/
│   └── capture/
│       ├── mod.rs
│       ├── routing_engine.rs
│       ├── whisper_service.rs
│       └── nlp_processor.rs
└── proto/
    └── capture.proto
```

### Key Files to Create

- `capture_service.dart` - Main capture logic
- `routing_engine.dart` - NLP routing decisions
- `voice_recorder.dart` - Audio recording
- `whisper_service.rs` - Transcription service
- `capture.proto` - gRPC definitions

### Dependencies

```yaml
dependencies:
  hotkey_manager: ^0.2.0
  record: ^4.4.0
  whisper_flutter: ^0.1.0
  home_widget: ^0.3.0
  system_tray: ^2.0.0
  window_manager: ^0.3.0
  
dev_dependencies:
  flutter_test:
    sdk: flutter
```

### Rust Dependencies

```toml
[dependencies]
whisper-rs = "0.11"
candle = "0.4"
tokenizers = "0.15"
regex = "1.10"
chrono = "0.4"
```

### Routing Rules

```dart
class RoutingEngine {
  RouteDestination route(String text) {
    // Task indicators → Guidance
    if (text.matchesAny([
      r'^(todo|task|do|fix|build|create|implement)',
      r'(need to|have to|should|must)',
      r'(tomorrow|next week|deadline)'
    ])) return RouteDestination.guidance;
    
    // Item/inventory indicators → Tracking  
    if (text.matchesAny([
      r'^(bought|purchased|received|have)',
      r'(inventory|stock|supplies)',
      r'(\d+\s*(pcs|pieces|units|x))',
      r'(location|stored|warehouse)'
    ])) return RouteDestination.tracking;
    
    // Default → Knowledge (notes)
    return RouteDestination.knowledge;
  }
}
```

## Testing Requirements

### Unit Tests

- [x] Routing engine accuracy
- [x] Voice recording logic
- [x] Transcription processing
- [x] Queue management
- [x] Hotkey registration

### Widget Tests

- [x] Overlay appearance
- [x] Input field behavior
- [x] Voice button states
- [x] Routing indicator
- [x] Success animations

### Integration Tests

- [x] Hotkey → Capture → Route
- [x] Voice → Transcribe → Save
- [x] Offline → Queue → Sync
- [x] Multi-app routing
- [x] Widget interaction

### Accessibility Tests

- [x] Screen reader flow
- [x] Keyboard operation
- [x] Voice-only usage
- [x] High contrast mode

## Definition of Done

- [x] Global hotkey working
- [x] Voice input functional
- [x] Routing accurate >90%
- [x] Mobile widget active
- [x] System tray operational
- [x] Offline queue working
- [x] Whisper transcription fast
- [x] All platforms supported
- [x] Tests passing
- [x] Performance metrics met
