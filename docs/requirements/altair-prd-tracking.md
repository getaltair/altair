# Altair Product Requirements Document

## Tracking: Inventory & Asset Management

| Field               | Value                                                 |
| ------------------- | ----------------------------------------------------- |
| **Version**         | 1.0                                                   |
| **Status**          | Draft                                                 |
| **Last Updated**    | 2026-01-08                                            |
| **Parent Document** | `altair-prd-core.md`                                  |
| **Dependencies**    | Guidance (quest linking), Knowledge (note references) |

---

## 1. Purpose

This document defines the product requirements for Tracking, the inventory and asset management
application in the Altair ecosystem. Tracking is designed for makers, DIY enthusiasts, and
households who need to manage parts, tools, and project materials with minimal friction.

For system-level architecture, design principles, and cross-app integration, see
`altair-prd-core.md`.

---

## 2. Problem Statement

Makers and DIY enthusiasts face specific inventory challenges:

- **Forgotten inventory**: Components purchased months ago are forgotten, leading to duplicate
  purchases or abandoned projects.
- **Location blindness**: Parts are stored "somewhere" but finding them requires physical search.
- **Project-component disconnect**: Notes about projects don't link to required materials; shopping
  lists are manual.
- **Capture friction**: Logging new items requires too many fields; items go untracked.
- **No material awareness**: Writing about a project doesn't surface relevant inventory.
- **Maintenance gaps**: Warranty expirations and maintenance schedules slip through the cracks.

Tracking addresses these through photo-first capture, automatic item detection in notes, Bill of
Materials intelligence, and location-aware inventory management.

---

## 3. User Stories

### 3.1 Capture & Organization

> **As a maker**, I want to photograph components as I receive them so that I can log inventory
> without typing part numbers.

> **As a maker**, I want to scan barcodes and QR codes so that item details populate automatically.

> **As a maker**, I want to track where items are stored so that I can find them later.

### 3.2 Project Integration

> **As a maker**, I want the system to detect when my notes mention inventory items so that project
> documentation links to materials.

> **As a maker**, I want to reserve items for specific quests so that I don't accidentally use
> components allocated to a project.

> **As a maker**, I want to generate shopping lists from Bills of Materials so that I know what I
> need to buy.

### 3.3 Maintenance & Lifecycle

> **As a homeowner**, I want to track warranty expiration dates so that I can make claims before
> they expire.

> **As a homeowner**, I want maintenance schedule reminders so that I don't forget regular upkeep
> tasks.

> **As a maker**, I want to track tool calibration dates so that my measurements stay accurate.

### 3.4 Shopping & Procurement

> **As a maker**, I want to see which items are low in stock so that I can reorder before running
> out.

> **As a maker**, I want to track suppliers and prices so that I can reorder efficiently.

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

### 4.3 Reservation System

Items can be reserved for specific Quests:

| Status        | Meaning                                |
| ------------- | -------------------------------------- |
| **Available** | Can be used or reserved                |
| **Reserved**  | Allocated to a Quest (quantity locked) |
| **In-use**    | Actively being used in Quest           |
| **Depleted**  | Quantity reached zero                  |

### 4.4 Bill of Materials (BoM)

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

---

### 5.2 Inventory Intelligence

#### Auto-Discovery in Notes

When writing in Knowledge, Tracking detects mentioned items:

**Requirements:**

- FR-T-038: Real-time text analysis for item mentions
- FR-T-039: Pattern matching against inventory names and aliases
- FR-T-040: Non-intrusive popup showing detected items
- FR-T-041: One-click to link item to note
- FR-T-042: Dismiss false positives (with learning)
- FR-T-043: Detection works for partial matches ("Pi 4" → "Raspberry Pi 4")

#### Smart Suggestions

**Requirements:**

- FR-T-044: Suggest related items based on project context
- FR-T-045: "Frequently used together" recommendations
- FR-T-046: Missing component alerts (BoM vs. inventory)
- FR-T-047: Reorder suggestions based on usage patterns

---

### 5.3 Reservation System

#### Reserving Items

**Requirements:**

- FR-T-048: Reserve item quantity for specific Quest
- FR-T-049: Reserved quantity deducted from available
- FR-T-050: Visual indicator of reserved items
- FR-T-051: View all reservations for an item
- FR-T-052: View all items reserved for a Quest

#### Lifecycle

**Requirements:**

- FR-T-053: Mark reserved items as "in-use" when Quest starts
- FR-T-054: Auto-release reservation on Quest completion
- FR-T-055: Manual release of reservation
- FR-T-056: Conflict warning when reserving more than available

---

### 5.4 Bill of Materials (BoM)

#### Parsing

**Requirements:**

- FR-T-057: Detect structured BoM in Knowledge notes
- FR-T-058: Parse quantity and item name
- FR-T-059: Handle common formats (lists, tables)
- FR-T-060: Fuzzy match to existing inventory
- FR-T-061: Unit conversion (e.g., "1 meter" vs. "100 cm")

#### Features

**Requirements:**

- FR-T-062: View BoM → Inventory match status
- FR-T-063: Generate shopping list from unmatched/insufficient items
- FR-T-064: Link BoM to Quest (reserve materials)
- FR-T-065: Track material usage per project
- FR-T-066: Cost estimation from stored prices
- FR-T-067: Supplier links for reordering

#### Shopping Lists

**Requirements:**

- FR-T-068: Interactive shopping list (check off as purchased)
- FR-T-069: Aggregate lists across multiple BoMs
- FR-T-070: Group by supplier or store
- FR-T-071: Add to inventory when purchased
- FR-T-072: Price tracking on purchase
- FR-T-073: Share list (export to text, email)

---

### 5.5 Maintenance Tracking

#### Warranty Management

**Requirements:**

- FR-T-074: Track purchase date and warranty period
- FR-T-075: Calculate warranty expiration date
- FR-T-076: Warranty expiration alerts (configurable lead time)
- FR-T-077: Store warranty documentation (photos, PDFs)
- FR-T-078: Mark warranty as used/claimed

#### Maintenance Schedules

**Requirements:**

- FR-T-079: Define recurring maintenance tasks per item
- FR-T-080: Frequency options: days, weeks, months, usage-based
- FR-T-081: Maintenance due reminders (push notifications)
- FR-T-082: Log maintenance performed
- FR-T-083: Maintenance history view
- FR-T-084: Generate maintenance Quest in Guidance

#### Service History

**Requirements:**

- FR-T-085: Log repairs and service events
- FR-T-086: Track service provider and cost
- FR-T-087: Attach service documentation
- FR-T-088: Total cost of ownership calculation

---

### 5.6 Search & Filtering

**Requirements:**

- FR-T-089: Full-text search across all item properties
- FR-T-090: Filter by category, tag, location
- FR-T-091: Filter by status (available, reserved, low stock)
- FR-T-092: Filter by custom field values
- FR-T-093: Sort by name, quantity, date added, last used
- FR-T-094: Save filter presets
- FR-T-095: Cross-app search integration

---

### 5.7 Views

#### List View

**Requirements:**

- FR-T-096: Thumbnail, name, quantity, location in list
- FR-T-097: Quick actions (increment, decrement, reserve)
- FR-T-098: Swipe actions for common operations (mobile)
- FR-T-099: Infinite scroll with performance optimization

#### Grid View

**Requirements:**

- FR-T-100: Photo-centric grid layout
- FR-T-101: Configurable grid density
- FR-T-102: Quick access to item details

#### Location View

**Requirements:**

- FR-T-103: Hierarchical location tree
- FR-T-104: Expand location to see contained items
- FR-T-105: Drag items between locations

#### Category View

**Requirements:**

- FR-T-106: Hierarchical category tree
- FR-T-107: Expand category to see items
- FR-T-108: Category item counts

---

## 6. Mobile-Specific Features

### 6.1 Full Feature Parity

All desktop features available on mobile with touch optimization:

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

- FR-T-109: Native camera integration (no switching apps)
- FR-T-110: Barcode scanning via camera
- FR-T-111: Batch photo mode for rapid capture
- FR-T-112: GPS location auto-tagging (optional)
- FR-T-113: Voice notes for item descriptions

### 6.3 Touch Optimizations

**Requirements:**

- FR-T-114: Swipe to change quantity (+/-)
- FR-T-115: Swipe to mark as reserved/available
- FR-T-116: Long-press for context menu
- FR-T-117: Pull-to-refresh for sync
- FR-T-118: Haptic feedback on actions

### 6.4 Notifications

**Requirements:**

- FR-T-119: Low stock alerts
- FR-T-120: Warranty expiration reminders
- FR-T-121: Maintenance due notifications
- FR-T-122: Reservation conflict alerts
- FR-T-123: Notification preferences per type

### 6.5 Widgets

**Requirements:**

- FR-T-124: Quick capture widget (camera to inventory)
- FR-T-125: Low stock summary widget
- FR-T-126: Recent items widget
- FR-T-127: Barcode scan widget

### 6.6 Advanced Mobile Features

**Requirements:**

- FR-T-128: NFC tag reading/writing for item identification
- FR-T-129: Bluetooth connection to smart labels (future)
- FR-T-130: Location-based reminders ("When I get home, check stock of X")
- FR-T-131: AR item visualization (future consideration)

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

### 8.1 Guidance Integration

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

### 8.2 Knowledge Integration

| From Tracking    | To Knowledge                |
| ---------------- | --------------------------- |
| Item             | Link to documentation/notes |
| Item with manual | Manual as attached note     |

| From Knowledge        | To Tracking               |
| --------------------- | ------------------------- |
| Note mentioning items | Auto-detect and link      |
| BoM in note           | Parse and match inventory |

### 8.3 Universal Features

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
6. Basic search and filtering
7. Mobile: Camera integration
8. Mobile: Offline mode

### High Priority

1. Auto-discovery in notes
2. BoM parsing and matching
3. Shopping list generation
4. Reservation system
5. Low stock alerts
6. QR code generation
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

## 10. Open Questions

1. **Barcode database**: Which product database(s) to integrate for barcode lookup?
2. **Photo storage limits**: Should there be a per-item photo limit, or total storage quota?
3. **BoM parsing accuracy**: How to handle ambiguous quantity/unit combinations?
4. **Indoor location**: Any beacon/NFC-based indoor positioning worth integrating?

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

---

## Appendix B: Keyboard Shortcuts (Desktop)

| Action                  | Shortcut     |
| ----------------------- | ------------ |
| New item                | Cmd/Ctrl + N |
| Search                  | Cmd/Ctrl + / |
| Filter panel            | Cmd/Ctrl + F |
| Toggle view (list/grid) | Cmd/Ctrl + L |
| Scan barcode            | Cmd/Ctrl + B |
| Generate QR             | Cmd/Ctrl + Q |
| Increment quantity      | + or =       |
| Decrement quantity      | -            |
| Reserve item            | Cmd/Ctrl + R |
