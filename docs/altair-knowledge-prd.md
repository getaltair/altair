# Altair -- Knowledge PRD

## Overview

Knowledge provides a persistent personal knowledge base.

## Knowledge Model

``` mermaid
erDiagram
    NOTE ||--o{ NOTE : references
    NOTE }o--o{ TAG : categorized
```

### Note

Primary information unit.

### Tag

Organizational metadata.

## Capture Methods

Users must be able to capture:

-   text notes
-   voice notes
-   images
-   imported documents

## Navigation

``` mermaid
graph TD
    NoteA --> NoteB
    NoteA --> NoteC
    NoteC --> NoteD
```

### Discovery

Users must be able to discover information through:

-   search
-   backlinks
-   relationship graphs
