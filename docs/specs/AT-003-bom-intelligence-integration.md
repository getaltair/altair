# Feature AT-003: BoM Intelligence Integration

## What it does

Parse Bill of Materials (BoM) from Knowledge app notes, match items to existing inventory, generate shopping lists for missing components, and link project material requirements to Guidance quests.

## User Journey

GIVEN Robert creates a note in Knowledge with a PCB project BoM containing 20 components
WHEN he triggers "Analyze BoM" from the note
THEN Tracking parses the BoM, matches 15 existing inventory items, highlights 5 missing components, generates a shopping list with distributor links, and offers to create a project kit

## Functional Requirements

### BoM Parsing
- Support common BoM formats: CSV, TSV, Markdown tables, plain text lists
- Extract fields: Quantity, Description/Part Name, Part Number, Reference Designator, Value (for electronics)
- Handle variations: "Qty", "QTY", "Amount" all recognized as quantity
- Fuzzy parsing: Recognize structured data even without perfect formatting
- Confidence scoring: High/Medium/Low confidence per line item
- Manual correction: User can fix misinterpreted lines
- Template-based parsing: Save custom parsers for specific distributor formats (DigiKey, Mouser, LCSC)

### Inventory Matching
- **Phase 1 (MVP):** Exact part number matching only
- **Phase 2:** Fuzzy matching on item name + part number
- **Phase 3:** Parametric matching (e.g., "10kΩ resistor" matches inventory item with resistance=10000)
- Match confidence: Exact (100%), Likely (>80%), Possible (50-80%), No match (<50%)
- Visual match review: Side-by-side comparison (BoM line vs matched inventory item)
- Manual matching: User can override auto-match or select different item
- Partial quantity matching: BoM needs 10, inventory has 5 → flag as "insufficient stock"

### Shopping List Generation
- Missing items: BoM items with no inventory match
- Insufficient items: Matched but quantity < required
- Shopping list: Grouped by distributor (if part number recognized)
- Distributor link generation: DigiKey, Mouser, LCSC, Arrow, Newark (regex-based URL construction)
- Export formats: CSV, printable PDF, Markdown, copy to clipboard
- Price estimation: Optional API integration to fetch current prices (future)
- Bulk order optimization: Suggest buying extra for common components (e.g., "Buy 100x 10kΩ resistors instead of 10x")

### Project Kit Creation
- One-click kit from BoM: Auto-populate project kit with matched items
- Kit name: Derived from BoM source (e.g., "Arduino Thermostat - Kit")
- Reserve quantities: Optionally deduct BoM quantities from inventory (mark as "allocated to kit")
- Kit status: Ready (all items matched), Partial (some missing), Pending (waiting for shopping list items)
- Kit assembly: Check out all items in kit simultaneously (see AT-008)

### Quest Material Linking
- Link BoM to Guidance quest: One quest can have multiple BoMs (e.g., mechanical + electrical)
- Quest material status: Ready/Waiting/Blocked based on BoM completion
- Notification: Alert when all quest materials available
- Visual indicator: Guidance quest card shows material readiness (green checkmark, yellow warning, red X)
- Cross-app navigation: Click "View materials" in Guidance → Opens Tracking with filtered BoM items

### Business Rules
- BoM source: Always stored in Knowledge (Tracking only references, doesn't duplicate)
- BoM parsing: Non-destructive (original note text unchanged)
- Inventory deduction: Optional and reversible (can un-reserve kit quantities)
- Shopping list: Read-only in Tracking (export for external purchasing)
- Match conflicts: User must resolve before kit creation (no auto-accept ambiguous matches)
- Version tracking: BoM updates in Knowledge trigger re-match in Tracking

## UI/UX Requirements

### Components

**Flutter Widgets:**
- `bom_parser_screen.dart` - Select Knowledge note → Parse → Review matches
- `bom_match_review_widget.dart` - Table view with BoM lines + matched inventory
- `bom_match_row_widget.dart` - Single row: BoM item | Match confidence | Inventory item | Actions
- `shopping_list_screen.dart` - Missing items with distributor links
- `bom_kit_creator_widget.dart` - Kit name + item selection + reserve options
- `quest_materials_panel_widget.dart` - Embedded in Guidance: Material status + link to Tracking

**Design System Components:**
- `match_confidence_badge` - Color-coded (green=exact, yellow=likely, red=no match)
- `distributor_chip` - Logo + name (DigiKey, Mouser, etc.)
- `quantity_comparison_widget` - Required vs Available (10 needed / 5 in stock)

### Visual Design

**Layout:**
- BoM parser: 3-step wizard (Select source → Parse → Review matches)
- Match review: Full-width table (6 columns: Checkbox, BoM Item, Confidence, Matched Item, Quantity, Actions)
- Shopping list: 2-column layout (Items left, Distributor links right)
- Kit creator: Single column form with item list preview

**Colors:**
- Exact match (100%): `#06D6A0` (teal)
- Likely match (>80%): `#FFD23F` (yellow)
- Possible match (50-80%): `#FFA559` (orange)
- No match (<50%): `#EE4266` (red)
- Sufficient stock: `#06D6A0` (teal)
- Insufficient stock: `#FFA559` (orange)
- Out of stock: `#EE4266` (red)

**Typography:**
- BoM item description: `Inter Semibold 14pt`
- Part number: `Fira Code Regular 12pt` (monospace for clarity)
- Quantity: `Inter Bold 16pt` (emphasis on numbers)
- Confidence score: `Inter Medium 12pt` with percentage

**Iconography:**
- Exact match: `check_circle` (green)
- Likely match: `help` (yellow question mark)
- No match: `cancel` (red X)
- Shopping cart: `shopping_cart`
- Kit: `inventory_2`
- Link to Guidance: `link`
- External link: `open_in_new`

**Borders/Shadows:**
- Match review rows: 2px bottom border (separator)
- Confidence badges: 2px solid border, color-coded
- Shopping list sections: 4px left border (distributor color)

### User Interactions

**Input Methods:**
- Click: Select BoM source, review matches, accept/reject matches
- Keyboard: Arrow keys to navigate match rows, Enter to accept, R to reject
- Drag-drop: Manually drag inventory item onto BoM row to match
- Search: Filter BoM items by description/part number during review

**Keyboard Shortcuts:**
- `Ctrl/Cmd+B`: Open BoM parser
- `Ctrl/Cmd+M`: Review matches
- `Ctrl/Cmd+K`: Create kit from BoM
- `Ctrl/Cmd+L`: View shopping list
- `A`: Accept all exact matches
- `R`: Reject selected match
- `Space`: Toggle item selection for kit

**Gestures (Mobile):**
- Swipe right on match row: Accept match
- Swipe left on match row: Reject match
- Long-press: Manual match picker
- Pull-to-refresh: Re-parse BoM from Knowledge

**Feedback:**
- Parsing: Progress bar for large BoMs (>50 lines)
- Match success: Green checkmark animation per row
- Match failure: Red X pulse
- Kit creation: Success toast "Project kit created with 15 items"
- Shopping list export: "Copied to clipboard" or "PDF downloaded"

### State Management

**Local State:**
- Current BoM parsing step (select/parse/review)
- Selected match overrides
- Expanded/collapsed shopping list sections
- Kit form validation state

**Global State (Riverpod):**
```dart
// Providers
final bomParserProvider = StateNotifierProvider<BomParserNotifier, BomParserState>(
  (ref) => BomParserNotifier(ref.read(bomServiceProvider))
);

final bomMatchProvider = StateNotifierProvider<BomMatchNotifier, BomMatchState>(
  (ref) => BomMatchNotifier(ref.read(matchingServiceProvider))
);

final shoppingListProvider = StateNotifierProvider<ShoppingListNotifier, ShoppingList>(
  (ref) => ShoppingListNotifier()
);

final projectKitProvider = StateNotifierProvider<ProjectKitNotifier, AsyncValue<ProjectKit?>>(
  (ref) => ProjectKitNotifier(ref.read(kitRepositoryProvider))
);

// States
class BomParserState {
  final String? sourceNoteId;
  final List<BomLine>? parsedLines;
  final BomParsingStatus status;
  final double? progress;
  final String? errorMessage;
}

enum BomParsingStatus { idle, selecting, parsing, parsed, error }

class BomMatchState {
  final List<BomMatch> matches;
  final int exactMatches;
  final int likelyMatches;
  final int noMatches;
  final bool isMatching;
}

class BomMatch {
  final BomLine bomLine;
  final Item? matchedItem;
  final double confidence; // 0.0 to 1.0
  final MatchStatus status; // pending, accepted, rejected, manual
  final int quantityAvailable;
  final int quantityRequired;
}

class ShoppingList {
  final List<ShoppingListItem> items;
  final Map<String, List<ShoppingListItem>> itemsByDistributor;
}

// Notifiers
class BomParserNotifier extends StateNotifier<BomParserState> {
  final BomService _service;
  
  BomParserNotifier(this._service) : super(BomParserState.initial());
  
  Future<void> selectKnowledgeNote(String noteId) async {
    state = state.copyWith(sourceNoteId: noteId, status: BomParsingStatus.selecting);
  }
  
  Future<void> parseBoM(String noteContent) async {
    state = state.copyWith(status: BomParsingStatus.parsing, progress: 0.0);
    
    try {
      final lines = await _service.parseBoM(
        content: noteContent,
        onProgress: (p) => state = state.copyWith(progress: p),
      );
      
      state = state.copyWith(
        parsedLines: lines,
        status: BomParsingStatus.parsed,
        progress: 1.0,
      );
      
      // Auto-trigger matching
      ref.read(bomMatchProvider.notifier).matchItems(lines);
    } catch (e) {
      state = state.copyWith(
        status: BomParsingStatus.error,
        errorMessage: e.toString(),
      );
    }
  }
}

class BomMatchNotifier extends StateNotifier<BomMatchState> {
  final MatchingService _service;
  
  BomMatchNotifier(this._service) : super(BomMatchState.initial());
  
  Future<void> matchItems(List<BomLine> bomLines) async {
    state = state.copyWith(isMatching: true);
    
    final matches = <BomMatch>[];
    for (final line in bomLines) {
      final match = await _service.findBestMatch(line);
      matches.add(match);
    }
    
    final exactCount = matches.where((m) => m.confidence == 1.0).length;
    final likelyCount = matches.where((m) => m.confidence > 0.8 && m.confidence < 1.0).length;
    final noMatchCount = matches.where((m) => m.confidence < 0.5).length;
    
    state = state.copyWith(
      matches: matches,
      exactMatches: exactCount,
      likelyMatches: likelyCount,
      noMatches: noMatchCount,
      isMatching: false,
    );
  }
  
  void acceptMatch(int index) {
    final updated = [...state.matches];
    updated[index] = updated[index].copyWith(status: MatchStatus.accepted);
    state = state.copyWith(matches: updated);
  }
  
  void rejectMatch(int index) {
    final updated = [...state.matches];
    updated[index] = updated[index].copyWith(status: MatchStatus.rejected);
    state = state.copyWith(matches: updated);
  }
  
  void setManualMatch(int index, Item item) {
    final updated = [...state.matches];
    updated[index] = updated[index].copyWith(
      matchedItem: item,
      status: MatchStatus.manual,
      confidence: 1.0,
    );
    state = state.copyWith(matches: updated);
  }
}
```

**Persistence:**
- BoM references: Stored in SurrealDB (links to Knowledge notes)
- Match overrides: Persisted for future re-matches
- Project kits: Full persistence with item allocations
- Shopping lists: Temporary (regenerated on demand)

### Responsive Behavior

**Desktop (>1200px):**
- Match review: Full table view (6 columns visible)
- Side-by-side: BoM source (left) and match review (right) for manual correction
- Keyboard shortcuts emphasized for power users

**Tablet (768-1199px):**
- Match review: 4 columns (hide confidence % on small screens, show icon only)
- Scrollable table with sticky header
- Touch-optimized row height (60px min)

**Mobile (<768px):**
- Match review: Card-based layout (one BoM item per card)
- Swipe gestures for accept/reject
- Simplified shopping list (distributor links as buttons)
- Stack layout for quantity comparison

**Breakpoint Strategy:**
- Desktop-first for match review (table optimal)
- Mobile-first for shopping list (simple list view)
- Progressive enhancement for drag-drop manual matching

### Accessibility Requirements

**Screen Reader:**
- Match confidence announced: "Exact match", "Likely match", "No match"
- Quantity status announced: "5 available, 10 required, insufficient stock"
- Action buttons labeled: "Accept match for 10kΩ resistor"
- Table headers properly marked with scope

**Keyboard Navigation:**
- Tab through match rows
- Enter to accept, R to reject
- Arrow keys to navigate table
- Space to select for kit creation
- Escape to cancel parsing

**Color Contrast:**
- Confidence badges: Icons + text (not color-only)
- Stock status: Icons (checkmark, warning, X) + color
- All text meets WCAG AA contrast

**Motion:**
- Parsing progress: Respect prefers-reduced-motion
- Match animations: Disable if motion sensitive
- Static alternatives for all animations

**Font Sizing:**
- Part numbers: Minimum 12pt monospace (legibility)
- Quantities: Minimum 14pt (important data)
- Descriptions: Scalable with user preference

### ADHD-Specific UI Requirements

**Cognitive Load:**
- Wizard workflow: One step at a time (select → parse → review)
- Auto-accept exact matches: Reduce decision fatigue (user can override)
- Hide complexity: Confidence scores shown as icons, percentage on hover
- Focus on mismatches: Auto-scroll to first unmatched item

**Focus Management:**
- Auto-focus first unmatch in review step
- Primary action: "Create kit" always visible (sticky footer)
- Clear next step: "3 items need manual matching" → Auto-scroll to first one
- Preserve context: Return to same scroll position after manual match

**Forgiveness:**
- Non-destructive: Original Knowledge note never modified
- Easy undo: Reject match → "Undo" toast appears
- Re-match anytime: Changes in Knowledge trigger re-parse offer
- Draft kits: Save kit without committing inventory allocation

**Visual Hierarchy:**
- Exact matches: De-emphasized (collapsed by default, "15 exact matches" summary)
- Mismatches: Highlighted (expanded, require attention)
- Quantities: Largest element in row (what you need to know)
- Actions: Secondary (available but not distracting)

**Immediate Feedback:**
- Parsing: Real-time progress bar (no "hang" anxiety)
- Matching: Instant confidence badges (<100ms per row)
- Accept/reject: Immediate row update (no confirmation modal)
- Kit creation: Success animation with confetti (dopamine hit)

## Non-Functional Requirements

### Performance Targets

- BoM parsing: <2s for 100-line BoM
- Inventory matching: <50ms per line (5s for 100 items total)
- Match review render: <200ms (large table)
- Shopping list generation: <500ms
- Kit creation: <1s (including database writes)
- Cross-app navigation: <300ms (Guidance → Tracking)

### Technical Constraints

- Flutter version: 3.16+
- Rust backend: Parsing service (nom or pest parser combinator)
- gRPC: Cross-app communication (Knowledge ↔ Tracking ↔ Guidance)
- SurrealDB: Graph queries for BoM → Item relationships
- Minimum BoM size: 1 line, maximum: 10,000 lines (performance target: 1000 realistic max)

### Security Requirements

**Data Privacy:**
- BoM data: Never cached outside Knowledge app
- Shopping lists: Ephemeral (not logged to avoid price/vendor tracking)
- Distributor links: Generated client-side (no external API calls in MVP)

**Input Validation:**
- Sanitize BoM text (prevent injection attacks)
- Validate part numbers (alphanumeric + hyphens only)
- Limit BoM size: 10MB max text file
- Rate limit: Max 10 BoM parses per minute

**Cross-App Security:**
- gRPC authentication required for Knowledge → Tracking calls
- BoM source verification: Ensure note exists in Knowledge before parsing
- Kit creation: Validate item IDs exist in Tracking

## Implementation Details

### Code Structure

```
altair-tracking/
├── lib/
│   ├── features/
│   │   └── bom_intelligence/
│   │       ├── presentation/
│   │       │   ├── screens/
│   │       │   │   ├── bom_parser_screen.dart
│   │       │   │   ├── bom_match_review_screen.dart
│   │       │   │   ├── shopping_list_screen.dart
│   │       │   │   └── kit_creator_screen.dart
│   │       │   ├── widgets/
│   │       │   │   ├── bom_match_table_widget.dart
│   │       │   │   ├── bom_match_row_widget.dart
│   │       │   │   ├── match_confidence_badge.dart
│   │       │   │   ├── quantity_comparison_widget.dart
│   │       │   │   ├── distributor_link_chip.dart
│   │       │   │   └── manual_match_picker_dialog.dart
│   │       │   └── providers/
│   │       │       ├── bom_parser_provider.dart
│   │       │       ├── bom_match_provider.dart
│   │       │       ├── shopping_list_provider.dart
│   │       │       └── project_kit_provider.dart
│   │       ├── domain/
│   │       │   ├── models/
│   │       │   │   ├── bom_line.dart
│   │       │   │   ├── bom_match.dart
│   │       │   │   ├── shopping_list_item.dart
│   │       │   │   ├── project_kit.dart
│   │       │   │   └── match_confidence.dart
│   │       │   ├── services/
│   │       │   │   ├── bom_parser_service.dart
│   │       │   │   ├── matching_service.dart
│   │       │   │   ├── shopping_list_service.dart
│   │       │   │   └── distributor_link_service.dart
│   │       │   └── repositories/
│   │       │       ├── bom_repository.dart
│   │       │       └── kit_repository.dart
│   │       └── data/
│   │           ├── services/
│   │           │   ├── bom_parser_service_impl.dart
│   │           │   ├── matching_service_impl.dart
│   │           │   └── distributor_link_service_impl.dart
│   │           ├── repositories/
│   │           │   ├── bom_repository_impl.dart
│   │           │   └── kit_repository_impl.dart
│   │           └── parsers/
│   │               ├── csv_bom_parser.dart
│   │               ├── markdown_table_parser.dart
│   │               └── plain_text_parser.dart
└── test/
    └── features/
        └── bom_intelligence/
            ├── presentation/
            ├── domain/
            │   └── services/
            │       └── bom_parser_test.dart
            └── data/
                └── parsers/
                    └── csv_bom_parser_test.dart
```

**Rust Backend:**
```
altair-backend/
├── src/
│   ├── services/
│   │   ├── bom_parser.rs
│   │   ├── item_matcher.rs
│   │   └── kit_service.rs
│   ├── parsers/
│   │   ├── csv_parser.rs
│   │   ├── markdown_parser.rs
│   │   └── text_parser.rs
│   └── models/
│       ├── bom.rs
│       └── match_result.rs
```

### Key Files to Create

**Flutter:**
1. `bom_parser_service.dart` - Parse various BoM formats
2. `matching_service.dart` - Match BoM lines to inventory
3. `bom_match_review_screen.dart` - Main UI for match review
4. `shopping_list_service.dart` - Generate missing items list
5. `distributor_link_service.dart` - Generate purchase URLs
6. `project_kit_repository.dart` - Persist kits

**Rust Backend:**
1. `bom_parser.rs` - High-performance BoM parsing (nom/pest)
2. `item_matcher.rs` - Matching algorithms (exact, fuzzy, parametric)
3. `kit_service.rs` - gRPC service for kit management

**gRPC Proto:**
1. `bom.proto` - BoM parsing and matching service definitions

### Dependencies

```yaml
# pubspec.yaml additions
dependencies:
  # CSV parsing
  csv: ^5.1.1
  
  # Markdown parsing
  markdown: ^7.1.1
  
  # Fuzzy matching (Phase 2)
  fuzzywuzzy: ^2.0.0
  
  # URL launching (distributor links)
  url_launcher: ^6.2.1
  
  # Data tables
  data_table_2: ^2.5.9
```

```toml
# Cargo.toml additions
[dependencies]
# BoM parsing
nom = "7.1"          # Or pest = "2.7" (choose one)
csv = "1.3"

# Fuzzy matching (Phase 2)
strsim = "0.11"      # String similarity
fuzzy-matcher = "0.3.7"

# Regex for part number extraction
regex = "1.10"
```

### gRPC Proto Definitions

```protobuf
// proto/bom.proto
syntax = "proto3";

package altair.tracking.v1;

service BomService {
  // Parse BoM from text
  rpc ParseBom(ParseBomRequest) returns (ParseBomResponse);
  
  // Match BoM lines to inventory
  rpc MatchBomToInventory(MatchBomRequest) returns (MatchBomResponse);
  
  // Create project kit from BoM
  rpc CreateKitFromBom(CreateKitFromBomRequest) returns (ProjectKitResponse);
  
  // Generate shopping list
  rpc GenerateShoppingList(GenerateShoppingListRequest) returns (ShoppingListResponse);
  
  // Link BoM to Guidance quest
  rpc LinkBomToQuest(LinkBomToQuestRequest) returns (LinkBomToQuestResponse);
}

message ParseBomRequest {
  string source_note_id = 1;  // Knowledge note ID
  string content = 2;
  BomFormat format = 3;
}

enum BomFormat {
  BOM_FORMAT_UNSPECIFIED = 0;
  BOM_FORMAT_CSV = 1;
  BOM_FORMAT_TSV = 2;
  BOM_FORMAT_MARKDOWN = 3;
  BOM_FORMAT_PLAIN_TEXT = 4;
  BOM_FORMAT_AUTO_DETECT = 5;
}

message ParseBomResponse {
  repeated BomLine lines = 1;
  int32 total_lines = 2;
  int32 parsed_lines = 3;
  repeated string parsing_errors = 4;
}

message BomLine {
  int32 line_number = 1;
  string description = 2;
  optional string part_number = 3;
  int32 quantity = 4;
  optional string reference_designator = 5;
  optional string value = 6;  // For electronics: "10kΩ", "100nF", etc.
  optional string manufacturer = 7;
  map<string, string> additional_fields = 8;
  double parsing_confidence = 9;
}

message MatchBomRequest {
  repeated BomLine bom_lines = 1;
  MatchingStrategy strategy = 2;
}

enum MatchingStrategy {
  MATCHING_STRATEGY_UNSPECIFIED = 0;
  MATCHING_STRATEGY_EXACT = 1;        // Part number exact match
  MATCHING_STRATEGY_FUZZY = 2;        // Fuzzy name + part number
  MATCHING_STRATEGY_PARAMETRIC = 3;   // Match by specifications
}

message MatchBomResponse {
  repeated BomMatchResult matches = 1;
  int32 exact_matches = 2;
  int32 likely_matches = 3;
  int32 possible_matches = 4;
  int32 no_matches = 5;
}

message BomMatchResult {
  BomLine bom_line = 1;
  repeated MatchCandidate candidates = 2;
  MatchCandidate best_match = 3;
  MatchStatus status = 4;
}

message MatchCandidate {
  string item_id = 1;
  Item item = 2;
  double confidence = 3;      // 0.0 to 1.0
  int32 quantity_available = 4;
  int32 quantity_required = 5;
  StockStatus stock_status = 6;
  string match_reason = 7;    // Why this was matched
}

enum MatchStatus {
  MATCH_STATUS_UNSPECIFIED = 0;
  MATCH_STATUS_PENDING = 1;
  MATCH_STATUS_ACCEPTED = 2;
  MATCH_STATUS_REJECTED = 3;
  MATCH_STATUS_MANUAL = 4;
}

enum StockStatus {
  STOCK_STATUS_UNSPECIFIED = 0;
  STOCK_STATUS_SUFFICIENT = 1;
  STOCK_STATUS_INSUFFICIENT = 2;
  STOCK_STATUS_OUT_OF_STOCK = 3;
}

message CreateKitFromBomRequest {
  string kit_name = 1;
  string source_note_id = 2;
  repeated BomMatchResult accepted_matches = 3;
  bool reserve_quantities = 4;
  optional string quest_id = 5;  // Link to Guidance quest
}

message ProjectKitResponse {
  ProjectKit kit = 1;
}

message ProjectKit {
  string id = 1;
  string name = 2;
  string description = 3;
  string source_note_id = 4;
  repeated KitItem items = 5;
  KitStatus status = 6;
  optional string quest_id = 7;
  string created_at = 8;
  string updated_at = 9;
}

message KitItem {
  string item_id = 1;
  Item item = 2;
  int32 quantity_required = 3;
  int32 quantity_allocated = 4;
  bool is_available = 5;
}

enum KitStatus {
  KIT_STATUS_UNSPECIFIED = 0;
  KIT_STATUS_DRAFT = 1;
  KIT_STATUS_READY = 2;
  KIT_STATUS_PARTIAL = 3;
  KIT_STATUS_PENDING = 4;
}

message GenerateShoppingListRequest {
  repeated BomMatchResult matches = 1;
  bool group_by_distributor = 2;
}

message ShoppingListResponse {
  repeated ShoppingListItem items = 1;
  map<string, ShoppingListSection> sections_by_distributor = 2;
}

message ShoppingListItem {
  BomLine bom_line = 1;
  int32 quantity_needed = 2;
  optional string part_number = 3;
  optional string distributor = 4;
  optional string purchase_url = 5;
  optional double estimated_price = 6;
}

message ShoppingListSection {
  string distributor_name = 1;
  repeated ShoppingListItem items = 2;
  optional double total_estimated_price = 3;
}

message LinkBomToQuestRequest {
  string bom_id = 1;
  string quest_id = 2;
}

message LinkBomToQuestResponse {
  bool success = 1;
  string message = 2;
}
```

### SurrealDB Schema

```sql
-- schema/bom.surql

-- BoM references (link to Knowledge notes)
DEFINE TABLE bom_references SCHEMAFULL;
DEFINE FIELD id ON bom_references TYPE record<bom_references>;
DEFINE FIELD source_note_id ON bom_references TYPE string ASSERT $value != NONE;
DEFINE FIELD parsed_lines ON bom_references TYPE array<object>;
DEFINE FIELD created_at ON bom_references TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON bom_references TYPE datetime DEFAULT time::now() VALUE time::now();

DEFINE INDEX bom_references_note_idx ON bom_references FIELDS source_note_id UNIQUE;

-- Project kits
DEFINE TABLE project_kits SCHEMAFULL;
DEFINE FIELD id ON project_kits TYPE record<project_kits>;
DEFINE FIELD name ON project_kits TYPE string ASSERT $value != NONE;
DEFINE FIELD description ON project_kits TYPE option<string>;
DEFINE FIELD source_note_id ON project_kits TYPE string;
DEFINE FIELD bom_reference_id ON project_kits TYPE record<bom_references>;
DEFINE FIELD quest_id ON project_kits TYPE option<string>;
DEFINE FIELD status ON project_kits TYPE string;
DEFINE FIELD created_at ON project_kits TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON project_kits TYPE datetime DEFAULT time::now() VALUE time::now();

DEFINE INDEX project_kits_name_idx ON project_kits FIELDS name;
DEFINE INDEX project_kits_quest_idx ON project_kits FIELDS quest_id;

-- Kit items (many-to-many with quantities)
DEFINE TABLE kit_items TYPE RELATION FROM project_kits TO items;
DEFINE FIELD quantity_required ON kit_items TYPE number ASSERT $value > 0;
DEFINE FIELD quantity_allocated ON kit_items TYPE number DEFAULT 0;
DEFINE FIELD is_available ON kit_items TYPE bool;

-- BoM match overrides (user corrections)
DEFINE TABLE bom_match_overrides SCHEMAFULL;
DEFINE FIELD id ON bom_match_overrides TYPE record<bom_match_overrides>;
DEFINE FIELD bom_reference_id ON bom_match_overrides TYPE record<bom_references>;
DEFINE FIELD line_number ON bom_match_overrides TYPE number;
DEFINE FIELD item_id ON bom_match_overrides TYPE record<items>;
DEFINE FIELD match_type ON bom_match_overrides TYPE string;  // "manual", "rejected"
DEFINE FIELD created_at ON bom_match_overrides TYPE datetime DEFAULT time::now();

DEFINE INDEX bom_match_overrides_idx ON bom_match_overrides FIELDS bom_reference_id, line_number UNIQUE;

-- Graph edges
DEFINE TABLE requires TYPE RELATION FROM project_kits TO items;
DEFINE TABLE sources_from TYPE RELATION FROM project_kits TO bom_references;
DEFINE TABLE linked_to_quest TYPE RELATION FROM project_kits TO quests;  // quests table in Guidance
```

## Testing Requirements

### Unit Tests

**BoM Parsing:**
- [ ] CSV parsing with various delimiters (comma, tab, semicolon)
- [ ] Markdown table parsing (standard, GFM)
- [ ] Plain text list parsing (bullets, numbers, dashes)
- [ ] Header detection and field mapping
- [ ] Malformed line handling (skip, log error, continue)
- [ ] Unicode character support

**Matching Algorithms:**
- [ ] Exact part number match (100% confidence)
- [ ] Fuzzy name matching (80% threshold)
- [ ] Case-insensitive matching
- [ ] Manufacturer prefix handling ("NXP" vs "NXP Semiconductors")
- [ ] Quantity availability logic
- [ ] Confidence scoring accuracy

**Distributor Links:**
- [ ] DigiKey URL generation (part number → https://www.digikey.com/products/en?keywords=XXX)
- [ ] Mouser URL generation
- [ ] LCSC URL generation
- [ ] Invalid part number handling (fallback to search page)

### Widget Tests

**Match Review Table:**
- [ ] Renders all BoM lines with confidence badges
- [ ] Accept match updates row state
- [ ] Reject match clears inventory item
- [ ] Manual match picker opens on click
- [ ] Quantity comparison displays correctly
- [ ] Empty state shows "No matches found"

**Shopping List:**
- [ ] Missing items grouped by distributor
- [ ] Export to CSV button works
- [ ] Copy to clipboard functionality
- [ ] Distributor links open in browser

**Kit Creator:**
- [ ] Kit name validation (required)
- [ ] Item selection updates preview
- [ ] Reserve quantities checkbox toggles
- [ ] Create button disabled until valid

### Integration Tests

**End-to-End BoM Flow:**
- [ ] Parse CSV BoM → Match 10 items → Create kit → Verify items allocated
- [ ] Parse Markdown table → Manual match 5 items → Generate shopping list → Export CSV
- [ ] Link kit to Guidance quest → Verify material status in Guidance
- [ ] Re-parse BoM after Knowledge note update → Verify matches refreshed

**Cross-App Integration:**
- [ ] Knowledge note with BoM → Trigger parse from Knowledge → View in Tracking
- [ ] Guidance quest → Link BoM → Check material readiness
- [ ] Tracking inventory update → Refresh kit status in Guidance

**Database Operations:**
- [ ] BoM reference creation and retrieval
- [ ] Kit creation with item allocations
- [ ] Match override persistence
- [ ] Graph relationship traversal (kit → items → locations)

### Accessibility Tests

**Screen Reader:**
- [ ] Match confidence announced clearly
- [ ] Stock status announced (sufficient/insufficient)
- [ ] Table headers properly labeled
- [ ] Action buttons have descriptive labels

**Keyboard-Only:**
- [ ] Tab through match review table
- [ ] Enter accepts match
- [ ] R rejects match
- [ ] Arrow keys navigate rows
- [ ] Space selects for kit

**Color Contrast:**
- [ ] Confidence badges meet WCAG AA
- [ ] Stock status icons + color
- [ ] Text on colored backgrounds compliant

**Performance:**
- [ ] Parse 100-line BoM <2s
- [ ] Match 100 items <5s
- [ ] Render match review table <200ms
- [ ] Shopping list generation <500ms

## Definition of Done

- [ ] CSV, TSV, Markdown, plain text BoM parsing working
- [ ] Exact part number matching implemented (Phase 1)
- [ ] Match review UI with accept/reject/manual match
- [ ] Shopping list generation with distributor links (DigiKey, Mouser, LCSC)
- [ ] Project kit creation from BoM
- [ ] Inventory allocation/reservation system
- [ ] Link kits to Guidance quests (gRPC integration)
- [ ] Material status visible in Guidance quest cards
- [ ] All gRPC endpoints implemented
- [ ] Unit tests passing (>80% coverage for parsers)
- [ ] Widget tests passing (match review, shopping list, kit creator)
- [ ] Integration tests passing (end-to-end BoM flow)
- [ ] Accessibility audit complete
- [ ] Performance metrics met
- [ ] Code review approved
- [ ] Documentation updated (supported BoM formats, matching rules)
- [ ] Dogfooding complete (used for actual project BoM, e.g., Arduino thermostat)
- [ ] Knowledge app integration tested
- [ ] Guidance app integration tested
- [ ] Fuzzy matching plan documented for Phase 2
- [ ] Parametric matching plan documented for Phase 3
