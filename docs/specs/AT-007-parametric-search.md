# Feature AT-007: Parametric Search

## What it does

Enables users to search and filter inventory items using custom fields and component parameters (e.g., "10kΩ resistors, 0603 package, ±1% tolerance") with real-time results and visual highlighting of matching criteria.

## User Journey

GIVEN a user has an inventory with components that have custom fields (resistance, package size, tolerance, voltage rating, etc.)
WHEN they open the parametric search interface and add filter criteria (e.g., "capacitance > 10µF AND voltage_rating >= 25V AND package = 0805")
THEN the system displays matching items in real-time, highlights which criteria each item satisfies, and allows saving search templates for reuse

## Functional Requirements

### Core Parametric Search
- Support for custom field filtering with operators: =, !=, <, >, <=, >=, CONTAINS, IN, BETWEEN
- Numeric range filtering with unit conversion (e.g., 10kΩ = 10000Ω, 0.01µF = 10nF)
- Multi-criteria AND/OR logic builder
- Real-time search as filters are applied
- Visual indication of which criteria each item matches
- Search result ranking by relevance
- Export search results to CSV

### Search Templates
- Save frequently used search queries with names
- Quick access to saved searches
- Share search templates between devices
- Template categories (resistors, capacitors, ICs, etc.)

### Integration Features
- Filter by availability status (in stock, low stock, out of stock)
- Filter by location/container
- Filter by project association
- Filter items needed for active quests (Guidance integration)

### Business Rules
- Unit conversions must be accurate and bidirectional
- Search must complete within 500ms for up to 10,000 items
- Saved searches limited to 50 per user (ADHD: reduce decision fatigue)
- Search history retained for 30 days

## UI/UX Requirements

### Components

**Existing Design System:**
- `SearchBar`, `FilterChip`, `DataTable`, `FAB`, `BottomSheet`, `Card`

**Custom Components:**
- `ParametricFilterBuilder` - Visual query builder with drag-and-drop
- `UnitConverter` - Inline unit conversion widget
- `MatchHighlighter` - Shows which criteria matched for each item
- `SearchTemplateCard` - Saved search quick access

### Visual Design

**Layout:** Search bar (40%) + Quick filters + Save button | Left sidebar (25%): Active filters | Main area (75%): Results table | Right panel: Templates

**Colors:** Primary: `--color-primary`, Active filters: `--color-success-light`, Match highlights: `--color-warning-light` (partial) / `--color-success` (exact)

**Typography:** Search: 18px medium, Criteria: 16px monospace, Results: 14px, Matches: 12px bold

**Borders/Shadows:** Filter chips: 3px solid black, Active: 4px primary + 2px shadow, Table: 2px solid black + zebra striping

### State Management

**Global State (Riverpod):**
```dart
@riverpod
class ParametricSearchController extends _$ParametricSearchController {
  Future<void> addFilter(ParametricFilter filter) async { }
  Future<void> executeSearch() async { }
}

@riverpod
class SearchTemplateRepository extends _$SearchTemplateRepository {
  Future<void> saveTemplate(SearchTemplate template) async { }
}

@riverpod
class UnitConverter extends _$UnitConverter {
  double convert(double value, String fromUnit, String toUnit) { }
}
```

### ADHD-Specific UI Requirements

**Cognitive Load:** Progressive disclosure, max 5 visible filters, collapsible groups, templates reduce memory burden

**Focus Management:** Autofocus search bar, clear active indicator, disable distracting animations

**Forgiveness:** Undo filter deletion (Ctrl+Z), "Reset" always visible, auto-save state, confirm before clearing >3 filters

**Visual Hierarchy:** Primary action largest/brightest, results count in large bold text

**Immediate Feedback:** <100ms filter application, progressive results loading, <200ms animations

## Non-Functional Requirements

**Performance:** Filter <100ms, Search <500ms for 10K items, Template load <100ms, Export <2s for 1K results

**Technical:** Flutter 3.16+, packages: riverpod, drift, units_converter, csv, fuzzywuzzy

**Security:** Validate inputs (prevent SurrealQL injection), sanitize queries, rate limit: 100 req/min

## Implementation Details

### Code Structure

```
lib/features/parametric_search/
├── presentation/
│   ├── widgets/ (parametric_filter_builder.dart, unit_converter_widget.dart, etc.)
│   ├── providers/ (parametric_search_controller.dart, search_template_repository.dart)
│   └── screens/ (parametric_search_screen.dart)
├── domain/
│   ├── models/ (parametric_filter.dart, search_template.dart)
│   └── repositories/ (search_repository.dart)
└── data/
    ├── repositories/ (search_repository_impl.dart)
    └── datasources/ (local_search_cache.dart, remote_search_service.dart)
```

### Rust Backend

```rust
// src/services/parametric_search_service.rs
pub struct ParametricSearchService {
    db: Arc<Surreal<Db>>,
}

impl ParametricSearchService {
    pub async fn execute_search(&self, request: SearchRequest) -> Result<Vec<InventoryItem>> {
        let query = self.build_query(&request)?;
        let results: Vec<InventoryItem> = self.db.query(query).await?.take(0)?;
        Ok(results)
    }
    
    fn build_query(&self, request: &SearchRequest) -> Result<String> {
        // Dynamic SurrealDB query generation based on filters
    }
}
```

### gRPC Proto

```protobuf
service ParametricSearchService {
  rpc ExecuteSearch(SearchRequest) returns (SearchResponse);
  rpc SaveTemplate(SaveTemplateRequest) returns (SaveTemplateResponse);
  rpc ConvertUnits(UnitConversionRequest) returns (UnitConversionResponse);
}
```

### SurrealDB Schema

```sql
DEFINE TABLE search_templates SCHEMAFULL;
DEFINE FIELD filters ON TABLE search_templates TYPE array;
DEFINE FIELD logic ON TABLE search_templates TYPE string;
DEFINE INDEX search_templates_user_idx ON TABLE search_templates COLUMNS user_id;
DEFINE INDEX inventory_items_custom_fields_idx ON TABLE inventory_items COLUMNS custom_fields;
```

## Testing Requirements

- [ ] Unit: Filter operators, unit conversions, query builder, template serialization
- [ ] Widget: Filter builder, unit converter, match highlighter, template cards
- [ ] Integration: Multi-filter search, template save/load, CSV export, cross-app filtering
- [ ] Performance: <500ms search, <100ms filter, <100ms template load
- [ ] Accessibility: Screen reader, keyboard nav, WCAG 2.1 AA contrast

## Dependencies

**Depends on:** AT-001 (Item CRUD), AT-003 (Custom Fields), SHARED-001 (SurrealDB), SHARED-002 (gRPC)

**Blocks:** AT-006 (BoM Intelligence), AT-010 (Low Stock Alerts)
