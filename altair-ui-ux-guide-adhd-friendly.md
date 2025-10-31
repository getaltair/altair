# Altair UI/UX Design Guide

## ADHD-Focused Local-First Productivity Apps

> **Your Mission:** Build a bold, accessible, lightning-fast productivity suite that ADHD users actually use

---

## 🎯 TL;DR - The Winning Formula

**Your competitive advantage:**

- **ADHD-optimized patterns** = Progressive disclosure + AI task breakdown + multiple views
- **Local-first speed** = <100ms response, works offline, data sovereignty
- **Functional neo-brutalism** = Bold clarity without chaos, strategic accent not immersion
- **Flutter consistency** = Seamless cross-platform with Material Design 3

**Bottom line:** No competitor combines all these elements with deep ADHD understanding.

---

## 📊 Quick Decision Matrix

| Question | Answer | Why |
|----------|--------|-----|
| Pure neo-brutalism? | ❌ No | Zero successful productivity apps use it - too intense |
| Neo-brutalist accents? | ✅ Yes | 2-4px borders, bold colors, strategic shadows |
| AI task breakdown? | ✅ Critical | #1 ADHD feature differentiator (Goblin Tools model) |
| Local-first mandatory? | ✅ Yes | Instant response = competitive advantage |
| Multiple views? | ✅ Yes | ADHD users switch based on cognitive state |
| WCAG 2.2 Level AA? | ✅ Must-have | 83.6% of sites fail contrast - be different |

---

## ⚡ Priority Roadmap

### Quick Wins (Weeks 1-6)

**Week 1-2: Foundation Accessibility**

- ✅ Keyboard navigation (Tab, Enter, Escape)
- ✅ 2px minimum focus indicators (3:1 contrast)
- ✅ Skip links ("Skip to main content")
- ✅ `prefers-reduced-motion` support
- ✅ Test with keyboard only

**Week 2-3: Neo-Brutalist Design System**

- ✅ ColorScheme.fromSeed() with soft blue (#6B9BD1)
- ✅ Verify 4.5:1 contrast (normal text), 3:1 (large text)
- ✅ Borders: 2-4px solid (NOT 8px)
- ✅ Shadows: 4px offset, black 60-80% opacity
- ✅ Whitespace: 24-32px padding between sections

**Week 3-4: Sync Status Visibility**

- ✅ Status bar at top (green=synced, blue=syncing, orange=pending)
- ✅ Document-level badges on tasks/nodes/items
- ✅ Settings panel with sync history
- ✅ "Sync Now" manual button

**Week 4-5: Command Palette**

- ✅ CMD/CTRL+K universal navigation
- ✅ Task creation, view switching, navigation
- ✅ Recent items (last 5)
- ✅ Keyboard shortcuts visible

**Week 5-6: Responsive Design**

- ✅ Mobile (0-599dp): bottom nav, single column
- ✅ Tablet (840-1239dp): two columns, optional side nav
- ✅ Desktop (1240+dp): multi-column, permanent nav
- ✅ Test on real devices

---

### Medium-Term (Months 2-4)

**Month 2: AI Task Decomposition**

What it does: Breaks overwhelming tasks into bite-sized subtasks

**Implementation:**

1. User inputs: "Plan team retreat"
2. Selects spicy level: 1-5 (granularity slider)
3. AI breaks down:
   - Set date
   - Choose venue
   - Plan activities
   - Send invitations
   - Arrange catering
4. Each subtask supports further breakdown

**Critical ADHD benefits:**

- Combats executive dysfunction
- Reduces task initiation paralysis
- Externalizes complexity without shame

**Tech stack:** FastAPI backend + GPT-4/Claude API

---

**Month 2: Multiple View Options**

| View | Best For | ADHD Benefit |
|------|----------|--------------|
| **List** | Quick entry, simple tasks | Low cognitive load |
| **Board** | Visual organization | Spatial thinking |
| **Timeline** | Time-sensitive work | Combats time blindness |

**Implementation:**

- View switcher: top-right, keyboard shortcuts (CMD+1/2/3)
- Persist per project (Marketing→board, Personal→list)
- Knowledge app: emphasize graph view with filters

---

**Month 2-3: Progressive Disclosure**

Break complex interfaces into manageable chunks:

**Before (❌):**

- One overwhelming screen with 20 fields
- Users abandon at Step 1

**After (✅):**

- Step 1 of 4: Title, project, due date
- "Show more options" expands to tags, priority, assignees
- Visual progress bar
- Save and return later

**Key pattern:** Each "next" click = micro-dopamine boost

---

**Month 3: Comprehensive Customization**

**Density controls:**

- Compact / Comfortable / Spacious

**Theme system:**

- Light / Dark / High Contrast / Custom

**Typography:**

- Font family (Inter, Roboto, OpenDyslexic)
- Size scaling (100-200%, 10% increments)
- Line height adjustment

**Preset modes:**

- 🧠 **Dyslexia Mode**: OpenDyslexic, 1.8 line spacing, 0.35ch letter spacing
- 🎯 **Focus Mode**: Hides non-essential UI, enables distraction blocking
- 😌 **Calm Mode**: Pastel colors, reduced contrast, minimal animations

**Critical:** Settings sync via SurrealDB across devices

---

**Month 3-4: Optional Gamification**

> 💡 **Tip:** Make it optional - some ADHD users find gamification stressful

**What works:**

- ✅ Streak counters (7-day completion 🔥)
- ✅ XP/points (small=10, medium=25, large=50)
- ✅ Progress trees (10 tasks → unlock customization)
- ✅ Celebration animations (confetti on completion)
- ✅ Surprise encouragement (variable-ratio rewards)

**What to avoid:**

- ❌ Punitive loss (losing streaks = failure feeling)
- ❌ Mandatory leaderboards (demotivating comparison)
- ❌ Complex systems (overwhelming)

**Failure messaging:** "Start a new streak today!" NOT "Streak lost"

---

### Long-Term (Months 4-12)

**Month 4-6: CRDT Implementation**

**What it solves:**

- Automatic merging of concurrent changes
- Eliminates most conflict scenarios
- Enables real-time collaboration

**Per-app strategy:**

| App | CRDT Type | Use Case |
|-----|-----------|----------|
| Guidance | Registers | Task properties (title, due date) |
| Guidance | Counters | Completion status |
| Knowledge | Text CRDT | Collaborative note editing |
| Tracking | Sets | Tags, categories |
| Tracking | Registers | Location, status |

**Libraries:** Automerge or Yjs

**Conflict resolution:** Only needed when two users modify same property simultaneously

---

**Month 5-7: Intelligent Notifications**

> ⚠️ **Warning:** ADHD users filter out 50+ daily pings - quality over quantity

**Context-aware triggers:**

- 📍 Location: "Buy milk" when near store
- ⏰ Time-of-day: Morning reminders for morning tasks
- 📅 Calendar: Before meetings, during commute
- ✅ Activity: After completing related task

**Progressive urgency:**

1. First: Gentle notification
2. Second (15 min): More persistent
3. Third (critical only): Full-screen modal

**Anti-habituation:**

- Rotate notification sounds monthly
- Vary format (sound/vibration/visual)
- Random motivational quotes
- Surprise timing (not exact same minute)

**Smart grouping:** Bundle non-urgent into daily review

---

**Month 6-8: Knowledge Graph Intelligence**

**AI-powered features:**

- **Auto-linking**: Suggests connections based on content similarity
- **Smart tagging**: NLP suggests relevant tags
- **Semantic search**: Understands intent ("find all accessibility notes")
- **Auto-summarization**: Executive summaries of long notes
- **Related concepts**: "Others also referenced..."

**Tana-style supertags:**

- Define object types: Book, Person, Project
- Auto-populate fields (Book → Author, Year, Genre)
- Reduces manual overhead

---

**Month 8-10: Advanced Focus Mode**

**Three-tier system:**

**Level 1: Gentle Guidance**

- Shows current goal in corner
- Easy one-click override
- Tracks time in session

**Level 2: Active Blocking**

- Blocks distracting apps/websites
- Visible countdown timer
- Requires typing full task name to override

**Level 3: Full Immersion**

- Full-screen mode
- ONLY current task + subtasks visible
- All notifications silenced (emergency calls only)
- Confirmation dialog to exit

**Scheduled distractions:**

- Pomodoro integration (25 min focus → 5 min break)
- Browser tab parking ("15 tabs → Read Later")
- Parking lot for intrusive thoughts

---

**Month 10-12: Cross-App Workflows**

**Integrated intelligence:**

| Trigger | Action | Result |
|---------|--------|--------|
| Select task | "Show notes" button | Links to Knowledge nodes |
| Write in Knowledge | "Create task?" prompt | Guidance task from note |
| Assign inventory | Project link | Tracking → Guidance |
| Unified search (CMD+K) | Searches all 3 apps | Filtered results |

**Dashboard insights:**

- Tasks with linked knowledge (integration health)
- Inventory allocated to projects (resource utilization)
- Knowledge creation by project (documentation health)

---

## 🎨 ADHD-Optimized Design Patterns

### Core Principle: External Executive Function

ADHD-friendly design **externalizes cognitive processes** that neurotypical users do automatically:

- **Task initiation** → AI breaks down into first tiny step
- **Time awareness** → Visual timelines, progress bars
- **Working memory** → Everything visible, nothing hidden
- **Sustained attention** → Dopamine-rewarding micro-interactions

---

### Pattern 1: Progressive Disclosure

**Problem:** Overwhelming interfaces cause task abandonment

**Solution:** Break information into dopamine-rewarding chunks

**Example: Bowling Reservation (Anti-Pattern)**

❌ **Bad - All at once:**

- Date
- Time
- Duration
- Food options
- Lane type
- Special requests

✅ **Good - 4 steps:**

1. Date & time
2. Duration & lane type
3. Food options (optional)
4. Special requests (optional)

**Result:** Each "Next" button = micro-dopamine boost

---

### Pattern 2: Multiple View Options

**Why it matters:** ADHD cognitive state fluctuates

**Monday morning ≠ Friday afternoon**

**Provide options:**

| View | Cognitive State | ADHD Benefit |
|------|----------------|--------------|
| **List** | Low energy | Minimal decisions |
| **Board** | Medium energy | Visual spatial |
| **Timeline** | High energy | Time awareness |

**Implementation:**

- Prominent view switcher (top-right)
- Keyboard shortcuts (CMD+1/2/3)
- Remember per-project preference

---

### Pattern 3: Visual Hierarchy Optimization

**Color system - Strategic not decorative:**

| Color | Purpose | Usage |
|-------|---------|-------|
| Neutral | Base palette | 80% of interface |
| Blue | Primary action | "Do this now" |
| Red | Urgent | Overdue, errors |
| Yellow | Warning | Needs attention |
| Green | Complete | Done, success |

> 💡 **Tip:** Reserve bright colors for function, never decoration

**Typography hierarchy:**

- H1: 24-32px (bold)
- H2: 20-24px (semibold)
- Body: 14-16px minimum
- Line height: 1.5-1.8
- Letter spacing: 0.02-0.04em

**Whitespace:** 1:1 ratio minimum with content

**Touch targets:** 44x44px minimum (Material Design 3 = 48x48dp)

---

### Pattern 4: Notification Intelligence

> ⚠️ **Critical:** 54% of ADHD users ignore reminders arriving when action is impossible

**Effective strategies:**

**Location-based:**

- "Buy milk" when near store
- "Call doctor" when at home

**Transition warnings:**

- "Meeting in 15 minutes" (prepare)
- "Meeting now" (go)

**Custom snooze:**

- "Remind when I arrive home"
- "Remind Monday morning"

**Limit categories:** 3-5 max

- Critical
- Time-sensitive
- Supportive nudges
- Optional
- Social

---

### Pattern 5: AI Task Decomposition

**The flagship ADHD feature** (Goblin Tools model)

**How it works:**

```
Input: "Clean the kitchen"

Spicy Level 3 →

Output:
1. Clear countertops
2. Load dishwasher
3. Wipe surfaces
4. Sweep floor
5. Take out trash

Each subtask supports further breakdown
```

**ADHD benefits:**

- Combats executive dysfunction
- Reduces task initiation paralysis
- Focuses on "first tiny step"
- No shame about complexity

**Implementation note:** Store user's preferred granularity, learn patterns

---

## ♿ Accessibility as Competitive Advantage

### WCAG 2.2 Critical Requirements

**Published:** October 2023
**Your target:** Level AA (6 months)

> ℹ️ **Note:** 83.6% of websites fail contrast - this is your opportunity

**New success criteria:**

| Criterion | Requirement | Altair Implementation |
|-----------|-------------|----------------------|
| 2.4.11 Focus Not Obscured | Keyboard focus visible | No sticky headers blocking focus |
| 2.5.7 Dragging Movements | Single-pointer alternative | Click + arrow keys for kanban |
| 2.5.8 Target Size | 24×24px minimum | Material 3 = 48x48dp ✅ |
| 3.3.7 Redundant Entry | Auto-populate | Remember previous inputs |

---

### Contrast Requirements

**Minimum ratios:**

| Content Type | Ratio | Example |
|--------------|-------|---------|
| Normal text | 4.5:1 | Body paragraphs |
| Large text (18pt+) | 3:1 | Headers, buttons |
| UI components | 3:1 | Form borders, icons |

**Your advantage:** Neo-brutalist high contrast naturally supports this

**Tool:** [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/)

---

### Keyboard Navigation

**Comprehensive implementation:**

| Function | Keys | Purpose |
|----------|------|---------|
| Major regions | F6 | Toolbar → Sidebar → Main |
| Command palette | CMD/CTRL+K | Universal navigation |
| Modal focus | Tab (trapped) | Stays within modal |
| Dismiss modal | Escape | Close, restore focus |
| Skip navigation | Skip link | Jump to main content |

**For Altair:**

- All interactive elements: Tab/Shift+Tab
- Activate: Enter or Space
- Visible focus: 2px minimum, 3:1 contrast
- No keyboard traps

---

### Neurodivergent Patterns

**ADHD-specific:**

- ✅ Minimize animations (respect `prefers-reduced-motion`)
- ✅ Avoid keyboard traps
- ✅ No auto-play content
- ✅ Time-blindness support without hiding clocks

**Autism-specific:**

- ✅ Absolute consistency (same icon = same action)
- ✅ Muted low-saturation colors
- ✅ Explicit step-by-step instructions
- ✅ Warn before context changes

**Dyslexia-specific:**

- ✅ Letter spacing: 0.35ch
- ✅ Line height: 1.5-2.0
- ✅ Sans-serif fonts (+ OpenDyslexic toggle)
- ✅ Avoid pure black on white (#333 on #F5F5F5)
- ✅ Never use italics (use bold or color)

---

### Customization = Empowerment

**Essential controls:**

**Visual:**

- Theme (light/dark/high contrast/custom)
- Font family (Inter/Roboto/OpenDyslexic/Atkinson Hyperlegible)
- Font size (100-200%, 10% increments)
- Line height (1.2-2.0)
- Letter spacing (0-0.5ch)

**Layout:**

- Density (compact/comfortable/spacious)
- Animation level (full/reduced/none)
- View preference per project

**Notifications:**

- Categories to enable/disable
- Frequency limits
- Delivery methods
- Sound rotation

**Presets (one-click):**

- 🧠 Dyslexia Mode
- 🎯 Focus Mode
- 😌 Calm Mode

---

## 🎯 Contemporary Design Patterns (2025)

### Micro-Interactions That Matter

**Statistics:** 18% improvement in satisfaction (Carnegie Mellon HCI)

**Standard patterns:**

| Type | Example | Purpose |
|------|---------|---------|
| Morphing | Spinner → Checkmark | Dopamine reward |
| Contextual tooltips | Hover for help | Feature discovery |
| Inline validation | Real-time feedback | Reduce errors |
| Celebration | Confetti on complete | ADHD motivation |
| Button states | Enlarge + haptic | Tactile confirmation |

**Constraints:**

- Under 400ms duration
- 60 FPS minimum
- Optional disable

---

### Information Architecture Approaches

**Comparison of leaders:**

| Approach | Strength | Weakness | Best For |
|----------|----------|----------|----------|
| **Notion** | Flexible databases, 7+ views | Can become cluttered | Teams, complex projects |
| **Obsidian** | Graph-based linking | "Hairball" with 10K+ notes | Knowledge workers |
| **Tana** | AI-native supertags | Learning curve | Power users |
| **Capacities** | Object-first (books, people) | Limited flexibility | Beginners |

**For Altair Knowledge:**

Hybrid approach:

- ✅ Capacities' object-first simplicity
- ✅ Obsidian's graph visualization
- ✅ Tana's smart tagging
- ✅ Local graph (2 degrees) with filters

**Object types:** Projects, Tasks, Resources, People, Ideas

---

### Command Palette = Expected Baseline

**Status:** As standard as save/undo in 2025

**Implementation:**

- CMD/CTRL+K trigger
- Natural language parsing
- Recent items prominent
- Keyboard navigation
- Visual shortcuts (show "CMD+N")

**Example queries:**

- "Show overdue tasks"
- "Switch to board view"
- "New task in Marketing"
- "Go to Project X"

---

### Onboarding That Works

**Statistics:**

- 76% continue after good onboarding
- 77% abandon within 3 days without it

**Checklist-driven activation:**

```
Welcome to Altair! Let's get you started:

✅ 1. Create first task (pre-checked)
⬜ 2. Break task into subtasks with AI
⬜ 3. Complete a task
⬜ 4. Try different view (board/timeline)
⬜ 5. Set up sync (optional)

Each step clickable → relevant UI
```

**Key principle:** Each pre-checked item = head start (Wistia pattern)

**Result:** 47% increase in activation rate (Attention Insight study)

---

## 🎨 Strategic Neo-Brutalism

### The Challenge

**Research finding:** Zero successful pure neo-brutalist productivity apps

**Why?** Too visually intense for sustained focus work

**Your opportunity:** "Functional brutalism" - bold clarity without chaos

---

### 2025 Neo-Brutalist Evolution

**Three phases:**

| Era | Style | Characteristics |
|-----|-------|----------------|
| **Pure brutalism** (2018-2020) | Bare, utilitarian | Minimal, monochrome, harsh |
| **Neo-brutalism** (2021-2023) | Vibrant, structured | Bright colors, balanced usability |
| **Refined neo-brutalism** (2024-2025) | Soft brutalism | Airier colors, rounded edges, accessibility |

**Current characteristics:**

- High-contrast palettes (vibrant + black/white)
- Thick borders (4-8px, though 2-4px better for productivity)
- Bold stark shadows (single-color, 100% opacity)
- Experimental typography
- Asymmetrical layouts
- Nostalgic elements (Windows 98)
- No gradients (flat colors only)

---

### ADHD Considerations - The Paradox

**Neo-brutalism conflicts with ADHD needs:**

| ADHD Need | Neo-Brutalism Risk |
|-----------|-------------------|
| Calm environment | Intense saturation |
| Minimal distractions | Chaotic asymmetry |
| Predictable interactions | Unconventional layouts |
| Animation restraint | Bold motion |

**Solution:** "Brutalist accent, not brutalist immersion"

---

### Elements to EMBRACE

**For Altair implementation:**

✅ **High-contrast color system**

- 2-3 bold colors maximum
- Soft blue #6B9BD1
- Warm coral #FF6B6B
- Black #000000
- White #FFFFFF
- Meet WCAG AA minimum

✅ **Thick clear borders**

- 2-4px on cards/containers (NOT 8px)
- Creates visual boundaries
- Helps ADHD chunking

✅ **Bold solid shadows**

- 4px offset
- Black at 60-80% opacity (softer than 100%)
- Elevated elements only

✅ **Strategic typography**

- Bold sans-serif headings (Montserrat/Inter)
- Clean 14-16pt minimum body
- 1.5-1.8 line height

✅ **Geometric accents**

- Icons, buttons, status indicators
- Functional not decorative

✅ **Minimal animation**

- Simple state transitions only
- No autoplay
- No continuous motion

✅ **Generous whitespace**

- 24-32px padding around sections
- 1.5-1.8 line height

---

### Elements to AVOID

**For productivity UX:**

❌ **Asymmetrical task lists**

- Keep work areas symmetrical
- Exception: marketing/landing pages OK

❌ **Excessive decoration**

- No shapes cluttering work areas
- Every element serves purpose

❌ **High-saturation backgrounds**

- Use vibrant colors for accents only
- Content areas: neutral backgrounds

❌ **Experimental body fonts**

- Reserve for headings/branding
- Body text: highly legible

❌ **Breaking UI conventions**

- Visual boldness ≠ functional novelty
- Don't reinvent buttons, forms, navigation

---

### Practical Implementation

**Button example:**

```css
/* Functional brutalist button */
border: 2px solid #000000;
box-shadow: 4px 4px 0 rgba(0, 0, 0, 0.6);
border-radius: 4px;
padding: 12px 24px;

/* Hover state */
:hover {
  box-shadow: 2px 2px 0 rgba(0, 0, 0, 0.6);
  background: lighten(5%);
}
```

**Card example:**

```css
/* Functional brutalist card */
border: 3px solid rgba(0, 0, 0, 0.8);
padding: 16-24px;
background: #FFFFFF or #F5F5F5;
/* No shadow for base containers */
/* Shadow only for elevated/modal */
```

**Task item:**

- Clean grid layout
- Geometric checkbox (circle or square, 2px border)
- Bold colored status dots
- Clear hover state (background tint)

---

## ⚡ Local-First UX

### The Seven Ideals

**From Ink & Switch manifesto:**

| Ideal | Altair Implementation |
|-------|----------------------|
| 1. No spinners | <100ms response, read/write local disk |
| 2. Multi-device | SurrealDB seamless sync |
| 3. Network optional | Full offline capabilities |
| 4. Seamless collaboration | CRDT-based merging |
| 5. The Long Now | Markdown/JSON export, works forever |
| 6. Security & privacy | End-to-end encryption, local-first |
| 7. User ownership | Your data, export anytime |

---

### Sync Status Communication

**WCAG requirement:** Color + shape + symbol + text (minimum 3 of 4)

**Three-tier system:**

**1. Status Bar (Top)**

| State | Color | Icon | Text | Animation |
|-------|-------|------|------|-----------|
| Synced | Green | ✓ | "Synced" | Static |
| Syncing | Blue | ⟳ | "Syncing..." | Rotating |
| Pending | Orange | ○ | "Pending changes" | Static |
| Offline | Gray | ✕ | "Offline" | Static |
| Error | Yellow | ! | "Sync failed" | Static |

**2. Document-Level**

- Colored badge on each task/node/item
- Shows individual sync status
- Same color scheme as status bar

**3. Settings Panel**

- Overall status
- Last sync timestamp
- Next scheduled sync
- Items pending
- Sync history log
- "Sync Now" button
- Troubleshooting (view conflicts, retry)

**Timing pattern (Logseq model):**

- Desktop: 20 seconds after stopping typing
- Mobile: 2 seconds after stopping typing

---

### Offline/Online Transitions

**Make offline feel normal, not exceptional**

**Initial sync:**

- Distinct onboarding step
- Time estimate
- Network requirements
- Can app remain open?
- Optimal WiFi location

**Ongoing sync:**

- Background without distraction
- Reassuring message: "You're offline. Updates will upload when reconnected"
- Avoid distracting details: NO "Retrying in 3:22"

**Context-dependent intervals:**

- After typing stops (2-20 sec)
- NOT rigid schedules

**"Last updated" timestamps:**

- On critical views (task lists, nodes, inventory)
- Helps users understand data currency

---

### Conflict Resolution

**Good news:** Conflicts are rare with CRDT

**Why?**

- Character-level text tracking
- Users avoid concurrent edits intuitively
- Different document parts merge cleanly

**Real conflicts:** Same property of same object modified simultaneously

**For Altair:**

| App | Strategy | Notes |
|-----|----------|-------|
| **Guidance** | Last-write-wins + timestamps | Simple tasks |
| **Knowledge** | Operational transformation | Concurrent text editing |
| **Tracking** | Version vectors | Manual resolution if needed |

**When manual resolution needed:**

Visual diff tool showing:

- Item type/ID
- Server value vs Local value (columns)
- Timestamps + attribution
- Actions: Keep Server / Keep Local / Merge Manually

**NOT Dropbox's "conflicted copy" pattern** - show actual differences

**Version history:** Maintain 1+ year (Obsidian standard)

---

### Building Trust

**Transparency about data location:**

**Your Data Location:**

- **Primary:** Your devices (local storage)
- **Sync:** Encrypted copies on servers
- **Region:** [data center location]

**Key messages:**

- "Your data lives on your devices first"
- "Servers only hold encrypted copies for sync"
- "Export/backup anytime"
- "Works fully offline—server not required"

**Prominent features:**

- Export in standard formats (Markdown, JSON, CSV)
- Delete account with clear confirmation
- "Last local backup" timestamp
- One-click backup

---

### Your SurrealDB Advantage

**Marketing positioning:**

✅ **Speed:**
"Your tasks, notes, inventory in high-performance local database"

- <100ms response times
- Zero latency
- Instant access

✅ **Control:**
"Optional cloud sync when you want it, where you want it"

- User decides
- Self-hosted option

✅ **Freedom:**
"Open data formats—never locked in"

- Markdown export
- JSON export
- Works without server

**vs Competitors:**

- Notion: 3-4 second page loads
- Todoist: Requires connection for many operations
- Asana: Fully cloud-dependent

---

## 🏗️ Flutter + Material Design 3

### Why Material Design 3

**Became default:** Flutter 3.16+

**Key benefits:**

- ✅ Systematic theming
- ✅ Enhanced accessibility
- ✅ Large screen optimization
- ✅ Harmonious color generation

---

### ColorScheme.fromSeed() Magic

**Revolution:** Single seed color → full harmonious palette

**For Altair:**

```dart
ColorScheme.fromSeed(
  seedColor: Color(0xFF6B9BD1), // Soft blue
  brightness: Brightness.light
)
```

**Result:**

- Automatic light/dark mode
- Proper contrast ratios
- Harmonious palette
- Accessibility by default

> ⚠️ **Warning:** Never override individual ColorScheme colors—breaks harmony

**Layer neo-brutalism on top:**

- Borders: 2-4px solid black
- Shadows: 4px offset, black 60-80% opacity
- Keep Material 3 color system intact

---

### Typography System

**Material 3 text styles:**

```dart
Theme.of(context).textTheme.displayLarge
Theme.of(context).textTheme.headlineMedium
Theme.of(context).textTheme.bodyLarge
```

**Platform fonts:**

- Android: Roboto (automatic)
- iOS: San Francisco (automatic)
- Fallback: System default

**Your customization:**

- Bold weights for headers
- 14-16pt minimum body
- 1.5-1.8 line height
- Generous letter spacing

---

### Cross-Platform Adaptation

**Automatic behavior:**

| Platform | Navigation | Typography | Buttons |
|----------|-----------|------------|---------|
| Android | Bottom-up | Roboto | Material ripple |
| iOS | Left-to-right | San Francisco | Cupertino |
| Web | Instant | System | Custom |

**Manual adaptations:**

```dart
// Use adaptive widgets
Switch.adaptive()
Slider.adaptive()

// Or conditional
Platform.isIOS ? CupertinoButton() : ElevatedButton()
```

**Better approach:** `flutter_platform_widgets` package

---

### Responsive Breakpoints

**Material Design standard:**

| Breakpoint | Range | Layout Strategy |
|------------|-------|----------------|
| **Mobile** | 0-599dp | Single column, bottom nav |
| **Mobile Large** | 600-839dp | Still single, optimized spacing |
| **Tablet** | 840-1239dp | Two columns, optional side nav |
| **Desktop** | 1240+dp | Multi-column, permanent nav |

**Implementation:**

```dart
class Responsive {
  static bool isMobile(BuildContext context) =>
    MediaQuery.of(context).size.width < 600;

  static bool isTablet(BuildContext context) =>
    MediaQuery.of(context).size.width >= 840 &&
    MediaQuery.of(context).size.width < 1240;

  static bool isDesktop(BuildContext context) =>
    MediaQuery.of(context).size.width >= 1240;
}
```

> 💡 **Tip:** Check available space, NOT device type

---

### Performance Optimization

**Flutter advantages:**

- Compiles to native ARM C/C++
- No JavaScript bridge
- Renders via Impeller/Skia
- 60fps target (16ms frame budget)

**Your responsibilities:**

✅ **const constructors**

```dart
const Text('Hello'); // Flutter skips rebuilds
```

✅ **StatelessWidget preference**

```dart
// Use StatelessWidget when no state needed
// Simpler, faster, less memory
```

✅ **RepaintBoundary**

```dart
// Isolate frequently changing sections
RepaintBoundary(
  child: AnimatedWidget()
)
```

✅ **Lazy loading**

```dart
// Always use .builder() for lists
ListView.builder(
  itemCount: tasks.length,
  itemBuilder: (context, index) => TaskItem(tasks[index])
)
```

**For Altair:**

- Task lists: ListView.builder (NEVER full list)
- Pagination: 100+ tasks
- Optimistic updates (immediate UI, background sync)
- Cache frequently accessed data

---

### Flutter Web Optimization

**Challenge:** Bundle size + initial load

**Default:** CanvasKit (~1.5MB overhead)

**Optimization strategies:**

✅ **Tree shaking**

- Automatic in release builds
- Can reduce 50%+

✅ **Deferred loading**

```dart
import 'package:localization/messages.dart' deferred as messages;

// Load when needed
await messages.loadLibrary();
```

✅ **Critical splash screen**

```html
<!-- Plain CSS/HTML displays instantly -->
<div id="loading">
  <div class="spinner"></div>
  <p>Loading Altair...</p>
</div>
```

✅ **Preload tags**

```html
<link rel="preload" href="main.dart.js" as="script">
```

✅ **WebP images**

- Better compression than PNG/JPEG

✅ **Disable page transitions**

- Faster perceived navigation

---

### State Management

**For Altair: Recommended Riverpod 2.0+**

**Why?**

- ✅ Compile-time safety
- ✅ Better DevTools
- ✅ No BuildContext required
- ✅ Perfect for local-first reactive model

**Alternatives:**

| Option | Best For | Complexity |
|--------|----------|-----------|
| **Riverpod** | Medium-large apps | Medium |
| **BLoC** | Enterprise, highly testable | High |
| **Provider** | Simple projects | Low |

**Local-first pairing:**

- SurrealDB changes → Riverpod reactive updates
- Granular rebuild control
- Automatic UI propagation

---

## ✅ Validation & Success Metrics

### Essential User Testing

**Recruit diverse ADHD population:**

- Inattentive, hyperactive, combined
- Various ages
- Different severity levels
- With/without medication

**Methods:**

| Method | What It Reveals | When |
|--------|----------------|------|
| Think-aloud | Confusion in real-time | During tasks |
| Time-on-task | Where users stuck | Every session |
| NASA-TLX | Cognitive load | After sessions |
| Diary studies | Real-world use | Week-long |
| A/B testing | Data-driven decisions | Feature launches |

---

### Critical Metrics

**Track these:**

| Metric | Target | What It Means |
|--------|--------|---------------|
| Task completion rate | >90% | Core workflows work |
| Onboarding time | <10 min | "Aha moment" speed |
| Feature adoption | >60% | Multiple views, AI |
| Daily active use | >40% | Retention |
| Qualitative satisfaction | 4.5+/5 | User happiness |

---

### Neo-Brutalist Validation

**Decision criteria:**

If bold aesthetic causes:

- Task completion drop >10%
- Time to completion increase >10%
- Error rate increase >15%

**Then:** Revert to softer implementation

**A/B test:** Traditional minimalist vs neo-brutalist

---

### Performance Benchmarks

**Your local-first advantage must be measurable:**

| Operation | Target | Competitor Average |
|-----------|--------|-------------------|
| Local response | <100ms | 300-1000ms |
| Sync completion | <5 sec | 10-30 sec |
| App launch | <2 sec | 3-5 sec |
| Offline degradation | 0% | 50-80% features lost |

**Track:** Compare to users' previous tools (Notion, Todoist, Asana)

---

### Accessibility Compliance

**Roadmap:**

**6 months:** WCAG 2.2 Level AA

- 4.5:1 contrast (normal text)
- 3:1 contrast (large text/components)
- Keyboard accessible
- ARIA labels
- Reduced motion support
- Focus management

**12 months:** Select AAA criteria

- 7:1 contrast (critical text)
- Enhanced focus appearance
- Comprehensive shortcuts
- Sign language onboarding videos

**Quarterly audits:**

- Automated tools: axe DevTools, WAVE, Lighthouse (catch 30-40%)
- Manual testing: Remaining 60-70%

**Biannual screen reader testing:**

- NVDA (Windows)
- VoiceOver (Mac/iOS)
- TalkBack (Android)

---

## 🎯 Market Positioning Strategy

### The Risk: Pure Neo-Brutalism

**Reality check:** No successful productivity apps use it

**Phased approach:**

**Phase 1 (Months 1-3): Accent Implementation**

- Neo-brutalist marketing/landing pages
- More conventional core application
- Test user response

**Phase 2 (Months 4-6): Optional Themes**

- "Bold Mode" vs "Classic Mode"
- Gather adoption data
- Monitor metrics

**Phase 3 (Months 7-12): Default Bold (if data supports)**

- Neo-brutalist as default
- Maintain alternatives
- Continuous monitoring

---

### Your Unique Differentiation

**No competitor has:**

✅ ADHD-optimized patterns

- Progressive disclosure
- AI task breakdown
- Multiple views
- Focus modes

✅ Local-first speed

- Instant response
- Offline capability
- Data sovereignty

✅ Strategic neo-brutalism

- Bold clarity
- Without chaos
- Functional not decorative

✅ Flutter consistency

- Seamless cross-platform
- Material Design 3
- Single codebase

**vs Notion:** No ADHD focus, no local-first
**vs Todoist:** No knowledge management, no bold aesthetic
**vs Obsidian:** No ADHD features, limited mobile
**vs All:** No one combines your complete feature set

---

## 🚀 Next Steps

### Immediate Actions (This Week)

1. **Set up keyboard navigation testing**
   - Disconnect mouse
   - Complete full workflow
   - Document issues

2. **Verify color contrast**
   - Run WebAIM checker on all combinations
   - Fix failures immediately

3. **Implement command palette**
   - Basic CMD/CTRL+K
   - Recent items
   - Test with keyboard only

4. **Create responsive breakpoint helper**
   - Responsive.isMobile(context)
   - Responsive.isTablet(context)
   - Responsive.isDesktop(context)

---

### Month 1 Milestones

- [ ] Keyboard navigation working everywhere
- [ ] Focus indicators visible (2px, 3:1 contrast)
- [ ] Skip links implemented
- [ ] `prefers-reduced-motion` support
- [ ] ColorScheme.fromSeed() with verified contrast
- [ ] Neo-brutalist design system codified
- [ ] Sync status three-tier system
- [ ] Command palette functional

---

### Success Criteria

**You'll know you're succeeding when:**

✅ ADHD users complete onboarding <10 min
✅ Task completion rate >90%
✅ Daily active use >40%
✅ Response time <100ms feels "instant"
✅ Users say "finally, something that works for my brain"
✅ Accessibility audit scores >90%
✅ Multiple views used regularly
✅ AI task breakdown drives engagement

---

## 📚 Quick Reference

### Design Decision Checklist

**Before implementing any feature, ask:**

1. ❓ Does this help ADHD users complete tasks more effectively?
2. ❓ Is it keyboard accessible?
3. ❓ Does it meet contrast requirements (4.5:1)?
4. ❓ Can I explain it in 2-3 sentences?
5. ❓ Does it work offline?
6. ❓ Is response time <100ms?
7. ❓ Can users customize it?
8. ❓ Does it respect reduced motion preference?

**If any answer is NO → Redesign before building**

---

### ADHD-Friendly Pattern Library

| Pattern | When to Use | Key Elements |
|---------|-------------|--------------|
| Progressive disclosure | Multi-step processes | Steps, progress bar, save/resume |
| AI task breakdown | Complex tasks | Spicy level, hierarchical subtasks |
| Multiple views | Different work styles | List, board, timeline + switcher |
| Focus mode | Distraction management | 3 tiers, gradual blocking |
| Visual timeline | Time-sensitive work | Color blocks, time awareness |
| Notification intelligence | Reminders | Context-aware, anti-habituation |
| Gamification (optional) | Motivation | Streaks, XP, gentle consequences |

---

### Color Palette Reference

**Your functional brutalist palette:**

```css
/* Primary colors (soft neo-brutalist) */
--soft-blue: #6B9BD1;
--warm-coral: #FF6B6B;
--neo-black: #000000;
--pure-white: #FFFFFF;
--off-white: #F5F5F5;

/* Semantic colors */
--urgent-red: #E63946;
--warning-yellow: #FFB703;
--success-green: #06D6A0;
--info-blue: #118AB2;

/* Borders and shadows */
--border-width: 2-4px;
--border-color: rgba(0, 0, 0, 0.8);
--shadow: 4px 4px 0 rgba(0, 0, 0, 0.6);
```

---

### Typography Scale

```css
/* Headers */
--h1: 32px/1.2 Bold;
--h2: 24px/1.3 Bold;
--h3: 20px/1.4 SemiBold;

/* Body */
--body-large: 16px/1.6 Regular;
--body: 14px/1.6 Regular;
--small: 12px/1.5 Regular;

/* Letter spacing */
--letter-spacing: 0.02-0.04em;
```

---

### Accessibility Quick Checks

**30-second validation:**

- [ ] Can I Tab through everything?
- [ ] Can I see keyboard focus?
- [ ] Can I dismiss modals with Escape?
- [ ] Is body text at least 14px?
- [ ] Are touch targets at least 44x44px?
- [ ] Does it work with reduced motion?

**WebAIM Contrast Checker:**
<https://webaim.org/resources/contrastchecker/>

**ARIA label example:**

```dart
Semantics(
  label: 'Complete task',
  child: IconButton(...)
)
```

---

## 🎓 Key Takeaways

### The Universal Truth

**What helps ADHD users helps everyone**

- Reduced cognitive load → Faster understanding
- Immediate feedback → Better UX
- Flexible structure → Accommodates preferences
- Visual clarity → Improved scannability
- Predictable patterns → Reduced learning curve

---

### The Winning Formula

```
ADHD optimization
  + Local-first speed
  + Functional neo-brutalism
  + Flutter consistency
  + Deep accessibility
= Competitive differentiation
```

---

### The Implementation Philosophy

**Clarity amplifies, chaos destroys**

Every design decision must answer:
**"Does this help users complete tasks more effectively?"**

If visual boldness compromises:

- Focus
- Efficiency
- Accessibility

**It fails, regardless of aesthetic appeal**

---

### Your Path Forward

**1. Build the foundation** (accessibility, performance)
**2. Add ADHD features** (AI breakdown, views, focus)
**3. Layer bold aesthetic** (carefully, validate metrics)
**4. Optimize local-first** (speed is competitive advantage)
**5. Test relentlessly** (with neurodivergent users)
**6. Iterate based on data** (not assumptions)

---

### The Market Gap

**Opportunity exists for:**

- Bold execution
- Underserved ADHD population
- Excellent design
- Genuine understanding
- Technical sophistication

**Your three-app ecosystem:**

- 🎯 Guidance (task management)
- 📚 Knowledge (PKM)
- 📦 Tracking (inventory)

**Each serves distinct ADHD needs**
**All share design language**
**Users adopt one or all three**

---

## 💡 Final Wisdom

**Ship thoughtfully. Iterate based on data.**

A focused product clearly communicating key insights proves far more valuable than feature-complete overwhelm.

**Follow the research.**
**Test with real users.**
**Measure what matters.**
**Build the ADHD-focused productivity suite that combines clarity with capability.**

---

**Your tagline says it all:**

## "Where focus takes flight" ✈️

Make sure the UX delivers on that promise.

---

_Document created with ADHD-friendly formatting principles_
_Last updated: October 2025_
