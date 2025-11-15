# Feature SHARED-007: Barcode/QR Scanner Service

## What it does

Provides a unified barcode and QR code scanning service for quick item identification, check-in/check-out operations, and adding items to inventory by scanning product barcodes or generated QR codes.

## User Journey

GIVEN a user needs to quickly identify or check in an item
WHEN they activate the scanner (camera or dedicated hardware) and point it at a QR code or barcode
THEN the system decodes the code, looks up the item in the database, and performs the requested action (view details, check in, add to inventory)

## Functional Requirements

### Scanning Capabilities
- QR code scanning (internal item IDs, custom QR codes)
- 1D barcode scanning (UPC, EAN-13, Code 128, Code 39)
- 2D barcode scanning (Data Matrix, PDF417)
- Multi-code detection (scan multiple codes in single frame)
- Real-time continuous scanning mode
- Single-shot mode (scan once, then stop)
- Torch/flashlight control for low-light scanning

### Code Generation
- Generate QR codes for inventory items (item ID, metadata)
- Generate printable labels (QR code + item name + location)
- Batch QR code generation for multiple items
- Export QR codes as PNG, SVG, PDF
- Customizable QR code size, error correction level

### Integration Points
- Item lookup by scanned code
- Quick add item from product barcode (via UPC database API)
- Check-out/check-in integration (AT-008)
- Location assignment via QR code
- Shopping list item matching (scan to find in inventory)

### Barcode Lookup
- Query Open Product Data (UPC database, Barcode Lookup API)
- Extract product name, manufacturer, category from UPC
- Suggest custom fields based on product type
- Cache barcode lookups locally
- Fallback to manual entry if no match

### Business Rules
- QR codes must encode item ID + checksum for validation
- Barcodes scanned but not found trigger "Add Item" flow
- Scanner requires camera permission (request on first use)
- Generated QR codes: version 4, error correction level M
- Label printing: 2" x 1" format (standard address labels)
- Maximum scan rate: 10 scans/second (prevent duplicate scans)

## UI/UX Requirements

### Components

**Existing Design System:** `Dialog`, `FAB`, `Button`, `Icon`, `SnackBar`

**Custom Components:**
- `QRScanner` - Full-screen camera scanner with overlay
- `QRGenerator` - QR code preview and export widget
- `ScanResultCard` - Display item after scan
- `BarcodeSearchLoader` - Loading state during API lookup
- `ScanTargetOverlay` - Visual guide for scanning area

### Visual Design

**Layout:** Full-screen scanner with centered crosshair overlay, torch button top-right, cancel button top-left, scan history bottom

**Colors:** Scanner overlay: semi-transparent dark (80% opacity), Target area: `--color-primary` (bright blue outline), Success flash: `--color-success` (green), Error: `--color-error` (red)

**Typography:** Scan instructions: 16px white, centered above target, Item name after scan: 20px bold

**Iconography:** Torch: `flash_on` (32px), Cancel: `close` (32px), History: `history` (24px), QR generate: `qr_code` (24px)

**Borders/Shadows:** Target overlay: 4px solid `--color-primary`, animated pulsing, Success result card: 3px solid green + 4px shadow

### User Interactions

**Input Methods:**
- Point camera at code (auto-scan)
- Tap screen to focus
- Toggle torch for low light
- Swipe down to cancel
- Tap scanned item card for details

**Keyboard Shortcuts:**
- `Space` - Activate scanner (desktop)
- `Escape` - Cancel scanner
- `F` - Toggle flashlight
- `Enter` - Confirm action on scanned item

**Gestures (Mobile):**
- Pinch-to-zoom camera view
- Double-tap torch toggle
- Swipe down dismiss scanner
- Long-press result card for options

**Feedback:**
- Haptic feedback on successful scan
- Audible beep on scan (optional, configurable)
- Green flash on success
- Red flash on error
- Vibration on multi-code detection

### State Management

**Local State:**
- `scannerActive: bool` - Scanner widget visible
- `torchOn: bool` - Flashlight state
- `lastScan: ScanResult?` - Most recent scan
- `scanMode: ScanMode` - Single/Continuous
- `cameraController: CameraController` - Camera instance

**Global State (Riverpod):**
```dart
@riverpod
class QRScannerController extends _$QRScannerController {
  Future<ScanResult> processCode(String code, String type) async {
    // Decode and validate code
    // Lookup item in database
    // Or query UPC database for product info
  }
  
  Uint8List generateQRCode(String data, int size) {
    // Generate QR code image
  }
  
  Future<void> printLabel(String itemId) async {
    // Generate printable label PDF
  }
}

@riverpod
class BarcodeAPI extends _$BarcodeAPI {
  Future<ProductInfo?> lookupUPC(String barcode) async {
    // Query external UPC database
  }
}

@riverpod
class ScanHistory extends _$ScanHistory {
  List<ScanResult> getRecentScans(int limit) {
    // Return last N scans
  }
}
```

**Persistence:**
- Scan history: Local SQLite cache (last 100 scans)
- QR code cache: File system (generated codes)
- Barcode lookup cache: 30-day expiry
- Scanner preferences: SharedPreferences

### Responsive Behavior

**Desktop (>1200px):**
- Scanner in centered modal (60% screen width)
- QR generator in side panel
- Scan history in left sidebar
- Webcam selector dropdown (multiple cameras)

**Tablet (768-1199px):**
- Full-screen scanner
- QR generator in bottom sheet
- History in floating panel

**Mobile (<768px):**
- Native full-screen scanner
- Optimized for rear camera
- Compact UI controls

**Breakpoint Strategy:**
- Mobile-first scanner optimization
- Desktop: external scanner device support (USB barcode readers)

### Accessibility Requirements

**Screen Reader:**
- Announce "Scanner active" when opened
- Announce scan results immediately
- Describe torch button state
- Alert on scan errors

**Keyboard Navigation:**
- All controls accessible via keyboard
- Tab order: Torch → Cancel → Results
- USB barcode scanner input supported

**Color Contrast:**
- Overlay text: 7:1 contrast on dark background
- Instructions: White text on semi-transparent dark

**Motion:**
- Respect `prefers-reduced-motion`: disable pulsing overlay animation
- No auto-focusing that causes motion sickness

**Font Sizing:**
- Minimum 16px for instructions
- Scalable UI without breaking scanner

### ADHD-Specific UI Requirements

**Cognitive Load:**
- Auto-scan mode (no button press needed)
- Clear visual target area (reduces aiming frustration)
- Scan history visible (don't lose recent scans)
- One-tap access to scanner (FAB on main screen)

**Focus Management:**
- Scanner auto-activates camera on open
- Clear dismiss action (large X button)
- No distracting UI elements while scanning
- Result appears immediately in focus area

**Forgiveness:**
- Undo last scan within 5 seconds
- Re-scan same code if needed (no cooldown)
- Cancel scanner anytime without penalty
- Scan history recoverable

**Visual Hierarchy:**
- Target area is brightest element
- Instructions clearly visible above target
- Action buttons in expected locations (top corners)

**Immediate Feedback:**
- <50ms scan detection
- Visual + haptic + audio feedback
- Result card appears instantly (<100ms)
- Error messages clear and actionable

## Non-Functional Requirements

**Performance:** Scan detection <50ms, Code decode <100ms, Item lookup <200ms, QR generation <500ms

**Technical:** Flutter 3.16+, packages: mobile_scanner (5.0.0+), qr_flutter (4.1.0+), image (4.0.0+), camera (0.10.0+)

**Security:** Validate all scanned codes, prevent code injection attacks, rate limit barcode API: 100 requests/day per user

## Implementation Details

### Code Structure

```
lib/features/qr_scanner/
├── presentation/
│   ├── widgets/
│   │   ├── qr_scanner_widget.dart
│   │   ├── qr_generator_widget.dart
│   │   ├── scan_target_overlay.dart
│   │   ├── scan_result_card.dart
│   │   └── barcode_search_loader.dart
│   ├── providers/
│   │   ├── qr_scanner_controller.dart
│   │   ├── barcode_api.dart
│   │   └── scan_history.dart
│   └── screens/
│       ├── qr_scanner_screen.dart
│       └── qr_generator_screen.dart
├── domain/
│   ├── models/
│   │   ├── scan_result.dart
│   │   ├── product_info.dart
│   │   └── qr_data.dart
│   └── repositories/
│       ├── scanner_repository.dart
│       └── barcode_repository.dart
└── data/
    ├── repositories/
    │   ├── scanner_repository_impl.dart
    │   └── barcode_repository_impl.dart
    └── datasources/
        ├── camera_datasource.dart
        ├── barcode_api_client.dart
        └── scan_history_cache.dart
```

### Rust Backend

```rust
// src/services/qr_service.rs
use qrcode::{QrCode, Version, EcLevel};
use image::{ImageBuffer, Luma};

pub struct QRService {
    db: Arc<Surreal<Db>>,
}

impl QRService {
    pub fn generate_qr_code(&self, data: &str, size: u32) -> Result<Vec<u8>> {
        let code = QrCode::with_error_correction_level(data, EcLevel::M)?;
        let image = code.render::<Luma<u8>>()
            .min_dimensions(size, size)
            .build();
        
        // Convert to PNG bytes
        let mut bytes = Vec::new();
        image.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png)?;
        Ok(bytes)
    }
    
    pub async fn validate_qr_code(&self, code: &str) -> Result<bool> {
        // Extract item ID and checksum
        // Validate checksum
        // Verify item exists in database
    }
    
    pub async fn lookup_item_by_code(&self, code: &str) -> Result<InventoryItem> {
        let item: Option<InventoryItem> = self.db
            .query("SELECT * FROM inventory_items WHERE id = $id OR qr_code = $code OR barcode = $code")
            .bind(("id", code))
            .bind(("code", code))
            .await?
            .take(0)?;
        
        item.ok_or(QRError::ItemNotFound)
    }
}
```

### gRPC Proto

```protobuf
service QRScannerService {
  rpc ProcessScan(ScanRequest) returns (ScanResponse);
  rpc GenerateQRCode(GenerateQRRequest) returns (GenerateQRResponse);
  rpc GenerateLabel(GenerateLabelRequest) returns (GenerateLabelResponse);
  rpc LookupBarcode(BarcodeRequest) returns (BarcodeResponse);
  rpc GetScanHistory(ScanHistoryRequest) returns (ScanHistoryResponse);
}

message ScanRequest {
  string code = 1;
  string code_type = 2; // QR, UPC, EAN13, etc.
  string action = 3; // view, check_in, add_item
}

message ScanResponse {
  InventoryItem item = 1;
  optional ProductInfo product_info = 2;
  bool success = 3;
  string message = 4;
}

message GenerateQRRequest {
  string item_id = 1;
  int32 size = 2; // pixels
  string error_correction = 3; // L, M, Q, H
}

message GenerateQRResponse {
  bytes qr_image = 1; // PNG bytes
  string data_encoded = 2;
}

message BarcodeRequest {
  string barcode = 1;
}

message BarcodeResponse {
  ProductInfo product_info = 1;
  bool found = 2;
}

message ProductInfo {
  string name = 1;
  string manufacturer = 2;
  string category = 3;
  string description = 4;
  optional string image_url = 5;
}
```

### SurrealDB Schema

```sql
-- Add QR/barcode fields to inventory_items
ALTER TABLE inventory_items ADD FIELD qr_code TYPE option<string>;
ALTER TABLE inventory_items ADD FIELD barcode TYPE option<string>;
DEFINE INDEX inventory_items_qr_idx ON TABLE inventory_items COLUMNS qr_code;
DEFINE INDEX inventory_items_barcode_idx ON TABLE inventory_items COLUMNS barcode;

-- Scan history table
DEFINE TABLE scan_history SCHEMAFULL;
DEFINE FIELD id ON TABLE scan_history TYPE string;
DEFINE FIELD code ON TABLE scan_history TYPE string;
DEFINE FIELD code_type ON TABLE scan_history TYPE string;
DEFINE FIELD item_id ON TABLE scan_history TYPE option<string>;
DEFINE FIELD action ON TABLE scan_history TYPE string;
DEFINE FIELD scanned_at ON TABLE scan_history TYPE datetime DEFAULT time::now();
DEFINE FIELD user_id ON TABLE scan_history TYPE string;

DEFINE INDEX scan_history_user_idx ON TABLE scan_history COLUMNS user_id;
DEFINE INDEX scan_history_item_idx ON TABLE scan_history COLUMNS item_id;
```

### External APIs

**UPC Database:**
- API: UPCitemdb.com or Barcode Lookup API
- Rate limit: 100 requests/day (free tier)
- Fallback: Open Food Facts API (food items)
- Cache: 30-day local storage of lookups

## Testing Requirements

- [ ] Unit: QR encode/decode, barcode validation, checksum verification, API mocking
- [ ] Widget: Scanner overlay, QR generator, result card, history
- [ ] Integration: End-to-end scan-to-action, barcode lookup, label generation
- [ ] Performance: Scan detect <50ms, decode <100ms, lookup <200ms
- [ ] Accessibility: Screen reader, keyboard controls, contrast

## Dependencies

**Depends on:** SHARED-001 (SurrealDB), SHARED-002 (gRPC), AT-001 (Item CRUD - for lookups)

**Blocks:** AT-008 (Check-Out/Check-In - uses scanner), AT-002 (Location Management - QR location codes), Quick add flow

## Notes

- Consider NFC tags as alternative to QR codes (future)
- Support external USB barcode scanners (HID keyboard input)
- QR codes should be printable on standard label sheets (Avery 5160)
- Implement rate limiting for barcode API to avoid quota exhaustion
- Cache barcode lookups aggressively (30-day expiry)
- Generate high-quality QR codes (Version 4, error correction M)
