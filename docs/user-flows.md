# Altair User Flows

**Version**: 2.0  
**Status**: APPROVED  
**Created**: 2025-11-29  
**Author**: Robert Hamilton

> **Core user workflows** for Guidance, Knowledge, Tracking, and Quick Capture

---

## Quick Reference

| App | Primary Flows | Key Interaction |
|-----|---------------|-----------------|
| **Guidance** | QBA Board, Focus Mode, Energy Check-in, Weekly Harvest | 6-column Kanban with WIP=1 |
| **Knowledge** | Daily Notes, Wiki-linking, Mind Maps | `[[wiki-links]]` while typing |
| **Tracking** | Items, Reservations, Maintenance | Quantity + location management |
| **Quick Capture** | Multi-modal capture → classify | One tap in, batch review out |

---

## 📥 Quick Capture

### QC-1: Capture Something

**Trigger:** User has thought/sees something worth saving

**Goal:** Zero-friction capture with no decisions

```
┌─────────────────────────────────────┐
│  Quick Capture                   ×  │
├─────────────────────────────────────┤
│                                     │
│  [📝 Text] [🎤 Voice] [📸 Photo] [📹 Video]
│                                     │
│  ─────────────────────────────────  │
│  │ Type or paste...             │  │
│  ─────────────────────────────────  │
│                                     │
│            [Capture ↵]              │
│                                     │
└─────────────────────────────────────┘
```

**Flow:**

1. User triggers Quick Capture (hotkey, menu bar, widget)
2. Capture window appears with text field focused
3. User either:
   - Types/pastes text → presses Enter
   - Clicks Photo → takes/selects photo
   - Clicks Voice → records audio (max 5 min)
   - Clicks Video → records video (max 2 min)
4. Capture saved with `status: pending`
5. Window closes immediately
6. Badge updates on all apps: "1 pending capture"

**Rules:**

- No destination selection at capture time
- No required fields except content
- Auto-captures: timestamp, location (if enabled), source app
- Window closes on capture — user returns to what they were doing
- Video auto-compresses, generates thumbnail

**Keyboard:** `Cmd+Shift+C` (global)

---

### QC-2: Review Pending Captures

**Trigger:** User opens any app with pending captures, or scheduled review

**Goal:** Efficiently classify captures to their destination

```
┌─────────────────────────────────────────────────────┐
│  Review Captures                     3 remaining    │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌───────────────────────────────────────────────┐  │
│  │ "Buy new HDMI cable for office monitor"       │  │
│  │ 📍 Home • ⏰ 2 hours ago                       │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│  AI suggests: 🎯 Quest (inventory related)          │
│                                                     │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐   │
│  │🎯 Quest │ │📚 Note  │ │📦 Item  │ │🗑 Skip  │   │
│  │   (1)   │ │   (2)   │ │   (3)   │ │  (0/d)  │   │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘   │
│       ↑                                             │
│  highlighted                                        │
│                                                     │
│  [← Previous]                        [Next →]       │
└─────────────────────────────────────────────────────┘
```

**Flow:**

1. User sees badge "3 pending captures"
2. User clicks badge or navigates to Capture Review
3. First capture displayed with AI suggestion highlighted
4. User selects destination:
   - **Quest** → Opens mini Quest form (title pre-filled)
   - **Note** → Opens mini Note form (content pre-filled)
   - **Item** → Opens mini Item form (name pre-filled)
   - **Skip** → Marks as `discarded`, moves to next
5. After selection, next capture auto-loads
6. When done: "All caught up! 🎉"

**Keyboard:**

- `1` — Quest
- `2` — Note
- `3` — Item
- `0` or `d` — Discard
- `←` `→` — Navigate without deciding

**Rules:**

- AI suggestion is pre-selected (user can override with one click)
- Mini forms have minimal required fields
- Can skip and return later
- 30-day auto-archive for untouched captures

---

## 🎯 Guidance (Quest Management)

### G-1: QBA Board Overview

**Trigger:** User opens Guidance app

**Goal:** See quest pipeline at a glance

```
┌──────────────────────────────────────────────────────────────────────────────────┐
│  Guidance                                        Energy: [Medium ▾]  👤 Lv.5     │
├──────────────────────────────────────────────────────────────────────────────────┤
│                                                                                  │
│  💡 Idea       📋 Quest Log    🎯 This Cycle   ⏭️ Next Up    🔥 In Progress  🌾 Harvested
│  Greenhouse    (12)           (1)            (3/5)        (1/1)          (8)
│  ┌──────────┐  ┌──────────┐   ┌──────────┐   ┌──────────┐  ┌──────────┐   ┌──────────┐
│  │ Learn    │  │ Fix auth │   │ Launch   │   │ Write    │  │ ████████ │   │ ✓ Setup  │
│  │ Rust     │  │ bug      │   │ MVP      │   │ tests    │  │ Review   │   │ CI/CD    │
│  │          │  │ ⚡⚡⚡ 45m │   │ ⚡⚡⚡⚡⚡   │   │ ⚡⚡⚡ 60m │  │ PR       │   │          │
│  ├──────────┤  ├──────────┤   └──────────┘   ├──────────┤  │ ⚡⚡ 30m  │   ├──────────┤
│  │ Try new  │  │ Refactor │                  │ Update   │  │          │   │ ✓ Write  │
│  │ editor   │  │ DB layer │                  │ docs     │  │ 🔥 Focus │   │ specs    │
│  │          │  │ ⚡⚡⚡⚡120m│                  │ ⚡⚡ 30m  │  └──────────┘   │          │
│  ├──────────┤  ├──────────┤                  ├──────────┤                 ├──────────┤
│  │ ...      │  │ ...      │                  │ Email    │                 │ ...      │
│  └──────────┘  └──────────┘                  │ team     │                 └──────────┘
│                                              │ ⚡ 15m   │
│                                              └──────────┘
│                                                                                  │
│  [+ Add Quest]                               [📊 Stats]  [🌾 Weekly Harvest]     │
└──────────────────────────────────────────────────────────────────────────────────┘
```

**Columns:**

| Column | Purpose | Limit |
|--------|---------|-------|
| 💡 Idea Greenhouse | Unrefined ideas | Unlimited |
| 📋 Quest Log | Refined, actionable | Unlimited |
| 🎯 This Cycle | Current cycle focus | **Max 1** |
| ⏭️ Next Up | Priority queue | **Max 5** |
| 🔥 In Progress | Active work (WIP=1) | **Strictly 1** |
| 🌾 Harvested | Completed | Unlimited |

**Interactions:**

- Drag-and-drop between columns
- Double-click to edit
- Right-click for context menu
- Click quest → detail panel

**Rules:**

- WIP=1 strictly enforced (warning if trying to add second)
- Visual warnings when limits exceeded
- Energy filter hides quests above current level

---

### G-2: Quick Add Quest

**Trigger:** User thinks of something they need to do

**Keyboard:** `Cmd+N`

```
┌─────────────────────────────────────────────────────┐
│  New Quest                                       ×  │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Title                                              │
│  ┌───────────────────────────────────────────────┐  │
│  │ Fix authentication bug in login flow          │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│  Energy                                             │
│  [⚡] [⚡⚡] [⚡⚡⚡] [⚡⚡⚡⚡] [⚡⚡⚡⚡⚡]               │
│  Tiny Small  Med   Large  Huge                     │
│              ↑ selected                            │
│                                                     │
│  Add to: [💡 Idea Greenhouse ▾]                    │
│                                                     │
│  Campaign: [None ▾]                                │
│                                                     │
│            [Create Quest ↵]    [More Options...]    │
└─────────────────────────────────────────────────────┘
```

**Flow:**

1. User triggers Quick Add (`Cmd+N`)
2. Modal appears with title focused
3. User types title
4. User selects energy level (5 options)
5. User selects column (defaults to Idea Greenhouse)
6. User optionally assigns campaign
7. User presses Enter or clicks Create
8. Quest created, modal closes

**More Options expands to:**

- Description (markdown)
- Due date (soft guidance)
- Estimated time
- Tags
- Dependencies (blocks/blocked by)
- Link notes/items

---

### G-3: Daily Energy Check-In

**Trigger:** Start of day (prompted) or manual

**Goal:** Set energy level to filter appropriate quests

```
┌─────────────────────────────────────────────────────┐
│  How's your energy today?                           │
├─────────────────────────────────────────────────────┤
│                                                     │
│    ┌─────┐  ┌─────┐  ┌─────┐  ┌─────┐  ┌─────┐    │
│    │ 😴  │  │ 🙂  │  │ 😊  │  │ 💪  │  │ 🔥  │    │
│    │Tiny │  │Small│  │ Med │  │Large│  │Huge │    │
│    │ ⚡  │  │ ⚡⚡ │  │⚡⚡⚡ │  │⚡⚡⚡⚡│  │⚡⚡⚡⚡⚡│   │
│    └─────┘  └─────┘  └─────┘  └─────┘  └─────┘    │
│                        ↑                            │
│                    selected                         │
│                                                     │
│  Optional: How are you feeling?                     │
│  ┌───────────────────────────────────────────────┐  │
│  │ Good sleep, ready to tackle that refactor     │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│            [Save & Show My Quests]                  │
│                                                     │
│  Your pattern: Usually Medium on Mondays           │
└─────────────────────────────────────────────────────┘
```

**Flow:**

1. User prompted at configurable time (default: 9am)
2. User selects current energy level
3. Optional: Add notes about how they're feeling
4. System records check-in
5. Board filters to show appropriate quests
6. Pattern recognition over time

**Rules:**

- Check-in awards 5 XP
- Patterns help predict capacity
- Can change energy during day
- Not mandatory — just helpful

---

### G-4: Enter Focus Mode (Zen Mode)

**Trigger:** User starts working on In Progress quest

**Keyboard:** `Cmd+Shift+F` or click "🔥 Focus" button

```
┌──────────────────────────────────────────────────────────────────────────────────┐
│  ┌────┐                                                          ┌────────────┐ │
│  │Lv.5│                    FOCUS MODE                            │ ⚡⚡⚡ Med   │ │
│  └────┘                                                          └────────────┘ │
├──────────────────────────────────────────────────────────────────────────────────┤
│                                                                                  │
│                                                                                  │
│                         ┌────────────────────────────┐                          │
│                         │                            │                          │
│                         │    Review PR from team     │                          │
│                         │                            │                          │
│                         │    ⚡⚡ Medium • 30 min     │                          │
│                         │                            │                          │
│                         └────────────────────────────┘                          │
│                                                                                  │
│                    ════════════════════════════════                              │
│                    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░  18m remaining              │
│                    ════════════════════════════════                              │
│                                                                                  │
│                         ☑ Open PR in browser                                    │
│                         ☐ Review code changes                                   │
│                         ☐ Test locally                                          │
│                         ☐ Leave comments                                        │
│                         ☐ Approve or request changes                            │
│                                                                                  │
│                                                                                  │
│                    ┌──────────────────────────────┐                              │
│                    │      MARK COMPLETE ✓         │                              │
│                    └──────────────────────────────┘                              │
│                                                                                  │
├──────────────────────────────────────────────────────────────────────────────────┤
│  On Deck:  Write tests (60m)  •  Update docs (30m)  •  Email team (15m)         │
└──────────────────────────────────────────────────────────────────────────────────┘
```

**Flow:**

1. User clicks Focus button on In Progress quest
2. Full-screen focus mode opens
3. Timer starts automatically (Pomodoro: 25m default)
4. User works through steps (checkboxes)
5. Timer shows visual progress bar
6. On completion or timer end:
   - Click "MARK COMPLETE" → Quest moves to Harvested
   - Or extend timer / take break

**UI Elements:**

- Level indicator (top left)
- Energy indicator (top right)
- Large quest title
- Visual timer (progress bar, not just numbers)
- Progressive disclosure of quest steps
- "On Deck" preview (Next Up queue)
- Large completion button

**Keyboard:**

- `Space` — Pause/resume timer
- `Enter` — Mark complete
- `Esc` — Exit focus mode (with confirmation)
- `1-9` — Check step 1-9

**Rules:**

- Focus session awards 15 XP on completion
- Steps checked off are saved immediately
- Can exit without completing (saves progress)
- Auto-advance to next quest option

---

### G-5: Complete Quest

**Trigger:** User finishes a task

```
┌─────────────────────────────────────────────────────┐
│  ✓ Quest Completed!                         +50 XP  │
├─────────────────────────────────────────────────────┤
│                                                     │
│  🎉 "Review PR from team"                          │
│                                                     │
│  Time spent: 28 minutes (estimated: 30m)           │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  Any notes or follow-ups?                           │
│  ┌───────────────────────────────────────────────┐  │
│  │ Found a bug, created follow-up quest          │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│  ☐ Create follow-up quest                          │
│  ☐ Link to a note                                  │
│                                                     │
│            [Done 🎉]                                │
└─────────────────────────────────────────────────────┘
```

**Flow:**

1. User completes quest (checkbox, button, or keyboard)
2. Celebration modal appears with XP gained
3. User optionally adds notes
4. User optionally creates follow-up or links note
5. Quest moves to Harvested column
6. XP added to profile

**Rules:**

- XP based on energy level (Tiny=10, Small=25, Med=50, Large=100, Huge=200)
- Quick complete: Shift+click skips modal
- Check for achievements after completion

---

### G-6: Weekly Harvest Ritual

**Trigger:** Sunday evening (configurable) or manual

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  🌾 Weekly Harvest                                           Sunday, Nov 24     │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  This Week's Harvest: 12 quests completed! 🎉                                  │
│                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │  ⚡ Tiny: 3    ⚡⚡ Small: 4    ⚡⚡⚡ Med: 4    ⚡⚡⚡⚡ Large: 1            │   │
│  │                                                                         │   │
│  │  XP Earned: 385 XP          Focus Time: 4h 20m                         │   │
│  │  Streak: 🔥 14 days                                                     │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  📈 Patterns:                                                                   │
│  • Most productive on Tuesday (4 quests)                                       │
│  • Energy estimates were accurate 80% of the time                              │
│  • Average focus session: 22 minutes                                           │
│                                                                                 │
│  ─────────────────────────────────────────────────────────────────────────────  │
│                                                                                 │
│  🗑️ Archive old harvested quests?                                              │
│  ☑ Archive 8 quests older than 7 days                                          │
│                                                                                 │
│  🎯 Set This Cycle's focus for next week:                                      │
│  [Launch MVP ▾]                                                                │
│                                                                                 │
│  📝 Reflection (optional):                                                      │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │ Good week! Finally got the auth system working.                        │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│            [Complete Harvest +50 XP]                                           │
└─────────────────────────────────────────────────────────────────────────────────┘
```

**Flow:**

1. Reminder appears Sunday evening
2. Summary shows week's achievements
3. Patterns displayed for reflection
4. User archives old quests
5. User sets next week's cycle focus
6. Optional reflection notes
7. Complete awards 50 XP

**Rules:**

- Non-judgmental tone
- Celebrates all progress
- Patterns help adjust future planning
- Reflection is optional

---

### G-7: View Quest Dependencies

**Trigger:** User wants to see quest relationships

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  Quest Dependencies                                    [Tree ▾] [Gantt] [Graph] │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│                         ┌──────────────────┐                                   │
│                         │ 🔴 Launch MVP     │                                   │
│                         │ This Cycle       │                                   │
│                         └────────┬─────────┘                                   │
│                    ┌─────────────┼─────────────┐                               │
│                    ▼             ▼             ▼                               │
│          ┌─────────────┐ ┌─────────────┐ ┌─────────────┐                       │
│          │ ✅ Setup    │ │ 🟡 Fix auth │ │ ⬜ Write    │                       │
│          │ CI/CD       │ │ In Progress │ │ docs        │                       │
│          └─────────────┘ └──────┬──────┘ └─────────────┘                       │
│                                 │                                               │
│                                 ▼                                               │
│                          ┌─────────────┐                                       │
│                          │ ⬜ Deploy   │                                       │
│                          │ staging     │                                       │
│                          └─────────────┘                                       │
│                                                                                 │
│  Legend: ✅ Completed  🟡 In Progress  🔴 Blocked  ⬜ Not Started              │
│                                                                                 │
│  Critical Path: Setup CI/CD → Fix auth → Deploy staging → Launch MVP           │
└─────────────────────────────────────────────────────────────────────────────────┘
```

**Layout Options:**

- **Tree** — Top-to-bottom hierarchy
- **Gantt** — Timeline view with dependencies
- **Graph** — Force-directed network

**Interactions:**

- Click node → Quest detail
- Drag to reorder (updates dependencies)
- Right-click → Add dependency

---

## 📚 Knowledge (PKM)

### K-1: Open Today's Daily Note

**Trigger:** User opens Knowledge app

**Goal:** Start with daily note as scratch pad

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  Knowledge                                    [🔍 Search]  [📊 Graph]  [+ Note] │
├─────────────────────────────────────────────────────────────────────────────────┤
│  Folders          │  Friday, November 29, 2024                                  │
│  ─────────────    │  ─────────────────────────────────────────────────────────  │
│  📁 Inbox         │                                                             │
│  📁 Projects      │  ## Morning                                                 │
│    └ Altair       │                                                             │
│    └ Home Lab     │  - Meeting with team at 10am                               │
│  📁 Areas         │  - Need to review [[Authentication Architecture]]          │
│  📁 Resources     │  - [[Fix auth bug]] is blocking launch                     │
│  📁 Archive       │                                                             │
│  ─────────────    │  ## Notes                                                   │
│  Recent Notes     │                                                             │
│  ─────────────    │  Learned about [[SurrealDB]] change feeds today.           │
│  • Nov 28         │  Could be useful for the sync engine.                      │
│  • Nov 27         │                                                             │
│  • Nov 26         │  ## Tasks captured                                         │
│                   │                                                             │
│                   │  - [ ] Email team about deployment                         │
│                   │  - [x] Review PR                                           │
│                   │                                                             │
│                   │─────────────────────────────────────────────────────────────│
│                   │  Backlinks (2)                                              │
│                   │  ← Nov 28 daily note                                       │
│                   │  ← Project Altair                                          │
└─────────────────────────────────────────────────────────────────────────────────┘
```

**Flow:**

1. User opens Knowledge
2. Today's daily note auto-opens (created if doesn't exist)
3. User types freely — scratch pad for the day
4. Wiki-links auto-complete as user types `[[`
5. Backlinks panel shows incoming links
6. Auto-save on every change (debounced)

**Rules:**

- Daily note created automatically
- Previous daily notes accessible in sidebar
- Format: `YYYY-MM-DD` or localized
- Serves as daily log and scratch pad

---

### K-2: Create Wiki-Link

**Trigger:** User types `[[` in any note

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  ...working on the [[                                                           │
│                    ┌─────────────────────────────────────┐                      │
│                    │ 🔍 Search notes...                  │                      │
│                    ├─────────────────────────────────────┤                      │
│                    │ 📚 Authentication Architecture      │ ← Recent            │
│                    │ 📚 SurrealDB                        │                      │
│                    │ 📚 Sync Engine Design               │                      │
│                    ├─────────────────────────────────────┤                      │
│                    │ + Create "auth flow"                │ ← New note          │
│                    └─────────────────────────────────────┘                      │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

**Flow:**

1. User types `[[`
2. Autocomplete popup appears
3. User types to filter
4. Select existing note or create new
5. Link inserted: `[[Note Title]]`
6. Bidirectional link created automatically

**Alias Support:**

- `[[note|Display Text]]` — Shows "Display Text" but links to "note"

---

### K-3: View Mind Map (Graph View)

**Trigger:** User clicks Graph button or `Cmd+G`

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  Graph View                               [Local ▾] [Filter...] [Layout: Force] │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│                        ┌──────────┐                                            │
│              ┌─────────│ Altair   │─────────┐                                  │
│              │         │ Project  │         │                                  │
│              ▼         └──────────┘         ▼                                  │
│      ┌──────────┐                    ┌──────────┐                              │
│      │ Auth     │                    │ Sync     │                              │
│      │ Design   │◄───────────────────│ Engine   │                              │
│      └────┬─────┘                    └────┬─────┘                              │
│           │                               │                                     │
│           ▼                               ▼                                     │
│      ┌──────────┐                    ┌──────────┐                              │
│      │🎯Fix auth│                    │ SurrealDB│◄──────┐                      │
│      │  quest   │                    │          │       │                      │
│      └──────────┘                    └──────────┘       │                      │
│                                                   ┌──────────┐                  │
│                                                   │ Change   │                  │
│                                                   │ Feeds    │                  │
│                                                   └──────────┘                  │
│                                                                                 │
│  Legend: 📚 Note  🎯 Quest  📦 Item  🏷️ Tag                                    │
│                                                                                 │
│  [Zoom: ────●────] [Reset View]                                                │
└─────────────────────────────────────────────────────────────────────────────────┘
```

**View Options:**

- **Local** — Current note's connections
- **Global** — All notes and relationships

**Node Types:**

- 📚 Notes (blue)
- 🎯 Quests (green)
- 📦 Items (orange)
- 🏷️ Tags (gray)

**Interactions:**

- Drag to move nodes
- Scroll to zoom
- Click node → open entity
- Layout persists between sessions

---

### K-4: Semantic Search

**Trigger:** User searches with `~` prefix or toggle

**Keyboard:** `Cmd+K`

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  Search Notes                                                          Cmd+K   │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │ ~how to handle user authentication                                       │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
│                                                                                 │
│  Mode: [Keyword] [Semantic] [●Hybrid]                                          │
│                                                                                 │
│  Results (semantic match):                                                      │
│  ─────────────────────────────────────────────────────────────────────────────  │
│                                                                                 │
│  📚 Authentication Architecture                              98% match         │
│     "OAuth 2.0 flow with PKCE for secure login..."                            │
│                                                                                 │
│  📚 Session Management                                       92% match         │
│     "JWT tokens with refresh rotation..."                                      │
│                                                                                 │
│  📚 Security Best Practices                                  85% match         │
│     "Password hashing with Argon2..."                                         │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

**Search Modes:**

| Mode | Behavior |
|------|----------|
| **Keyword** | Traditional BM25 text search |
| **Semantic** | Vector embedding similarity |
| **Hybrid** | Combined (default), best of both |

**Syntax:**

- `~query` — Force semantic mode
- `in:note` — Filter to notes
- `#tag` — Filter by tag

---

## 📦 Tracking (Inventory)

### T-1: Add Item

**Trigger:** User wants to track new item

**Keyboard:** `Cmd+N`

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  Add Item                                                                    ×  │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Name                                                                           │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │ Raspberry Pi 4 Model B 8GB                                               │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
│                                                                                 │
│  Quantity        Category                                                       │
│  ┌────────┐      ┌──────────────────────────────────┐                          │
│  │ [3]    │      │ Electronics ▾                    │                          │
│  └────────┘      └──────────────────────────────────┘                          │
│                                                                                 │
│  Location                                                                       │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │ 🔍 Office > Shelf                                                        │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
│                                                                                 │
│            [Create Item]    [More Options...]                                   │
└─────────────────────────────────────────────────────────────────────────────────┘
```

**More Options:**

- Description
- Photo (camera or gallery)
- Tags
- Custom fields
- QR/barcode generation
- Link to notes

---

### T-2: Reserve Item for Quest

**Trigger:** Quest requires specific items

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  Reserve Items for Quest                                                        │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Quest: Build Home Lab Cluster                                                  │
│                                                                                 │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │ 🔍 Search items...                                                       │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
│                                                                                 │
│  Selected for reservation:                                                      │
│  ─────────────────────────────────────────────────────────────────────────────  │
│                                                                                 │
│  📦 Raspberry Pi 4                    [2] of 3 available              [Remove] │
│  📦 32GB SD Card                      [2] of 5 available              [Remove] │
│  📦 USB-C Power Supply                [2] of 4 available              [Remove] │
│                                                                                 │
│  ─────────────────────────────────────────────────────────────────────────────  │
│                                                                                 │
│            [Create Reservations]                                               │
└─────────────────────────────────────────────────────────────────────────────────┘
```

**Flow:**

1. From Quest → Link Items, or from Item → Reserve
2. Search and select items
3. Specify quantity to reserve
4. Create reservations
5. Item status updates (reserved)
6. Quest shows required items

**Rules:**

- Reservations are pending until quest In Progress
- Auto-released when quest completed
- Warning if trying to reserve unavailable items

---

### T-3: Auto-Detect BoM (Bill of Materials)

**Trigger:** User writes note mentioning inventory items

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  Note: Home Lab Build Plan                                                      │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ## Parts List                                                                  │
│                                                                                 │
│  For this project I'll need:                                                   │
│  - 2x Raspberry Pi 4                                                           │
│  - 2x 32GB SD cards                                                            │
│  - 1x network switch                                                           │
│  - 1x USB hub                                                                  │
│                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │ 📦 Found in inventory:                                                  │   │
│  │                                                                         │   │
│  │   2x Raspberry Pi 4          ✓ 3 available                             │   │
│  │   2x 32GB SD Card            ✓ 5 available                             │   │
│  │   1x Network Switch          ✗ 0 available (need to acquire)           │   │
│  │   1x USB Hub                 ✓ 2 available                             │   │
│  │                                                                         │   │
│  │   [Create Reservations]  [Add Missing Items]  [Dismiss]                │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

**Flow:**

1. User writes note with item mentions
2. System detects quantities and item names
3. Non-intrusive popup shows matches
4. User can:
   - Create reservations for available items
   - Add missing items to inventory (quantity 0)
   - Dismiss suggestion

**Rules:**

- Real-time detection (debounced)
- Pattern matching for quantities (2x, x2, two)
- Fuzzy matching to inventory
- Context-aware (RPi = Raspberry Pi)

---

### T-4: Maintenance Reminder

**Trigger:** Scheduled maintenance due

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  🔧 Maintenance Due                                                             │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  3D Printer - Bed Leveling                                                      │
│                                                                                 │
│  Due: Today                                                                     │
│  Last performed: 30 days ago                                                    │
│  Interval: Every 30 days                                                        │
│                                                                                 │
│  Notes: Level all four corners, then center. Use paper test.                   │
│                                                                                 │
│            [Mark Complete]    [Snooze 7 days]    [Skip]                        │
└─────────────────────────────────────────────────────────────────────────────────┘
```

**Flow:**

1. Notification appears when maintenance due
2. User can mark complete (updates schedule)
3. Or snooze for later
4. Dashboard shows upcoming maintenance

---

## 🔗 Cross-App Flows

### X-1: Global Search

**Trigger:** User looking for something across all apps

**Keyboard:** `Cmd+Space` (global)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  Search Everything                                                    Cmd+Space │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │ 🔍 raspberry                                                             │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
│                                                                                 │
│  🎯 Quests                                                                      │
│     Build Raspberry Pi cluster                    ⚡⚡⚡⚡ Large                  │
│     Set up Pi-hole                                ⚡⚡⚡ Medium                   │
│                                                                                 │
│  📚 Notes                                                                       │
│     Raspberry Pi GPIO Pinout                                                   │
│     RPi Network Configuration                                                  │
│     Home Lab Architecture                                                      │
│                                                                                 │
│  📦 Items                                                                       │
│     Raspberry Pi 4 Model B                        Qty: 3 • Office > Shelf      │
│     Raspberry Pi Zero W                           Qty: 2 • Office > Drawer     │
│                                                                                 │
│  [in:quest] [in:note] [in:item] [#tag]                                        │
└─────────────────────────────────────────────────────────────────────────────────┘
```

**Syntax:**

- `in:quest` — Filter to quests
- `in:note` — Filter to notes
- `in:item` — Filter to items
- `#tag` — Filter by tag
- `~semantic query` — Semantic search

---

### X-2: Gamification Dashboard

**Trigger:** User views profile or level indicator

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  Your Progress                                                                  │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │                                                                         │   │
│  │      Level 5: Expert                                                   │   │
│  │      ════════════════════════════════════════                          │   │
│  │      ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░  1,385 / 2,000 XP             │   │
│  │                                                                         │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  🔥 Streaks                                                                     │
│  ├── Daily Check-in: 14 days (🔥 longest: 21)                                  │
│  ├── Quest Completion: 7 days                                                  │
│  └── Focus Sessions: 5 days                                                    │
│                                                                                 │
│  🏆 Recent Achievements                                                         │
│  ├── 🧘 Focus Master (10 focus sessions)                     Nov 28           │
│  ├── 📚 Knowledge Seeker (50 notes)                          Nov 25           │
│  └── 🔥 Week Warrior (7-day streak)                          Nov 22           │
│                                                                                 │
│  📊 This Week                                                                   │
│  ├── Quests completed: 12                                                      │
│  ├── XP earned: 485                                                            │
│  ├── Focus time: 4h 20m                                                        │
│  └── Notes created: 8                                                          │
│                                                                                 │
│  [View All Achievements]  [Customize Gamification]                             │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## ⚙️ Settings Flows

### S-1: Configure Energy Check-in

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  Energy Check-in Settings                                                       │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Daily Reminder                                                                 │
│  ☑ Enable daily energy check-in prompt                                         │
│  Time: [9:00 AM ▾]                                                             │
│                                                                                 │
│  Default Energy Filter                                                          │
│  When no check-in today, default to: [Medium ▾]                                │
│                                                                                 │
│  Pattern Learning                                                               │
│  ☑ Show energy patterns in check-in                                           │
│  ☑ Suggest based on day of week                                               │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### S-2: Configure Gamification

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  Gamification Settings                                                          │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ☑ Enable gamification features                                                │
│                                                                                 │
│  Visible Elements                                                               │
│  ☑ XP and level indicator                                                      │
│  ☑ Streak counters                                                             │
│  ☑ Achievement notifications                                                   │
│  ☐ Leaderboard (future: if sharing enabled)                                   │
│                                                                                 │
│  Celebration Intensity                                                          │
│  [Subtle ───●─── Festive]                                                      │
│                                                                                 │
│  Streak Grace Period                                                            │
│  Hours before streak breaks: [24 ▾]                                            │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Flow Summary

### Guidance
| Flow | Hotkey | Priority |
|------|--------|----------|
| G-1: QBA Board Overview | — | Essential |
| G-2: Quick Add Quest | `Cmd+N` | Essential |
| G-3: Daily Energy Check-in | — | Essential |
| G-4: Focus Mode | `Cmd+Shift+F` | Essential |
| G-5: Complete Quest | `Cmd+Enter` | Essential |
| G-6: Weekly Harvest | — | Important |
| G-7: Quest Dependencies | — | Important |

### Knowledge
| Flow | Hotkey | Priority |
|------|--------|----------|
| K-1: Daily Note | — | Essential |
| K-2: Wiki-Links | `[[` | Essential |
| K-3: Mind Map/Graph | `Cmd+G` | Important |
| K-4: Semantic Search | `Cmd+K` | Essential |

### Tracking
| Flow | Hotkey | Priority |
|------|--------|----------|
| T-1: Add Item | `Cmd+N` | Essential |
| T-2: Reserve for Quest | `Cmd+L` | Important |
| T-3: Auto-Detect BoM | — | Important |
| T-4: Maintenance | — | Nice-to-have |

### Quick Capture
| Flow | Hotkey | Priority |
|------|--------|----------|
| QC-1: Capture | `Cmd+Shift+C` | Essential |
| QC-2: Review | — | Essential |

### Cross-App
| Flow | Hotkey | Priority |
|------|--------|----------|
| X-1: Global Search | `Cmd+Space` | Essential |
| X-2: Gamification | — | Important |
