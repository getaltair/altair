# Feature SHARED-008: Image Recognition Service

## What it does

Provides AI-powered image recognition for automatic component identification, allowing users to take photos of items to extract metadata (type, specifications, part numbers) and suggest matching inventory or purchasing options.

## User Journey

GIVEN a user has an unknown electronic component or tool
WHEN they take a photo and activate image recognition
THEN the system identifies the component type, extracts visible text (part numbers, specs), suggests matches in inventory or online distributors, and offers to create a new inventory item with pre-filled fields

## Functional Requirements

### Image Recognition Capabilities
- Component type classification (resistor, capacitor, IC, connector, etc.)
- OCR for part numbers, markings, and specifications
- Size estimation from reference objects (coins, rulers)
- Color band detection (resistor color codes)
- Package type identification (DIP, SMD, QFN, etc.)
- Multi-component detection (identify all items in photo)

### AI/ML Models
- Local image classification model (MobileNet, EfficientNet)
- Cloud OCR service (Google Vision API, AWS Rekognition, or Azure Computer Vision)
- Optional: Local OCR fallback (Tesseract)
- Resistor color code decoder (rule-based + vision)
- Part number normalization and validation

### Metadata Extraction
- Extract visible text from component body
- Parse part numbers to extract manufacturer, series, specs
- Decode resistor color bands to resistance value
- Identify IC packages from markings
- Extract specifications from text (voltage, capacity, tolerance)

### Integration Points
- Pre-fill custom fields when adding new item
- Suggest existing inventory matches by part number
- Query distributor APIs for purchasing options
- Link to datasheets via part number search
- Add to shopping list if not in inventory

### Business Rules
- Image recognition requires camera permission
- Cloud OCR limited to 100 requests/month per user (free tier)
- Local model runs on-device (no internet required)
- Confidence threshold: 70% for auto-suggestions
- Multi-component photos: max 10 components per image
- Image processing timeout: 10 seconds

## UI/UX Requirements

### Components

**Existing Design System:** `Dialog`, `Card`, `Button`, `Chip`, `ProgressIndicator`

**Custom Components:**
- `ImageCaptureWidget` - Camera with overlay guides
- `RecognitionResultCard` - Display identified component
- `ConfidenceIndicator` - Visual confidence percentage
- `SuggestionList` - Matching items or distributors
- `FieldPreviewCard` - Pre-filled custom fields

### Visual Design

**Layout:** Full-screen camera with guide overlay, processing overlay during recognition, results in bottom sheet

**Colors:** Processing: `--color-primary` (blue progress), High confidence (>80%): green indicator, Medium (50-80%): yellow, Low (<50%): red

**Typography:** Component name: 20px bold, Part number: 16px monospace, Confidence: 14px, Suggestions: 14px

**Iconography:** Camera: `camera_alt` (32px), Flash: `flash_on` (24px), Processing: `hourglass` (animated, 32px), Check: `check_circle` (green, 20px)

**Borders/Shadows:** Result cards: 3px solid black + 4px shadow, High confidence: 3px solid green + 4px shadow

### User Interactions

**Input Methods:**
- Tap capture button to take photo
- Select photo from gallery
- Pinch-to-zoom camera preview
- Drag photo to crop recognition area
- Tap suggestion to view details

**Keyboard Shortcuts:**
- `Space` - Capture photo
- `Escape` - Cancel recognition
- `Enter` - Accept suggestion
- `R` - Retry recognition

**Gestures (Mobile):**
- Pinch-to-zoom preview
- Swipe result cards left/right
- Tap to expand suggestion details
- Long-press to save image

**Feedback:**
- Haptic on photo capture
- Progress indicator during processing
- Success animation on identification
- Error toast if recognition fails
- Confidence percentage updates in real-time

### State Management

**Local State:**
- `captureActive: bool` - Camera widget active
- `processing: bool` - Recognition in progress
- `result: RecognitionResult?` - Identified component
- `suggestions: List<InventoryItem>` - Matching items
- `selectedSuggestion: InventoryItem?` - User selection

**Global State (Riverpod):**
```dart
@riverpod
class ImageRecognitionController extends _$ImageRecognitionController {
  Future<RecognitionResult> processImage(File image) async {
    // Run local classification model
    // Extract text via OCR
    // Parse part numbers and specs
    // Return structured result
  }
  
  Future<List<InventoryItem>> findMatches(String partNumber) async {
    // Query inventory for matching part numbers
    // Query distributor APIs for purchasing options
  }
  
  Future<void> createItemFromRecognition(RecognitionResult result) async {
    // Pre-fill item fields from recognized data
    // Open add item dialog
  }
}

@riverpod
class OCRService extends _$OCRService {
  Future<String> extractText(File image) async {
    // Cloud OCR API call
    // Or local Tesseract fallback
  }
}

@riverpod
class ResistorColorDecoder extends _$ResistorColorDecoder {
  Future<double> decodeColorBands(File image) async {
    // Detect color bands
    // Decode to resistance value
  }
}
```

**Persistence:**
- Recognition history: SQLite (last 50 results)
- Cached models: File system
- OCR results: 7-day cache
- User corrections: Training data for model improvement

### Responsive Behavior

**Desktop (>1200px):**
- Camera in centered window (60% screen)
- Results in side panel
- Gallery view of recent recognitions

**Tablet (768-1199px):**
- Full-screen camera
- Results in bottom sheet
- Split-screen for comparison

**Mobile (<768px):**
- Native full-screen camera
- Compact result cards
- Swipe navigation

**Breakpoint Strategy:**
- Mobile-first camera optimization
- Desktop: webcam or file upload

### Accessibility Requirements

**Screen Reader:**
- Announce "Processing image" during recognition
- Announce confidence level with result
- Describe identified component
- Alert on recognition errors

**Keyboard Navigation:**
- All controls keyboard accessible
- Tab order: Capture → Flash → Gallery → Results
- Arrow keys for suggestion navigation

**Color Contrast:**
- Result text: 7:1 contrast
- Confidence indicators: color + icon (not color alone)

**Motion:**
- Respect `prefers-reduced-motion`: disable animations
- No auto-panning or auto-zooming

**Font Sizing:**
- Minimum 14px for all text
- Scalable without breaking layout

### ADHD-Specific UI Requirements

**Cognitive Load:**
- One-tap capture and process
- Auto-suggest most likely match
- Hide low-confidence results (reduce decision fatigue)
- Default to first suggestion

**Focus Management:**
- Camera auto-focuses on center
- Result appears immediately in focus area
- Clear accept/reject actions

**Forgiveness:**
- Retry recognition unlimited times
- Edit recognized fields before accepting
- Undo item creation within 5 minutes

**Visual Hierarchy:**
- Primary suggestion largest and first
- Secondary options smaller, below
- Tertiary actions (cancel) text links

**Immediate Feedback:**
- Processing starts immediately on capture
- Progress bar shows processing stages
- Result appears as soon as available

## Non-Functional Requirements

**Performance:** Image capture <100ms, Local classification <2s, Cloud OCR <5s, Total processing <10s

**Technical:** Flutter 3.16+, packages: camera (0.10.0+), image (4.0.0+), tflite_flutter (0.10.0+), google_ml_kit (0.16.0+)

**Security:** Validate image format and size, rate limit OCR API, encrypt cached recognition data

## Implementation Details

### Code Structure

```
lib/features/image_recognition/
├── presentation/
│   ├── widgets/
│   │   ├── image_capture_widget.dart
│   │   ├── recognition_result_card.dart
│   │   ├── confidence_indicator.dart
│   │   ├── suggestion_list.dart
│   │   └── field_preview_card.dart
│   ├── providers/
│   │   ├── image_recognition_controller.dart
│   │   ├── ocr_service.dart
│   │   └── resistor_color_decoder.dart
│   └── screens/
│       ├── image_capture_screen.dart
│       └── recognition_history_screen.dart
├── domain/
│   ├── models/
│   │   ├── recognition_result.dart
│   │   ├── component_type.dart
│   │   └── extracted_metadata.dart
│   └── repositories/
│       └── recognition_repository.dart
└── data/
    ├── repositories/
    │   └── recognition_repository_impl.dart
    ├── datasources/
    │   ├── local_classifier.dart
    │   ├── cloud_ocr_client.dart
    │   └── recognition_cache.dart
    └── ml_models/
        ├── component_classifier.tflite
        └── resistor_decoder.tflite
```

### Rust Backend

```rust
// src/services/image_recognition_service.rs
use tch::{nn, Device, Tensor};
use image::{DynamicImage, ImageBuffer};

pub struct ImageRecognitionService {
    db: Arc<Surreal<Db>>,
    model: nn::VarStore,
}

impl ImageRecognitionService {
    pub async fn classify_component(&self, image_bytes: Vec<u8>) -> Result<ComponentType> {
        let image = image::load_from_memory(&image_bytes)?;
        let tensor = self.preprocess_image(image);
        
        // Run inference
        let output = self.model.forward(&tensor);
        let predictions = output.softmax(-1, tch::Kind::Float);
        
        // Get top prediction
        let (confidence, class_idx) = predictions.max_dim(1, false);
        
        Ok(ComponentType {
            name: self.class_names[class_idx],
            confidence: confidence.double_value(&[]),
        })
    }
    
    pub async fn extract_text(&self, image_bytes: Vec<u8>) -> Result<String> {
        // Call cloud OCR API (Google Vision, AWS Rekognition, etc.)
        // Or run local Tesseract
    }
    
    pub async fn decode_resistor(&self, image_bytes: Vec<u8>) -> Result<f64> {
        // Detect color bands using computer vision
        // Decode to resistance value
    }
}
```

### gRPC Proto

```protobuf
service ImageRecognitionService {
  rpc RecognizeComponent(RecognitionRequest) returns (RecognitionResponse);
  rpc ExtractText(OCRRequest) returns (OCRResponse);
  rpc DecodeResistor(ResistorRequest) returns (ResistorResponse);
  rpc FindMatches(MatchRequest) returns (MatchResponse);
  rpc SaveRecognition(SaveRecognitionRequest) returns (SaveRecognitionResponse);
}

message RecognitionRequest {
  bytes image_bytes = 1;
  optional string hint = 2; // User can hint component type
}

message RecognitionResponse {
  ComponentType component_type = 1;
  ExtractedMetadata metadata = 2;
  repeated InventoryMatch matches = 3;
  float processing_time_ms = 4;
}

message ComponentType {
  string name = 1;
  float confidence = 2;
  string category = 3;
}

message ExtractedMetadata {
  optional string part_number = 1;
  optional string manufacturer = 2;
  optional string package = 3;
  map<string, string> specifications = 4;
}

message InventoryMatch {
  string item_id = 1;
  string name = 2;
  float similarity = 3;
  bool in_stock = 4;
}

message OCRRequest {
  bytes image_bytes = 1;
}

message OCRResponse {
  string text = 1;
  repeated TextAnnotation annotations = 2;
}

message TextAnnotation {
  string text = 1;
  BoundingBox box = 2;
  float confidence = 3;
}

message BoundingBox {
  int32 x = 1;
  int32 y = 2;
  int32 width = 3;
  int32 height = 4;
}

message ResistorRequest {
  bytes image_bytes = 1;
}

message ResistorResponse {
  double resistance_ohms = 1;
  string formatted_value = 2; // e.g., "10 kΩ"
  string tolerance = 3;
  float confidence = 4;
}
```

### SurrealDB Schema

```sql
-- Recognition history table
DEFINE TABLE recognition_history SCHEMAFULL;
DEFINE FIELD id ON TABLE recognition_history TYPE string;
DEFINE FIELD image_path ON TABLE recognition_history TYPE string;
DEFINE FIELD component_type ON TABLE recognition_history TYPE string;
DEFINE FIELD confidence ON TABLE recognition_history TYPE float;
DEFINE FIELD metadata ON TABLE recognition_history TYPE object;
DEFINE FIELD recognized_at ON TABLE recognition_history TYPE datetime DEFAULT time::now();
DEFINE FIELD user_id ON TABLE recognition_history TYPE string;
DEFINE FIELD action_taken ON TABLE recognition_history TYPE option<string>; // added_item, matched_existing, ignored

DEFINE INDEX recognition_history_user_idx ON TABLE recognition_history COLUMNS user_id;
DEFINE INDEX recognition_history_date_idx ON TABLE recognition_history COLUMNS recognized_at;
```

### ML Models

**Component Classifier:**
- Model: MobileNetV3 or EfficientNet-Lite
- Input: 224x224 RGB images
- Output: 50 component classes
- Training: Custom dataset of electronics components
- Size: ~5MB (compressed)

**OCR:**
- Cloud: Google Vision API (100 requests/month free)
- Local fallback: Tesseract 5.0
- Optimized for: Component markings, part numbers

**Resistor Decoder:**
- Model: Custom CNN for color band detection
- Input: Cropped resistor images
- Output: Resistance value + tolerance
- Size: ~2MB

## Testing Requirements

- [ ] Unit: Image preprocessing, classification logic, OCR parsing, resistor decoding
- [ ] Widget: Camera capture, result cards, suggestion list
- [ ] Integration: End-to-end recognition flow, inventory matching, item creation
- [ ] Performance: Capture <100ms, Classification <2s, OCR <5s, Total <10s
- [ ] Accuracy: Component classification >80%, OCR >90%, Resistor decoding >95%
- [ ] Accessibility: Screen reader, keyboard controls

## Dependencies

**Depends on:** SHARED-001 (SurrealDB), SHARED-002 (gRPC), AT-001 (Item CRUD), AT-003 (Custom Fields)

**Blocks:** Quick add item flow, Advanced search by image, Distributor integration

## Notes

- Train custom models on electronics components dataset
- Consider user feedback loop to improve model accuracy
- Implement caching aggressively to reduce API costs
- Support offline mode with local models only
- Add "Report incorrect recognition" button for user feedback
- Future: Support for PCB identification and tracing
