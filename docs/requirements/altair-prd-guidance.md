# Altair Product Requirements Document
## Guidance: Quest-Based Agile Task Management

| Field | Value |
|-------|-------|
| **Version** | 1.0 |
| **Status** | Draft |
| **Last Updated** | 2026-01-08 |
| **Parent Document** | `altair-prd-core.md` |
| **Dependencies** | Knowledge (note linking), Tracking (item requirements) |

---

## 1. Purpose

This document defines the product requirements for Guidance, the task management application in the Altair ecosystem. Guidance implements Quest-Based Agile (QBA), a methodology designed specifically for ADHD productivity patterns.

For system-level architecture, design principles, and cross-app integration, see `altair-prd-core.md`.

---

## 2. Problem Statement

Traditional task management fails ADHD users in specific ways:

- **Infinite backlogs**: Tools encourage capturing everything, creating overwhelming lists that trigger avoidance.
- **Context switching**: No enforcement of single-tasking; users can start infinite parallel work streams.
- **Willpower-dependent prioritization**: Users must constantly decide "what's next" when executive function is already impaired.
- **Shame accumulation**: Incomplete tasks remain visible, reinforcing negative self-perception.
- **Time blindness**: No support for estimating, tracking, or visualizing time spent on tasks.
- **Binary outcomes**: Tasks are either done or not-done; no support for pausing, pivoting, or recovering from abandonment.

Guidance addresses these through enforced WIP limits, energy-based filtering, swap-first behavior, and shame-free recovery mechanisms.

---

## 3. User Stories

### 3.1 Daily Workflow

> **As an ADHD user**, I want to start my day with a simple check-in so that I can match my work to my current capacity without decision fatigue.

> **As an ADHD user**, I want the system to enforce a single active task so that I cannot scatter my attention across multiple incomplete items.

> **As an ADHD user**, I want to swap tasks without guilt so that changing focus doesn't feel like failure.

### 3.2 Focus & Flow

> **As an ADHD user**, I want a distraction-free focus mode so that I can enter and maintain flow state on my current task.

> **As an ADHD user**, I want visible timers and progress so that I can combat time blindness and see how long I've been working.

> **As an ADHD user**, I want to extend focus sessions when I'm in the zone so that artificial time boxes don't break my flow.

### 3.3 Planning & Recovery

> **As an ADHD user**, I want a weekly ritual that celebrates wins first so that I build positive associations with review and planning.

> **As an ADHD user**, I want to recover from an abandoned board without shame so that I can restart fresh when I've been away too long.

> **As an ADHD user**, I want emergency mode when I'm overwhelmed so that I can simplify my view to just a few essential tasks.

### 3.4 Motivation

> **As an ADHD user**, I want optional gamification so that I can get dopamine hits from progress without being forced into point optimization.

> **As an ADHD user**, I want to see my completed work as a trophy room so that I have evidence of progress on hard days.

---

## 4. Core Concepts

### 4.1 Quest-Based Agile (QBA)

QBA adapts agile methodology for individual ADHD users:

| Agile Concept | QBA Adaptation |
|---------------|----------------|
| Sprint | **Cycle** — Weekly timeboxed period |
| Epic | **Epic** — Large initiative with multiple Quests |
| Story | **Quest** — Discrete, completable task |
| Sprint backlog | **Next Up** — Limited queue of ready Quests |
| WIP limits | **WIP=1** — One Quest in progress at a time |
| Retrospective | **Harvest** — Weekly celebration and planning |

### 4.2 Energy System

Daily subjective capacity rating (not medical/health tracking):

| Level | Symbol | Meaning |
|-------|--------|---------|
| 1 - Tiny | ⚡ | Minimal tasks only |
| 2 - Small | ⚡⚡ | Light work |
| 3 - Medium | ⚡⚡⚡ | Standard tasks |
| 4 - Large | ⚡⚡⚡⚡ | Complex work |
| 5 - Huge | ⚡⚡⚡⚡⚡ | Major projects |

Zero-energy days are explicitly supported. Rest is valid; there are no failure states.

### 4.3 Key Behaviors

| Behavior | Description |
|----------|-------------|
| **Swap-first** | Starting a new Quest automatically returns the current Quest to Next Up (pinned at top) |
| **Emergency override** | WIP limit can be exceeded with explicit friction and reminder of why WIP=1 exists |
| **Neutral language** | No shame terminology; incomplete = paused, not failed |
| **Reversible actions** | Archive/delete always undoable |

---

## 5. Functional Requirements

### 5.1 QBA Board

The board consists of six columns with specific semantics:

#### Column 1: Inbox

- Rapid capture of ideas and impulses
- No commitment implied by presence in Inbox
- Reviewed during Weekly Harvest
- Items may be archived or deleted (reversible)

**Requirements:**
- FR-G-001: Quick capture input always accessible
- FR-G-002: Global keyboard shortcut for capture
- FR-G-003: Items sortable by creation date
- FR-G-004: Bulk select and move operations
- FR-G-005: Archive with undo (minimum 30 days recoverable)

#### Column 2: Epic Backlog

- Library of Epics (large initiatives)
- Epics may exist as title/description only—no task breakdown required until activated
- Epics owned by Guidance but may reference Knowledge notes and Tracking items

**Requirements:**
- FR-G-006: Create Epic with title only (description optional)
- FR-G-007: Link Epic to Knowledge notes
- FR-G-008: Link Epic to Tracking items (required materials)
- FR-G-009: Visual indicator of Epic size/complexity (optional)
- FR-G-010: Collapse/expand Epic details

#### Column 3: This Cycle's Epic

- Exactly one Epic represents current focus
- Changing Epic mid-cycle is allowed and non-shaming
- Provides direction without rigid commitment

**Requirements:**
- FR-G-011: Maximum one Epic in this column (enforced)
- FR-G-012: Moving new Epic here automatically returns previous Epic to backlog
- FR-G-013: Visual prominence for active Epic
- FR-G-014: Quick action to break Epic into Quests

#### Column 4: Next Up

- 3-5 ready-to-execute Quests derived from active Epic
- Supports pinned/paused Quests (swapped out of In Progress)
- Priority queue for upcoming work

**Requirements:**
- FR-G-015: Soft limit of 5 Quests (warning, not hard block)
- FR-G-016: Pinned Quest indicator (top of queue)
- FR-G-017: Drag-and-drop reordering
- FR-G-018: Energy requirement visible per Quest
- FR-G-019: Estimated time visible per Quest (optional)

#### Column 5: In Progress

- **WIP=1 by default**: Only one Quest active at a time
- Swap-first behavior when selecting new Quest
- Emergency override available with friction

**Requirements:**
- FR-G-020: Maximum one Quest enforced by default
- FR-G-021: Swap-first: selecting new Quest moves current to Next Up (pinned)
- FR-G-022: Emergency override with confirmation dialog explaining WIP=1 rationale
- FR-G-023: Override count tracked (for personal awareness, not judgment)
- FR-G-024: Visual timer showing time in current Quest

#### Column 6: Harvested

- Completed Quests as durable wins log
- Default: keep visible (anti-shame, progress evidence)
- Supports volume-based grouping into Seasons (collapsible)

**Requirements:**
- FR-G-025: Completed Quests visible by default
- FR-G-026: Auto-group into Seasons when count exceeds threshold (configurable)
- FR-G-027: Seasons collapsible to manage visual overwhelm
- FR-G-028: Archive/delete allowed but reversible
- FR-G-029: Completion date and time tracked
- FR-G-030: XP/points visible per Quest (if gamification enabled)

#### Board-Level Features

**Requirements:**
- FR-G-031: Drag-and-drop between all columns
- FR-G-032: Energy-based filtering (show only Quests matching current energy)
- FR-G-033: Tag filtering
- FR-G-034: Search within board
- FR-G-035: Bulk operations (archive, tag, move)
- FR-G-036: Board export (JSON, Markdown)

---

### 5.2 Focus/Zen Mode

Full-screen distraction-free interface for the active Quest.

#### UI Components

| Component | Location | Description |
|-----------|----------|-------------|
| Quest title | Center, prominent | Current Quest name |
| Timer | Center | Visual timer with time remaining |
| Progress bar | Below timer | Elapsed time visualization |
| Complete button | Bottom center | Large "MARK COMPLETE" action |
| On Deck preview | Bottom | Shows Next Up queue |
| Energy indicator | Top right | Current energy status |
| Level indicator | Top left | Current XP level (if gamification enabled) |

#### Functionality

**Requirements:**
- FR-G-037: Timer starts only when user explicitly starts Focus Mode
- FR-G-038: Pomodoro timer with configurable cycle length
- FR-G-039: Flow override: extend session without penalty
- FR-G-040: Progressive disclosure of Quest steps (subtasks)
- FR-G-041: Check off steps as completed
- FR-G-042: Auto-advance to next Quest option (user choice, not default)
- FR-G-043: Keyboard shortcuts for complete (Cmd/Ctrl+Enter) and exit (Escape)
- FR-G-044: Progress saved on exit (steps completed, time elapsed)
- FR-G-045: Haptic feedback on step/Quest completion (mobile)
- FR-G-046: Audio cue options for timer milestones

---

### 5.3 Energy Management

#### Daily Check-In

**Requirements:**
- FR-G-047: Morning energy assessment prompt (time configurable)
- FR-G-048: 1-5 scale with visual representation
- FR-G-049: Optional notes field ("how are you feeling?")
- FR-G-050: Skip check-in option (no shame)
- FR-G-051: Check-in history viewable
- FR-G-052: Pattern recognition over time (trends, correlations)

#### Energy-Based Filtering

**Requirements:**
- FR-G-053: Each Quest has energy requirement (1-5)
- FR-G-054: Filter board to show only Quests ≤ current energy
- FR-G-055: Visual indicator when Quests are filtered out
- FR-G-056: Quick toggle to show/hide filtered Quests

---

### 5.4 Daily Check-In Ritual

Structured workflow for starting the day.

**Flow:**
1. Optional energy check-in
2. Review Next Up queue
3. Select one Quest for today
4. Selected Quest moves to In Progress (swap-first if occupied)
5. Prompt: "Start Focus Mode?" (explicit choice, not automatic)

**Requirements:**
- FR-G-057: Daily check-in accessible from home screen
- FR-G-058: Energy check-in integrated (optional)
- FR-G-059: Next Up queue visible for selection
- FR-G-060: Single-tap Quest selection
- FR-G-061: Focus Mode prompt after selection (skippable)
- FR-G-062: Skip entire ritual option

---

### 5.5 Weekly Harvest Ritual

Structured weekly reflection and planning wizard.

**Flow:**
1. **Reminder**: Sunday evening notification (time configurable)
2. **Celebrate**: Review completed Quests first (wins before work)
3. **Reflect**: What was easy/fun? What was hard/stuck? Any new ideas?
4. **Adapt & Plan**: Interest check on current Epic; generate 3-5 Quests for Next Up
5. **Hygiene** (optional): Inbox cleanup, Season grouping
6. **Commit**: Explicit "Start Next Cycle" action

**Requirements:**
- FR-G-063: Configurable reminder (day, time)
- FR-G-064: Step-by-step wizard UI
- FR-G-065: Celebrate step shows Harvested Quests from past week
- FR-G-066: Reflect step with structured prompts
- FR-G-067: Interest check: "Still excited about [Epic]?" with easy pivot option
- FR-G-068: AI-assisted Quest generation (optional, explicit invocation)
- FR-G-069: Board hygiene prompts (archive stale Inbox items, etc.)
- FR-G-070: Cycle transition logged for history

---

### 5.6 Recovery & Emergency Support

#### Emergency Mode

Temporary simplified view for overwhelm.

**Requirements:**
- FR-G-071: One-click activation from settings or board
- FR-G-072: Reduced to 3 columns: Inbox, Focus (max 3), Done
- FR-G-073: All other Quests hidden (not deleted)
- FR-G-074: Easy exit back to full board
- FR-G-075: No shame language ("Taking it easy" not "Struggling")

#### Recovery Wizard

For returning after extended absence or board abandonment.

**Options:**
1. Archive current board (reversible)
2. Move everything to Inbox (reversible)
3. Start completely fresh (reversible)
4. "Phoenix Rising" acknowledgment

**Requirements:**
- FR-G-076: Triggered by extended inactivity (configurable threshold)
- FR-G-077: Manual trigger always available
- FR-G-078: Clear explanation of each option
- FR-G-079: All actions reversible with prominent undo
- FR-G-080: Celebratory language for fresh start ("Phoenix Rising")
- FR-G-081: Previous board state recoverable for minimum 90 days

---

### 5.7 Gamification System

Optional motivation layer.

#### Elements

| Element | Description |
|---------|-------------|
| **XP** | Points earned for Quest completion |
| **Levels** | Progression tiers with unlockable perks |
| **Badges** | Achievement recognition |
| **Streaks** | Consecutive days with completed Quests |
| **Challenges** | Optional weekly/monthly goals |
| **Dashboard** | Personal progress visualization |

#### Philosophy

- Opt-in during initial setup
- Default XP tied to Quest energy/effort (not all Quests equal)
- Simple mode available (flat XP) to avoid optimization anxiety
- Always fully disable-able
- Streaks reward progress but don't shame gaps

**Requirements:**
- FR-G-082: Gamification opt-in at setup
- FR-G-083: XP calculation configurable (effort-based or flat)
- FR-G-084: Level progression with meaningful perks (themes, sounds, etc.)
- FR-G-085: Badge system for milestones
- FR-G-086: Streak tracking with grace period (1 day miss doesn't break streak)
- FR-G-087: Weekly/monthly challenges (optional participation)
- FR-G-088: Progress dashboard with historical view
- FR-G-089: Full disable without losing history
- FR-G-090: Export gamification data

---

### 5.8 Quest Dependency Graphs

Visual representation of Quest relationships.

#### Relationship Types

| Type | Meaning |
|------|---------|
| **Blocks/Blocked by** | Quest A must complete before Quest B can start |
| **Related to** | Quests share context but no dependency |
| **Parent/Child** | Quest contains sub-Quests |
| **Follows/Precedes** | Soft ordering suggestion |

#### Visualization

**Requirements:**
- FR-G-091: DAG (Directed Acyclic Graph) visualization
- FR-G-092: Critical path highlighting
- FR-G-093: Progress visualization (completed/in-progress/blocked)
- FR-G-094: Interactive node selection
- FR-G-095: Zoom and pan navigation

#### Layout Options

- FR-G-096: Top-to-bottom tree layout
- FR-G-097: Left-to-right timeline layout
- FR-G-098: Force-directed network layout
- FR-G-099: Gantt-style view
- FR-G-100: Layout preference persisted

---

### 5.9 AI Assistance

AI features for planning support (v1 scope).

**Design principles:**
- Explicit invocation only (no background automation)
- User always in control
- Suggestions, not decisions

**Integration points:**

| Context | AI Action |
|---------|-----------|
| Epic creation | "Help me break this into Quests" |
| Quest creation | "Suggest smaller Quests / sub-quests" |
| Weekly Harvest | "Help me plan next cycle" |

**Requirements:**
- FR-G-101: AI button visible at relevant creation points
- FR-G-102: Generated suggestions are proposals (user accepts/rejects each)
- FR-G-103: AI respects energy levels in suggestions
- FR-G-104: Works with local AI (desktop) or cloud AI (mobile)
- FR-G-105: Graceful fallback when AI unavailable

---

## 6. Mobile-Specific Features

### 6.1 Full Feature Parity

All desktop features available on mobile with touch optimization:

- Complete QBA Board (all six columns)
- Focus/Zen Mode with timer
- Quest CRUD operations
- Energy system and daily check-in
- Weekly Harvest wizard
- Gamification (if enabled)
- Quick capture (global shortcut, widget, share target)

### 6.2 Touch Optimizations

**Requirements:**
- FR-G-106: Touch-optimized drag-and-drop between columns
- FR-G-107: Swipe gestures for common actions (complete, move, archive)
- FR-G-108: Haptic feedback on Quest/step completion
- FR-G-109: Pull-to-refresh for sync
- FR-G-110: Long-press for context menu

### 6.3 Notifications

**Requirements:**
- FR-G-111: Push notifications for timer completion
- FR-G-112: Daily energy check-in reminder
- FR-G-113: Weekly Harvest reminder
- FR-G-114: Quest deadline alerts (if deadlines set)
- FR-G-115: Notification preferences granular per type
- FR-G-116: Do Not Disturb integration

### 6.4 Widgets

**Requirements:**
- FR-G-117: Active Quest widget (shows current In Progress)
- FR-G-118: Quick capture widget (one-tap to add)
- FR-G-119: Energy status widget
- FR-G-120: Next Up preview widget
- FR-G-121: Daily stats widget (Quests completed today)

### 6.5 Background Operation

**Requirements:**
- FR-G-122: Pomodoro timer continues in background
- FR-G-123: Sync continues in background
- FR-G-124: Notification sounds respect device settings

---

## 7. Non-Functional Requirements

### 7.1 Performance

| Metric | Target |
|--------|--------|
| Quest creation | < 500ms |
| Board render (100 Quests) | < 1 second |
| Focus Mode launch | < 500ms |
| Dependency graph (50 nodes) | < 1 second |
| Search results | < 500ms |

### 7.2 Data Integrity

- All Quest state changes logged with timestamp
- Undo available for minimum 30 days
- No data loss on app crash or force quit
- Sync conflicts surfaced to user (never silent data loss)

### 7.3 Accessibility

- Full keyboard navigation (desktop)
- Screen reader support (VoiceOver, TalkBack)
- Dynamic type support (mobile)
- High contrast mode
- Reduce motion option
- Minimum touch targets (44x44 points)

---

## 8. Integration Points

### 8.1 Knowledge Integration

| From Guidance | To Knowledge |
|---------------|--------------|
| Quest | Link to research notes |
| Epic | Link to project documentation |
| Quest completion | Option to create reflection note |

| From Knowledge | To Guidance |
|----------------|-------------|
| Note | Extract actionable items as Quests |
| BoM in note | Generate shopping Quests |

### 8.2 Tracking Integration

| From Guidance | To Tracking |
|---------------|-------------|
| Quest | Link required materials/items |
| Quest start | Check item availability |
| Quest complete | Release reserved items |

| From Tracking | To Guidance |
|---------------|-------------|
| Low stock alert | Generate restock Quest |
| Maintenance due | Generate maintenance Quest |

---

## 9. Feature Priority

### Critical (Must Have)

1. QBA Board with WIP=1 enforcement
2. Focus/Zen Mode with timer
3. Quick capture (global activation)
4. Drag-and-drop between columns
5. Swap-first behavior
6. Basic Quest CRUD
7. Mobile: Push notifications
8. Mobile: Offline mode

### High Priority

1. Energy management system
2. Daily check-in ritual
3. Weekly Harvest wizard
4. Dependency graphs
5. Quest ↔ Note linking
6. Mobile: Widgets
7. Mobile: Haptic feedback

### Medium Priority

1. Gamification system
2. AI assistance for planning
3. Emergency Mode
4. Recovery Wizard
5. Gantt view
6. Bulk operations

---

## 10. Open Questions

1. **WIP limit flexibility**: Should users be able to configure WIP > 1 as a preference, or only via emergency override?
2. **Streak grace period**: Is 1 day the right grace period, or should it be configurable?
3. **AI model for planning**: Which local model provides acceptable Quest breakdown quality?
4. **Harvest skip behavior**: If a user skips Harvest for multiple weeks, should recovery wizard trigger?

---

## Appendix: Keyboard Shortcuts (Desktop)

| Action | Shortcut |
|--------|----------|
| Quick capture | Cmd/Ctrl + N |
| Complete current Quest | Cmd/Ctrl + Enter |
| Start Focus Mode | Cmd/Ctrl + F |
| Exit Focus Mode | Escape |
| Next Quest step | Cmd/Ctrl + ↓ |
| Previous Quest step | Cmd/Ctrl + ↑ |
| Toggle energy filter | Cmd/Ctrl + E |
| Open dependency graph | Cmd/Ctrl + G |
| Search board | Cmd/Ctrl + / |
