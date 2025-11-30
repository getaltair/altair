# Altair User Flows

> **Core user workflows** for Guidance, Knowledge, Tracking, and Quick Capture

---

## Quick Reference

| App | Primary Flows | Key Interaction |
|-----|---------------|-----------------|
| **Guidance** | Quest lifecycle, daily planning | Energy-based task selection |
| **Knowledge** | Note creation, wiki-linking | `[[wiki-links]]` while typing |
| **Tracking** | Item management, location | Quantity adjustments |
| **Quick Capture** | Capture → classify | One tap in, batch review out |

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
│  [📸 Photo] [🎤 Voice] [✏️ Text]   │
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
   - Clicks Voice → records audio
4. Capture saved with `status: pending`
5. Window closes immediately
6. Badge updates on all apps: "1 pending capture"

**Rules:**

- No destination selection at capture time
- No required fields except content
- Auto-captures: timestamp, location (if enabled), source app
- Window closes on capture — user returns to what they were doing

---

### QC-2: Review Pending Captures

**Trigger:** User opens any app with pending captures, or scheduled review time

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
│  │         │ │         │ │         │ │         │   │
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

**Keyboard shortcuts:**

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

### G-1: Quick Add Quest

**Trigger:** User thinks of something they need to do

**Goal:** Capture task with minimal friction, add details later

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
│  Energy    ○ Low   ● Medium   ○ High   ○ Variable  │
│                                                     │
│  Campaign  [None ▾]                                 │
│                                                     │
│            [Create Quest ↵]    [More Options...]    │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Flow:**

1. User triggers Quick Add (`Cmd+N`, `+` button, or hotkey)
2. Modal appears with title focused
3. User types title
4. User optionally selects energy level (defaults to Medium)
5. User optionally assigns to campaign
6. User presses Enter or clicks Create
7. Quest created with `status: backlog`
8. Modal closes, quest appears in list

**More Options expands to:**

- Description (markdown)
- Due date
- Priority (0-100 slider)
- Estimated time
- Tags
- Link notes/items

---

### G-2: Daily Planning (Energy-Based)

**Trigger:** Start of day or planning session

**Goal:** Select quests based on current energy level

```
┌─────────────────────────────────────────────────────┐
│  Today's Quests                    Energy: [Med ▾]  │
├─────────────────────────────────────────────────────┤
│                                                     │
│  How's your energy today?                           │
│  [😴 Low] [😊 Medium] [🔥 High]                     │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  Suggested for Medium Energy:              3 quests │
│                                                     │
│  ☐ Fix authentication bug             ⚡ Med  45m   │
│  ☐ Review PR from teammate            ⚡ Med  30m   │
│  ☐ Write unit tests for sync          ⚡ Med  60m   │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  Also available:                                    │
│  ☐ Refactor database schema           ⚡ High 120m  │
│  ☐ Organize desktop files             ⚡ Low  20m   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Flow:**

1. User opens Guidance or navigates to "Today" view
2. Energy selector shown (remembers last selection)
3. User selects current energy level
4. Quests filtered and sorted by:
   - Energy match (primary)
   - Priority (secondary)
   - Due date (tertiary)
5. User checks quests to commit to today
6. Checked quests move to "Active" status

**Rules:**

- Selecting "Low" shows only Low energy quests
- Selecting "Medium" shows Low + Medium
- Selecting "High" shows all
- Time estimates help user gauge capacity
- "Also available" shows other energy levels collapsed

---

### G-3: Complete Quest

**Trigger:** User finishes a task

**Goal:** Mark complete with optional reflection

```
┌─────────────────────────────────────────────────────┐
│  ✓ Quest Completed!                                 │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Fix authentication bug                             │
│  ⏱ Estimated: 45m  •  Actual: 52m                   │
│                                                     │
│  Quick note (optional):                             │
│  ┌───────────────────────────────────────────────┐  │
│  │ Root cause was session timeout config         │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│  ☐ Create follow-up quest                          │
│  ☐ Link to note                                    │
│                                                     │
│            [Done ↵]              [Just Complete]    │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Flow:**

1. User clicks checkbox or completion button on quest
2. Completion modal appears (can be disabled in settings)
3. User optionally:
   - Adds completion note
   - Creates follow-up quest
   - Links to existing/new note
4. User clicks Done or Just Complete
5. Quest moves to `status: completed`
6. Celebration moment (subtle animation)

**Quick complete:** If user holds Shift while clicking checkbox, skips modal entirely.

---

### G-4: Manage Campaigns

**Trigger:** User wants to group related quests

**Goal:** Create/organize quest containers

```
┌─────────────────────────────────────────────────────┐
│  Campaigns                                     [+]  │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ● Active                                           │
│    ├── 🟣 Altair Development         12 quests     │
│    ├── 🟢 Q4 Goals                    5 quests     │
│    └── 🔵 Home Renovation             8 quests     │
│                                                     │
│  ○ Completed                                        │
│    └── 🟡 Website Redesign            0 active     │
│                                                     │
│  ○ Archived                                         │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Flow — Create Campaign:**

1. User clicks `+` in Campaigns view
2. Modal: Title, Color, Description (optional)
3. Campaign created as `active`
4. User can drag quests into campaign or assign via quest edit

**Flow — Complete Campaign:**

1. User clicks campaign → "Mark Complete"
2. Prompt: "Archive X remaining quests?"
3. Campaign moves to Completed section
4. Can still view quests within

---

### G-5: Link Quest to Note/Item

**Trigger:** Quest relates to documentation or requires materials

**Goal:** Create cross-app references

```
┌─────────────────────────────────────────────────────┐
│  Link to Quest: Fix authentication bug              │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Search notes and items...                          │
│  ┌───────────────────────────────────────────────┐  │
│  │ auth                                          │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│  📚 Notes                                           │
│    ├── Authentication Architecture     ☐ Link      │
│    └── Login Flow Documentation        ☐ Link      │
│                                                     │
│  📦 Items                                           │
│    └── YubiKey Security Key            ☐ Requires  │
│                                                     │
│  [+ Create New Note]    [+ Create New Item]         │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Flow:**

1. User opens quest → clicks "Link"
2. Search modal appears
3. User types to filter notes/items
4. User checks items to link:
   - Notes: `references` relationship
   - Items: `requires` relationship
5. Links created, visible in quest detail view

---

## 📚 Knowledge (PKM)

### K-1: Create Note

**Trigger:** User wants to capture knowledge

**Goal:** Create note with minimal friction, rich editing

```
┌─────────────────────────────────────────────────────┐
│  ← Back                              📁 Inbox   ⋮   │
├─────────────────────────────────────────────────────┤
│                                                     │
│  # Authentication Flow Design                       │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  The auth system uses JWT tokens with refresh       │
│  rotation. See [[Session Management]] for details.  │
│                                                     │
│  ## Components                                      │
│                                                     │
│  - Auth Provider Plugin                             │
│  - Token Store                                      │
│  - Session Manager                                  │
│                                                     │
│  Related: [[API Security]], [[OAuth Setup]]         │
│                                                     │
│                                                     │
├─────────────────────────────────────────────────────┤
│  Tags: #architecture #security                      │
└─────────────────────────────────────────────────────┘
```

**Flow:**

1. User triggers New Note (`Cmd+N`, `+` button)
2. Editor opens with title focused
3. User types title, presses Enter
4. Cursor moves to body
5. User writes in markdown
6. Auto-save triggers on pause (debounced 1s)
7. Embedding generates in background

**Wiki-link flow:**

1. User types `[[`
2. Autocomplete dropdown appears
3. User types to filter existing notes
4. User selects note or presses Enter to create new
5. Link inserted: `[[Note Title]]`

---

### K-2: Navigate Wiki-Links

**Trigger:** User sees `[[linked note]]` and wants to follow

**Goal:** Seamless navigation between connected notes

**Flow:**

1. User clicks wiki-link in note
2. If note exists: Opens linked note
3. If note doesn't exist: Prompt to create
4. Backlinks panel shows "Referenced by: X notes"

**Keyboard:** `Cmd+Click` opens in split view

---

### K-3: Search Notes

**Trigger:** User looking for specific knowledge

**Goal:** Find notes by keyword or semantic meaning

```
┌─────────────────────────────────────────────────────┐
│  Search Knowledge                                   │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌───────────────────────────────────────────────┐  │
│  │ 🔍 authentication token refresh               │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│  ○ Keyword   ● Semantic   ○ Both                   │
│                                                     │
│  Results (5)                                        │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  📚 Session Management                    92% match │
│     "...refresh tokens are rotated on each..."      │
│                                                     │
│  📚 Authentication Architecture           87% match │
│     "...JWT tokens with configurable expiry..."     │
│                                                     │
│  📚 API Security                          71% match │
│     "...validates token signature before..."        │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Flow:**

1. User triggers search (`Cmd+K` or `/`)
2. Search bar appears/focuses
3. User types query
4. Results update in real-time (debounced)
5. User clicks result or uses arrow keys + Enter
6. Note opens

**Search modes:**

- **Keyword:** Traditional BM25 text matching
- **Semantic:** Vector similarity (embeddings)
- **Both:** Hybrid with rank fusion (default)

---

### K-4: Organize with Folders

**Trigger:** User wants to structure their knowledge

**Goal:** Move notes into hierarchical folders

```
┌─────────────────────────────────────────────────────┐
│  Folders                                       [+]  │
├─────────────────────────────────────────────────────┤
│                                                     │
│  📁 Inbox                                  12       │
│  📁 Projects                               ▶        │
│     ├── 📁 Altair                          24       │
│     └── 📁 Work                            18       │
│  📁 Areas                                  ▶        │
│     ├── 📁 Health                           8       │
│     └── 📁 Finance                          5       │
│  📁 Resources                              31       │
│  📁 Archive                                ▶        │
│                                                     │
│  ─────────────────────────────────────────────────  │
│  📝 Unfiled Notes                           3       │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Flow — Move Note:**

1. User opens note or selects in list
2. User clicks folder icon or drags note
3. Folder picker appears
4. User selects destination folder
5. Note moved, UI updates

**Flow — Create Folder:**

1. User clicks `+` in folder panel
2. User types folder name
3. User optionally selects parent folder
4. Folder created

**Rules:**

- Notes can exist without folders (shown in "Unfiled")
- Folders can nest (no arbitrary limit, recommend ≤3 deep)
- Deleting folder moves notes to parent (configurable)

---

### K-5: Link Note to Item

**Trigger:** Note documents a physical item

**Goal:** Create `documents` relationship

```
┌─────────────────────────────────────────────────────┐
│  Link Note to Item                                  │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Note: YubiKey Setup Guide                          │
│                                                     │
│  Search items...                                    │
│  ┌───────────────────────────────────────────────┐  │
│  │ yubikey                                       │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│  📦 Items                                           │
│    └── YubiKey 5 NFC                   ☐ Documents │
│                                                     │
│  [+ Create New Item]                                │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Flow:**

1. User opens note → clicks "Link Item"
2. Search modal appears
3. User finds/creates item
4. `documents` edge created
5. Item shows "Documented by: Note X"
6. Note shows "Documents: Item Y"

---

## 📦 Tracking (Inventory)

### T-1: Add Item

**Trigger:** User acquires something or wants to track existing item

**Goal:** Record item with location and quantity

```
┌─────────────────────────────────────────────────────┐
│  New Item                                        ×  │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Name                                               │
│  ┌───────────────────────────────────────────────┐  │
│  │ USB-C to HDMI Adapter                         │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│  Quantity        Category                           │
│  ┌─────────┐     ┌────────────────────────────────┐ │
│  │    1    │     │ Electronics               ▾   │ │
│  └─────────┘     └────────────────────────────────┘ │
│                                                     │
│  Location                                           │
│  ┌───────────────────────────────────────────────┐  │
│  │ Office > Desk > Top Drawer               ▾   │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│            [Create Item ↵]       [More Options...]  │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Flow:**

1. User triggers Add Item (`Cmd+N`, `+` button)
2. Modal appears with name focused
3. User enters name
4. User sets quantity (defaults to 1)
5. User selects category (optional)
6. User selects location (optional, searchable tree)
7. User clicks Create
8. Item saved, appears in list

**More Options:**

- Description
- Photo (attachment)
- Tags
- Link to notes
- Barcode/SKU

---

### T-2: Adjust Quantity

**Trigger:** User uses/acquires more of an item

**Goal:** Update quantity with optional note

```
┌─────────────────────────────────────────────────────┐
│  USB-C to HDMI Adapter                              │
├─────────────────────────────────────────────────────┤
│                                                     │
│              ┌─────────────────┐                    │
│      [-]     │       3         │     [+]            │
│              └─────────────────┘                    │
│                                                     │
│  Quick adjust:  [-5] [-1]  [+1] [+5]               │
│                                                     │
│  Note: ________________________________            │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Flow:**

1. User opens item or clicks quantity in list
2. Quantity adjuster appears
3. User clicks +/- or types number
4. User optionally adds note ("Gave one to Alex")
5. Quantity updated
6. History logged (not shown to user by default)

**Rules:**

- Quantity minimum is 0
- At 0, item remains (not deleted)
- Optional low-stock alert threshold

---

### T-3: Move Item

**Trigger:** User relocates a physical item

**Goal:** Update item's location

**Flow:**

1. User opens item → clicks location
2. Location tree picker appears
3. User navigates/searches for new location
4. User selects location
5. Location updated

**Alternate:** Drag item to location in sidebar

---

### T-4: Manage Locations

**Trigger:** User needs to define where things are stored

**Goal:** Create/organize location hierarchy

```
┌─────────────────────────────────────────────────────┐
│  Locations                                     [+]  │
├─────────────────────────────────────────────────────┤
│                                                     │
│  📍 Home                                            │
│     ├── 📍 Kitchen                                  │
│     │   ├── 📍 Pantry                    8 items   │
│     │   └── 📍 Fridge                    12 items  │
│     ├── 📍 Office                                   │
│     │   ├── 📍 Desk                                │
│     │   │   ├── 📍 Top Drawer            5 items   │
│     │   │   └── 📍 Bottom Drawer         3 items   │
│     │   └── 📍 Shelf                     14 items  │
│     └── 📍 Garage                                   │
│         └── 📍 Toolbox                   22 items  │
│                                                     │
│  📍 Work                                            │
│     └── 📍 Desk                          4 items   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Flow — Create Location:**

1. User clicks `+` or right-clicks parent location
2. Modal: Name, Parent (pre-filled if right-clicked), Geo (optional)
3. Location created

**Flow — Delete Location:**

1. User right-clicks location → Delete
2. Prompt: "Move X items to parent location?"
3. Options: Move to parent / Move to no location / Cancel
4. Location deleted, items reassigned

---

### T-5: Search Items

**Trigger:** User looking for something

**Goal:** Find items by name, category, or location

```
┌─────────────────────────────────────────────────────┐
│  Search Inventory                                   │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌───────────────────────────────────────────────┐  │
│  │ 🔍 hdmi                                       │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│  Filters: [Category ▾] [Location ▾] [In Stock ▾]   │
│                                                     │
│  Results (3)                                        │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  📦 USB-C to HDMI Adapter              Qty: 3      │
│     📍 Office > Desk > Top Drawer                  │
│                                                     │
│  📦 HDMI Cable 6ft                     Qty: 2      │
│     📍 Office > Shelf                              │
│                                                     │
│  📦 HDMI Cable 10ft                    Qty: 1      │
│     📍 Garage > Toolbox                            │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Flow:**

1. User triggers search (`Cmd+K` or `/`)
2. Search bar focuses
3. User types query
4. Results filter in real-time
5. User optionally applies filters
6. User clicks result to open item

---

## 🔗 Cross-App Flows

### X-1: Global Search

**Trigger:** User looking for something, doesn't know which app

**Goal:** Search across all domains

```
┌─────────────────────────────────────────────────────┐
│  Search Everything                       Cmd+Space  │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌───────────────────────────────────────────────┐  │
│  │ 🔍 authentication                             │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│  🎯 Quests                                          │
│     Fix authentication bug                          │
│     Implement OAuth flow                            │
│                                                     │
│  📚 Notes                                           │
│     Authentication Architecture                     │
│     Session Management                              │
│     OAuth Setup Guide                               │
│                                                     │
│  📦 Items                                           │
│     YubiKey 5 NFC                                  │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Flow:**

1. User triggers global search (hotkey from anywhere)
2. Omnibar appears
3. User types query
4. Results grouped by domain
5. User selects result → opens in appropriate app

**Shortcuts:**

- `in:quest` — Filter to quests only
- `in:note` — Filter to notes only
- `in:item` — Filter to items only
- `#tag` — Filter by tag

---

### X-2: View Linked Entities

**Trigger:** User viewing entity with cross-app links

**Goal:** See and navigate relationships

```
┌─────────────────────────────────────────────────────┐
│  Quest: Fix authentication bug                      │
├─────────────────────────────────────────────────────┤
│                                                     │
│  [...quest details...]                              │
│                                                     │
│  ─────────────────────────────────────────────────  │
│                                                     │
│  🔗 Linked                                          │
│                                                     │
│  📚 References (2 notes)                            │
│     ├── Authentication Architecture      →         │
│     └── Session Management               →         │
│                                                     │
│  📦 Requires (1 item)                               │
│     └── YubiKey 5 NFC                    →         │
│                                                     │
│  [+ Link Note]  [+ Link Item]                       │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Flow:**

1. User views quest/note/item with links
2. Linked section shows relationships grouped by type
3. User clicks `→` to navigate to linked entity
4. Opens in appropriate app/view

---

### X-3: Quick Capture from Any App

**Trigger:** User in any app, wants to capture without leaving

**Goal:** Capture without context switch

**Flow:**

1. User presses capture hotkey (e.g., `Cmd+Shift+C`)
2. Quick Capture overlay appears
3. User types/records
4. Presses Enter
5. Overlay closes, user remains in current app
6. Badge updates across all apps

---

## ⚙️ Settings Flows

### S-1: Configure Location Privacy

**Trigger:** User wants control over location tracking

```
┌─────────────────────────────────────────────────────┐
│  Location Settings                                  │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Auto-tag location on:                              │
│  ☐ Notes                                           │
│  ☐ Captures                                        │
│  ☑ Items (manual only)                             │
│                                                     │
│  Precision:                                         │
│  ○ City          (e.g., "Houston")                 │
│  ● Neighborhood  (e.g., "Clear Lake")              │
│  ○ Exact         (GPS coordinates)                 │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

### S-2: Configure AI Features

**Trigger:** User wants to enable/disable AI assistance

```
┌─────────────────────────────────────────────────────┐
│  AI Settings                                        │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Capture Classification                             │
│  ☑ Enable AI suggestions                           │
│     Provider: [Claude ▾]                           │
│                                                     │
│  Semantic Search                                    │
│  ☑ Enable (uses local embeddings)                  │
│                                                     │
│  Rate Limits                                        │
│     Daily requests: [100    ]                      │
│     Daily cost cap: [$5.00  ]                      │
│                                                     │
│  Providers                                          │
│  ├── claude     [Configure...]    ● Connected      │
│  ├── openai     [Configure...]    ○ Not configured │
│  └── ollama     [Configure...]    ○ Not configured │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## Flow Summary by App

### Guidance (Quest Management)

| Flow | Hotkey | Priority |
|------|--------|----------|
| G-1: Quick Add Quest | `Cmd+N` | Essential |
| G-2: Daily Planning | — | Essential |
| G-3: Complete Quest | `Cmd+Enter` | Essential |
| G-4: Manage Campaigns | — | Important |
| G-5: Link to Note/Item | `Cmd+L` | Important |

### Knowledge (PKM)

| Flow | Hotkey | Priority |
|------|--------|----------|
| K-1: Create Note | `Cmd+N` | Essential |
| K-2: Navigate Wiki-Links | `Click` / `Cmd+Click` | Essential |
| K-3: Search Notes | `Cmd+K` | Essential |
| K-4: Organize Folders | — | Important |
| K-5: Link to Item | `Cmd+L` | Nice-to-have |

### Tracking (Inventory)

| Flow | Hotkey | Priority |
|------|--------|----------|
| T-1: Add Item | `Cmd+N` | Essential |
| T-2: Adjust Quantity | — | Essential |
| T-3: Move Item | — | Important |
| T-4: Manage Locations | — | Important |
| T-5: Search Items | `Cmd+K` | Essential |

### Quick Capture

| Flow | Hotkey | Priority |
|------|--------|----------|
| QC-1: Capture Something | `Cmd+Shift+C` | Essential |
| QC-2: Review Captures | — | Essential |

### Cross-App

| Flow | Hotkey | Priority |
|------|--------|----------|
| X-1: Global Search | `Cmd+Space` | Essential |
| X-2: View Linked Entities | — | Important |
| X-3: Quick Capture Overlay | `Cmd+Shift+C` | Essential |
