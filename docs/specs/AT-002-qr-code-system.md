# Feature AT-002: QR Code System

## What it does

Generate and scan QR codes for quick item identification and mobile access. Each item gets a unique QR code linking to item details, enabling fast lookup via smartphone camera without app login.

## User Journey

GIVEN Robert has labeled storage bins with printed QR codes
WHEN he scans a bin's QR code with his phone camera
THEN Altair Tracking opens directly to that bin's items list with option to quick-add new items to that location

## Functional Requirements

### QR Code Generation
- Auto-generate unique QR code for every item on creation
- QR data format: `altair://tracking/item/{uuid}` deep link
- Generate location QR codes: `altair://tracking/location/{uuid}`
- Batch QR generation for multiple items (printable sheet)
- QR code image formats: SVG (vector), PNG (raster), PDF (print-ready)
- Customizable QR code size (small label vs large poster)
- Embed basic item data in QR for offline access (optional): name, location, quantity

### QR Code Scanning
- Camera-based scanning (mobile + desktop with webcam)
- Deep link handling: `altair://` protocol registered
- Fallback to web URL if app not installed: `https://getaltair.app/t/{uuid}` redirects to app install
- Multi-QR scan mode: rapid sequential scanning for inventory audit
- Scan history log (what was scanned, when)
- Offline QR support: cached item data displayed if no network

### Printable Labels
- Label templates: Small (1"x1"), Medium (2"x2"), Large (4"x4")
- Print layouts: Avery 5160 (address labels), Dymo 30334 (medium labels), Brother DK-1201 (large labels)
- Label content: QR code + item name + location (configurable)
- Batch print: Select multiple items → generate PDF with all labels
- Label design customization: Include/exclude fields, font size, border

### Web Access (Future)
- Public QR codes for sharing: `https://getaltair.app/t/{uuid}?share={token}`
- View-only access without app installation
- Expire shared links after N days
- Require password for sensitive items

### Business Rules
- QR codes immutable once generated (tied to item UUID)
- Deleted items: QR code shows "Item archived" message
- Location QR codes: Scan opens location's item list + quick-add button
- Invalid QR codes: Error message with manual search option
- Duplicate prevention: Same item can't have multiple active QR codes
- Offline grace period: 7 days cached data before "please connect" message

## UI/UX Requirements

### Components

**Flutter Widgets:**
- `qr_generator_widget.dart` - Display QR code with download/print options
- `qr_scanner_widget.dart` - Camera viewfinder with scan overlay
- `qr_scanner_screen.dart` - Full-screen scanner with torch toggle
- `qr_batch_print_screen.dart` - Select items → preview labels → print
- `qr_label_template_picker.dart` - Choose label size and layout
- `qr_scan_history_screen.dart` - List of recent scans with timestamps

**Design System Components:**
- `neo_qr_display` - QR code with neo-brutalist border
- `neo_camera_viewfinder` - Camera preview with scan guides
- `neo_label_preview` - Print preview with zoom/pan

### Visual Design

**Layout:**
- QR generator: Centered QR code (300x300px), download/print buttons below
- Scanner: Full-screen camera, scan guides (centered square), torch button top-right
- Batch print: 2-column layout (item selection left, preview right)
- Label preview: Mockup of physical label, 1:1 scale option

**Colors:**
- QR code foreground: `#000000` (pure black for max contrast)
- QR code background: `#FFFFFF` (pure white)
- Scan guide: `#FF6B35` (vibrant orange, 50% opacity)
- Success scan: `#06D6A0` (teal) flash animation
- Error scan: `#EE4266` (red) shake animation
- Scanner background: Black (90% opacity overlay on camera feed)

**Typography:**
- Label item name: `Inter Bold 14pt` (small), `18pt` (medium), `24pt` (large)
- Label location: `Inter Regular 10pt` (small), `12pt` (medium), `16pt` (large)
- Scanner instructions: `Inter Semibold 16pt` white text with shadow

**Iconography:**
- QR code: `qr_code_2` (Material Icons)
- Camera: `photo_camera`
- Flashlight: `flashlight_on` / `flashlight_off`
- Print: `print`
- Download: `download`
- Share: `share`
- All icons 24pt, white for scanner overlay

**Borders/Shadows:**
- QR display: 4px black border, no border-radius
- Scan guide: 3px dashed border, rounded corners (8px) for friendliness
- Label previews: Realistic drop shadow to simulate physical label

### User Interactions

**Input Methods:**
- Tap: Open scanner, download QR, select items for batch print
- Camera: Point at QR code, auto-detect and scan
- Pinch-zoom: Label preview zoom
- Long-press: Quick actions (share QR, copy link)

**Keyboard Shortcuts:**
- `Ctrl/Cmd+Q`: Open QR scanner
- `Ctrl/Cmd+P`: Print selected labels
- `Ctrl/Cmd+D`: Download QR as PNG
- `Escape`: Close scanner
- `F`: Toggle flashlight (when scanner active)

**Gestures (Mobile):**
- Tap screen: Focus camera (tap-to-focus)
- Double-tap: Toggle flashlight
- Swipe up from scanner: View scan history
- Pinch-zoom: Camera zoom (if supported)

**Feedback:**
- Scan success: Haptic vibration + green flash + success sound (optional)
- Scan failure: Red border pulse on scan guide
- QR generation: Instant (no loading, it's local)
- Camera permission denied: Clear modal with "Grant permission" button
- Invalid QR: Toast "QR code not recognized" with retry option

### State Management

**Local State:**
- Camera active/inactive
- Flashlight on/off
- Selected label template
- Batch print item selection
- Scanner viewfinder focus point

**Global State (Riverpod):**
```dart
// Providers
final qrScannerProvider = StateNotifierProvider<QRScannerNotifier, QRScannerState>(
  (ref) => QRScannerNotifier(ref.read(qrServiceProvider))
);

final qrGeneratorProvider = Provider<QRGeneratorService>(
  (ref) => QRGeneratorService()
);

final scanHistoryProvider = StateNotifierProvider<ScanHistoryNotifier, List<QRScan>>(
  (ref) => ScanHistoryNotifier()
);

final labelTemplateProvider = StateProvider<LabelTemplate>(
  (ref) => LabelTemplate.medium()
);

// States
class QRScannerState {
  final bool isScanning;
  final bool flashlightOn;
  final String? lastScannedCode;
  final DateTime? lastScanTime;
  final String? errorMessage;
}

// Notifiers
class QRScannerNotifier extends StateNotifier<QRScannerState> {
  final QRService _service;
  
  QRScannerNotifier(this._service) : super(QRScannerState.initial());
  
  Future<void> startScanning() async {
    state = state.copyWith(isScanning: true, errorMessage: null);
  }
  
  Future<void> stopScanning() async {
    state = state.copyWith(isScanning: false);
  }
  
  void toggleFlashlight() {
    state = state.copyWith(flashlightOn: !state.flashlightOn);
  }
  
  Future<void> onCodeScanned(String code) async {
    try {
      final itemId = _parseQRCode(code);
      final item = await _service.getItemByQR(code);
      
      // Navigate to item detail
      ref.read(selectedItemProvider.notifier).state = item;
      
      // Log scan
      ref.read(scanHistoryProvider.notifier).addScan(QRScan(
        code: code,
        itemId: itemId,
        timestamp: DateTime.now(),
      ));
      
      state = state.copyWith(
        lastScannedCode: code,
        lastScanTime: DateTime.now(),
      );
    } catch (e) {
      state = state.copyWith(
        errorMessage: 'Invalid QR code: ${e.toString()}',
      );
    }
  }
  
  String? _parseQRCode(String code) {
    // Parse altair://tracking/item/{uuid} format
    final uri = Uri.parse(code);
    if (uri.scheme != 'altair' || uri.host != 'tracking') {
      throw Exception('Invalid Altair QR code');
    }
    return uri.pathSegments.last;
  }
}

class QRGeneratorService {
  Future<Uint8List> generateQR({
    required String itemId,
    QRFormat format = QRFormat.png,
    int size = 300,
    bool embedOfflineData = false,
  }) async {
    final deepLink = 'altair://tracking/item/$itemId';
    // Use qr_flutter package for generation
    // Return image bytes
  }
  
  Future<void> generateBatchPDF({
    required List<Item> items,
    required LabelTemplate template,
    required String outputPath,
  }) async {
    // Use pdf package to create multi-label sheet
    // Layout labels according to template (Avery 5160, etc.)
    // Save to file
  }
}
```

**Persistence:**
- Scan history: Local Drift database, last 100 scans
- Label templates: User preferences in shared_preferences
- QR code images: Generated on-demand, cached in memory
- Offline item data: Embedded in QR (up to 500 bytes) if enabled

### Responsive Behavior

**Desktop (>1200px):**
- QR generator: Side-by-side QR display and actions
- Scanner: Webcam feed (if available), 640x480 min
- Batch print: 3-column preview grid
- Keyboard shortcuts emphasized

**Tablet (768-1199px):**
- Scanner: Full-screen (primary use case on tablet)
- Batch print: 2-column preview grid
- Touch-optimized scan button (80px diameter)

**Mobile (<768px):**
- Scanner: Full-screen with minimal UI (only torch button visible)
- QR generator: Single column layout
- Batch print: Single column preview
- Camera is primary input method
- Haptic feedback on scan success

**Breakpoint Strategy:**
- Mobile-first for scanner (most common use case)
- Progressive enhancement for desktop (batch printing, webcam)
- Test with actual QR codes at various distances (6 inches to 3 feet)

### Accessibility Requirements

**Screen Reader:**
- Scanner announces "Camera active, point at QR code"
- Successful scan announces item name
- Error announces "Invalid code, please try again"
- QR code images have alt text: "QR code for {item_name}"

**Keyboard Navigation:**
- Tab to scanner activation button
- Space to toggle flashlight
- Escape to close scanner
- All print preview actions keyboard-accessible

**Color Contrast:**
- Scanner instructions: White text with black shadow (infinite contrast)
- Error messages: Red text on white background (4.5:1 minimum)
- QR codes: Always pure black on white (maximal contrast)

**Motion:**
- Scan flash animation respects prefers-reduced-motion
- Disable auto-scan vibration if motion sensitivity enabled
- Static scan guide option (no pulsing animation)

**Font Sizing:**
- Label text scales with user preference (within label bounds)
- Scanner instructions minimum 16pt
- Print preview zoom up to 200%

### ADHD-Specific UI Requirements

**Cognitive Load:**
- Scanner: Zero UI (just camera feed + guide) until scan completes
- One-tap scan (no manual entry fallback shown initially)
- Auto-navigate to item detail on successful scan (no intermediate confirmation)
- Batch print: Preview first, configure later (progressive disclosure)

**Focus Management:**
- Scanner auto-starts on screen open (immediate action)
- Successful scan provides instant haptic + visual feedback
- Failed scan recovers gracefully (try again, no shame)
- Scanner stays active for rapid multi-scan workflows

**Forgiveness:**
- Accidental scans: Easy "Undo" or "Back" button
- Invalid QR: Suggest manual search instead of dead-end error
- Camera permission: Clear explanation + one-tap fix
- Batch print mistakes: Edit preview before printing

**Visual Hierarchy:**
- Scanner: Scan guide largest element (impossible to miss)
- QR display: QR code front-and-center, actions secondary
- Batch print: Selected items list > preview > actions
- Success feedback: Full-screen flash (unmissable)

**Immediate Feedback:**
- QR generation: Instant (<100ms)
- Scan recognition: <300ms from barcode detection
- Item detail navigation: <500ms end-to-end
- Haptic vibration on mobile (satisfying click feel)

## Non-Functional Requirements

### Performance Targets

- QR generation: <100ms per code
- Batch PDF generation: <2s for 50 labels
- Scan recognition: <300ms from detection to item lookup
- Camera initialization: <1s on mobile, <2s on desktop
- Deep link handling: <500ms app launch to item display
- Offline QR decode: <50ms (no network required)

### Technical Constraints

- Flutter version: 3.16+
- QR package: qr_flutter ^4.1.0
- Camera: camera ^0.10.5 (mobile), webcam_qr_scanner (desktop)
- PDF generation: pdf ^3.10.0
- Deep linking: uni_links ^0.5.1 (deprecated, migrate to app_links ^3.5.0)
- Barcode scanning: mobile_scanner ^3.5.0 (ML Kit based, fast)

### Security Requirements

**Data Privacy:**
- QR codes contain only UUID (no sensitive data by default)
- Offline embed mode: Encrypted payload (AES-256) if enabled
- Shared QR links: Unique token per share, revocable
- Scan history: Local-only, not synced to cloud

**Input Validation:**
- Validate QR code format before parsing
- Sanitize deep link parameters (prevent injection attacks)
- Rate limit scan requests (prevent DoS via rapid scanning)
- Verify item UUIDs exist before displaying data

**Camera Permissions:**
- Request camera permission with clear explanation
- Handle permission denial gracefully (fallback to manual entry)
- No camera recording (live viewfinder only)
- Respect OS-level camera disable settings

## Implementation Details

### Code Structure

```
altair-tracking/
├── lib/
│   ├── features/
│   │   └── qr_system/
│   │       ├── presentation/
│   │       │   ├── screens/
│   │       │   │   ├── qr_scanner_screen.dart
│   │       │   │   ├── qr_batch_print_screen.dart
│   │       │   │   └── qr_scan_history_screen.dart
│   │       │   ├── widgets/
│   │       │   │   ├── qr_generator_widget.dart
│   │       │   │   ├── qr_scanner_widget.dart
│   │       │   │   ├── qr_label_preview_widget.dart
│   │       │   │   ├── qr_label_template_picker.dart
│   │       │   │   └── scan_guide_overlay.dart
│   │       │   └── providers/
│   │       │       ├── qr_scanner_provider.dart
│   │       │       ├── qr_generator_provider.dart
│   │       │       ├── scan_history_provider.dart
│   │       │       └── label_template_provider.dart
│   │       ├── domain/
│   │       │   ├── models/
│   │       │   │   ├── qr_scan.dart
│   │       │   │   ├── label_template.dart
│   │       │   │   └── qr_format.dart
│   │       │   ├── services/
│   │       │   │   ├── qr_generator_service.dart
│   │       │   │   ├── qr_parser_service.dart
│   │       │   │   └── label_printer_service.dart
│   │       │   └── repositories/
│   │       │       └── scan_history_repository.dart
│   │       └── data/
│   │           ├── services/
│   │           │   ├── qr_generator_service_impl.dart
│   │           │   ├── qr_parser_service_impl.dart
│   │           │   └── pdf_label_service.dart
│   │           └── repositories/
│   │               └── scan_history_repository_impl.dart
│   └── shared/
│       └── utils/
│           └── deep_link_handler.dart
└── test/
    ├── features/
    │   └── qr_system/
    │       ├── presentation/
    │       ├── domain/
    │       └── data/
    └── integration/
        └── qr_scan_flow_test.dart
```

### Key Files to Create

**Flutter:**
1. `qr_scanner_screen.dart` - Full-screen camera scanner
2. `qr_generator_service.dart` - QR code generation logic
3. `qr_scanner_provider.dart` - Camera state management
4. `pdf_label_service.dart` - Batch label PDF generation
5. `deep_link_handler.dart` - Handle altair:// protocol
6. `scan_history_repository.dart` - Persist scan logs

**Rust Backend:**
1. `qr_service.rs` - gRPC endpoints for QR operations
2. `models/qr_scan.rs` - Scan log model
3. `services/qr_validator.rs` - Validate QR code format

**gRPC Proto:**
1. `qr.proto` - QR service definitions (extend tracking.proto)

### Dependencies

```yaml
# pubspec.yaml additions
dependencies:
  # QR Code
  qr_flutter: ^4.1.0
  mobile_scanner: ^3.5.0  # Fast ML Kit scanner
  qr: ^3.0.1  # QR code generation
  
  # Camera
  camera: ^0.10.5
  permission_handler: ^11.0.1
  
  # PDF Generation
  pdf: ^3.10.0
  printing: ^5.11.1  # Print/preview PDFs
  
  # Deep Linking
  app_links: ^3.5.0
  
  # Image Processing
  image: ^4.1.3
  
  # Share
  share_plus: ^7.2.1
```

```toml
# Cargo.toml additions
[dependencies]
# QR validation
qrcode = "0.13"
image = "0.24"
```

### gRPC Proto Definitions

```protobuf
// proto/qr.proto (or extend tracking.proto)
syntax = "proto3";

package altair.tracking.v1;

service QRService {
  // Generate QR code for item
  rpc GenerateItemQR(GenerateQRRequest) returns (GenerateQRResponse);
  
  // Validate QR code
  rpc ValidateQR(ValidateQRRequest) returns (ValidateQRResponse);
  
  // Get item by QR code
  rpc GetItemByQR(GetItemByQRRequest) returns (ItemResponse);
  
  // Log scan
  rpc LogScan(LogScanRequest) returns (LogScanResponse);
  
  // Get scan history
  rpc GetScanHistory(GetScanHistoryRequest) returns (GetScanHistoryResponse);
  
  // Generate batch labels PDF
  rpc GenerateBatchLabelsPDF(GenerateBatchLabelsRequest) returns (stream BatchLabelChunk);
}

message GenerateQRRequest {
  string item_id = 1;
  QRFormat format = 2;
  int32 size = 3;
  bool embed_offline_data = 4;
}

enum QRFormat {
  QR_FORMAT_UNSPECIFIED = 0;
  QR_FORMAT_PNG = 1;
  QR_FORMAT_SVG = 2;
  QR_FORMAT_PDF = 3;
}

message GenerateQRResponse {
  bytes qr_image = 1;
  string deep_link = 2;
  string web_url = 3;
}

message ValidateQRRequest {
  string qr_code_data = 1;
}

message ValidateQRResponse {
  bool is_valid = 1;
  string item_id = 2;
  string error_message = 3;
}

message GetItemByQRRequest {
  string qr_code_data = 1;
}

message LogScanRequest {
  string qr_code_data = 1;
  string item_id = 2;
  optional string scanned_by = 3;
  string timestamp = 4;
}

message LogScanResponse {
  string scan_id = 1;
}

message GetScanHistoryRequest {
  optional string item_id = 1;
  int32 limit = 2;
  int32 offset = 3;
}

message GetScanHistoryResponse {
  repeated QRScanLog scans = 1;
  int32 total_count = 2;
}

message QRScanLog {
  string id = 1;
  string qr_code_data = 2;
  string item_id = 3;
  optional string scanned_by = 4;
  string timestamp = 5;
}

message GenerateBatchLabelsRequest {
  repeated string item_ids = 1;
  LabelTemplate template = 2;
}

message LabelTemplate {
  LabelSize size = 1;
  LabelLayout layout = 2;
  bool include_name = 3;
  bool include_location = 4;
  bool include_part_number = 5;
  int32 font_size = 6;
}

enum LabelSize {
  LABEL_SIZE_UNSPECIFIED = 0;
  LABEL_SIZE_SMALL = 1;   // 1"x1"
  LABEL_SIZE_MEDIUM = 2;  // 2"x2"
  LABEL_SIZE_LARGE = 3;   // 4"x4"
}

enum LabelLayout {
  LABEL_LAYOUT_UNSPECIFIED = 0;
  LABEL_LAYOUT_AVERY_5160 = 1;    // 30 labels per sheet
  LABEL_LAYOUT_DYMO_30334 = 2;    // 2-1/4" x 1-1/4" labels
  LABEL_LAYOUT_BROTHER_DK1201 = 3; // Die-cut labels
}

message BatchLabelChunk {
  bytes pdf_chunk = 1;
  int32 chunk_number = 2;
  bool is_final_chunk = 3;
}
```

### SurrealDB Schema

```sql
-- schema/qr_scans.surql

-- QR scan logs
DEFINE TABLE qr_scans SCHEMAFULL;
DEFINE FIELD id ON qr_scans TYPE record<qr_scans>;
DEFINE FIELD qr_code_data ON qr_scans TYPE string ASSERT $value != NONE;
DEFINE FIELD item_id ON qr_scans TYPE record<items> ASSERT $value != NONE;
DEFINE FIELD scanned_by ON qr_scans TYPE option<string>;
DEFINE FIELD timestamp ON qr_scans TYPE datetime DEFAULT time::now();

DEFINE INDEX qr_scans_item_idx ON qr_scans FIELDS item_id;
DEFINE INDEX qr_scans_timestamp_idx ON qr_scans FIELDS timestamp;
```

## Testing Requirements

### Unit Tests

**QR Generation:**
- [ ] Generate QR with correct deep link format
- [ ] PNG/SVG/PDF format outputs
- [ ] Different sizes (100px, 300px, 500px)
- [ ] Offline data embedding (encrypted payload)
- [ ] Batch generation for 50 items

**QR Parsing:**
- [ ] Valid altair:// link parsed correctly
- [ ] Invalid format throws exception
- [ ] Web URL fallback handling
- [ ] Malformed UUID rejection

**Label Generation:**
- [ ] Avery 5160 layout (30 labels, correct positioning)
- [ ] Dymo layout (2.25" x 1.25" size)
- [ ] Custom template application
- [ ] Font scaling for different label sizes

### Widget Tests

**Scanner Widget:**
- [ ] Camera permission request shown
- [ ] Scan guide overlay visible
- [ ] Flashlight toggle works
- [ ] Successful scan shows green flash
- [ ] Failed scan shows error message

**Generator Widget:**
- [ ] QR code image rendered
- [ ] Download button saves PNG
- [ ] Share button opens system share
- [ ] Different format options work

**Batch Print:**
- [ ] Item selection updates preview
- [ ] Template picker changes layout
- [ ] Print button generates PDF
- [ ] Preview shows accurate 1:1 scale

### Integration Tests

**End-to-End QR Flow:**
- [ ] Generate QR → Print label → Scan QR → View item
- [ ] Batch print 10 items → PDF contains all labels
- [ ] Scan invalid QR → Error → Manual search
- [ ] Scan offline QR → Cached data shown
- [ ] Deep link opens app from browser

**Cross-App Integration:**
- [ ] Location QR scanned → Item list + quick-add (AT-001)
- [ ] Item QR scanned → Detail screen (AT-001)
- [ ] Scan history logged (SHARED-005)

**Camera Operations:**
- [ ] Request permission → Grant → Camera starts
- [ ] Toggle flashlight → Light on → Toggle → Light off
- [ ] Scan QR → Navigate → Back → Resume scanning

### Accessibility Tests

**Screen Reader:**
- [ ] Scanner announces "Camera active"
- [ ] Scan success announces item name
- [ ] Flashlight toggle announces state
- [ ] Download/print buttons labeled

**Keyboard-Only:**
- [ ] Space toggles flashlight
- [ ] Enter downloads QR
- [ ] Escape closes scanner
- [ ] Tab through batch print UI

**Color Contrast:**
- [ ] Scanner instructions (white on black) verified
- [ ] QR codes (black on white) verified
- [ ] Error messages WCAG AA compliant

**Performance:**
- [ ] Scanner runs at 30fps minimum
- [ ] QR generation <100ms
- [ ] Camera init <2s on desktop
- [ ] Batch PDF <2s for 50 labels

## Definition of Done

- [ ] QR generation working (PNG, SVG, PDF formats)
- [ ] Camera scanner functional on mobile and desktop
- [ ] Deep link handling (altair:// protocol registered)
- [ ] Batch label PDF generation (Avery 5160 template minimum)
- [ ] Scan history logged and viewable
- [ ] Offline QR support (embedded data optional)
- [ ] All gRPC endpoints implemented
- [ ] Unit tests passing (>80% coverage)
- [ ] Widget tests passing (scanner, generator, batch print)
- [ ] Integration tests passing (end-to-end QR flow)
- [ ] Accessibility audit complete
- [ ] Performance metrics met
- [ ] Code review approved
- [ ] Documentation updated (how to print labels)
- [ ] Dogfooding complete (printed labels for workshop bins)
- [ ] iOS/Android app link handling tested
- [ ] Webcam scanning tested on Linux/Windows/macOS
- [ ] Label templates tested with actual label sheets
- [ ] Flashlight toggle tested on physical devices
