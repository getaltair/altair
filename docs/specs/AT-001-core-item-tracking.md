# Feature AT-001: Core Item Tracking

## What it does

Provides CRUD operations for inventory items with location management, photo attachments, custom fields, and quantity tracking. Foundation for all Tracking features.

## User Journey

GIVEN Robert has an ESP32 module in his workshop
WHEN he adds it to inventory with location "Electronics Drawer 3", quantity 5, and a photo
THEN the item appears in searchable inventory with QR code generation option and low-stock alerts can be configured

## Functional Requirements

### Item Management
- Create, read, update, delete inventory items
- Required fields: name, quantity, unit (pieces/meters/kg/etc)
- Optional fields: description, category, manufacturer, part_number, datasheet_url, cost_per_unit, reorder_threshold, notes
- Photo attachments (multiple per item, stored locally)
- Custom fields with typed values (string, number, boolean, date, url)
- Tags for flexible categorization
- Archive/restore instead of hard delete

### Location Management
- Hierarchical location structure (e.g., Workshop > Electronics Bench > Drawer 3)
- Visual location tree with drag-drop reorganization
- Location templates (e.g., "Standard Tool Chest" with predefined drawers)
- Photo attachments for locations (helpful for remembering physical layout)
- Multiple items can share one location
- Items can have quantity distributed across multiple locations

### Quantity Tracking
- Current quantity with unit specification
- Low stock threshold with visual warnings
- Quantity history log (increases/decreases with timestamps and reasons)
- Support for fractional quantities (e.g., 2.5 meters of wire)
- Batch adjustment for inventory counts
- Consumption tracking integration (see AT-005)

### Photo Management
- Primary photo displayed in lists/cards
- Gallery view for multiple photos
- Mobile camera integration for quick capture
- Local filesystem storage (not in database)
- Thumbnail generation for performance
- Image metadata extraction (EXIF data for timestamps)

### Custom Fields System
- Admin-defined field schemas per category
- Field types: text, number, boolean, date, URL, enum (dropdown)
- Maker-specific presets: resistance, capacitance, voltage_rating, current_rating, package_type, pinout_url
- Conditional fields (show voltage_rating only for power_supply category)
- Validation rules per field type
- Bulk edit custom fields for multiple items

### Business Rules
- Quantity cannot go negative (validation error)
- Archived items excluded from search by default
- Location deletions cascade to "Unsorted" for contained items
- Photo uploads limited to 10MB per image, converted to JPEG
- Custom field values validated against schema before save
- Concurrent edits use optimistic locking (SurrealDB record versioning)

## UI/UX Requirements

### Components

**Flutter Widgets:**
- `inventory_screen.dart` - Main list/grid view with search and filters
- `item_detail_screen.dart` - Full item details with edit mode
- `item_card_widget.dart` - Compact card for list/grid display
- `item_form_widget.dart` - Form for create/edit operations
- `location_picker_widget.dart` - Tree view for location selection
- `photo_gallery_widget.dart` - Image carousel with zoom
- `custom_fields_editor_widget.dart` - Dynamic form for custom fields
- `quantity_adjuster_widget.dart` - Quick +/- buttons with history modal

**Design System Components:**
- `neo_card` - Neo-brutalist card with thick border
- `neo_button` - Primary/secondary action buttons
- `neo_text_field` - Input fields with validation states
- `neo_dropdown` - Custom dropdown with search
- `neo_chip` - Tags and categories display
- `neo_modal` - Bottom sheet and dialog overlays

### Visual Design

**Layout:**
- Grid view (default): 2-4 columns depending on screen width, masonry layout for varied image sizes
- List view: Single column with photo thumbnail left, details right
- Detail screen: Hero animation from grid/list, 2-column on desktop (photo left, details right)
- Responsive breakpoints: 600px (mobile/tablet), 900px (tablet/desktop), 1200px (wide desktop)

**Colors (Neo-brutalist palette):**
- Background: `#FAFAFA` (off-white)
- Cards: `#FFFFFF` with `#000000` 4px border
- Primary action: `#FF6B35` (vibrant orange)
- Secondary action: `#004E89` (deep blue)
- Success/in-stock: `#06D6A0` (teal)
- Warning/low-stock: `#FFD23F` (yellow)
- Error/out-of-stock: `#EE4266` (red)
- Text primary: `#1A1A1A`
- Text secondary: `#6B6B6B`

**Typography:**
- Item name: `Inter Bold 18pt` on cards, `24pt` on detail screen
- Quantity: `Inter Semibold 16pt` with unit suffix
- Metadata: `Inter Regular 14pt`
- Field labels: `Inter Medium 12pt uppercase` with letter-spacing 0.5pt

**Iconography:**
- Location: `location_on` (Material Icons)
- Quantity: `inventory_2`
- Photo: `photo_camera`
- Edit: `edit`
- Archive: `archive`
- Custom fields: `tune`
- QR code: `qr_code_2`
- All icons 24pt, scaled to 20pt in compact views

**Borders/Shadows:**
- Card borders: 4px solid black, no border-radius
- Button borders: 3px solid black
- Drop shadows: `offset(4, 4) blur(0) color(#00000040)` for lifted effect
- Focus states: 2px inset border with primary color
- Hover states: Translate(-2px, -2px) with shadow adjustment for "pop" effect

### User Interactions

**Input Methods:**
- Click/tap: Primary interaction for navigation and selection
- Keyboard: Tab navigation, Enter to submit forms, Escape to cancel
- Drag-drop: Reorder photos in gallery, move items between locations
- Long-press: Context menu for quick actions (edit, archive, QR code)
- Voice: Optional voice-to-text for notes field (mobile)
- Barcode scanner: Integrate SHARED-007 for part number lookup

**Keyboard Shortcuts:**
- `Ctrl/Cmd+N`: Create new item
- `Ctrl/Cmd+F`: Focus search field
- `Ctrl/Cmd+G`: Toggle grid/list view
- `Escape`: Close modal/cancel edit
- `Ctrl/Cmd+S`: Save changes
- `/`: Quick search activation (like Slack)
- Arrow keys: Navigate grid/list items
- Space: Select/deselect for batch operations

**Gestures (Mobile):**
- Swipe left on card: Archive item
- Swipe right on card: Quick quantity adjustment
- Pinch-zoom: Photo gallery zoom
- Pull-to-refresh: Sync data (if cloud enabled)
- Long-press: Context menu

**Feedback:**
- Loading: Skeleton cards during initial load, spinner for operations <2s
- Success: Green checkmark toast (2s auto-dismiss) for saves, green border flash on updated card
- Error: Red toast with retry button, form field validation in real-time
- Empty states: Illustration + "Add your first item" CTA
- Optimistic updates: Immediate UI update, rollback on error

### State Management

**Local State (Component-level):**
- Form validation state
- Photo gallery current index
- Expanded/collapsed sections
- Modal open/closed
- Selection state for batch operations

**Global State (Riverpod):**
```dart
// Providers
final itemsProvider = StateNotifierProvider<ItemsNotifier, AsyncValue<List<Item>>>(
  (ref) => ItemsNotifier(ref.read(itemRepositoryProvider))
);

final selectedItemProvider = StateProvider<Item?>((ref) => null);

final locationsProvider = StateNotifierProvider<LocationsNotifier, AsyncValue<List<Location>>>(
  (ref) => LocationsNotifier(ref.read(locationRepositoryProvider))
);

final itemSearchQueryProvider = StateProvider<String>((ref) => '');

final itemFiltersProvider = StateProvider<ItemFilters>((ref) => ItemFilters.none());

final viewModeProvider = StateProvider<ViewMode>((ref) => ViewMode.grid);

// Notifiers
class ItemsNotifier extends StateNotifier<AsyncValue<List<Item>>> {
  final ItemRepository _repository;
  
  ItemsNotifier(this._repository) : super(const AsyncValue.loading()) {
    loadItems();
  }
  
  Future<void> loadItems() async {
    state = const AsyncValue.loading();
    state = await AsyncValue.guard(() => _repository.getAllItems());
  }
  
  Future<void> createItem(Item item) async {
    final result = await _repository.createItem(item);
    result.fold(
      (error) => state = AsyncValue.error(error, StackTrace.current),
      (newItem) => state.whenData((items) => state = AsyncValue.data([...items, newItem]))
    );
  }
  
  Future<void> updateItem(Item item) async {
    // Optimistic update
    state.whenData((items) {
      final index = items.indexWhere((i) => i.id == item.id);
      final updated = [...items];
      updated[index] = item;
      state = AsyncValue.data(updated);
    });
    
    final result = await _repository.updateItem(item);
    result.fold(
      (error) {
        // Rollback on error
        loadItems();
        // Show error toast
      },
      (_) {} // Success already reflected
    );
  }
  
  Future<void> deleteItem(String id) async {
    state.whenData((items) => 
      state = AsyncValue.data(items.where((i) => i.id != id).toList())
    );
    await _repository.deleteItem(id);
  }
  
  Future<void> adjustQuantity(String id, double delta, String reason) async {
    // Implementation
  }
}
```

**Persistence:**
- Items, locations, custom field schemas saved to SurrealDB via gRPC
- Photos saved to local filesystem: `~/.altair/tracking/photos/{item_id}/{filename}.jpg`
- Thumbnails cached: `~/.altair/tracking/thumbnails/{item_id}_thumb.jpg`
- Offline queue for pending operations (Drift local database)
- Auto-save drafts every 30s when editing

### Responsive Behavior

**Desktop (>1200px):**
- 4-column grid view, 600px max card width
- Side panel for filters (persistent, not modal)
- Detail screen: 2-column layout (photo gallery 40%, details 60%)
- Keyboard shortcuts emphasized
- Hover states active

**Tablet (768-1199px):**
- 3-column grid view, 400px max card width
- Bottom sheet for filters
- Detail screen: Single column with sticky photo header
- Touch-optimized targets (48px min)

**Mobile (<768px):**
- 1-2 column grid view (2 columns for small items like resistors)
- Full-screen modals
- FAB for quick add
- Swipe gestures for common actions
- Bottom navigation bar

**Breakpoint Strategy:**
- Mobile-first CSS (flexbox with wrap)
- Progressive enhancement for desktop features
- Fluid typography using clamp()
- Test on real devices (iPhone SE, Pixel 7, iPad Pro, desktop monitors)

### Accessibility Requirements

**Screen Reader:**
- Semantic HTML/Flutter semantics tree
- ARIA labels for icon buttons: `Semantics(label: 'Edit item', child: Icon(Icons.edit))`
- Image alt text from item name + description
- Live regions for toasts and updates
- Skip links for keyboard navigation

**Keyboard Navigation:**
- Logical tab order (left-to-right, top-to-bottom)
- Focus visible on all interactive elements
- Modal trap focus within dialog
- Escape key closes modals
- Enter key submits forms
- Arrow keys navigate grid/list

**Color Contrast:**
- WCAG 2.1 AA compliance (4.5:1 for text, 3:1 for UI components)
- Test with color blindness simulators
- Don't rely solely on color (use icons + text for status)
- High contrast mode support

**Motion:**
- Respect `prefers-reduced-motion` media query
- Disable animations when preference set
- Provide instant transitions as fallback
- Avoid auto-playing videos/GIFs

**Font Sizing:**
- Scalable text (rem units, not px)
- Minimum 14pt body text
- Support browser zoom up to 200%
- Line height 1.5 for readability

### ADHD-Specific UI Requirements

**Cognitive Load:**
- Progressive disclosure: Show only name, quantity, location by default
- Advanced fields collapsed behind "Show more" button
- Custom fields hidden until category selected
- Limit visible items to 50, lazy load on scroll

**Focus Management:**
- Auto-focus first field in forms
- Clear visual hierarchy (largest = most important)
- Primary action always top-right (consistent placement)
- One primary action per screen (avoid choice paralysis)

**Forgiveness:**
- Undo button in toast for 5s after delete
- "Restore from archive" easy access
- Auto-save drafts (can't lose work)
- Non-destructive edits (version history if possible)
- Confirm destructive actions with clear consequences

**Visual Hierarchy:**
- Photo as largest element (visual anchor)
- Item name next (what is it?)
- Quantity/stock status (is it available?)
- Location third (where is it?)
- Everything else collapsed

**Immediate Feedback:**
- Animations <200ms for quick operations
- Instant search filtering (no debounce >300ms)
- Optimistic UI updates
- Progress indicators for >1s operations
- Haptic feedback on mobile (subtle vibration)

## Non-Functional Requirements

### Performance Targets

- Item list render: <100ms (1000 items)
- Search filtering: <50ms keystroke-to-result
- Photo load: <200ms (with thumbnail caching)
- Form submission: <500ms end-to-end
- Batch operations: <5s (100 items)
- Grid/list view toggle: <50ms (instant)
- Database query: <50ms (indexed fields)
- Offline operation: 100% for CRUD (sync when online)

### Technical Constraints

- Flutter version: 3.16+
- Dart SDK: 3.2+
- Target platforms: Linux, Windows, macOS, Android, iOS
- Minimum Android: API 21 (Lollipop)
- Minimum iOS: 13.0
- SurrealDB: 2.0+ (vector search support)
- Rust backend: 1.91+
- gRPC: tonic 0.12+

### Security Requirements

**Data Encryption:**
- Photos encrypted at rest (AES-256) if sensitive items
- Database credentials in secure storage (not plaintext config)
- gRPC TLS for local communication (optional, for multi-device)

**Input Validation:**
- Sanitize all user input before DB insertion
- Validate custom field values against schema
- Prevent SQL injection (parameterized queries, SurrealQL safe)
- File upload validation (type, size, virus scan if cloud)

**XSS Prevention:**
- Escape HTML in user-generated content (notes, descriptions)
- CSP headers if web version ever created
- Sanitize image EXIF data (strip GPS coordinates unless explicitly kept)

## Implementation Details

### Code Structure

```
altair-tracking/
├── lib/
│   ├── features/
│   │   └── inventory/
│   │       ├── presentation/
│   │       │   ├── screens/
│   │       │   │   ├── inventory_screen.dart
│   │       │   │   ├── item_detail_screen.dart
│   │       │   │   └── item_form_screen.dart
│   │       │   ├── widgets/
│   │       │   │   ├── item_card_widget.dart
│   │       │   │   ├── item_list_tile_widget.dart
│   │       │   │   ├── location_picker_widget.dart
│   │       │   │   ├── photo_gallery_widget.dart
│   │       │   │   ├── custom_fields_editor_widget.dart
│   │       │   │   ├── quantity_adjuster_widget.dart
│   │       │   │   └── item_search_bar_widget.dart
│   │       │   └── providers/
│   │       │       ├── items_provider.dart
│   │       │       ├── locations_provider.dart
│   │       │       ├── selected_item_provider.dart
│   │       │       ├── search_provider.dart
│   │       │       └── view_mode_provider.dart
│   │       ├── domain/
│   │       │   ├── models/
│   │       │   │   ├── item.dart
│   │       │   │   ├── item.freezed.dart
│   │       │   │   ├── item.g.dart
│   │       │   │   ├── location.dart
│   │       │   │   ├── custom_field.dart
│   │       │   │   ├── quantity_log.dart
│   │       │   │   └── item_filters.dart
│   │       │   ├── repositories/
│   │       │   │   ├── item_repository.dart
│   │       │   │   └── location_repository.dart
│   │       │   └── use_cases/
│   │       │       ├── create_item_use_case.dart
│   │       │       ├── update_item_use_case.dart
│   │       │       ├── delete_item_use_case.dart
│   │       │       ├── search_items_use_case.dart
│   │       │       └── adjust_quantity_use_case.dart
│   │       └── data/
│   │           ├── repositories/
│   │           │   ├── item_repository_impl.dart
│   │           │   └── location_repository_impl.dart
│   │           ├── data_sources/
│   │           │   ├── item_remote_data_source.dart (gRPC)
│   │           │   ├── item_local_data_source.dart (Drift)
│   │           │   └── photo_storage_service.dart
│   │           └── models/
│   │               ├── item_dto.dart
│   │               └── location_dto.dart
│   ├── shared/
│   │   ├── widgets/
│   │   │   ├── neo_card.dart
│   │   │   ├── neo_button.dart
│   │   │   ├── neo_text_field.dart
│   │   │   └── neo_dropdown.dart
│   │   └── utils/
│   │       ├── image_utils.dart
│   │       └── validation_utils.dart
│   └── generated/
│       └── proto/
│           ├── tracking.pb.dart
│           ├── tracking.pbgrpc.dart
│           └── tracking.pbenum.dart
└── test/
    ├── features/
    │   └── inventory/
    │       ├── presentation/
    │       │   ├── widgets/
    │       │   └── providers/
    │       ├── domain/
    │       │   └── use_cases/
    │       └── data/
    │           └── repositories/
    └── integration/
        └── inventory_flow_test.dart
```

### Key Files to Create

**Flutter:**
1. `item.dart` - Core domain model with freezed
2. `item_repository.dart` - Abstract repository interface
3. `item_repository_impl.dart` - gRPC + local cache implementation
4. `items_provider.dart` - Riverpod state notifier
5. `inventory_screen.dart` - Main UI screen
6. `item_card_widget.dart` - Reusable item display
7. `photo_storage_service.dart` - Local filesystem + thumbnail generation

**Rust Backend:**
1. `tracking_service.rs` - gRPC service implementation
2. `item_repository.rs` - SurrealDB data access
3. `models/item.rs` - Domain model
4. `schema/items.surql` - Database schema

**gRPC Proto:**
1. `tracking.proto` - Service definitions

### Dependencies

```yaml
# pubspec.yaml
dependencies:
  flutter:
    sdk: flutter
  
  # State Management
  flutter_riverpod: ^2.4.0
  riverpod_annotation: ^2.3.0
  
  # Data Models
  freezed_annotation: ^2.4.1
  json_annotation: ^4.8.1
  
  # Local Database
  drift: ^2.12.0
  sqlite3_flutter_libs: ^0.5.18
  path_provider: ^2.1.1
  path: ^1.8.3
  
  # gRPC
  grpc: ^3.2.4
  protobuf: ^3.1.0
  
  # Image Handling
  image_picker: ^1.0.4
  cached_network_image: ^3.3.0
  image: ^4.1.3  # For thumbnail generation
  
  # UI Components
  flutter_staggered_grid_view: ^0.7.0
  photo_view: ^0.14.0
  
  # Utilities
  uuid: ^4.1.0
  intl: ^0.18.1
  url_launcher: ^6.2.1

dev_dependencies:
  flutter_test:
    sdk: flutter
  
  # Code Generation
  build_runner: ^2.4.6
  freezed: ^2.4.5
  json_serializable: ^6.7.1
  riverpod_generator: ^2.3.0
  
  # Testing
  mockito: ^5.4.3
  integration_test:
    sdk: flutter
```

```toml
# Cargo.toml (Rust backend)
[dependencies]
tokio = { version = "1.35", features = ["full"] }
tonic = "0.12"
prost = "0.13"
surrealdb = "2.0"
axum = "0.7"
tower = "0.4"
tower-http = { version = "0.5", features = ["trace"] }
tracing = "0.1"
tracing-subscriber = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
anyhow = "1.0"
image = "0.24"  # For thumbnail generation

[dev-dependencies]
tonic-build = "0.12"
```

### gRPC Proto Definitions

```protobuf
// proto/tracking.proto
syntax = "proto3";

package altair.tracking.v1;

service TrackingService {
  // Item CRUD
  rpc CreateItem(CreateItemRequest) returns (ItemResponse);
  rpc GetItem(GetItemRequest) returns (ItemResponse);
  rpc UpdateItem(UpdateItemRequest) returns (ItemResponse);
  rpc DeleteItem(DeleteItemRequest) returns (DeleteItemResponse);
  rpc ListItems(ListItemsRequest) returns (ListItemsResponse);
  rpc SearchItems(SearchItemsRequest) returns (ListItemsResponse);
  
  // Quantity management
  rpc AdjustQuantity(AdjustQuantityRequest) returns (ItemResponse);
  rpc GetQuantityHistory(GetQuantityHistoryRequest) returns (QuantityHistoryResponse);
  
  // Location management
  rpc CreateLocation(CreateLocationRequest) returns (LocationResponse);
  rpc GetLocation(GetLocationRequest) returns (LocationResponse);
  rpc UpdateLocation(UpdateLocationRequest) returns (LocationResponse);
  rpc DeleteLocation(DeleteLocationRequest) returns (DeleteLocationResponse);
  rpc ListLocations(ListLocationsRequest) returns (ListLocationsResponse);
  rpc GetLocationHierarchy(GetLocationHierarchyRequest) returns (LocationHierarchyResponse);
  
  // Photo management
  rpc UploadPhoto(stream UploadPhotoRequest) returns (UploadPhotoResponse);
  rpc DeletePhoto(DeletePhotoRequest) returns (DeletePhotoResponse);
  
  // Custom fields
  rpc GetCustomFieldSchema(GetCustomFieldSchemaRequest) returns (CustomFieldSchemaResponse);
  rpc UpdateCustomFieldSchema(UpdateCustomFieldSchemaRequest) returns (CustomFieldSchemaResponse);
}

message Item {
  string id = 1;
  string name = 2;
  string description = 3;
  double quantity = 4;
  string unit = 5;
  string location_id = 6;
  repeated string photo_urls = 7;
  double cost_per_unit = 8;
  double reorder_threshold = 9;
  string manufacturer = 10;
  string part_number = 11;
  string datasheet_url = 12;
  string category = 13;
  repeated string tags = 14;
  map<string, CustomFieldValue> custom_fields = 15;
  string notes = 16;
  bool is_archived = 17;
  string created_at = 18;
  string updated_at = 19;
}

message CustomFieldValue {
  oneof value {
    string string_value = 1;
    double number_value = 2;
    bool bool_value = 3;
    string date_value = 4;
    string url_value = 5;
  }
}

message Location {
  string id = 1;
  string name = 2;
  string description = 3;
  string parent_id = 4;
  string full_path = 5;
  LocationType type = 6;
  repeated string photo_urls = 7;
  map<string, string> metadata = 8;
  string created_at = 9;
  string updated_at = 10;
}

enum LocationType {
  LOCATION_TYPE_UNSPECIFIED = 0;
  LOCATION_TYPE_BUILDING = 1;
  LOCATION_TYPE_ROOM = 2;
  LOCATION_TYPE_FURNITURE = 3;
  LOCATION_TYPE_CONTAINER = 4;
  LOCATION_TYPE_SHELF = 5;
  LOCATION_TYPE_BIN = 6;
}

message CreateItemRequest {
  Item item = 1;
}

message GetItemRequest {
  string id = 1;
}

message UpdateItemRequest {
  Item item = 1;
}

message DeleteItemRequest {
  string id = 1;
}

message DeleteItemResponse {
  bool success = 1;
}

message ListItemsRequest {
  optional string location_id = 1;
  optional string category = 2;
  repeated string tags = 3;
  bool include_archived = 4;
  int32 limit = 5;
  int32 offset = 6;
}

message ListItemsResponse {
  repeated Item items = 1;
  int32 total_count = 2;
}

message SearchItemsRequest {
  string query = 1;
  SearchMode mode = 2;
  optional string location_id = 3;
  optional string category = 4;
  int32 limit = 5;
}

enum SearchMode {
  SEARCH_MODE_UNSPECIFIED = 0;
  SEARCH_MODE_KEYWORD = 1;
  SEARCH_MODE_SEMANTIC = 2;
  SEARCH_MODE_HYBRID = 3;
}

message ItemResponse {
  Item item = 1;
}

message AdjustQuantityRequest {
  string item_id = 1;
  double delta = 2;
  string reason = 3;
  optional string project_id = 4;
}

message GetQuantityHistoryRequest {
  string item_id = 1;
  optional string start_date = 2;
  optional string end_date = 3;
}

message QuantityHistoryResponse {
  repeated QuantityLogEntry entries = 1;
}

message QuantityLogEntry {
  string id = 1;
  string item_id = 2;
  double quantity_before = 3;
  double quantity_after = 4;
  double delta = 5;
  string reason = 6;
  optional string project_id = 7;
  string timestamp = 8;
}

// Location messages (similar pattern)
message CreateLocationRequest {
  Location location = 1;
}

message GetLocationRequest {
  string id = 1;
}

message UpdateLocationRequest {
  Location location = 1;
}

message DeleteLocationRequest {
  string id = 1;
}

message DeleteLocationResponse {
  bool success = 1;
  repeated string affected_item_ids = 2;
}

message ListLocationsRequest {
  optional string parent_id = 1;
  int32 limit = 2;
  int32 offset = 3;
}

message ListLocationsResponse {
  repeated Location locations = 1;
  int32 total_count = 2;
}

message GetLocationHierarchyRequest {
  optional string root_id = 1;
}

message LocationHierarchyResponse {
  repeated LocationNode nodes = 1;
}

message LocationNode {
  Location location = 1;
  repeated LocationNode children = 2;
  int32 item_count = 3;
}

// Photo upload (chunked streaming)
message UploadPhotoRequest {
  oneof data {
    PhotoMetadata metadata = 1;
    bytes chunk = 2;
  }
}

message PhotoMetadata {
  string item_id = 1;
  string filename = 2;
  string mime_type = 3;
  int64 total_size = 4;
}

message UploadPhotoResponse {
  string photo_url = 1;
  string thumbnail_url = 2;
}

message DeletePhotoRequest {
  string item_id = 1;
  string photo_url = 2;
}

message DeletePhotoResponse {
  bool success = 1;
}

// Custom field schema
message GetCustomFieldSchemaRequest {
  string category = 1;
}

message CustomFieldSchemaResponse {
  string category = 1;
  repeated CustomFieldDefinition fields = 2;
}

message CustomFieldDefinition {
  string field_name = 1;
  string label = 2;
  FieldType type = 3;
  bool required = 4;
  optional string default_value = 5;
  repeated string enum_values = 6;
  optional ValidationRule validation = 7;
}

enum FieldType {
  FIELD_TYPE_UNSPECIFIED = 0;
  FIELD_TYPE_STRING = 1;
  FIELD_TYPE_NUMBER = 2;
  FIELD_TYPE_BOOLEAN = 3;
  FIELD_TYPE_DATE = 4;
  FIELD_TYPE_URL = 5;
  FIELD_TYPE_ENUM = 6;
}

message ValidationRule {
  optional double min_value = 1;
  optional double max_value = 2;
  optional string regex_pattern = 3;
  optional int32 max_length = 4;
}

message UpdateCustomFieldSchemaRequest {
  string category = 1;
  repeated CustomFieldDefinition fields = 2;
}
```

### SurrealDB Schema

```sql
-- schema/items.surql

-- Items table
DEFINE TABLE items SCHEMAFULL;
DEFINE FIELD id ON items TYPE record<items>;
DEFINE FIELD name ON items TYPE string ASSERT $value != NONE;
DEFINE FIELD description ON items TYPE option<string>;
DEFINE FIELD quantity ON items TYPE number DEFAULT 0 ASSERT $value >= 0;
DEFINE FIELD unit ON items TYPE string DEFAULT "pieces";
DEFINE FIELD location_id ON items TYPE option<record<locations>>;
DEFINE FIELD photo_urls ON items TYPE array<string> DEFAULT [];
DEFINE FIELD cost_per_unit ON items TYPE option<number>;
DEFINE FIELD reorder_threshold ON items TYPE option<number>;
DEFINE FIELD manufacturer ON items TYPE option<string>;
DEFINE FIELD part_number ON items TYPE option<string>;
DEFINE FIELD datasheet_url ON items TYPE option<string>;
DEFINE FIELD category ON items TYPE option<string>;
DEFINE FIELD tags ON items TYPE array<string> DEFAULT [];
DEFINE FIELD custom_fields ON items TYPE option<object>;
DEFINE FIELD notes ON items TYPE option<string>;
DEFINE FIELD is_archived ON items TYPE bool DEFAULT false;
DEFINE FIELD created_at ON items TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON items TYPE datetime DEFAULT time::now() VALUE time::now();

-- Indexes
DEFINE INDEX items_name_idx ON items FIELDS name SEARCH ANALYZER ascii BM25;
DEFINE INDEX items_part_number_idx ON items FIELDS part_number;
DEFINE INDEX items_category_idx ON items FIELDS category;
DEFINE INDEX items_location_idx ON items FIELDS location_id;
DEFINE INDEX items_tags_idx ON items FIELDS tags;
DEFINE INDEX items_archived_idx ON items FIELDS is_archived;

-- Locations table
DEFINE TABLE locations SCHEMAFULL;
DEFINE FIELD id ON locations TYPE record<locations>;
DEFINE FIELD name ON locations TYPE string ASSERT $value != NONE;
DEFINE FIELD description ON locations TYPE option<string>;
DEFINE FIELD parent_id ON locations TYPE option<record<locations>>;
DEFINE FIELD full_path ON locations TYPE string;
DEFINE FIELD type ON locations TYPE string;
DEFINE FIELD photo_urls ON locations TYPE array<string> DEFAULT [];
DEFINE FIELD metadata ON locations TYPE option<object>;
DEFINE FIELD created_at ON locations TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON locations TYPE datetime DEFAULT time::now() VALUE time::now();

DEFINE INDEX locations_name_idx ON locations FIELDS name;
DEFINE INDEX locations_parent_idx ON locations FIELDS parent_id;
DEFINE INDEX locations_type_idx ON locations FIELDS type;

-- Quantity logs table
DEFINE TABLE quantity_logs SCHEMAFULL;
DEFINE FIELD id ON quantity_logs TYPE record<quantity_logs>;
DEFINE FIELD item_id ON quantity_logs TYPE record<items> ASSERT $value != NONE;
DEFINE FIELD quantity_before ON quantity_logs TYPE number;
DEFINE FIELD quantity_after ON quantity_logs TYPE number;
DEFINE FIELD delta ON quantity_logs TYPE number;
DEFINE FIELD reason ON quantity_logs TYPE string;
DEFINE FIELD project_id ON quantity_logs TYPE option<string>;
DEFINE FIELD timestamp ON quantity_logs TYPE datetime DEFAULT time::now();

DEFINE INDEX quantity_logs_item_idx ON quantity_logs FIELDS item_id;
DEFINE INDEX quantity_logs_timestamp_idx ON quantity_logs FIELDS timestamp;

-- Custom field schemas table
DEFINE TABLE custom_field_schemas SCHEMAFULL;
DEFINE FIELD id ON custom_field_schemas TYPE record<custom_field_schemas>;
DEFINE FIELD category ON custom_field_schemas TYPE string ASSERT $value != NONE;
DEFINE FIELD fields ON custom_field_schemas TYPE array<object>;
DEFINE FIELD created_at ON custom_field_schemas TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON custom_field_schemas TYPE datetime DEFAULT time::now() VALUE time::now();

DEFINE INDEX custom_field_schemas_category_idx ON custom_field_schemas FIELDS category UNIQUE;

-- Graph edges for relationships
DEFINE TABLE located_in TYPE RELATION FROM items TO locations;
DEFINE TABLE contains TYPE RELATION FROM locations TO items;
DEFINE TABLE child_of TYPE RELATION FROM locations TO locations;
```

## Testing Requirements

### Unit Tests

**Business Logic:**
- [ ] Quantity validation (no negatives)
- [ ] Location path calculation (hierarchical)
- [ ] Custom field validation against schema
- [ ] Photo URL generation and parsing
- [ ] Tag deduplication and normalization

**State Management:**
- [ ] Items provider CRUD operations
- [ ] Optimistic updates and rollback
- [ ] Search query debouncing
- [ ] Filter application logic

**Data Transformations:**
- [ ] DTO <-> Domain model conversion
- [ ] gRPC message serialization
- [ ] Custom field type conversions

### Widget Tests

**Component Rendering:**
- [ ] ItemCard displays correct data
- [ ] PhotoGallery handles empty/single/multiple photos
- [ ] LocationPicker shows hierarchy
- [ ] CustomFieldsEditor renders dynamic fields
- [ ] QuantityAdjuster +/- buttons work

**User Interactions:**
- [ ] Form validation shows errors
- [ ] Search input filters list
- [ ] Grid/list toggle switches view
- [ ] Archive swipe gesture works
- [ ] Modal open/close animations

**State Updates:**
- [ ] Updating item refreshes card
- [ ] Creating item adds to list
- [ ] Deleting item removes from view
- [ ] Search query updates results

### Integration Tests

**End-to-End Flows:**
- [ ] Create item with photo and location
- [ ] Edit item quantity and see history
- [ ] Search by name and filter by category
- [ ] Archive item and restore
- [ ] Move item between locations
- [ ] Batch update multiple items

**Cross-App Integration:**
- [ ] BoM parser creates items (AT-003)
- [ ] Quest material linking (AG-xxx)
- [ ] QR code generation (AT-002)

**Database Operations:**
- [ ] SurrealDB connection and queries
- [ ] Graph relationship traversal
- [ ] Full-text search accuracy
- [ ] Concurrent update handling

### Accessibility Tests

**Screen Reader:**
- [ ] All buttons have labels
- [ ] Images have descriptive alt text
- [ ] Form errors announced
- [ ] Navigation landmarks present

**Keyboard-Only:**
- [ ] Tab through entire screen
- [ ] Enter submits forms
- [ ] Escape closes modals
- [ ] Arrow keys navigate grid

**Color Contrast:**
- [ ] WCAG AA compliance verified
- [ ] High contrast mode works
- [ ] Status not color-only (icons too)

**Performance:**
- [ ] Renders 1000 items without jank
- [ ] Search <100ms on 10k items
- [ ] Photo load <200ms with caching
- [ ] Accessibility tree depth <10 levels

## Definition of Done

- [ ] Feature complete per requirements
- [ ] All gRPC endpoints implemented and tested
- [ ] SurrealDB schema deployed
- [ ] Flutter UI matches design system
- [ ] Unit tests passing (>80% coverage)
- [ ] Widget tests passing (all components)
- [ ] Integration tests passing (end-to-end flows)
- [ ] Accessibility audit complete (WCAG 2.1 AA)
- [ ] Performance metrics met (see targets)
- [ ] Code review approved (2 reviewers)
- [ ] Documentation updated (README, inline comments)
- [ ] Dogfooding complete (used for personal inventory)
- [ ] No critical bugs in issue tracker
- [ ] Keyboard shortcuts documented
- [ ] Mobile gestures tested on device
- [ ] ADHD-friendly principles verified (user testing)
