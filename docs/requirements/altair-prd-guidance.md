# Altair Product Requirements Document

## Guidance: Quest-Based Task Execution

| Field               | Value                                                  |
| ------------------- | ------------------------------------------------------ |
| **Version**         | 2.0                                                    |
| **Status**          | Draft                                                  |
| **Last Updated**    | 2026-01-14                                             |
| **Parent Document** | `altair-prd-core.md`                                   |
| **Integrates With** | Knowledge (note linking), Tracking (item requirements) |

---

## 1. Purpose

This document defines the product requirements for Guidance, the task execution application in the Altair ecosystem. Guidance implements Quest-Based Agile (QBA), a methodology that enforces focus through WIP limits, adapts to variable capacity through energy-based filtering, and eliminates shame through recovery-friendly design.

While these patterns are particularly valuable for users with ADHD or executive function challenges, they benefit anyone who struggles with traditional task management's tendency toward overwhelm and guilt.

For system-level architecture, design principles, Initiatives, Routines, and Universal Inbox, see `altair-prd-core.md`.

---

## 2. Problem Statement

Traditional task management creates specific execution problems:

- **Infinite backlogs**: Tools encourage capturing everything, creating overwhelming lists that trigger avoidance.
- **Context switching**: No enforcement of single-tasking; users can start infinite parallel work streams.
- **Willpower-dependent prioritization**: Users must constantly decide "what's next" when energy and focus are limited.
- **Shame accumulation**: Incomplete tasks remain visible, reinforcing negative self-perception.
- **Time blindness**: No support for estimating, tracking, or visualizing time spent on tasks.
- **Binary outcomes**: Tasks are either done or not-done; no support for pausing, pivoting, or recovering from abandonment.

Guidance addresses these through:
- **WIP=1 default**: Only one Quest active at a time (adjustable in settings)
- **Swap-first behavior**: Changing tasks doesn't abandon them—they return to queue
- **Energy filtering**: Only see tasks matching current capacity
- **Shame-free recovery**: Phoenix Rising and Emergency Mode for overwhelmed states

---

## 3. User Stories

### 3.1 Daily Workflow

> **As a user**, I want to start my day with a simple check-in so that I can match my work to my current capacity without decision fatigue.

> **As a user**, I want the system to enforce a single active task by default so that I maintain focus without relying on willpower.

> **As a user**, I want to swap tasks without guilt so that changing focus doesn't feel like failure.

> **As a user**, I want to end my day with a brief summary so I can see what I accomplished.

### 3.2 Focus & Flow

> **As a user**, I want a distraction-reduced focus mode so that I can enter and maintain flow state on my current task.

> **As a user**, I want visible timers and progress so that I can see how long I've been working.

> **As a user**, I want to extend focus sessions when I'm in the zone so that artificial time boxes don't break my flow.

### 3.3 Planning & Recovery

> **As a user**, I want a weekly ritual that celebrates wins first so that I build positive associations with review and planning.

> **As a user**, I want to recover from an abandoned board without shame so that I can restart fresh when I've been away too long.

> **As a user**, I want emergency mode when I'm overwhelmed so that I can simplify my view to just a few essential tasks.

### 3.4 Recurring Work

> **As a user**, I want to define routines that repeat on schedule so that recurring tasks appear automatically without manual re-entry.

> **As a user**, I want to see today's routines alongside my Quests so that I have a complete picture of what needs doing.

---

## 4. Core Concepts

### 4.1 Quest-Based Agile (QBA)

QBA adapts agile methodology for individual productivity:

| Agile Concept  | QBA Adaptation                                   |
| -------------- | ------------------------------------------------ |
| Sprint         | **Cycle** — Weekly timeboxed period              |
| Epic           | **Epic** — Group of related Quests (optional)    |
| Story          | **Quest** — Discrete, completable task           |
| Sprint backlog | **Next Up** — Limited queue of ready Quests      |
| WIP limits     | **WIP=1** — One Quest in progress (default, adjustable) |
| Retrospective  | **Harvest** — Weekly celebration and planning    |

### 4.2 Relationship to Initiatives

Epics can optionally belong to an **Initiative** (defined in core PRD):

- Initiative → Epic → Quest (hierarchical)
- Epics without an Initiative are standalone
- Quests can exist without an Epic (standalone tasks)
- Initiative Card in Guidance shows cross-app context

### 4.3 Relationship to Routines

**Routines** (defined in core PRD) generate Quest instances in Guidance:

- Routine Template defines the recurring pattern
- System spawns Quest instances based on schedule
- Routine instances appear in "Today" view alongside regular Quests
- Completing a routine instance works like completing any Quest

### 4.4 Relationship to Universal Inbox

**Universal Inbox** (defined in core PRD) feeds Quests into Guidance:

- Inbox items are untyped until triaged
- Triage action "This is a Quest" creates Quest in Guidance
- New Quests from Inbox go to Next Up (or Backlog based on user preference)

### 4.5 Energy System

Daily subjective capacity rating (not medical/health tracking):

| Level      | Symbol | Meaning            |
| ---------- | ------ | ------------------ |
| 1 - Tiny   | ⚡     | Minimal tasks only |
| 2 - Small  | ⚡⚡   | Light work         |
| 3 - Medium | ⚡⚡⚡ | Standard tasks     |
| 4 - Large  | ⚡⚡⚡⚡ | Complex work       |
| 5 - Huge   | ⚡⚡⚡⚡⚡ | Major projects     |

Zero-energy days are explicitly supported. Rest is valid; there are no failure states.

### 4.6 Key Behaviors

| Behavior               | Description                                                                             |
| ---------------------- | --------------------------------------------------------------------------------------- |
| **Swap-first**         | Starting a new Quest automatically returns the current Quest to Next Up (pinned at top) |
| **WIP limit (default 1)** | Configurable in settings; default enforces single-tasking |
| **Emergency override** | WIP limit can be exceeded with explicit friction and reminder of why limits help |
| **Neutral language**   | No shame terminology; incomplete = paused, not failed                                   |
| **Reversible actions** | Archive/delete always undoable                                                          |

---

## 5. Functional Requirements

### 5.1 QBA Board

The board shows work organized for execution. The exact visual layout (columns, swimlanes, or unified list) is a UX decision, but the logical groupings are:

#### Epic Backlog

- Library of Epics (groups of related Quests)
- Epics may exist as title/description only—no Quest breakdown required until activated
- Epics can optionally link to an Initiative

**Requirements:**

- FR-G-001: Create Epic with title only (description optional)
- FR-G-002: Link Epic to Initiative (optional)
- FR-G-003: Link Epic to Knowledge notes
- FR-G-004: Link Epic to Tracking items (required materials)
- FR-G-005: Visual indicator of Epic size/complexity (optional)
- FR-G-006: Collapse/expand Epic details

#### Active Epic

- One Epic represents current focus (optional—users can work without an active Epic)
- Changing Epic mid-cycle is allowed and non-shaming
- Provides direction without rigid commitment

**Requirements:**

- FR-G-007: Maximum one Epic marked as "active" (soft limit, warning if exceeded)
- FR-G-008: Visual prominence for active Epic
- FR-G-009: Quick action to break Epic into Quests
- FR-G-010: Standalone Quests (no Epic) fully supported

#### Next Up

- 3-5 ready-to-execute Quests (from active Epic, standalone, or routine instances)
- Supports pinned/paused Quests (swapped out of In Progress)
- Priority queue for upcoming work

**Requirements:**

- FR-G-011: Soft limit of 5 Quests (warning, not hard block)
- FR-G-012: Pinned Quest indicator (top of queue)
- FR-G-013: Drag-and-drop reordering
- FR-G-014: Energy requirement visible per Quest
- FR-G-015: Estimated time visible per Quest (optional)
- FR-G-016: Routine instances visually distinguished

#### In Progress

- **WIP=1 by default**: Only one Quest active at a time
- Swap-first behavior when selecting new Quest
- Emergency override available with friction

**Requirements:**

- FR-G-017: WIP limit enforced (default: 1, configurable in settings)
- FR-G-018: Swap-first: selecting new Quest moves current to Next Up (pinned)
- FR-G-019: Emergency override with confirmation dialog explaining WIP rationale
- FR-G-020: Override count tracked (for personal awareness, not judgment)
- FR-G-021: Visual timer showing time in current Quest

#### Harvested

- Completed Quests as durable wins log
- Default: keep visible (progress evidence)
- Supports volume-based grouping into Seasons (collapsible)

**Requirements:**

- FR-G-022: Completed Quests visible by default
- FR-G-023: Auto-group into Seasons when count exceeds threshold (configurable)
- FR-G-024: Seasons collapsible to manage visual overwhelm
- FR-G-025: Archive/delete allowed but reversible
- FR-G-026: Completion date and time tracked

#### Board-Level Features

**Requirements:**

- FR-G-027: Drag-and-drop between all groupings
- FR-G-028: Energy-based filtering (show only Quests matching current energy)
- FR-G-029: Tag filtering
- FR-G-030: Search within board
- FR-G-031: Bulk operations (archive, tag, move)
- FR-G-032: Board export (JSON, Markdown)

---

### 5.2 Today View

A focused view showing what's relevant for today:

**Contents:**
- Active Quest (if any)
- Routine instances due today
- Next Up queue (filtered to current energy if enabled)
- Energy status

**Requirements:**

- FR-G-033: Today view as default landing for Guidance
- FR-G-034: Show active Quest prominently
- FR-G-035: List routine instances due today with check-off
- FR-G-036: Show Next Up queue (optionally energy-filtered)
- FR-G-037: Quick actions: start Quest, check off routine, log energy

---

### 5.3 Focus Mode

A distraction-reduced interface for the active Quest. Presentation is configurable (dedicated view, floating widget, system tray with timer, or watch/phone remote control).

#### Core Elements

- Quest title and description
- Timer (elapsed or Pomodoro countdown)
- Progress indicator
- Checkpoints (if Quest has sub-steps)
- Complete action
- On Deck preview (Next Up queue)

#### Functionality

**Requirements:**

- FR-G-038: Timer starts only when user explicitly starts Focus Mode
- FR-G-039: Pomodoro timer with configurable cycle length
- FR-G-040: Flow override: extend session without penalty
- FR-G-041: Progressive disclosure of Quest checkpoints
- FR-G-042: Check off checkpoints as completed
- FR-G-043: Auto-advance to next Quest option (user choice, not default)
- FR-G-044: Keyboard/gesture shortcuts for complete and exit
- FR-G-045: Progress saved on exit (checkpoints completed, time elapsed)
- FR-G-046: Haptic feedback on checkpoint/Quest completion (mobile)
- FR-G-047: Audio cue options for timer milestones
- FR-G-048: Remote control from watch or secondary device
- FR-G-049: Configurable presentation (dedicated view, widget, system tray)

---

### 5.4 Energy Management

#### Daily Check-In

**Requirements:**

- FR-G-050: Morning energy assessment prompt (time configurable)
- FR-G-051: 1-5 scale with visual representation
- FR-G-052: Optional notes field ("how are you feeling?")
- FR-G-053: Skip check-in option (no shame)
- FR-G-054: Check-in history viewable
- FR-G-055: Pattern recognition over time (trends, correlations)

#### Energy-Based Filtering

**Requirements:**

- FR-G-056: Each Quest has energy requirement (1-5)
- FR-G-057: Filter board to show only Quests ≤ current energy
- FR-G-058: Visual indicator when Quests are filtered out
- FR-G-059: Quick toggle to show/hide filtered Quests

---

### 5.5 Daily Rituals

#### Morning Check-In

Structured workflow for starting the day.

**Flow:**

1. Optional energy check-in
2. Show today's routine instances
3. Review Next Up queue
4. Select one Quest for today (or start a routine)
5. Selected Quest moves to In Progress (swap-first if occupied)
6. Prompt: "Start Focus Mode?" (explicit choice, not automatic)

**Requirements:**

- FR-G-060: Daily check-in accessible from home screen
- FR-G-061: Energy check-in integrated (optional)
- FR-G-062: Routine instances shown for today
- FR-G-063: Next Up queue visible for selection
- FR-G-064: Single-tap Quest selection
- FR-G-065: Focus Mode prompt after selection (skippable)
- FR-G-066: Skip entire ritual option

#### Evening Wrap-Up

Optional end-of-day summary.

**Contents:**

- Quests completed today
- Time spent in Focus Mode
- Routine instances completed/skipped
- Optional: reflection prompt ("What went well?")

**Requirements:**

- FR-G-067: Evening wrap-up prompt (time configurable)
- FR-G-068: Summary of day's completions
- FR-G-069: Time tracking summary
- FR-G-070: Optional reflection note (saved to Knowledge if desired)
- FR-G-071: Skip wrap-up option

---

### 5.6 Weekly Harvest Ritual

Structured weekly reflection and planning wizard.

**Flow:**

1. **Reminder**: Notification at configured day and time
2. **Celebrate**: Review completed Quests first (wins before work)
3. **Reflect**: What was easy/fun? What was hard/stuck? Any new ideas?
4. **Adapt & Plan**: Interest check on current Epic; generate 3-5 Quests for Next Up
5. **Routines**: Review routine performance (completed/skipped)
6. **Hygiene** (optional): Archive stale items, Season grouping
7. **Commit**: Explicit "Start Next Cycle" action

**Requirements:**

- FR-G-072: Reminder day and time fully configurable (not just Sunday)
- FR-G-073: Step-by-step wizard UI
- FR-G-074: Celebrate step shows Harvested Quests from past week
- FR-G-075: Reflect step with structured prompts
- FR-G-076: Interest check: "Still excited about [Epic]?" with easy pivot option
- FR-G-077: Routine summary: completion rate, skipped instances
- FR-G-078: AI-assisted Quest generation (optional, explicit invocation)
- FR-G-079: Hygiene prompts (archive stale items, etc.)
- FR-G-080: Cycle transition logged for history

---

### 5.7 Recovery & Emergency Support

#### Emergency Mode

Temporary simplified view for overwhelm.

**Requirements:**

- FR-G-081: One-click activation from settings or board
- FR-G-082: Reduced to 3 groupings: capture, Focus (max 3), Done
- FR-G-083: All other Quests hidden (not deleted)
- FR-G-084: Easy exit back to full board
- FR-G-085: No shame language ("Taking it easy" not "Struggling")

#### Recovery Wizard

For returning after extended absence or board abandonment.

**Options:**

1. Archive current board (reversible)
2. Move everything to Inbox (reversible)
3. Start completely fresh (reversible)
4. "Phoenix Rising" acknowledgment

**Requirements:**

- FR-G-086: Triggered by extended inactivity (configurable threshold, default 2 weeks)
- FR-G-087: Manual trigger always available
- FR-G-088: Clear explanation of each option
- FR-G-089: All actions reversible with prominent undo
- FR-G-090: Celebratory language for fresh start ("Phoenix Rising")
- FR-G-091: Previous board state recoverable for minimum 90 days

---

### 5.8 AI Assistance

AI features for planning support.

**Design principles:**

- Default: explicit invocation only (user clicks button)
- Adjustable: users can enable proactive suggestions if desired
- User always in control
- Suggestions, not decisions

**Integration points:**

| Context        | AI Action                             |
| -------------- | ------------------------------------- |
| Epic creation  | "Help me break this into Quests"      |
| Quest creation | "Suggest smaller steps"               |
| Weekly Harvest | "Help me plan next cycle"             |

**Requirements:**

- FR-G-092: AI button visible at relevant creation points
- FR-G-093: Generated suggestions are proposals (user accepts/rejects each)
- FR-G-094: AI respects energy levels in suggestions
- FR-G-095: Works with server AI or configured provider
- FR-G-096: Graceful fallback when AI unavailable
- FR-G-097: Setting to enable proactive AI suggestions (default: off)
- FR-G-098: Proactive suggestions are dismissible and non-intrusive

---

### 5.9 Checkpoints

Quests can have ordered sub-steps for complex tasks.

**Requirements:**

- FR-G-099: Add checkpoints to any Quest
- FR-G-100: Checkpoints have title and completed status
- FR-G-101: Reorder checkpoints (drag-and-drop)
- FR-G-102: Check off checkpoints in Focus Mode
- FR-G-103: Display progress as fraction (e.g., "3/5") on Quest cards
- FR-G-104: Completing all checkpoints does NOT auto-complete Quest

---

### 5.10 Gamification (v1.1)

Optional motivation layer. **Deferred to v1.1** to focus v1 on core execution flow.

#### Planned Elements

| Element        | Description                             |
| -------------- | --------------------------------------- |
| **XP**         | Points earned for Quest completion      |
| **Levels**     | Progression tiers with unlockable perks |
| **Badges**     | Achievement recognition                 |
| **Streaks**    | Consecutive days with completed Quests  |
| **Challenges** | Optional weekly/monthly goals           |
| **Dashboard**  | Personal progress visualization         |

#### Philosophy (for v1.1 implementation)

- Opt-in during initial setup
- Always fully disable-able
- Streaks reward progress but don't shame gaps (grace period)
- Simple mode available to avoid optimization anxiety

---

## 6. Mobile Features

Mobile is a **daily driver** for Guidance, not just a companion. All core features are available with touch optimization.

### 6.1 Full Feature Parity

All execution features available on mobile:

- Complete QBA Board views
- Today view with routines and Quests
- Focus Mode with timer
- Quest CRUD operations
- Energy system and daily check-in
- Evening wrap-up
- Weekly Harvest wizard
- Routine check-off

### 6.2 Touch Optimizations

**Requirements:**

- FR-G-105: Touch-optimized drag-and-drop between columns
- FR-G-106: Swipe gestures for common actions (complete, move, archive)
- FR-G-107: Haptic feedback on Quest/checkpoint completion
- FR-G-108: Pull-to-refresh for sync
- FR-G-109: Long-press for context menu

### 6.3 Notifications

**Requirements:**

- FR-G-110: Push notifications for timer completion
- FR-G-111: Daily energy check-in reminder
- FR-G-112: Evening wrap-up reminder
- FR-G-113: Weekly Harvest reminder
- FR-G-114: Routine due reminders (at specified time)
- FR-G-115: Notification preferences granular per type
- FR-G-116: Do Not Disturb integration

### 6.4 Widgets

**Requirements:**

- FR-G-117: Active Quest widget (shows current In Progress)
- FR-G-118: Today view widget (routines + next Quest)
- FR-G-119: Energy status widget
- FR-G-120: Daily stats widget (Quests completed today)

### 6.5 Background Operation

**Requirements:**

- FR-G-121: Pomodoro timer continues in background
- FR-G-122: Sync continues in background
- FR-G-123: Notification sounds respect device settings

---

## 7. Non-Functional Requirements

### 7.1 Performance

| Metric                      | Target     |
| --------------------------- | ---------- |
| Quest creation              | < 500ms    |
| Board render (100 Quests)   | < 1 second |
| Focus Mode launch           | < 500ms    |
| Today view load             | < 500ms    |
| Search results              | < 500ms    |

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

### 8.1 Universal Inbox Integration

| From Universal Inbox | To Guidance                     |
| -------------------- | ------------------------------- |
| Triage "This is a Quest" | Creates Quest in Next Up or Backlog |
| Link to Initiative   | Quest inherits Initiative context |

### 8.2 Initiative Integration

| From Guidance | To Initiative                    |
| ------------- | -------------------------------- |
| Epic          | Can belong to an Initiative      |
| Quest         | Shows Initiative context via Epic |
| Progress      | Contributes to Initiative summary |

| From Initiative | To Guidance                      |
| --------------- | -------------------------------- |
| Initiative Card | Shows Quest count and status     |
| Focus setting   | Highlights related Epics/Quests  |

### 8.3 Routine Integration

| From Core (Routines) | To Guidance                     |
| -------------------- | ------------------------------- |
| Routine Template     | Spawns Quest instances          |
| Schedule             | Determines when instances appear |
| Time of Day          | Triggers notifications          |

| From Guidance     | To Core (Routines)              |
| ----------------- | ------------------------------- |
| Instance complete | Updates Routine history         |
| Instance skip     | Logged without penalty          |

### 8.4 Knowledge Integration

| From Guidance    | To Knowledge                     |
| ---------------- | -------------------------------- |
| Quest            | Link to research notes           |
| Epic             | Link to project documentation    |
| Quest completion | Option to create reflection note |
| Evening wrap-up  | Save reflection to Knowledge     |

| From Knowledge | To Guidance                        |
| -------------- | ---------------------------------- |
| Note           | Extract actionable items as Quests |
| BoM in note    | Generate shopping Quests           |

### 8.5 Tracking Integration

| From Guidance  | To Tracking                   |
| -------------- | ----------------------------- |
| Quest          | Link required materials/items |
| Quest start    | Check item availability       |
| Quest complete | Release reserved items        |

| From Tracking   | To Guidance                |
| --------------- | -------------------------- |
| Low stock alert | Generate restock Quest     |
| Maintenance due | Generate maintenance Quest |

---

## 9. Feature Priority

### Critical (Must Have)

1. QBA Board with WIP limit (default 1, configurable)
2. Today view with routines and Quests
3. Swap-first behavior
4. Basic Quest CRUD
5. Routine instance display and completion
6. Quest ↔ Note linking (core Altair differentiator)
7. Mobile: Push notifications
8. Mobile: Offline mode

### High Priority

1. Focus Mode with timer
2. Energy management system
3. Daily check-in ritual
4. Evening wrap-up
5. Weekly Harvest wizard
6. Epic → Initiative linking
7. Checkpoints (Quest sub-steps)
8. Mobile: Widgets
9. Mobile: Haptic feedback

### Medium Priority

1. AI assistance for planning
2. Proactive AI suggestions (opt-in)
3. Emergency Mode
4. Recovery Wizard
5. Bulk operations
6. Energy pattern analysis

### Deferred (v1.1)

1. Gamification system (XP, levels, badges, streaks)

### Deferred (UI Plugin)

1. Dependency graph visualization
2. Gantt-style view

---

## 10. Resolved Design Decisions

| Question | Resolution |
|----------|------------|
| WIP limit flexibility | Configurable in settings (default: 1); emergency override also available |
| AI invocation style | Default explicit; adjustable setting to enable proactive suggestions |
| Harvest skip behavior | Recovery wizard triggers after 2+ weeks of inactivity |
| Inbox ownership | Universal Inbox at system level (see core PRD); Guidance receives triaged Quests |

---

## Appendix: Keyboard-Accessible Actions

The following actions should be accessible via configurable keyboard shortcuts on desktop:

| Action                    | Description                          |
| ------------------------- | ------------------------------------ |
| Complete current Quest    | Mark active Quest as done            |
| Start Focus Mode          | Enter Focus Mode for active Quest    |
| Exit Focus Mode           | Leave Focus Mode                     |
| Next checkpoint           | Advance to next checkpoint           |
| Previous checkpoint       | Go back to previous checkpoint       |
| Toggle energy filter      | Show/hide energy-filtered Quests     |
| Search board              | Open search within Guidance          |
| Open Today view           | Navigate to Today view               |
| Log energy                | Open energy check-in                 |
| New Quest                 | Create new Quest                     |
