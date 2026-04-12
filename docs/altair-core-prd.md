# Altair Platform -- Core PRD

## Overview

Altair is a personal operating system for managing knowledge, goals, and
resources across everyday life.\
The platform integrates three primary domains:

-   Guidance (goals, initiatives, routines)
-   Knowledge (notes and linked information)
-   Tracking (inventory and resource monitoring)

Altair must operate across multiple device classes so that users can
capture and manage information wherever they are.

## Platform Strategy

Tier 1 Platforms - Android Mobile - Web Application

Tier 2 Platforms - Linux Desktop - Windows Desktop

Tier 3 Platforms - WearOS (future) - iOS (community / best effort)

## System Overview

``` mermaid
flowchart LR
    Mobile --> Sync
    Web --> Sync
    Desktop --> Sync
    Sync --> DataStore
    Sync --> Search
    Sync --> AI
```

## Core Entities

``` mermaid
erDiagram
    USER ||--o{ NOTE : owns
    USER ||--o{ TASK : manages
    USER ||--o{ ITEM : tracks
    NOTE }o--o{ NOTE : links
```

## Core Capabilities

### Identity

Users must be able to authenticate locally or via hosted service.

### Synchronization

Clients must operate offline and synchronize changes when connectivity
returns.

### Attachments

Users may attach images, audio, or documents to entities.

### Search

Global search must operate across knowledge, tasks, and tracked items.

### Tagging

Entities may be tagged and cross-linked.

## Non Functional Requirements

Performance Targets

-   Local actions \<200ms
-   Remote actions \<1s

Reliability

-   Must tolerate intermittent connectivity
-   Data must not be lost during sync conflicts
