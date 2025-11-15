# Feature AT-006: Project Kits Management

## What it does

Organizes inventory items into project-specific kits, bins, and collections for makers. Enables creation of reusable component sets, tracking kit completeness, managing physical storage containers, and streamlining project setup by grouping all required materials together. Integrates with Guidance quests to automatically suggest kit items based on project BoMs.

## User Journey

**Scenario 1: Creating a Project Kit**
```
GIVEN user is starting a new Arduino IoT project
WHEN user creates kit "Arduino Weather Station"
AND adds components (Arduino Uno, DHT22 sensor, breadboard, jumper wires)
THEN system creates kit with physical location assignment
AND tracks individual item status (in stock, low, missing)
AND generates QR code label for physical bin
```

**Scenario 2: Kit Checkout for Active Project**
```
GIVEN user has complete "ESP32 Dev Kit" in storage bin B-03
WHEN user checks out kit for active quest "Smart Doorbell"
THEN system marks all kit items as "in use"
AND associates kit with quest in Guidance
AND tracks project location (workbench, lab, etc.)
AND sets expected return date
```

**Scenario 3: Kit Template Reuse**
```
GIVEN user successfully completed "Basic LED Blinker" project
WHEN user creates kit template from project
THEN system saves component list as reusable template
AND future projects can clone this kit
AND template includes recommended quantities and notes
```

## Functional Requirements

### FR-1: Kit Creation and Management
- Create named kits with description and project association
- Add items to kits (individual components or bulk selection)
- Set kit quantities (e.g., "5x 220Ω resistor")
- Assign physical storage location (shelf, bin, drawer)
- Generate QR code labels for physical identification
- Support kit categories (dev kits, sensor kits, prototyping kits, tool kits)
- Track kit creation date, last modified, and usage count

### FR-2: Kit Templates
- Save kits as reusable templates
- Template library with community-shared kits (future)
- Clone template to create new kit instance
- Version control for template updates
- Template categories (Arduino, ESP32, Raspberry Pi, 3D printing, etc.)
- Import/export templates as JSON
- Template search and filtering

### FR-3: Kit Completeness Tracking
- Visual indicators: Complete ✅, Partial ⚠️, Missing ❌
- Real-time item availability checking
- "What's missing?" report showing unavailable items
- Substitution suggestions for missing components
- Progress bar showing kit completion percentage
- Auto-update status when inventory changes

### FR-4: Physical Container Management
- Assign kit to physical container (bin, box, drawer)
- Container metadata: size, color, label, location
- Multi-level organization: Shelf → Section → Bin
- Photo attachments for container identification
- Container capacity tracking (volume or item count)
- Generate printable labels with QR codes

### FR-5: Kit Checkout/Check-In
- Check out entire kit for active project
- Associate kit with Guidance quest
- Track checkout location (workbench A, lab, workshop)
- Set expected return date with reminders
- Partial returns supported (some items back, others still in use)
- Check-in validation: Scan QR code to verify all items returned
- Track item condition after return (damaged, missing, intact)

### FR-6: Kit Intelligence
- Auto-suggest kit items from quest BoM
- Detect duplicate items across kits (shared components)
- Recommend kit consolidation (merge similar kits)
- Kit usage analytics (most used, never used)
- Smart kit assembly: Optimize item pulling order by location
- "Build this kit" checklist with item retrieval steps

### FR-7: Multi-Kit Projects
- Assign multiple kits to single project
- Track dependencies (Kit A required before Kit B)
- Cross-kit component sharing with conflict detection
- Kit combination templates (e.g., "Arduino Kit" + "Sensor Kit" = "IoT Starter Kit")

## UI/UX Requirements

### Components

**Flutter Widgets:**
- `KitCard` - Visual kit representation with status
- `KitDetailView` - Full kit contents with item status
- `KitCreationWizard` - Step-by-step kit creation flow
- `KitCompletenessIndicator` - Progress circle showing kit readiness
- `PhysicalContainerWidget` - Container visualization with location
- `KitCheckoutSheet` - Bottom sheet for checkout flow
- `KitTemplateGallery` - Browse template library
- `KitAssemblyChecklistWidget` - Step-by-step assembly guide
- `QRCodeLabelGenerator` - Printable label creator

**Design System Components:**
- `Card` with neo-brutalist 2px borders
- `Chip` for kit categories
- `Badge` for item counts
- `ProgressCircle` for completion status
- `IconButton` for quick actions
- `DropdownButton` for location selection

### Visual Design

**Layout:**
- Desktop: 3-column grid (kit cards), side panel for details
- Tablet: 2-column grid with expandable cards
- Mobile: Single column, full-screen detail view
- Grid spacing: 16px gaps, 24px margins

**Colors:**
- Complete kit: Green (#10B981) border, light green (#D1FAE5) background
- Partial kit: Orange (#F97316) border, light orange (#FED7AA) background
- Missing items: Red (#EF4444) border, light red (#FEE2E2) background
- Inactive kit: Gray (#6B7280) border, light gray (#F3F4F6) background
- Checked out: Blue (#3B82F6) border, light blue (#DBEAFE) background

**Typography:**
- Kit name: Inter Bold, 18px
- Item count: JetBrains Mono, 16px
- Location: Inter Regular, 14px, gray
- Status: Inter Medium, 12px, uppercase

**Iconography:**
- Kit: `cube_box` (32px)
- Complete: `checkmark_circle_fill` (24px, green)
- Partial: `exclamationmark_triangle` (24px, orange)
- Missing: `xmark_circle_fill` (24px, red)
- Location: `mappin_circle` (20px)
- Template: `doc_on_doc` (20px)
- Checkout: `arrow_right_circle` (24px)

**Borders/Shadows:**
- Kit cards: 2px solid black, 4px offset shadow (status-colored)
- Active kit: 3px border for emphasis
- Hover: Translate (-2px, -2px), adjust shadow

### User Interactions

**Input Methods:**
- Click: Open kit details
- Double-click: Quick checkout
- Right-click: Context menu (edit, delete, duplicate, print label)
- Drag-and-drop: Add items to kit, reorder items

**Keyboard Shortcuts:**
- `Ctrl+K` - Create new kit
- `Ctrl+T` - Create from template
- `Ctrl+Shift+K` - Kit library view
- `Ctrl+Shift+O` - Checkout kit
- `Ctrl+P` - Print labels
- `Escape` - Close dialogs

**Gestures (Mobile):**
- Swipe right: Quick checkout
- Swipe left: Check kit completeness
- Long-press: Multi-select kits
- Pinch: Preview container photo

**Feedback:**
- Kit created: Success toast + kit card animates in
- Item added to kit: Item slides into position
- Checkout: Haptic feedback + confirmation modal
- Label generated: PDF download notification
- Animation timing: <250ms

### State Management

**Global State (Riverpod Providers):**
```dart
final kitsProvider = StateNotifierProvider<KitsNotifier, List<Kit>>;
final kitTemplatesProvider = StateNotifierProvider<KitTemplatesNotifier, List<KitTemplate>>;
final activeKitProvider = StateProvider<Kit?>;
final kitFiltersProvider = StateProvider<KitFilters>;
final physicalContainersProvider = StateNotifierProvider<ContainersNotifier, List<PhysicalContainer>>;

// Computed providers
final completeKitsProvider = Provider<List<Kit>>;
final incompleteKitsProvider = Provider<List<Kit>>;
final checkedOutKitsProvider = Provider<List<Kit>>;
final kitsByLocationProvider = Provider.family<List<Kit>, String>;

// Async providers
final kitByIdProvider = FutureProvider.family<Kit, String>;
final kitCompletenessProvider = FutureProvider.family<KitCompleteness, String>;
```

**Persistence:**
- Auto-save kit edits every 15 seconds
- Sync kit status changes immediately
- Cache kit templates locally
- Offline mode: Queue checkout operations

### Responsive Behavior

**Desktop (>1200px):**
- 3-column grid, side panel for details
- Drag-and-drop enabled
- Hover tooltips
- Multi-select with Ctrl+Click

**Tablet (768-1199px):**
- 2-column grid
- Expandable cards for details
- Touch-optimized controls

**Mobile (<768px):**
- Single column
- Full-screen detail view
- Bottom sheet for quick actions
- Swipe gestures

### Accessibility Requirements

**Screen Reader:**
- Kit cards: "Arduino Weather Station kit, 75% complete, 8 items, location B-03"
- Item status: "10x 220Ω resistor, in stock"
- Checkout action: "Check out kit for project Smart Doorbell"

**Keyboard Navigation:**
- Tab through kit cards
- Arrow keys within item lists
- Enter to open details
- Space to select items

**Color Contrast:**
- Status colors meet WCAG AA (4.5:1)
- Icons supplement color coding
- High contrast mode supported

**Motion:**
- Respect prefers-reduced-motion
- Fade transitions instead of slides
- Disable parallax effects when reduced motion

### ADHD-Specific UI Requirements

**Cognitive Load:**
- Show 12 kits max per page, load more on scroll
- "Simple view": Only complete/incomplete status, hide details
- Progressive disclosure: Basic info → Details → Full inventory

**Focus Management:**
- Highlight most recently used kits
- Autofocus on search when kit library opens
- Visual pulse on incomplete kits (subtle, 3s interval)

**Forgiveness:**
- Undo kit checkout (30-minute window)
- Easy kit editing without confirmation dialogs
- Non-destructive archive instead of delete

**Visual Hierarchy:**
- Complete kits fade to background (lower opacity)
- Incomplete kits prominent at top
- Checked-out kits in dedicated section

**Immediate Feedback:**
- Instant visual confirmation for all actions
- Optimistic UI updates
- Progress indicators for multi-step operations

## Non-Functional Requirements

### Performance Targets
- Kit grid render: <150ms for 50 kits
- Completeness check: <100ms per kit
- Label generation: <1s per label
- Template search: <50ms

### Technical Constraints

**Flutter Dependencies:**
```yaml
dependencies:
  flutter_riverpod: ^2.4.0
  qr_flutter: ^4.1.0  # QR code generation
  pdf: ^3.10.0  # PDF label creation
  printing: ^5.12.0  # Print labels
  reorderable_grid_view: ^2.2.0  # Drag-and-drop grid
```

**Rust Backend Dependencies:**
```toml
[dependencies]
qrcode = "0.13"  # QR code generation (optional, can use Flutter)
```

## Implementation Details

### Code Structure

```
lib/
├── features/
│   └── project_kits/
│       ├── presentation/
│       │   ├── screens/
│       │   │   ├── kit_library_screen.dart
│       │   │   ├── kit_detail_screen.dart
│       │   │   ├── kit_creation_wizard_screen.dart
│       │   │   └── template_gallery_screen.dart
│       │   ├── widgets/
│       │   │   ├── kit_card.dart
│       │   │   ├── kit_completeness_indicator.dart
│       │   │   ├── physical_container_widget.dart
│       │   │   ├── kit_checkout_sheet.dart
│       │   │   ├── qr_code_label_generator.dart
│       │   │   └── kit_assembly_checklist_widget.dart
│       │   └── providers/
│       │       ├── kits_provider.dart
│       │       ├── kit_templates_provider.dart
│       │       └── physical_containers_provider.dart
│       ├── domain/
│       │   ├── models/
│       │   │   ├── kit.dart
│       │   │   ├── kit_template.dart
│       │   │   ├── kit_item.dart
│       │   │   ├── physical_container.dart
│       │   │   └── kit_checkout.dart
│       │   ├── repositories/
│       │   │   └── kit_repository.dart
│       │   └── use_cases/
│       │       ├── create_kit.dart
│       │       ├── check_kit_completeness.dart
│       │       ├── checkout_kit.dart
│       │       └── generate_qr_label.dart
│       └── data/
│           ├── repositories/
│           │   └── kit_repository_impl.dart
│           └── data_sources/
│               ├── kit_local_data_source.dart
│               └── kit_remote_data_source.dart
```

### gRPC Service Definition

```protobuf
syntax = "proto3";

package altair.tracking.kits;

import "google/protobuf/timestamp.proto";

service KitService {
  rpc CreateKit(CreateKitRequest) returns (KitResponse);
  rpc UpdateKit(UpdateKitRequest) returns (KitResponse);
  rpc DeleteKit(DeleteKitRequest) returns (DeleteResponse);
  rpc GetKit(GetKitRequest) returns (KitResponse);
  rpc ListKits(ListKitsRequest) returns (ListKitsResponse);
  
  rpc CheckKitCompleteness(CheckKitCompletenessRequest) returns (KitCompletenessResponse);
  rpc CheckoutKit(CheckoutKitRequest) returns (CheckoutResponse);
  rpc CheckinKit(CheckinKitRequest) returns (CheckinResponse);
  
  rpc CreateKitTemplate(CreateKitTemplateRequest) returns (KitTemplateResponse);
  rpc ListKitTemplates(ListKitTemplatesRequest) returns (ListKitTemplatesResponse);
  rpc CloneTemplate(CloneTemplateRequest) returns (KitResponse);
  
  rpc GenerateQRLabel(GenerateQRLabelRequest) returns (QRLabelResponse);
}

message Kit {
  string id = 1;
  string name = 2;
  string description = 3;
  repeated KitItem items = 4;
  PhysicalContainer container = 5;
  string category = 6;
  KitStatus status = 7;
  string project_id = 8;  // Optional, from Guidance
  google.protobuf.Timestamp created_at = 9;
  google.protobuf.Timestamp updated_at = 10;
  int32 usage_count = 11;
}

message KitItem {
  string item_id = 1;
  int32 quantity_required = 2;
  int32 quantity_available = 3;
  bool is_available = 4;
  string substitution_suggestion = 5;  // Optional
}

message PhysicalContainer {
  string id = 1;
  string label = 2;
  string location = 3;  // e.g., "Shelf A, Section 2, Bin 5"
  string size = 4;  // e.g., "Small", "Medium", "Large"
  string color = 5;
  repeated string photo_urls = 6;
  int32 capacity = 7;  // Max items or volume
}

enum KitStatus {
  KIT_STATUS_UNSPECIFIED = 0;
  KIT_STATUS_COMPLETE = 1;  // All items available
  KIT_STATUS_PARTIAL = 2;   // Some items missing
  KIT_STATUS_MISSING = 3;   // Many/critical items missing
  KIT_STATUS_CHECKED_OUT = 4;
  KIT_STATUS_INACTIVE = 5;
}

message KitTemplate {
  string id = 1;
  string name = 2;
  string description = 3;
  repeated TemplateItem items = 4;
  string category = 5;
  int32 usage_count = 6;
  google.protobuf.Timestamp created_at = 7;
}

message TemplateItem {
  string item_name = 1;
  string item_type = 2;  // e.g., "Resistor", "Capacitor", "IC"
  int32 quantity = 3;
  string notes = 4;
}

message KitCheckout {
  string id = 1;
  string kit_id = 2;
  string project_id = 3;  // From Guidance
  string checkout_location = 4;
  google.protobuf.Timestamp checked_out_at = 5;
  google.protobuf.Timestamp expected_return_date = 6;
  bool is_returned = 7;
  google.protobuf.Timestamp returned_at = 8;
}

message KitCompleteness {
  string kit_id = 1;
  int32 total_items = 2;
  int32 available_items = 3;
  int32 missing_items = 4;
  double completion_percentage = 5;
  repeated string missing_item_names = 6;
  repeated string substitution_suggestions = 7;
}

message CreateKitRequest {
  Kit kit = 1;
}

message UpdateKitRequest {
  Kit kit = 1;
}

message DeleteKitRequest {
  string id = 1;
}

message GetKitRequest {
  string id = 1;
}

message ListKitsRequest {
  string category = 1;  // Optional filter
  repeated KitStatus statuses = 2;  // Optional filter
  int32 page_size = 3;
  string page_token = 4;
}

message KitResponse {
  Kit kit = 1;
}

message ListKitsResponse {
  repeated Kit kits = 1;
  string next_page_token = 2;
  int32 total_count = 3;
}

message CheckKitCompletenessRequest {
  string kit_id = 1;
}

message KitCompletenessResponse {
  KitCompleteness completeness = 1;
}

message CheckoutKitRequest {
  string kit_id = 1;
  string project_id = 2;
  string checkout_location = 3;
  google.protobuf.Timestamp expected_return_date = 4;
}

message CheckoutResponse {
  KitCheckout checkout = 1;
}

message CheckinKitRequest {
  string kit_id = 1;
  repeated string damaged_item_ids = 2;
  repeated string missing_item_ids = 3;
}

message CheckinResponse {
  bool success = 1;
  string message = 2;
}

message CreateKitTemplateRequest {
  KitTemplate template = 1;
}

message ListKitTemplatesRequest {
  string category = 1;  // Optional filter
  int32 page_size = 2;
  string page_token = 3;
}

message KitTemplateResponse {
  KitTemplate template = 1;
}

message ListKitTemplatesResponse {
  repeated KitTemplate templates = 1;
  string next_page_token = 2;
  int32 total_count = 3;
}

message CloneTemplateRequest {
  string template_id = 1;
  string new_kit_name = 2;
}

message GenerateQRLabelRequest {
  string kit_id = 1;
  bool include_items_list = 2;
}

message QRLabelResponse {
  bytes pdf_data = 1;
  string qr_code_data = 2;  // Encoded kit ID
}

message DeleteResponse {
  bool success = 1;
  string message = 2;
}
```

### SurrealDB Schema

```sql
-- Kits table
DEFINE TABLE kits SCHEMAFULL;
DEFINE FIELD name ON kits TYPE string;
DEFINE FIELD description ON kits TYPE string;
DEFINE FIELD items ON kits TYPE array<object>;
DEFINE FIELD container ON kits TYPE option<object>;
DEFINE FIELD category ON kits TYPE string;
DEFINE FIELD status ON kits TYPE string
  ASSERT $value IN ['complete', 'partial', 'missing', 'checked_out', 'inactive'];
DEFINE FIELD project_id ON kits TYPE option<string>;
DEFINE FIELD created_at ON kits TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON kits TYPE datetime DEFAULT time::now();
DEFINE FIELD usage_count ON kits TYPE int DEFAULT 0;

DEFINE INDEX kit_category_idx ON kits FIELDS category;
DEFINE INDEX kit_status_idx ON kits FIELDS status;
DEFINE INDEX kit_project_idx ON kits FIELDS project_id;

-- Kit templates table
DEFINE TABLE kit_templates SCHEMAFULL;
DEFINE FIELD name ON kit_templates TYPE string;
DEFINE FIELD description ON kit_templates TYPE string;
DEFINE FIELD items ON kit_templates TYPE array<object>;
DEFINE FIELD category ON kit_templates TYPE string;
DEFINE FIELD usage_count ON kit_templates TYPE int DEFAULT 0;
DEFINE FIELD created_at ON kit_templates TYPE datetime DEFAULT time::now();

DEFINE INDEX template_category_idx ON kit_templates FIELDS category;

-- Physical containers table
DEFINE TABLE physical_containers SCHEMAFULL;
DEFINE FIELD label ON physical_containers TYPE string;
DEFINE FIELD location ON physical_containers TYPE string;
DEFINE FIELD size ON physical_containers TYPE string;
DEFINE FIELD color ON physical_containers TYPE string;
DEFINE FIELD photo_urls ON physical_containers TYPE array<string>;
DEFINE FIELD capacity ON physical_containers TYPE int;
DEFINE FIELD created_at ON physical_containers TYPE datetime DEFAULT time::now();

-- Kit checkouts table
DEFINE TABLE kit_checkouts SCHEMAFULL;
DEFINE FIELD kit_id ON kit_checkouts TYPE record<kits>;
DEFINE FIELD project_id ON kit_checkouts TYPE string;
DEFINE FIELD checkout_location ON kit_checkouts TYPE string;
DEFINE FIELD checked_out_at ON kit_checkouts TYPE datetime DEFAULT time::now();
DEFINE FIELD expected_return_date ON kit_checkouts TYPE datetime;
DEFINE FIELD is_returned ON kit_checkouts TYPE bool DEFAULT false;
DEFINE FIELD returned_at ON kit_checkouts TYPE option<datetime>;

DEFINE INDEX checkout_kit_idx ON kit_checkouts FIELDS kit_id;
DEFINE INDEX checkout_returned_idx ON kit_checkouts FIELDS is_returned;
```

### Rust Backend Implementation

```rust
// src/services/kit_service.rs
use crate::models::{Kit, KitItem, KitCompleteness, KitCheckout};
use crate::repositories::{KitRepository, ItemRepository};
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct KitService {
    kit_repo: KitRepository,
    item_repo: ItemRepository,
}

impl KitService {
    pub fn new(kit_repo: KitRepository, item_repo: ItemRepository) -> Self {
        Self { kit_repo, item_repo }
    }
    
    /// Check kit completeness against current inventory
    pub async fn check_kit_completeness(
        &self,
        kit_id: &str,
    ) -> Result<KitCompleteness, ServiceError> {
        let kit = self.kit_repo.get_kit(kit_id).await?;
        let mut completeness = KitCompleteness {
            kit_id: kit_id.to_string(),
            total_items: kit.items.len() as i32,
            available_items: 0,
            missing_items: 0,
            completion_percentage: 0.0,
            missing_item_names: Vec::new(),
            substitution_suggestions: Vec::new(),
        };
        
        for kit_item in &kit.items {
            let item = self.item_repo.get_item(&kit_item.item_id).await?;
            
            if item.quantity >= kit_item.quantity_required {
                completeness.available_items += 1;
            } else {
                completeness.missing_items += 1;
                completeness.missing_item_names.push(item.name.clone());
                
                // Find substitutions (simplified logic)
                if let Some(sub) = self.find_substitution(&item).await? {
                    completeness.substitution_suggestions.push(
                        format!("Use {} instead of {}", sub.name, item.name)
                    );
                }
            }
        }
        
        completeness.completion_percentage = 
            (completeness.available_items as f64 / completeness.total_items as f64) * 100.0;
        
        Ok(completeness)
    }
    
    /// Check out kit for active project
    pub async fn checkout_kit(
        &self,
        kit_id: &str,
        project_id: &str,
        checkout_location: &str,
        expected_return_date: DateTime<Utc>,
    ) -> Result<KitCheckout, ServiceError> {
        let mut kit = self.kit_repo.get_kit(kit_id).await?;
        
        // Verify kit is complete before checkout
        let completeness = self.check_kit_completeness(kit_id).await?;
        if completeness.completion_percentage < 100.0 {
            return Err(ServiceError::IncompletKit(
                format!("Kit is only {}% complete", completeness.completion_percentage)
            ));
        }
        
        // Update kit status
        kit.status = "checked_out".to_string();
        kit.project_id = Some(project_id.to_string());
        self.kit_repo.update_kit(kit).await?;
        
        // Create checkout record
        let checkout = KitCheckout {
            id: Uuid::new_v4().to_string(),
            kit_id: kit_id.to_string(),
            project_id: project_id.to_string(),
            checkout_location: checkout_location.to_string(),
            checked_out_at: Utc::now(),
            expected_return_date,
            is_returned: false,
            returned_at: None,
        };
        
        self.kit_repo.create_checkout(checkout.clone()).await?;
        
        Ok(checkout)
    }
    
    /// Check in kit after project completion
    pub async fn checkin_kit(
        &self,
        kit_id: &str,
        damaged_item_ids: Vec<String>,
        missing_item_ids: Vec<String>,
    ) -> Result<(), ServiceError> {
        let mut kit = self.kit_repo.get_kit(kit_id).await?;
        
        // Update kit status
        kit.status = "complete".to_string();
        kit.project_id = None;
        self.kit_repo.update_kit(kit).await?;
        
        // Update checkout record
        let mut checkout = self.kit_repo.get_active_checkout(kit_id).await?;
        checkout.is_returned = true;
        checkout.returned_at = Some(Utc::now());
        self.kit_repo.update_checkout(checkout).await?;
        
        // Handle damaged/missing items
        for item_id in damaged_item_ids {
            // Log damage, potentially trigger alerts
            self.item_repo.mark_damaged(&item_id).await?;
        }
        
        for item_id in missing_item_ids {
            // Decrement inventory, trigger low stock alert
            self.item_repo.mark_missing(&item_id).await?;
        }
        
        Ok(())
    }
    
    /// Generate QR code label PDF
    pub async fn generate_qr_label(
        &self,
        kit_id: &str,
        include_items_list: bool,
    ) -> Result<Vec<u8>, ServiceError> {
        let kit = self.kit_repo.get_kit(kit_id).await?;
        
        // QR code data: kit ID for scanning
        let qr_data = kit_id.to_string();
        
        // Generate PDF with QR code and kit info
        // This would use a PDF library to create the label
        // Simplified placeholder:
        let pdf_data = self.create_label_pdf(&kit, &qr_data, include_items_list)?;
        
        Ok(pdf_data)
    }
    
    /// Clone kit template to create new kit
    pub async fn clone_template(
        &self,
        template_id: &str,
        new_kit_name: &str,
    ) -> Result<Kit, ServiceError> {
        let template = self.kit_repo.get_template(template_id).await?;
        
        let mut kit = Kit {
            id: Uuid::new_v4().to_string(),
            name: new_kit_name.to_string(),
            description: template.description.clone(),
            items: Vec::new(),
            container: None,
            category: template.category.clone(),
            status: "inactive".to_string(),
            project_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            usage_count: 0,
        };
        
        // Convert template items to kit items
        for template_item in &template.items {
            // Try to find matching item in inventory
            if let Ok(item) = self.item_repo.find_by_name(&template_item.item_name).await {
                kit.items.push(KitItem {
                    item_id: item.id,
                    quantity_required: template_item.quantity,
                    quantity_available: item.quantity,
                    is_available: item.quantity >= template_item.quantity,
                    substitution_suggestion: None,
                });
            }
        }
        
        self.kit_repo.create_kit(kit.clone()).await?;
        
        // Increment template usage count
        self.kit_repo.increment_template_usage(template_id).await?;
        
        Ok(kit)
    }
    
    /// Find substitution for missing item
    async fn find_substitution(&self, item: &Item) -> Result<Option<Item>, ServiceError> {
        // Simplified logic: Find item in same category with similar specs
        // In reality, this would be more sophisticated
        let similar_items = self.item_repo.find_by_category(&item.category).await?;
        
        for candidate in similar_items {
            if candidate.id != item.id && candidate.quantity > 0 {
                return Ok(Some(candidate));
            }
        }
        
        Ok(None)
    }
    
    fn create_label_pdf(&self, kit: &Kit, qr_data: &str, include_items: bool) -> Result<Vec<u8>, ServiceError> {
        // Placeholder for PDF generation logic
        // Would use a library like printpdf or pdf-canvas
        Ok(vec![])
    }
}
```

## Testing Requirements

### Unit Tests
- [ ] Kit completeness calculation
- [ ] Checkout validation (complete kit required)
- [ ] Template cloning logic
- [ ] Substitution suggestion algorithm
- [ ] QR code data encoding

### Widget Tests
- [ ] Kit card displays correct status colors
- [ ] Completeness indicator updates in real-time
- [ ] Checkout sheet validates inputs
- [ ] Template gallery filters correctly
- [ ] QR label generator produces valid output

### Integration Tests
- [ ] Create kit → Add items → Check completeness → Checkout → Checkin flow
- [ ] Template clone creates functional kit
- [ ] Kit status updates when inventory changes
- [ ] Physical container assignment persists
- [ ] Multi-kit project association

### Accessibility Tests
- [ ] Screen reader announces kit status
- [ ] Keyboard navigation through kit library
- [ ] Color contrast for all status indicators
- [ ] Motion preferences respected

### Performance Tests
- [ ] Render 50 kits in <150ms
- [ ] Completeness check for kit with 20 items in <100ms
- [ ] QR label generation in <1s

## Definition of Done
- [ ] All functional requirements implemented
- [ ] UI matches design specifications
- [ ] All tests passing (>90% coverage)
- [ ] Accessibility audit complete
- [ ] Performance metrics met
- [ ] Code review approved
- [ ] Documentation updated

---

## Dependencies

**Required:**
- AT-001: Item CRUD Operations
- AT-002: Location Management
- SHARED-001: Authentication
- SHARED-002: Database (SurrealDB)
- SHARED-007: QR Scanner (for checkout validation)

**Optional:**
- AT-003: BoM Intelligence (for auto-suggesting kit items from quests)
- AG-001: Quest Board (for project association)

---

## Future Enhancements
- Community template sharing
- Kit popularity rankings
- AR visualization of kit assembly
- Voice-guided kit assembly
- Kit cost tracking
- Subscription kits (recurring replenishment)

---

*Optimized for AI-assisted development with Cursor IDE.*
