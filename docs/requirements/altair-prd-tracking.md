# Altair Product Requirements Document

## Tracking: Inventory & Asset Management

| Field               | Value                                                 |
| ------------------- | ----------------------------------------------------- |
| **Version**         | 2.0                                                   |
| **Status**          | Draft                                                 |
| **Last Updated**    | 2026-01-14                                            |
| **Parent Document** | `altair-prd-core.md`                                  |
| **Integrates With** | Guidance (quest linking), Knowledge (note references) |

---

## 1. Purpose

This document defines the product requirements for Tracking, the inventory and asset management application in the Altair ecosystem. Tracking is designed for makers, DIY enthusiasts, and households who need to manage parts, tools, and project materials with minimal friction.

For system-level architecture, design principles, Initiatives, and Universal Inbox, see `altair-prd-core.md`.

---

## 2. Problem Statement

Managing inventory presents specific challenges:

- **Forgotten inventory**: Components purchased months ago are forgotten, leading to duplicate purchases or abandoned projects.
- **Location blindness**: Parts are stored "somewhere" but finding them requires physical search.
- **Project-component disconnect**: Notes about projects don't link to required materials; shopping lists are manual.
- **Capture friction**: Logging new items requires too many fields; items go untracked.
- **No material awareness**: Writing about a project doesn't surface relevant inventory.
- **Maintenance gaps**: Warranty expirations and maintenance schedules slip through the cracks.

Tracking addresses these through:
- **Photo-first capture**: Photograph items instantly, add details later
- **Automatic item detection**: Notes mentioning items create links automatically
- **Bill of Materials intelligence**: Parse BoMs, match against inventory, generate shopping lists
- **Location-aware management**: Hierarchical locations with search and filtering
- **Initiative integration**: Items belong to Initiatives for project context

---

## 3. User Stories

### 3.1 Capture & Organization

> **As a user**, I want to photograph components as I receive them so that I can log inventory without typing part numbers.

> **As a user**, I want to scan barcodes and QR codes so that item details populate automatically.

> **As a user**, I want to track where items are stored so that I can find them later.

### 3.2 Project Integration

> **As a user**, I want the system to detect when my notes mention inventory items so that project documentation links to materials.

> **As a user**, I want to reserve items for specific quests so that I don't accidentally use components allocated to a project.

> **As a user**, I want to generate shopping lists from Bills of Materials so that I know what I need to buy.

### 3.3 Maintenance & Lifecycle

> **As a user**, I want to track warranty expiration dates so that I can make claims before they expire.

> **As a user**, I want maintenance schedule reminders so that I don't forget regular upkeep tasks.

> **As a user**, I want to track tool calibration dates so that my measurements stay accurate.

### 3.4 Shopping & Procurement

> **As a user**, I want to see which items are low in stock so that I can reorder before running out.

> **As a user**, I want to track suppliers and prices so that I can reorder efficiently.

---

## 4. Core Concepts

### 4.1 Item Model

Items are the atomic unit of Tracking:

| Property          | Description                           |
| ----------------- | ------------------------------------- |
| **Name**          | Item name/description                 |
| **Quantity**      | Current count (with unit)             |
| **Location**      | Where stored (hierarchical)           |
| **Category**      | Item category (hierarchical)          |
| **Tags**          | Flexible tagging                      |
| **Photos**        | One or more images                    |
| **Barcode/QR**    | Scanned or generated codes            |
| **Status**        | Available, Reserved, In-use, Depleted |
| **Custom fields** | User-defined properties               |

### 4.2 Location Model

Hierarchical location tracking:

```
Home
├── Garage
│   ├── Workbench
│   │   ├── Drawer 1
│   │   └── Drawer 2
│   └── Shelf A
└── Office
    └── Desk
        └── Component bins
```

### 4.3 Relationship to Initiatives

Items can belong to an **Initiative** (defined in core PRD):

- Items can link to one or more Initiatives
- Initiative Card in Tracking shows related items and quantities
- Filtering by Initiative shows all related items

### 4.4 Relationship to Universal Inbox

**Universal Inbox** (defined in core PRD) can feed Items:

- Inbox items are untyped until triaged
- Triage action "This is an Item" creates Item in Tracking
- Tracking also supports direct item creation (bypassing Inbox)

### 4.5 Reservation System

Items can be reserved for specific Quests:

| Status        | Meaning                                |
| ------------- | -------------------------------------- |
| **Available** | Can be used or reserved                |
| **Reserved**  | Allocated to a Quest (quantity locked) |
| **In-use**    | Actively being used in Quest           |
| **Depleted**  | Quantity reached zero                  |

### 4.6 Bill of Materials (BoM)

Structured list of required items for a project:

```markdown
## Required Materials

- 2x Raspberry Pi 4
- 1x 32GB SD Card
- 4x M3 screws
- 1x Project enclosure
```

Tracking can parse BoMs from Knowledge notes and match against inventory.

---

## 5. Functional Requirements

### 5.1 Item Management

#### Core CRUD

**Requirements:**

- FR-T-001: Create item with photo-first interface
- FR-T-002: Edit item properties
- FR-T-003: Delete item (soft delete, recoverable 30+ days)
- FR-T-004: Duplicate item
- FR-T-005: Bulk edit (tags, location, category)

#### Photo Capture

**Requirements:**

- FR-T-006: Primary photo required (or explicit skip)
- FR-T-007: Multiple photos per item
- FR-T-008: Camera capture or gallery import
- FR-T-009: Photo cropping and rotation
- FR-T-010: Batch photo mode (rapid multi-item capture)
- FR-T-011: Thumbnail generation for lists

#### Barcode/QR Support

**Requirements:**

- FR-T-012: Scan barcode to populate item details (if in database)
- FR-T-013: Scan QR code for custom item lookup
- FR-T-014: Generate QR code for any item
- FR-T-015: Print QR labels (export to PDF)
- FR-T-016: Bulk QR code generation

#### Quantity Management

**Requirements:**

- FR-T-017: Track quantity with unit (e.g., "50 pcs", "2.5 m")
- FR-T-018: Increment/decrement quantity with single tap
- FR-T-019: Set minimum stock threshold
- FR-T-020: Low stock alert when below threshold
- FR-T-021: Quantity history log
- FR-T-022: Support for consumable vs. reusable items

#### Location Tracking

**Requirements:**

- FR-T-023: Assign item to location (hierarchical)
- FR-T-024: Create/edit location hierarchy
- FR-T-025: Move item between locations
- FR-T-026: View all items in a location
- FR-T-027: GPS tagging for mobile locations (optional)
- FR-T-028: Indoor location notes (e.g., "top shelf, left side")

#### Categories & Tags

**Requirements:**

- FR-T-029: Assign item to category (hierarchical)
- FR-T-030: Multiple tags per item
- FR-T-031: Shared tag taxonomy with Knowledge/Guidance
- FR-T-032: Filter by category or tag
- FR-T-033: Category/tag management UI

#### Custom Fields

**Requirements:**

- FR-T-034: Define custom fields per category
- FR-T-035: Field types: text, number, date, URL, dropdown
- FR-T-036: Custom fields appear in item detail view
- FR-T-037: Search includes custom field values

#### Initiative Linking

**Requirements:**

- FR-T-038: Link item to Initiative (optional)
- FR-T-039: View all items for an Initiative
- FR-T-040: Initiative Card shows item count and status

---

### 5.2 Inventory Intelligence

#### Auto-Discovery in Notes

When writing in Knowledge, Tracking detects mentioned items:

**Requirements:**

- FR-T-041: Real-time text analysis for item mentions
- FR-T-042: Pattern matching against inventory names and aliases
- FR-T-043: Non-intrusive popup showing detected items
- FR-T-044: One-click to link item to note
- FR-T-045: Dismiss false positives (with learning)
- FR-T-046: Detection works for partial matches ("Pi 4" → "Raspberry Pi 4")

#### Smart Suggestions

**Requirements:**

- FR-T-047: Suggest related items based on project context
- FR-T-048: "Frequently used together" recommendations
- FR-T-049: Missing component alerts (BoM vs. inventory)
- FR-T-050: Reorder suggestions based on usage patterns

---

### 5.3 Reservation System

#### Reserving Items

**Requirements:**

- FR-T-051: Reserve item quantity for specific Quest
- FR-T-052: Reserved quantity deducted from available
- FR-T-053: Visual indicator of reserved items
- FR-T-054: View all reservations for an item
- FR-T-055: View all items reserved for a Quest

#### Lifecycle

**Requirements:**

- FR-T-056: Mark reserved items as "in-use" when Quest starts
- FR-T-057: Auto-release reservation on Quest completion
- FR-T-058: Manual release of reservation
- FR-T-059: Conflict warning when reserving more than available

---

### 5.4 Bill of Materials (BoM)

#### Parsing

**Requirements:**

- FR-T-060: Detect structured BoM in Knowledge notes
- FR-T-061: Parse quantity and item name
- FR-T-062: Handle common formats (lists, tables)
- FR-T-063: Fuzzy match to existing inventory
- FR-T-064: Unit conversion (e.g., "1 meter" vs. "100 cm")

#### Features

**Requirements:**

- FR-T-065: View BoM → Inventory match status
- FR-T-066: Generate shopping list from unmatched/insufficient items
- FR-T-067: Link BoM to Quest (reserve materials)
- FR-T-068: Track material usage per project
- FR-T-069: Cost estimation from stored prices
- FR-T-070: Supplier links for reordering

#### Shopping Lists

**Requirements:**

- FR-T-071: Interactive shopping list (check off as purchased)
- FR-T-072: Aggregate lists across multiple BoMs
- FR-T-073: Group by supplier or store
- FR-T-074: Add to inventory when purchased
- FR-T-075: Price tracking on purchase
- FR-T-076: Share list (export to text, email)

---

### 5.5 Maintenance Tracking

#### Warranty Management

**Requirements:**

- FR-T-077: Track purchase date and warranty period
- FR-T-078: Calculate warranty expiration date
- FR-T-079: Warranty expiration alerts (configurable lead time)
- FR-T-080: Store warranty documentation (photos, PDFs)
- FR-T-081: Mark warranty as used/claimed

#### Maintenance Schedules

**Requirements:**

- FR-T-082: Define recurring maintenance tasks per item
- FR-T-083: Frequency options: days, weeks, months, usage-based
- FR-T-084: Maintenance due reminders (push notifications)
- FR-T-085: Log maintenance performed
- FR-T-086: Maintenance history view
- FR-T-087: Generate maintenance Quest in Guidance

#### Service History

**Requirements:**

- FR-T-088: Log repairs and service events
- FR-T-089: Track service provider and cost
- FR-T-090: Attach service documentation
- FR-T-091: Total cost of ownership calculation

---

### 5.6 Search & Filtering

**Requirements:**

- FR-T-092: Full-text search across all item properties
- FR-T-093: Filter by category, tag, location, Initiative
- FR-T-094: Filter by status (available, reserved, low stock)
- FR-T-095: Filter by custom field values
- FR-T-096: Sort by name, quantity, date added, last used
- FR-T-097: Save filter presets
- FR-T-098: Cross-app search integration

---

### 5.7 Views

#### List View

**Requirements:**

- FR-T-099: Thumbnail, name, quantity, location in list
- FR-T-100: Quick actions (increment, decrement, reserve)
- FR-T-101: Swipe actions for common operations (mobile)
- FR-T-102: Infinite scroll with performance optimization

#### Grid View

**Requirements:**

- FR-T-103: Photo-centric grid layout
- FR-T-104: Configurable grid density
- FR-T-105: Quick access to item details

#### Location View

**Requirements:**

- FR-T-106: Hierarchical location tree
- FR-T-107: Expand location to see contained items
- FR-T-108: Drag items between locations

#### Category View

**Requirements:**

- FR-T-109: Hierarchical category tree
- FR-T-110: Expand category to see items
- FR-T-111: Category item counts

---

## 6. Mobile Features

Mobile is the **primary platform** for Tracking capture. Store visits, receiving packages, and workshop organization all happen with phone in hand.

### 6.1 Full Feature Parity

All features available on mobile:

- Complete inventory management
- Photo-first capture with camera integration
- Barcode/QR scanning
- Location and GPS tagging
- BoM processing
- Maintenance tracking
- Shopping lists

### 6.2 Mobile Capture Advantages

Mobile is the _preferred_ platform for Tracking capture:

**Requirements:**

- FR-T-112: Native camera integration (no switching apps)
- FR-T-113: Barcode scanning via camera
- FR-T-114: Batch photo mode for rapid capture
- FR-T-115: GPS location auto-tagging (optional)
- FR-T-116: Voice notes for item descriptions

### 6.3 Touch Optimizations

**Requirements:**

- FR-T-117: Swipe to change quantity (+/-)
- FR-T-118: Swipe to mark as reserved/available
- FR-T-119: Long-press for context menu
- FR-T-120: Pull-to-refresh for sync
- FR-T-121: Haptic feedback on actions

### 6.4 Notifications

**Requirements:**

- FR-T-122: Low stock alerts
- FR-T-123: Warranty expiration reminders
- FR-T-124: Maintenance due notifications
- FR-T-125: Reservation conflict alerts
- FR-T-126: Notification preferences per type

### 6.5 Widgets

**Requirements:**

- FR-T-127: Quick capture widget (camera to inventory)
- FR-T-128: Low stock summary widget
- FR-T-129: Recent items widget
- FR-T-130: Barcode scan widget

### 6.6 Advanced Mobile Features

**Requirements:**

- FR-T-131: NFC tag reading/writing for item identification
- FR-T-132: Location-based reminders ("When I get home, check stock of X")

---

## 7. Non-Functional Requirements

### 7.1 Performance

| Metric                      | Target      |
| --------------------------- | ----------- |
| Item creation               | < 500ms     |
| Photo capture to saved      | < 2 seconds |
| Barcode scan to lookup      | < 1 second  |
| Search (10k items)          | < 1 second  |
| List render (100 items)     | < 500ms     |
| Inventory detection in text | < 500ms     |

### 7.2 Data Integrity

- Item changes logged with timestamp
- Quantity changes auditable
- Photos backed up with items
- No orphaned photos on item deletion
- Sync conflicts surfaced to user

### 7.3 Storage Efficiency

- Photo compression to balance quality and size
- Thumbnail generation for fast list rendering
- Configurable photo resolution
- Total storage estimate visible
- Option to reduce photo quality retroactively

### 7.4 Accessibility

- Full keyboard navigation (desktop)
- Screen reader support
- High contrast mode
- Dynamic type support (mobile)
- Minimum touch targets for quantity buttons

---

## 8. Integration Points

### 8.1 Universal Inbox Integration

| From Universal Inbox | To Tracking                     |
| -------------------- | ------------------------------- |
| Triage "This is an Item" | Creates Item in Tracking    |
| Link to Initiative   | Item inherits Initiative context |

### 8.2 Initiative Integration

| From Tracking | To Initiative                    |
| ------------- | -------------------------------- |
| Item          | Can belong to one or more Initiatives |
| Item count    | Shown in Initiative Card         |
| Low stock     | Surfaced in Initiative context   |

| From Initiative | To Tracking                      |
| --------------- | -------------------------------- |
| Initiative Card | Shows item count and status      |
| Focus setting   | Filters items to Initiative      |

### 8.3 Guidance Integration

| From Tracking   | To Guidance                |
| --------------- | -------------------------- |
| Low stock alert | Generate restock Quest     |
| Maintenance due | Generate maintenance Quest |
| Reserved items  | Link to Quest requirements |

| From Guidance             | To Tracking            |
| ------------------------- | ---------------------- |
| Quest requiring materials | Reserve items          |
| Quest complete            | Release reserved items |
| Shopping Quest complete   | Add items to inventory |

### 8.4 Knowledge Integration

| From Tracking    | To Knowledge                |
| ---------------- | --------------------------- |
| Item             | Link to documentation notes |
| Item with manual | Manual as attached note     |

| From Knowledge        | To Tracking               |
| --------------------- | ------------------------- |
| Note mentioning items | Auto-detect and link      |
| BoM in note           | Parse and match inventory |

### 8.5 Universal Features

- Universal search includes items
- Shared tag taxonomy
- Items visible in cross-app graph (via linked notes/quests)

---

## 9. Feature Priority

### Critical (Must Have)

1. Item CRUD with photo capture
2. Location tracking (hierarchical)
3. Quantity management
4. Category and tag organization
5. Barcode/QR scanning
6. QR code generation (enables location labels and quick lookup)
7. Basic search and filtering
8. Item ↔ Note linking (core Altair differentiator)
9. Mobile: Camera integration
10. Mobile: Offline mode

### High Priority

1. Auto-discovery in notes
2. BoM parsing and matching
3. Shopping list generation
4. Reservation system
5. Low stock alerts
6. Item ↔ Initiative linking
7. Mobile: Batch photo mode
8. Mobile: Widgets

### Medium Priority

1. Maintenance tracking
2. Warranty management
3. Supplier and price tracking
4. NFC tag support
5. Custom fields
6. Advanced filtering presets

---

## 10. Resolved Design Decisions

| Question | Resolution |
|----------|------------|
| Barcode database | Open Food Facts + UPC Database for consumer goods; custom lookup for components |
| Photo storage | Per-user storage quota (configurable by admin); compression on upload |
| BoM parsing accuracy | Best-effort with user confirmation; ambiguous cases flagged for review |
| Indoor location | v1: text-based location notes; NFC tags for quick lookup; beacon integration deferred |

---

## Appendix A: Sample BoM Formats

The parser should handle these common formats:

**Markdown List:**

```markdown
- 2x Raspberry Pi 4
- 1x 32GB SD Card
- 4x M3 screws
```

**Markdown Table:**

```markdown
| Qty | Item           | Notes        |
| --- | -------------- | ------------ |
| 2   | Raspberry Pi 4 | Model B      |
| 1   | SD Card        | 32GB minimum |
| 4   | M3 screws      | 10mm length  |
```

**Plain Text:**

```
2 × Raspberry Pi 4
1 × 32GB SD Card
4 × M3 screws
```

**Parenthetical:**

```
Raspberry Pi 4 (x2)
SD Card (1)
M3 screws (4 pcs)
```

**CSV:**

```csv
qty,item,notes
2,Raspberry Pi 4,Model B
1,SD Card,32GB minimum
4,M3 screws,10mm length
```

**JSON:**

```json
{
  "items": [
    { "qty": 2, "item": "Raspberry Pi 4", "notes": "Model B" },
    { "qty": 1, "item": "SD Card", "notes": "32GB minimum" },
    { "qty": 4, "item": "M3 screws", "notes": "10mm length" }
  ]
}
```

**XML (supported but not recommended):**

```xml
<bom>
  <item qty="2" notes="Model B">Raspberry Pi 4</item>
  <item qty="1" notes="32GB minimum">SD Card</item>
  <item qty="4" notes="10mm length">M3 screws</item>
</bom>
```

---

## Appendix B: Keyboard-Accessible Actions

The following actions should be accessible via configurable keyboard shortcuts on desktop:

| Action               | Description                   |
| -------------------- | ----------------------------- |
| New item             | Create new item               |
| Search               | Open search                   |
| Filter panel         | Open filter options           |
| Toggle view          | Switch between list/grid view |
| Scan barcode         | Activate barcode scanner      |
| Generate QR          | Generate QR for selected item |
| Increment quantity   | Add one to quantity           |
| Decrement quantity   | Subtract one from quantity    |
| Reserve item         | Reserve for Quest             |
| Link to Initiative   | Add Initiative link to item   |
