# Feature AK-004: Voice Quick-Capture

## What it does

Provides zero-friction voice capture with automatic transcription using Whisper, enabling instant thought recording through mobile widgets, desktop hotkeys, and in-app recording with auto-save to daily notes.

## User Journey

GIVEN a user has a sudden thought while away from their desk
WHEN they tap the voice widget on their phone and speak
THEN their voice is transcribed and automatically saved to today's daily note with timestamp

## Functional Requirements

- One-tap voice recording from anywhere (widget/hotkey)
- Local Whisper transcription (privacy-first)
- Auto-save to current daily note
- Timestamp injection with each capture
- Background transcription queue
- Voice language detection
- Pause/resume recording
- Audio playback of recordings
- Edit transcription before saving
- Voice commands for formatting
- Batch transcription for multiple recordings
- Export audio with transcription
- Voice memos organization

## UI/UX Requirements

### Components

```dart
// Voice capture components
VoiceRecordButton
TranscriptionDisplay
RecordingWaveform
VoiceWidget
TranscriptionQueue
AudioPlaybackControls
VoiceCommandPanel
TranscriptionEditor
RecordingTimer
VoiceSettingsPanel
```

### Visual Design

- **Layout:**
  - Floating record button: 64px diameter
  - Recording overlay: full screen with 80% opacity
  - Transcription panel: bottom sheet 400px height
  - Waveform display: 200px height
  - Mobile widget: 2x2 grid size
  
- **Colors:**
  ```dart
  recordingActive: Color(0xFFEF4444), // Red for active recording
  recordingPaused: Color(0xFFF59E0B), // Amber for paused
  transcribing: Color(0xFF3B82F6), // Blue for processing
  transcriptionComplete: Color(0xFF10B981), // Green for done
  waveformColor: Color(0xFF6366F1), // Indigo waveform
  ```
  
- **Typography:**
  - Recording timer: 24px monospace
  - Transcription text: 16px regular
  - Voice commands: 14px italic
  - Status messages: 12px regular
  
- **Iconography:**
  - Microphone: mic icon 32px
  - Stop: square icon 24px
  - Pause: pause icon 24px
  - Play: play icon 24px
  - Processing: spinner 20px
  
- **Borders/Shadows:**
  - Record button: 4px shadow, pulsing
  - Transcription card: 3px border
  - Active recording: red glow effect

### User Interactions

- **Input Methods:**
  - Tap to start/stop recording
  - Hold for continuous recording
  - Voice commands during recording
  - Keyboard editing of transcription
  
- **Keyboard Shortcuts:**
  - `Ctrl+Shift+R`: Start/stop recording
  - `Space`: Pause/resume (while recording)
  - `Escape`: Cancel recording
  - `Enter`: Save transcription
  - `Ctrl+P`: Playback audio
  
- **Gestures:**
  - Swipe up: Quick capture from widget
  - Swipe down: Dismiss recording
  - Pinch: Zoom waveform
  - Double-tap: Quick save
  
- **Feedback:**
  - Haptic feedback on start/stop
  - Visual waveform during recording
  - Processing spinner
  - Success vibration on save

### State Management

```dart
// Riverpod providers
final recordingStateProvider = StateNotifierProvider<RecordingStateNotifier, RecordingState>(
  (ref) => RecordingStateNotifier(),
);

final audioRecorderProvider = Provider<AudioRecorderService>((ref) {
  return AudioRecorderService();
});

final transcriptionQueueProvider = StateNotifierProvider<TranscriptionQueueNotifier, List<TranscriptionJob>>(
  (ref) => TranscriptionQueueNotifier(),
);

final currentTranscriptionProvider = StreamProvider<TranscriptionProgress>((ref) {
  return ref.read(whisperServiceProvider).transcriptionStream;
});

final voiceSettingsProvider = StateNotifierProvider<VoiceSettingsNotifier, VoiceSettings>(
  (ref) => VoiceSettingsNotifier(),
);

final recentRecordingsProvider = FutureProvider<List<VoiceRecording>>((ref) async {
  return ref.read(voiceStorageProvider).getRecentRecordings(limit: 10);
});
```

### Responsive Behavior

- **Desktop (>1200px):**
  - Persistent record button in corner
  - Full waveform visualization
  - Side-by-side audio/text view
  
- **Tablet (768-1199px):**
  - Floating action button
  - Modal recording overlay
  - Bottom sheet transcription
  
- **Mobile (<768px):**
  - Home screen widget
  - Full screen recording
  - Swipe gestures primary

### Accessibility Requirements

- **Screen Reader:**
  - Recording status announced
  - Transcription read aloud
  - Timer updates announced
  
- **Keyboard Navigation:**
  - Full keyboard control
  - Clear focus indicators
  - Shortcut documentation
  
- **Color Contrast:**
  - High contrast recording indicator
  - Alternative status indicators
  
- **Motion:**
  - Optional waveform animation
  - Reduced motion mode
  
- **Font Sizing:**
  - Adjustable transcription text
  - Large recording timer

### ADHD-Specific UI Requirements

- **Cognitive Load:**
  - One-tap recording start
  - Automatic daily note placement
  - No configuration required
  - Clear recording status
  
- **Focus Management:**
  - Minimal UI during recording
  - Auto-dismiss after save
  - Background transcription
  
- **Forgiveness:**
  - Undo transcription save
  - Edit before saving
  - Recovery of failed transcriptions
  - Keep audio backup
  
- **Visual Hierarchy:**
  - Large, prominent record button
  - Clear recording indicator
  - Simple save/cancel options
  
- **Immediate Feedback:**
  - Instant recording start (<50ms)
  - Live waveform display
  - Quick transcription preview
  - Haptic confirmation

## Non-Functional Requirements

### Performance Targets

- Recording start latency: <50ms
- Transcription speed: 0.5x real-time or faster
- Audio compression: 10:1 ratio
- Widget launch: <200ms
- Background processing: no UI blocking

### Technical Constraints

- Flutter version: 3.16+
- Whisper model: base.en (140MB)
- Audio format: WAV/M4A
- Platform audio APIs
- Background service support

### Security Requirements

- Local transcription only (no cloud)
- Encrypted audio storage
- Secure widget communication
- Permission management
- Privacy-first design

## Implementation Details

### Code Structure

```
lib/features/voice_capture/
├── presentation/
│   ├── widgets/
│   │   ├── voice_record_button.dart
│   │   ├── recording_waveform.dart
│   │   ├── transcription_display.dart
│   │   ├── recording_timer.dart
│   │   └── audio_playback_controls.dart
│   ├── providers/
│   │   ├── recording_provider.dart
│   │   ├── transcription_provider.dart
│   │   └── audio_provider.dart
│   └── screens/
│       └── voice_capture_screen.dart
├── domain/
│   ├── models/
│   │   ├── voice_recording.dart
│   │   ├── transcription_job.dart
│   │   └── voice_settings.dart
│   ├── repositories/
│   │   └── voice_repository.dart
│   └── use_cases/
│       ├── start_recording.dart
│       ├── transcribe_audio.dart
│       └── save_to_daily_note.dart
└── data/
    ├── services/
    │   ├── audio_recorder_service.dart
    │   ├── whisper_service.dart
    │   └── voice_storage_service.dart
    └── repositories/
        └── voice_repository_impl.dart
```

### Rust Backend Service

```rust
// rust-backend/services/whisper/src/lib.rs
use axum::{Router, Json, Extension};
use whisper_rs::{WhisperContext, FullParams, SamplingStrategy};
use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TranscriptionRequest {
    audio_path: String,
    language: Option<String>,
    model: WhisperModel,
    timestamps: bool,
}

#[derive(Serialize, Deserialize)]
struct TranscriptionResult {
    text: String,
    segments: Vec<TranscriptionSegment>,
    language: String,
    duration_ms: u64,
    processing_time_ms: u64,
}

#[derive(Serialize, Deserialize)]
struct TranscriptionSegment {
    text: String,
    start_ms: u64,
    end_ms: u64,
    confidence: f32,
}

pub struct WhisperService {
    context: WhisperContext,
    queue: mpsc::Sender<TranscriptionJob>,
}

impl WhisperService {
    pub async fn new(model_path: &str) -> Result<Self, Error> {
        let context = WhisperContext::new(model_path)?;
        let (tx, rx) = mpsc::channel(100);
        
        // Start background processing
        tokio::spawn(async move {
            process_transcription_queue(rx, context).await;
        });
        
        Ok(Self {
            context,
            queue: tx,
        })
    }
    
    pub async fn transcribe(&self, req: TranscriptionRequest) -> Result<TranscriptionResult, Error> {
        let start_time = std::time::Instant::now();
        
        // Load audio file
        let audio = load_audio_file(&req.audio_path)?;
        
        // Set parameters
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(req.language.as_deref());
        params.set_print_timestamps(req.timestamps);
        params.set_print_progress(false);
        
        // Run transcription
        let mut state = self.context.create_state()?;
        state.full(params, &audio)?;
        
        // Extract segments
        let num_segments = state.full_n_segments()?;
        let mut segments = Vec::new();
        
        for i in 0..num_segments {
            let text = state.full_get_segment_text(i)?;
            let start_ms = state.full_get_segment_t0(i)? * 10;
            let end_ms = state.full_get_segment_t1(i)? * 10;
            
            segments.push(TranscriptionSegment {
                text,
                start_ms,
                end_ms,
                confidence: 0.95, // Whisper doesn't provide confidence
            });
        }
        
        let full_text = segments.iter()
            .map(|s| s.text.clone())
            .collect::<Vec<_>>()
            .join(" ");
        
        Ok(TranscriptionResult {
            text: full_text,
            segments,
            language: state.full_lang_id()?,
            duration_ms: audio.len() as u64 * 1000 / 16000,
            processing_time_ms: start_time.elapsed().as_millis() as u64,
        })
    }
    
    pub async fn queue_transcription(&self, job: TranscriptionJob) -> Result<String, Error> {
        let job_id = generate_job_id();
        self.queue.send(job).await?;
        Ok(job_id)
    }
}

async fn process_transcription_queue(
    mut rx: mpsc::Receiver<TranscriptionJob>,
    context: WhisperContext,
) {
    while let Some(job) = rx.recv().await {
        // Process job and update status
        match transcribe_job(&job, &context).await {
            Ok(result) => {
                // Save result to database
                save_transcription_result(&job.id, &result).await;
            }
            Err(e) => {
                // Log error and update job status
                log::error!("Transcription failed for job {}: {}", job.id, e);
            }
        }
    }
}
```

### gRPC Service Definition

```proto
// protos/voice_capture.proto
syntax = "proto3";
package altair.knowledge.voice;

import "google/protobuf/timestamp.proto";

service VoiceCaptureService {
  rpc StartRecording(StartRecordingRequest) returns (RecordingSession);
  rpc StopRecording(StopRecordingRequest) returns (RecordingResult);
  rpc TranscribeAudio(TranscribeRequest) returns (stream TranscriptionProgress);
  rpc GetTranscription(GetTranscriptionRequest) returns (TranscriptionResult);
  rpc SaveToDaily(SaveToDailyRequest) returns (SaveResult);
  rpc GetRecordings(GetRecordingsRequest) returns (RecordingList);
}

message StartRecordingRequest {
  string session_id = 1;
  AudioFormat format = 2;
  int32 sample_rate = 3;
}

message RecordingSession {
  string session_id = 1;
  google.protobuf.Timestamp started_at = 2;
  RecordingStatus status = 3;
}

message TranscribeRequest {
  bytes audio_data = 1;
  string audio_path = 2;
  WhisperModel model = 3;
  string language = 4;
  bool enable_timestamps = 5;
}

message TranscriptionProgress {
  string job_id = 1;
  float progress = 2;
  string partial_text = 3;
  TranscriptionStatus status = 4;
}

message TranscriptionResult {
  string text = 1;
  repeated TranscriptionSegment segments = 2;
  string detected_language = 3;
  int64 duration_ms = 4;
  int64 processing_time_ms = 5;
}

message TranscriptionSegment {
  string text = 1;
  int64 start_ms = 2;
  int64 end_ms = 3;
  float confidence = 4;
}

enum WhisperModel {
  TINY = 0;
  BASE = 1;
  SMALL = 2;
  MEDIUM = 3;
  LARGE = 4;
}

enum RecordingStatus {
  RECORDING = 0;
  PAUSED = 1;
  STOPPED = 2;
  FAILED = 3;
}

enum TranscriptionStatus {
  QUEUED = 0;
  PROCESSING = 1;
  COMPLETED = 2;
  FAILED = 3;
}
```

### Platform-Specific Implementation

```dart
// lib/features/voice_capture/data/services/audio_recorder_service.dart
import 'package:record/record.dart';
import 'package:path_provider/path_provider.dart';

class AudioRecorderService {
  final _recorder = AudioRecorder();
  StreamController<RecordingState>? _stateController;
  
  Future<void> startRecording() async {
    if (await _recorder.hasPermission()) {
      final dir = await getApplicationDocumentsDirectory();
      final path = '${dir.path}/recording_${DateTime.now().millisecondsSinceEpoch}.m4a';
      
      await _recorder.start(
        RecordConfig(
          encoder: AudioEncoder.aacLc,
          bitRate: 128000,
          sampleRate: 44100,
        ),
        path: path,
      );
      
      _stateController?.add(RecordingState.recording);
    }
  }
  
  Future<String?> stopRecording() async {
    final path = await _recorder.stop();
    _stateController?.add(RecordingState.stopped);
    return path;
  }
  
  Stream<double> get amplitudeStream => _recorder.onAmplitudeChanged;
}

// Mobile widget implementation (Android)
// android/app/src/main/kotlin/VoiceCaptureWidget.kt
class VoiceCaptureWidget : AppWidgetProvider() {
    override fun onUpdate(context: Context, appWidgetManager: AppWidgetManager, appWidgetIds: IntArray) {
        for (appWidgetId in appWidgetIds) {
            val intent = Intent(context, VoiceCaptureService::class.java)
            val pendingIntent = PendingIntent.getService(context, 0, intent, PendingIntent.FLAG_IMMUTABLE)
            
            val views = RemoteViews(context.packageName, R.layout.voice_widget)
            views.setOnClickPendingIntent(R.id.record_button, pendingIntent)
            
            appWidgetManager.updateAppWidget(appWidgetId, views)
        }
    }
}
```

### SurrealDB Schema

```sql
-- Voice recordings table
DEFINE TABLE voice_recordings SCHEMAFULL;
DEFINE FIELD audio_path ON voice_recordings TYPE string;
DEFINE FIELD transcription ON voice_recordings TYPE string;
DEFINE FIELD duration_ms ON voice_recordings TYPE int;
DEFINE FIELD created_at ON voice_recordings TYPE datetime DEFAULT time::now();
DEFINE FIELD daily_note_id ON voice_recordings TYPE record<daily_notes>;
DEFINE FIELD language ON voice_recordings TYPE string;
DEFINE FIELD segments ON voice_recordings TYPE array;

-- Transcription jobs queue
DEFINE TABLE transcription_jobs SCHEMAFULL;
DEFINE FIELD audio_path ON transcription_jobs TYPE string;
DEFINE FIELD status ON transcription_jobs TYPE string;
DEFINE FIELD created_at ON transcription_jobs TYPE datetime DEFAULT time::now();
DEFINE FIELD completed_at ON transcription_jobs TYPE datetime;
DEFINE FIELD result ON transcription_jobs TYPE object;
DEFINE FIELD error ON transcription_jobs TYPE string;

-- Voice settings
DEFINE TABLE voice_settings SCHEMAFULL;
DEFINE FIELD user_id ON voice_settings TYPE string;
DEFINE FIELD model ON voice_settings TYPE string DEFAULT 'base';
DEFINE FIELD language ON voice_settings TYPE string DEFAULT 'auto';
DEFINE FIELD auto_save ON voice_settings TYPE bool DEFAULT true;
DEFINE FIELD timestamps ON voice_settings TYPE bool DEFAULT true;
```

## Testing Requirements

### Unit Tests

```dart
// test/features/voice_capture/domain/use_cases/transcribe_audio_test.dart
void main() {
  group('TranscribeAudio', () {
    test('transcribes audio file correctly', () async {
      final useCase = TranscribeAudio(mockWhisperService);
      final result = await useCase('test_audio.m4a');
      expect(result.text, isNotEmpty);
      expect(result.segments, isNotEmpty);
    });
    
    test('handles transcription failure gracefully', () async {
      // Test error handling
    });
  });
}
```

### Widget Tests

```dart
// test/features/voice_capture/presentation/widgets/voice_record_button_test.dart
void main() {
  testWidgets('Record button starts recording on tap', (tester) async {
    await tester.pumpWidget(VoiceRecordButton());
    await tester.tap(find.byIcon(Icons.mic));
    await tester.pump();
    expect(find.byIcon(Icons.stop), findsOneWidget);
  });
}
```

### Integration Tests

```dart
// integration_test/voice_capture_flow_test.dart
void main() {
  testWidgets('Complete voice capture flow', (tester) async {
    await tester.pumpWidget(MyApp());
    
    // Start recording
    await tester.tap(find.byIcon(Icons.mic));
    await tester.pump(Duration(seconds: 3));
    
    // Stop recording
    await tester.tap(find.byIcon(Icons.stop));
    await tester.pumpAndSettle();
    
    // Verify transcription appears
    expect(find.byType(TranscriptionDisplay), findsOneWidget);
    
    // Save to daily note
    await tester.tap(find.text('Save'));
    await tester.pumpAndSettle();
    
    // Verify saved
    expect(find.text('Saved to daily note'), findsOneWidget);
  });
}
```

### Accessibility Tests

- [ ] Voice recording with screen reader
- [ ] Keyboard control of recording
- [ ] Status announcements
- [ ] Transcription editing accessibility

## Definition of Done

- [ ] One-tap recording works on all platforms
- [ ] Whisper transcription accurate >90%
- [ ] Auto-save to daily notes functional
- [ ] Mobile widget operational
- [ ] Desktop hotkey working
- [ ] Background transcription queue
- [ ] All tests passing (>80% coverage)
- [ ] Accessibility audit complete
- [ ] Performance targets met
- [ ] Documentation updated
- [ ] Code review approved
