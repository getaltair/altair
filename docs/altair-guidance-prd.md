# Altair -- Guidance PRD

## Overview

Guidance helps users translate long-term goals into daily actions
through structured planning.

## Core Concepts

``` mermaid
classDiagram
    Goal --> Initiative
    Initiative --> Task
    Task --> Routine
```

### Goal

Long-term desired outcome.

### Initiative

Structured effort toward a goal.

### Task

Discrete actionable unit.

### Routine

Recurring habit or behavior.

## Daily Workflow

``` mermaid
sequenceDiagram
    participant User
    participant Mobile
    participant Sync
    participant Server

    User->>Mobile: Complete task
    Mobile->>Sync: Queue mutation
    Sync->>Server: Push change
    Server-->>Sync: Ack
```

## Capabilities

### Planning

Users must be able to create goals and break them into initiatives and
tasks.

### Daily Guidance

The system must present today's tasks and routines.

### Reflection

Users must be able to review completed work and historical progress.
