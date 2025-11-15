# Feature SHARED-006: Whisper Transcription Service

## What it does

Provides local speech-to-text transcription using OpenAI's Whisper models for voice capture, audio notes, and accessibility features across all Altair applications with complete privacy and offline capability.

## User Journey

GIVEN any Altair app needs to transcribe audio to text
WHEN the app sends audio data to the Whisper service
THEN it receives accurate transcription with timestamps and language detection

## Functional Requirements

- Local Whisper model execution
- Multiple model sizes (tiny to large)
- Real-time transcription streaming
- Batch audio processing
- Language auto-detection
- Timestamp generation
- Speaker diarization (future)
- Noise suppression
- Audio format conversion
- Queue management
- Partial transcription updates
- Punctuation restoration
- Word-level timestamps
- Translation capability

## UI/UX Requirements

### Components

```dart
// Whisper service monitoring components
WhisperServiceStatus
ModelSizeSelector
TranscriptionQueue
AudioWaveformDisplay
LanguageIndicator
TranscriptionProgress
AccuracyMetrics
ModelDownloader
ServiceHealthCheck
QueueManager
```

### Visual Design

- **Layout:**
  - Status indicator: 180px width
  - Model selector: 240px dropdown
  - Queue monitor: 350x250px panel
  - Progress bar: full width bottom
  
- **Colors:**
  ```dart
  transcribing: Color(0xFF3B82F6), // Blue for active
  queued: Color(0xFFF59E0B), // Amber for pending
  completed: Color(0xFF10B981), // Green for done
  error: Color(0xFFEF4444), // Red for failed
  modelLoading: Color(0xFF8B5CF6), // Purple for loading
  ```
  
- **Typography:**
  - Status: 12px regular
  - Model name: 14px semibold
  - Queue count: 16px monospace
  - Language: 13px italic
  
- **Iconography:**
  - Microphone: mic icon 20px
  - Processing: waveform animation
  - Language: globe icon 16px
  - Model: chip icon 16px
  
- **Borders/Shadows:**
  - Active transcription: pulsing border
  - Queue items: 2px divider
  - Model selector: 3px on focus

### User Interactions

- **Input Methods:**
  - Automatic service initialization
  - Model size selection
  - Language preference setting
  - Queue priority adjustment
  
- **Keyboard Shortcuts:**
  - `Ctrl+Shift+W`: Whisper status
  - `Ctrl+Shift+L`: Language selector
  - `Ctrl+Shift+P`: Pause/resume queue
  
- **Gestures:**
  - Drag to reorder queue
  - Swipe to cancel job
  
- **Feedback:**
  - Live transcription updates
  - Progress percentage
  - ETA for completion
  - Error notifications

### State Management

```dart
// Riverpod providers
final whisperServiceProvider = Provider<WhisperService>((ref) {
  return WhisperService();
});

final whisperStatusProvider = StreamProvider<ServiceStatus>((ref) {
  return ref.read(whisperServiceProvider).statusStream;
});

final currentWhisperModelProvider = StateProvider<WhisperModel>((ref) {
  return WhisperModel.base;
});

final transcriptionQueueProvider = StateNotifierProvider<TranscriptionQueueNotifier, List<TranscriptionJob>>(
  (ref) => TranscriptionQueueNotifier(),
);

final activeTranscriptionProvider = StreamProvider<TranscriptionProgress>((ref) {
  return ref.read(whisperServiceProvider).progressStream;
});

final languageDetectionProvider = Provider<bool>((ref) => true);

final whisperAccuracyProvider = Provider<AccuracyMetrics>((ref) {
  return ref.read(whisperServiceProvider).getAccuracyMetrics();
});
```

### Responsive Behavior

- **Desktop (>1200px):**
  - Full monitoring panel
  - Multiple model options
  - Detailed metrics
  
- **Tablet (768-1199px):**
  - Simplified status
  - Essential controls only
  
- **Mobile (<768px):**
  - Minimal indicator
  - Background operation

### Accessibility Requirements

- **Screen Reader:**
  - Transcription progress announced
  - Language detection communicated
  - Queue updates spoken
  
- **Keyboard Navigation:**
  - Full keyboard control
  - Clear focus indicators
  
- **Color Contrast:**
  - Status indicators accessible
  - Text alternatives provided
  
- **Motion:**
  - Optional waveform animation
  - Static progress option
  
- **Font Sizing:**
  - Scalable status text
  - Clear queue display

### ADHD-Specific UI Requirements

- **Cognitive Load:**
  - Auto-select best model
  - Hidden complexity
  - Smart defaults
  - One-click operation
  
- **Focus Management:**
  - Background processing
  - Non-intrusive status
  - Silent operation default
  
- **Forgiveness:**
  - Auto-retry failures
  - Queue persistence
  - Partial results saved
  
- **Visual Hierarchy:**
  - Clear progress indication
  - Critical errors prominent
  - Details hidden by default
  
- **Immediate Feedback:**
  - Instant queue addition
  - Live progress updates
  - Quick status checks

## Non-Functional Requirements

### Performance Targets

- Transcription speed: 0.5x real-time or faster
- Model loading: <10 seconds
- Memory usage: <1GB for base model
- Queue processing: parallel up to CPU cores
- Partial updates: every 2 seconds
- Audio preprocessing: <500ms

### Technical Constraints

- Whisper.cpp for performance
- Model sizes: 39MB (tiny) to 1.5GB (large)
- CPU-only operation standard
- GPU acceleration optional
- Cross-platform compatibility

### Security Requirements

- Local-only processing
- No audio leaves device
- Secure IPC channels
- Temporary file cleanup
- Model integrity checks

## Implementation Details

### Code Structure

```
// Service structure
whisper-service/
├── src/
│   ├── models/
│   │   ├── model_loader.rs
│   │   ├── whisper_wrapper.rs
│   │   └── model_config.rs
│   ├── audio/
│   │   ├── preprocessor.rs
│   │   ├── format_converter.rs
│   │   └── noise_filter.rs
│   ├── transcription/
│   │   ├── engine.rs
│   │   ├── streaming.rs
│   │   └── post_processor.rs
│   ├── queue/
│   │   ├── job_queue.rs
│   │   └── priority_manager.rs
│   ├── api/
│   │   └── grpc_server.rs
│   └── monitoring/
│       ├── metrics.rs
│       └── health.rs
├── models/
│   └── [downloaded models]
└── Cargo.toml

// Flutter integration
lib/shared/services/whisper/
├── whisper_service.dart
├── whisper_client.dart
├── models/
│   ├── transcription_job.dart
│   └── transcription_result.dart
└── providers/
    └── whisper_provider.dart
```

### Rust Implementation

```rust
// rust-backend/services/whisper/src/lib.rs
use whisper_rs::{WhisperContext, FullParams, SamplingStrategy};
use tokio::sync::mpsc;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use hound::WavReader;

pub struct WhisperService {
    context: Arc<WhisperContext>,
    queue: mpsc::Sender<TranscriptionJob>,
    config: WhisperConfig,
}

#[derive(Serialize, Deserialize)]
struct WhisperConfig {
    model_path: String,
    model_size: ModelSize,
    language: Option<String>,
    enable_timestamps: bool,
    enable_translate: bool,
    num_threads: usize,
}

#[derive(Serialize, Deserialize)]
enum ModelSize {
    Tiny,
    Base,
    Small,
    Medium,
    Large,
}

#[derive(Serialize, Deserialize)]
pub struct TranscriptionJob {
    pub id: String,
    pub audio_path: String,
    pub language: Option<String>,
    pub priority: i32,
    pub options: TranscriptionOptions,
}

#[derive(Serialize, Deserialize)]
pub struct TranscriptionResult {
    pub text: String,
    pub segments: Vec<Segment>,
    pub language: String,
    pub duration_ms: i64,
    pub processing_time_ms: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Segment {
    pub text: String,
    pub start_ms: i64,
    pub end_ms: i64,
    pub tokens: Vec<Token>,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub word: String,
    pub start_ms: i64,
    pub end_ms: i64,
    pub probability: f32,
}

impl WhisperService {
    pub async fn new(config: WhisperConfig) -> Result<Self, Error> {
        // Load Whisper model
        let context = WhisperContext::new(&config.model_path)?;
        
        // Create processing queue
        let (tx, mut rx) = mpsc::channel::<TranscriptionJob>(100);
        
        // Spawn worker threads
        let context_arc = Arc::new(context);
        for _ in 0..config.num_threads {
            let ctx = context_arc.clone();
            let mut queue_rx = rx.clone();
            
            tokio::spawn(async move {
                while let Some(job) = queue_rx.recv().await {
                    process_transcription_job(ctx.clone(), job).await;
                }
            });
        }
        
        Ok(Self {
            context: context_arc,
            queue: tx,
            config,
        })
    }
    
    pub async fn transcribe(
        &self,
        audio_path: &str,
        options: TranscriptionOptions,
    ) -> Result<TranscriptionResult, Error> {
        let start_time = std::time::Instant::now();
        
        // Load and preprocess audio
        let audio_data = load_audio_file(audio_path)?;
        let preprocessed = preprocess_audio(&audio_data)?;
        
        // Configure Whisper parameters
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        
        if let Some(lang) = &options.language {
            params.set_language(Some(lang));
        } else {
            params.set_language(None); // Auto-detect
        }
        
        params.set_print_timestamps(options.enable_timestamps);
        params.set_token_timestamps(options.word_timestamps);
        params.set_translate(options.translate);
        params.set_no_context(false);
        params.set_single_segment(false);
        params.set_print_progress(false);
        
        // Create state and run transcription
        let mut state = self.context.create_state()?;
        state.full(params, &preprocessed)?;
        
        // Extract results
        let num_segments = state.full_n_segments()?;
        let mut segments = Vec::new();
        
        for i in 0..num_segments {
            let text = state.full_get_segment_text(i)?;
            let start_ms = state.full_get_segment_t0(i)? * 10;
            let end_ms = state.full_get_segment_t1(i)? * 10;
            
            let mut tokens = Vec::new();
            if options.word_timestamps {
                let num_tokens = state.full_n_tokens(i)?;
                for j in 0..num_tokens {
                    let token_text = state.full_get_token_text(i, j)?;
                    let token_start = state.full_get_token_t0(i, j)? * 10;
                    let token_end = state.full_get_token_t1(i, j)? * 10;
                    let token_prob = state.full_get_token_p(i, j)?;
                    
                    tokens.push(Token {
                        word: token_text,
                        start_ms: token_start,
                        end_ms: token_end,
                        probability: token_prob,
                    });
                }
            }
            
            segments.push(Segment {
                text,
                start_ms,
                end_ms,
                tokens,
            });
        }
        
        let full_text = segments.iter()
            .map(|s| s.text.clone())
            .collect::<Vec<_>>()
            .join(" ");
        
        let language = state.full_lang_id()?;
        
        Ok(TranscriptionResult {
            text: full_text,
            segments,
            language,
            duration_ms: (preprocessed.len() as i64 * 1000) / 16000,
            processing_time_ms: start_time.elapsed().as_millis() as i64,
        })
    }
    
    pub async fn transcribe_stream(
        &self,
        audio_stream: impl Stream<Item = Vec<f32>>,
        options: TranscriptionOptions,
    ) -> impl Stream<Item = PartialTranscription> {
        // Real-time streaming transcription
        let (tx, rx) = mpsc::channel(10);
        
        tokio::spawn(async move {
            let mut buffer = Vec::new();
            let mut last_segment = String::new();
            
            pin_mut!(audio_stream);
            
            while let Some(chunk) = audio_stream.next().await {
                buffer.extend(chunk);
                
                // Process when we have enough audio (e.g., 2 seconds)
                if buffer.len() >= 16000 * 2 {
                    let partial = self.process_buffer(&buffer, &options).await;
                    
                    // Send only new text
                    if partial.text != last_segment {
                        tx.send(partial).await.ok();
                        last_segment = partial.text.clone();
                    }
                    
                    // Keep overlap for context
                    buffer.drain(..buffer.len() - 16000);
                }
            }
            
            // Process remaining audio
            if !buffer.is_empty() {
                let final_partial = self.process_buffer(&buffer, &options).await;
                tx.send(final_partial).await.ok();
            }
        });
        
        ReceiverStream::new(rx)
    }
    
    pub async fn queue_transcription(&self, job: TranscriptionJob) -> Result<String, Error> {
        self.queue.send(job.clone()).await?;
        Ok(job.id)
    }
    
    pub async fn detect_language(&self, audio_path: &str) -> Result<String, Error> {
        let audio_data = load_audio_file(audio_path)?;
        let sample = &audio_data[..audio_data.len().min(16000 * 30)]; // First 30 seconds
        
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(None); // Force auto-detect
        
        let mut state = self.context.create_state()?;
        state.full(params, sample)?;
        
        Ok(state.full_lang_id()?)
    }
}

fn load_audio_file(path: &str) -> Result<Vec<f32>, Error> {
    // Support multiple audio formats
    let extension = std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    
    match extension {
        "wav" => load_wav(path),
        "mp3" => convert_and_load_mp3(path),
        "m4a" | "aac" => convert_and_load_m4a(path),
        _ => Err(Error::UnsupportedFormat),
    }
}

fn load_wav(path: &str) -> Result<Vec<f32>, Error> {
    let mut reader = WavReader::open(path)?;
    let spec = reader.spec();
    
    // Resample to 16kHz if necessary
    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / i16::MAX as f32)
        .collect();
    
    if spec.sample_rate != 16000 {
        resample(&samples, spec.sample_rate, 16000)
    } else {
        Ok(samples)
    }
}

fn preprocess_audio(audio: &[f32]) -> Result<Vec<f32>, Error> {
    // Apply preprocessing pipeline
    let mut processed = audio.to_vec();
    
    // Normalize audio
    let max_val = processed.iter().map(|x| x.abs()).fold(0.0f32, f32::max);
    if max_val > 0.0 {
        for sample in &mut processed {
            *sample /= max_val;
        }
    }
    
    // Apply noise gate
    apply_noise_gate(&mut processed, 0.01);
    
    // Apply high-pass filter to remove low-frequency noise
    apply_high_pass_filter(&mut processed, 80.0, 16000);
    
    Ok(processed)
}

async fn process_transcription_job(
    context: Arc<WhisperContext>,
    job: TranscriptionJob,
) -> Result<(), Error> {
    // Process job and update status
    let result = transcribe_with_context(&context, &job).await?;
    
    // Store result in database
    store_transcription_result(&job.id, &result).await?;
    
    // Notify completion
    notify_completion(&job.id).await?;
    
    Ok(())
}
```

### gRPC Service Definition

```proto
// protos/whisper.proto
syntax = "proto3";
package altair.shared.whisper;

import "google/protobuf/timestamp.proto";

service WhisperService {
  rpc Transcribe(TranscribeRequest) returns (TranscriptionResult);
  rpc TranscribeStream(stream AudioChunk) returns (stream PartialTranscription);
  rpc QueueTranscription(QueueJobRequest) returns (JobResponse);
  rpc GetJobStatus(JobStatusRequest) returns (JobStatus);
  rpc DetectLanguage(LanguageDetectRequest) returns (LanguageResult);
  rpc GetServiceStatus(StatusRequest) returns (ServiceStatus);
  rpc SwitchModel(SwitchModelRequest) returns (SwitchModelResponse);
}

message TranscribeRequest {
  bytes audio_data = 1;
  string audio_path = 2;
  TranscriptionOptions options = 3;
}

message TranscriptionOptions {
  string language = 1;
  bool enable_timestamps = 2;
  bool word_timestamps = 3;
  bool translate = 4;
  float temperature = 5;
  int32 max_context = 6;
}

message TranscriptionResult {
  string text = 1;
  repeated Segment segments = 2;
  string detected_language = 3;
  int64 duration_ms = 4;
  int64 processing_time_ms = 5;
}

message Segment {
  string text = 1;
  int64 start_ms = 2;
  int64 end_ms = 3;
  repeated Token tokens = 4;
}

message Token {
  string word = 1;
  int64 start_ms = 2;
  int64 end_ms = 3;
  float probability = 4;
}

message AudioChunk {
  bytes audio_data = 1;
  int32 sample_rate = 2;
  bool is_final = 3;
}

message PartialTranscription {
  string text = 1;
  bool is_final = 2;
  float confidence = 3;
  google.protobuf.Timestamp timestamp = 4;
}

message QueueJobRequest {
  string job_id = 1;
  string audio_path = 2;
  TranscriptionOptions options = 3;
  int32 priority = 4;
}

message JobStatus {
  string job_id = 1;
  JobState state = 2;
  float progress = 3;
  string error_message = 4;
  TranscriptionResult result = 5;
}

enum JobState {
  QUEUED = 0;
  PROCESSING = 1;
  COMPLETED = 2;
  FAILED = 3;
  CANCELLED = 4;
}

message ServiceStatus {
  bool is_running = 1;
  string current_model = 2;
  int32 queue_length = 3;
  int32 active_jobs = 4;
  float memory_usage_mb = 5;
  float cpu_usage_percent = 6;
  map<string, int32> language_stats = 7;
}

enum WhisperModel {
  TINY = 0;
  BASE = 1;
  SMALL = 2;
  MEDIUM = 3;
  LARGE = 4;
  TINY_EN = 5;
  BASE_EN = 6;
  SMALL_EN = 7;
  MEDIUM_EN = 8;
}
```

### Flutter Client Integration

```dart
// lib/shared/services/whisper/whisper_client.dart
import 'package:grpc/grpc.dart';
import 'package:record/record.dart';

class WhisperClient {
  late ClientChannel _channel;
  late WhisperServiceClient _stub;
  final _activeJobs = <String, JobStatus>{};
  
  Future<void> connect({String host = 'localhost', int port = 50053}) async {
    _channel = ClientChannel(
      host,
      port: port,
      options: const ChannelOptions(
        credentials: ChannelCredentials.insecure(),
      ),
    );
    
    _stub = WhisperServiceClient(_channel);
  }
  
  Future<TranscriptionResult> transcribe(
    String audioPath, {
    String? language,
    bool timestamps = true,
    bool translate = false,
  }) async {
    final request = TranscribeRequest()
      ..audioPath = audioPath
      ..options = (TranscriptionOptions()
        ..language = language ?? ''
        ..enableTimestamps = timestamps
        ..translate = translate);
    
    final response = await _stub.transcribe(request);
    return TranscriptionResult.fromProto(response);
  }
  
  Stream<PartialTranscription> transcribeStream(
    Stream<List<int>> audioStream,
  ) async* {
    final requestStream = audioStream.map((chunk) {
      return AudioChunk()
        ..audioData = chunk
        ..sampleRate = 16000;
    });
    
    final responseStream = _stub.transcribeStream(requestStream);
    
    await for (final partial in responseStream) {
      yield PartialTranscription.fromProto(partial);
    }
  }
  
  Future<String> queueTranscription(
    String audioPath, {
    int priority = 0,
    TranscriptionOptions? options,
  }) async {
    final jobId = generateJobId();
    
    final request = QueueJobRequest()
      ..jobId = jobId
      ..audioPath = audioPath
      ..priority = priority;
    
    if (options != null) {
      request.options = options;
    }
    
    await _stub.queueTranscription(request);
    return jobId;
  }
  
  Future<JobStatus> getJobStatus(String jobId) async {
    final request = JobStatusRequest()..jobId = jobId;
    final response = await _stub.getJobStatus(request);
    return JobStatus.fromProto(response);
  }
  
  Stream<JobStatus> watchJob(String jobId) async* {
    while (true) {
      final status = await getJobStatus(jobId);
      yield status;
      
      if (status.state == JobState.completed || 
          status.state == JobState.failed) {
        break;
      }
      
      await Future.delayed(Duration(seconds: 1));
    }
  }
  
  Future<String> detectLanguage(String audioPath) async {
    final request = LanguageDetectRequest()..audioPath = audioPath;
    final response = await _stub.detectLanguage(request);
    return response.language;
  }
  
  Future<ServiceStatus> getServiceStatus() async {
    final response = await _stub.getServiceStatus(StatusRequest());
    return ServiceStatus.fromProto(response);
  }
  
  void dispose() {
    _channel.shutdown();
  }
}

// Audio recording helper
class AudioRecorder {
  final _recorder = Record();
  
  Future<bool> requestPermission() async {
    return await _recorder.hasPermission();
  }
  
  Future<void> startRecording(String path) async {
    if (await requestPermission()) {
      await _recorder.start(
        path: path,
        encoder: AudioEncoder.wav,
        bitRate: 128000,
        samplingRate: 16000,
      );
    }
  }
  
  Future<String?> stopRecording() async {
    return await _recorder.stop();
  }
  
  Stream<double> get amplitudeStream => 
      _recorder.onAmplitudeChanged.map((a) => a.current);
}
```

### SurrealDB Schema

```sql
-- Transcription jobs table
DEFINE TABLE transcription_jobs SCHEMAFULL;
DEFINE FIELD job_id ON transcription_jobs TYPE string;
DEFINE FIELD audio_path ON transcription_jobs TYPE string;
DEFINE FIELD status ON transcription_jobs TYPE string;
DEFINE FIELD priority ON transcription_jobs TYPE int DEFAULT 0;
DEFINE FIELD created_at ON transcription_jobs TYPE datetime DEFAULT time::now();
DEFINE FIELD started_at ON transcription_jobs TYPE datetime;
DEFINE FIELD completed_at ON transcription_jobs TYPE datetime;
DEFINE FIELD result ON transcription_jobs TYPE object;
DEFINE FIELD error ON transcription_jobs TYPE string;
DEFINE FIELD options ON transcription_jobs TYPE object;

-- Transcription results cache
DEFINE TABLE transcription_cache SCHEMAFULL;
DEFINE FIELD audio_hash ON transcription_cache TYPE string;
DEFINE FIELD transcription ON transcription_cache TYPE string;
DEFINE FIELD segments ON transcription_cache TYPE array;
DEFINE FIELD language ON transcription_cache TYPE string;
DEFINE FIELD model_used ON transcription_cache TYPE string;
DEFINE FIELD created_at ON transcription_cache TYPE datetime DEFAULT time::now();

-- Index for fast lookup
DEFINE INDEX audio_hash_idx ON transcription_cache FIELDS audio_hash UNIQUE;
DEFINE INDEX job_status_idx ON transcription_jobs FIELDS status;

-- Model registry
DEFINE TABLE whisper_models SCHEMAFULL;
DEFINE FIELD name ON whisper_models TYPE string;
DEFINE FIELD size ON whisper_models TYPE string;
DEFINE FIELD path ON whisper_models TYPE string;
DEFINE FIELD size_mb ON whisper_models TYPE float;
DEFINE FIELD parameters ON whisper_models TYPE string;
DEFINE FIELD is_active ON whisper_models TYPE bool DEFAULT false;
DEFINE FIELD downloaded ON whisper_models TYPE bool DEFAULT false;

-- Function to get pending jobs
DEFINE FUNCTION fn::get_pending_jobs($limit: int) {
  RETURN SELECT * FROM transcription_jobs 
  WHERE status = 'queued' 
  ORDER BY priority DESC, created_at ASC 
  LIMIT $limit;
};

-- Function to update job status
DEFINE FUNCTION fn::update_job_status($job_id: string, $status: string) {
  UPDATE transcription_jobs 
  SET status = $status,
      started_at = IF $status = 'processing' THEN time::now() ELSE started_at END,
      completed_at = IF $status IN ['completed', 'failed'] THEN time::now() ELSE completed_at END
  WHERE job_id = $job_id;
};
```

## Testing Requirements

### Unit Tests

```dart
// test/shared/services/whisper/whisper_service_test.dart
void main() {
  group('WhisperService', () {
    test('transcribes audio accurately', () async {
      final service = WhisperService();
      final testAudio = 'test_assets/sample_speech.wav';
      
      final result = await service.transcribe(testAudio);
      
      expect(result.text, contains('expected text'));
      expect(result.language, equals('en'));
    });
    
    test('detects language correctly', () async {
      final service = WhisperService();
      final spanishAudio = 'test_assets/spanish_speech.wav';
      
      final language = await service.detectLanguage(spanishAudio);
      
      expect(language, equals('es'));
    });
    
    test('handles queue properly', () async {
      final service = WhisperService();
      final jobs = List.generate(5, (i) => 'audio_$i.wav');
      
      final jobIds = <String>[];
      for (final audio in jobs) {
        final id = await service.queueTranscription(audio);
        jobIds.add(id);
      }
      
      expect(jobIds.length, equals(5));
    });
  });
}
```

### Integration Tests

```dart
// integration_test/whisper_service_test.dart
void main() {
  testWidgets('Whisper service end-to-end', (tester) async {
    final client = WhisperClient();
    await client.connect();
    
    // Record audio
    final recorder = AudioRecorder();
    final audioPath = '/tmp/test_recording.wav';
    
    await recorder.startRecording(audioPath);
    await Future.delayed(Duration(seconds: 3));
    await recorder.stopRecording();
    
    // Transcribe
    final result = await client.transcribe(audioPath);
    
    expect(result.text, isNotEmpty);
    expect(result.segments, isNotEmpty);
    
    // Check service status
    final status = await client.getServiceStatus();
    expect(status.isRunning, isTrue);
  });
}
```

### Performance Tests

```dart
// test/shared/services/whisper/performance_test.dart
void main() {
  test('meets real-time factor target', () async {
    final service = WhisperService();
    final audioPath = 'test_assets/30_second_audio.wav';
    
    final stopwatch = Stopwatch()..start();
    final result = await service.transcribe(audioPath);
    stopwatch.stop();
    
    final processingTime = stopwatch.elapsedMilliseconds;
    final audioLength = 30000; // 30 seconds
    final rtf = processingTime / audioLength;
    
    expect(rtf, lessThan(0.5)); // Faster than 0.5x real-time
  });
}
```

### Accessibility Tests

- [ ] Transcription status announced
- [ ] Language detection communicated
- [ ] Queue updates accessible
- [ ] Error states clearly indicated

## Definition of Done

- [ ] Whisper service runs locally
- [ ] Multiple model sizes supported
- [ ] Real-time streaming works
- [ ] Language auto-detection functional
- [ ] Queue management operational
- [ ] Performance targets met (0.5x real-time)
- [ ] gRPC communication established
- [ ] All audio formats supported
- [ ] All tests passing (>80% coverage)
- [ ] Cross-platform compatible
- [ ] Documentation complete
- [ ] Code review approved
