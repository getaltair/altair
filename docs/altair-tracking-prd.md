# Altair -- Tracking PRD

## Overview

Tracking enables monitoring of personal resources and inventory.

Examples:

-   household inventory
-   consumables
-   equipment

## Data Model

``` mermaid
erDiagram
    ITEM ||--o{ LOCATION : stored_in
    ITEM ||--o{ EVENT : usage
```

### Item

Physical or digital resource.

### Location

Storage location.

### Event

Consumption or change record.

## Item Creation

Items may be created using:

-   manual entry
-   barcode scanning
-   image capture

## Usage Tracking

``` mermaid
sequenceDiagram
    participant User
    participant Mobile
    participant Sync
    participant Server

    User->>Mobile: Consume item
    Mobile->>Sync: Record event
    Sync->>Server: Update inventory
```

## Inventory Capabilities

Users must be able to:

-   track quantities
-   track locations
-   record consumption
