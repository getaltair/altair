# Altair Features

**ADHD-Friendly Project Management by Design**

> This document details Altair's features and explains why they're specifically designed for ADHD brains.

📊 **Visual References:**
- [ADHD Features Mindmap](diagrams/04-roadmap-planning.md#adhd-features-mindmap)
- [User Flow Diagrams](diagrams/03-user-flows.md)
- [UI Component Architecture](diagrams/05-component-architecture.md#ui-component-hierarchy)

---

## Table of Contents

1. [Design Philosophy](#design-philosophy)
2. [Core Features](#core-features)
3. [ADHD-Specific Features](#adhd-specific-features)
4. [User Experience Principles](#user-experience-principles)
5. [Accessibility](#accessibility)
6. [Privacy & Security](#privacy--security)
7. [Comparison with Traditional Tools](#comparison-with-traditional-tools)

---

## Design Philosophy

### The ADHD Challenge

Traditional productivity tools are built for neurotypical brains. They assume:

❌ **Linear thinking** - ADHD brains jump between thoughts  
❌ **Sustained attention** - ADHD attention is inconsistent  
❌ **Working memory** - ADHD working memory is limited  
❌ **Time perception** - ADHD experiences time blindness  
❌ **Executive function** - ADHD has executive dysfunction  
❌ **Intrinsic motivation** - ADHD needs external structure  

### The Altair Approach

Altair doesn't try to "fix" ADHD. Instead, it **works with** neurodivergent thinking patterns:

✅ **Capture thoughts instantly** before they vanish  
✅ **Visual representations** over abstract concepts  
✅ **Flexible structure** that adapts to your brain  
✅ **Forgiving UX** that allows mistakes and changes  
✅ **External reminders** that don't rely on memory  
✅ **Gentle guidance** without pressure or shame  

**[View ADHD-Optimized Design Principles](diagrams/04-roadmap-planning.md#adhd-features-mindmap)**

---

## Core Features

### 1. Quick Task Capture

**The Problem:**  
ADHD brains experience thoughts fleetingly. By the time you open a traditional app, navigate to the right place, and start typing—the thought is gone.

**The Solution:**  
Ultra-fast task capture accessible from anywhere.

**How it Works:**
- Global keyboard shortcut (Ctrl+N / Cmd+N)
- Floating action button always visible
- Voice input option (future)
- Browser extension for web capture (future)
- Email forwarding to create tasks (future)

**Technical Implementation:**
```
User presses Ctrl+N
  → Overlay appears instantly (<50ms)
  → User types title
  → Presses Enter
  → Task saved to local storage immediately
  → Syncs to server in background
  → Confirmation shown
```

**Design Details:**
- Minimal required fields (just title)
- All other fields optional
- Auto-save while typing
- Optimistic UI (feels instant)
- Works offline

**[View Quick Capture Flow Diagram](diagrams/03-user-flows.md#quick-task-capture-flow)**

---

### 2. AI-Powered Task Breakdown

**The Problem:**  
ADHD brains struggle to break large projects into actionable steps. "Clean the house" feels impossible, but "wipe kitchen counter" is doable.

**The Solution:**  
AI analyzes your task and suggests specific, manageable subtasks.

**How it Works:**

**[View Task Breakdown Flow](diagrams/03-user-flows.md#ai-task-breakdown-flow)**

```
User creates task: "Prepare presentation"
  → User clicks "Break it down"
  → AI analyzes task
  → Suggests subtasks:
    • Research topic (30 min, medium focus)
    • Create outline (15 min, low focus)
    • Design slides (45 min, high focus)
    • Practice delivery (20 min, medium energy)
  → User accepts/modifies suggestions
  → Subtasks created
```

**AI Considerations:**
- Estimates time required
- Considers focus level needed
- Suggests order of completion
- Accounts for context switching
- Includes break recommendations

**Example Breakdown:**

```
Original: "Write blog post about ADHD productivity"

Suggested Subtasks:
1. Brainstorm 5 key points (10min, low focus)
2. Research each point (30min, high focus) [morning recommended]
3. Write rough draft (45min, high focus) [morning recommended]
4. Take 15min break ☕
5. Edit for clarity (20min, medium focus)
6. Add examples (15min, low focus)
7. Proofread (10min, low focus) [can do anytime]
8. Find/create images (20min, low focus)
9. Format for publishing (10min, low focus)
10. Final review (5min, low focus)
```

**[View AI Breakdown Architecture](diagrams/01-system-architecture.md#ai-task-breakdown-service)**

---

### 3. Visual Time Awareness

**The Problem:**  
Time blindness is a core ADHD symptom. "5 more minutes" turns into 2 hours. Tasks feel infinite or instantaneous.

**The Solution:**  
Visual, tangible time representations that make time *visible*.

**Visual Elements:**

**[View Time Tracking UI Components](diagrams/05-component-architecture.md#time-tracking-components)**

**Progress Bars:**
```
Task: Write email (estimated 10 min)
───────────────────────────────────────
[████████████████·············] 60% (6 min)
└─ Green: On track
```

**Time Buckets:**
```
Today's Time Budget: 4 hours
╔════════════════════════════════╗
║ Work          ████████░░  2.5h ║
║ Personal      ███░░░░░░░  1.0h ║
║ Available     █░░░░░░░░░  0.5h ║
╚════════════════════════════════╝
```

**Visual Timer:**
```
     ╭───────╮
     │ 15:00 │  ← Large, impossible to miss
     ╰───────╯
     ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░  75% elapsed
```

**Color Coding:**
- 🟢 Green: Plenty of time left
- 🟡 Yellow: Halfway through estimate
- 🟠 Orange: 80% of estimated time used
- 🔴 Red: Over estimate (not a failure!)

**[View Time Awareness Flow](diagrams/03-user-flows.md#time-tracking-session)**

---

### 4. Focus Mode

**The Problem:**  
ADHD brains are easily distracted. Notifications, sidebar clutter, and "just checking" other tasks derail focus.

**The Solution:**  
Single-task view that eliminates all distractions.

**How it Works:**

**[View Focus Mode Diagram](diagrams/03-user-flows.md#focus-mode-session)**

```
User clicks "Focus on this task"
  → App transitions to fullscreen mode
  → Only current task visible
  → All notifications muted
  → Timer starts automatically
  → Gentle background music option
  → Exit requires confirmation (avoid accidental switching)
```

**Focus Mode Features:**
- Fullscreen single task
- Built-in Pomodoro timer
- Break reminders
- "Can't focus?" button (suggests alternative tasks)
- Distraction log (record what pulled you away)
- Ambient sounds/music integration

**Customization:**
- Duration preferences (20/25/45/60 min)
- Break length
- Background sounds
- Dark mode intensity
- Minimal vs. guided mode

---

### 5. Gentle Gamification

**The Problem:**  
ADHD responds to novel stimuli and immediate rewards, but exploitative gamification (streaks, infinite scrolling) is harmful.

**The Solution:**  
Progress tracking that celebrates wins without creating anxiety or addiction.

**Design Principles:**

**[View Gamification Philosophy](diagrams/04-roadmap-planning.md#feature-priority-matrix)**

✅ **DO:**
- Celebrate task completion
- Show daily progress
- Offer achievement badges
- Provide visual progress indicators
- Acknowledge effort and improvement

❌ **DON'T:**
- Create streaks that cause anxiety when broken
- Use infinite scroll feeds
- Punish for missing days
- Create FOMO mechanics
- Encourage overwork

**Examples:**

**Daily Completion:**
```
╔═══════════════════════════════╗
║     Today's Wins! 🎉          ║
║                                ║
║  ✓ Morning routine            ║
║  ✓ Reply to 3 emails          ║
║  ✓ 30min focus session        ║
║                                ║
║  You completed 5 of 7 tasks!  ║
║  Great work. Rest now. 💙     ║
╚═══════════════════════════════╝
```

**Weekly Review:**
```
This Week: October 1-7, 2025
────────────────────────────────
• 12 tasks completed
• 8.5 hours of focused time
• 3 projects moved forward
• Longest focus session: 45 min

Progress is progress! 🚀
No matter how small.
```

**Achievements (Optional):**
- "First Focus Session" - Tried focus mode
- "Week Warrior" - Completed tasks 5+ days
- "Deep Diver" - 2+ hours focused in one day
- "Breaker of Tasks" - Used AI breakdown 10 times
- "Time Tracker" - Tracked time for 7 consecutive days

**Critical:** All gamification is **opt-in** and can be disabled entirely.

---

### 6. Automatic Documentation

**The Problem:**  
ADHD brains generate insights during work, but stopping to document them breaks flow. Those insights are then lost forever.

**The Solution:**  
Capture notes, decisions, and learnings without leaving your flow state.

**How it Works:**

**[View Documentation Flow](diagrams/03-user-flows.md#documentation-capture)**

**Quick Notes:**
```
While working on task
  → User has insight
  → Presses hotkey (Ctrl+/)
  → Mini-note panel slides in
  → User types thought
  → Presses Enter
  → Note saved, linked to task
  → Panel slides out
  → User continues working
```

**Auto-Capture:**
- Time when task was started/stopped
- Duration of focus sessions
- Tasks completed in session
- Any files opened/created
- Decisions made (prompted at end of session)

**Review & Reflection:**
```
End of day prompt:
┌─────────────────────────────────────┐
│ Today you:                          │
│ • Completed 5 tasks                 │
│ • Focused for 3.5 hours             │
│ • Added 7 quick notes               │
│                                     │
│ Want to add anything before         │
│ wrapping up?                        │
│                                     │
│ [Add reflection]  [Skip for today] │
└─────────────────────────────────────┘
```

**[View Documentation Architecture](diagrams/05-component-architecture.md#documentation-service)**

---

## ADHD-Specific Features

### Working Memory Support

**Short-Term Task List:**
```
Right Now:
1. Reply to Sarah's email
2. Call dentist
3. Review pull request

Later (saved for you):
• 23 other tasks safely stored
• You won't forget them
• Focus on now
```

### Context Switching

**Tag-Based Context:**
```
Contexts:
• @computer 💻 (12 tasks)
• @phone ☎️ (3 tasks)
• @errands 🚗 (5 tasks)
• @home 🏠 (8 tasks)
• @high-energy ⚡ (6 tasks)
• @low-energy 🍃 (9 tasks)
```

**Smart Filtering:**
- Show only tasks for current context
- "What can I do right now?" mode
- Energy-level aware suggestions
- Location-based filtering (future)

### Time Blindness Accommodations

**Estimated vs Actual:**
```
Task: Write documentation
──────────────────────────
Estimated: 30 minutes
Actual:    2 hours

Learning: Writing takes 4x estimate
Next time: Auto-suggest 2 hour estimate
```

**Time Budget Warnings:**
```
⚠️  You have 3 tasks scheduled for today
    totaling 5 hours.

    You have 2 hours available.

    Want help reprioritizing?
    [Yes, help me]  [I know, it's fine]
```

### Rejection Sensitivity

**Gentle Language:**
```
❌ "You failed to complete your tasks"
✅ "You completed 3 of 5 tasks. Progress!"

❌ "Task overdue by 3 days"
✅ "Task from Monday. Still relevant?"

❌ "You broke your 7-day streak"
✅ "Welcome back! Ready to start fresh?"
```

### Hyperfocus Protection

**Hyperfocus Mode:**
```
You've been focused for 3 hours straight!
────────────────────────────────────────
Consider taking a break:
• Stand and stretch
• Drink water
• Eat something
• Check the time

[5 more minutes]  [Yes, break time]
```

---

## User Experience Principles

### 1. Instant Feedback

Every action gets immediate visual confirmation:
- Button press → visual feedback
- Task created → appears immediately
- Time tracked → timer updates live
- Sync happening → subtle indicator

### 2. Forgiving Interface

Mistakes are expected and easy to fix:
- Undo available for all actions
- "Are you sure?" for destructive actions
- Deleted tasks in trash (not permanent)
- Auto-save prevents data loss
- Confirmation dialogs are clear

### 3. Multiple Pathways

Different brains work differently:
- Keyboard shortcuts for power users
- Visual click-through for explorers
- Voice commands (future)
- Drag-and-drop for organizers
- Text-based quick commands

### 4. Reduce Cognitive Load

**[View UI Simplification Strategy](diagrams/05-component-architecture.md#ui-component-hierarchy)**

- Progressive disclosure (advanced features hidden until needed)
- Sane defaults that work for most people
- Smart suggestions based on patterns
- Minimal required decisions
- Clear visual hierarchy

### 5. Sensory Considerations

**[View Accessibility Options](diagrams/05-component-architecture.md#accessibility-features)**

- **Reduce motion** option (no animations)
- **High contrast** mode
- **Large text** option
- **Color-blind** friendly palette
- **Dark mode** (OLED-friendly)
- **Sound notifications** optional

---

## Accessibility

### WCAG 2.1 Level AA Compliance

- ✅ All functionality available via keyboard
- ✅ 4.5:1 color contrast minimum
- ✅ Screen reader compatible
- ✅ Resizable text up to 200%
- ✅ Clear focus indicators
- ✅ No time limits (or user-controlled)

### ADHD-Specific Accessibility

- ✅ Ultra-fast task capture
- ✅ Minimal required fields
- ✅ Auto-save (never lose work)
- ✅ Multiple organization methods
- ✅ Flexible navigation
- ✅ Customizable notifications
- ✅ Reduce motion option
- ✅ Forgiveness (undo everything)

### Screen Reader Support

All UI elements properly labeled:
```html
<button aria-label="Quick capture task (Ctrl+N)">
  <PlusIcon aria-hidden="true" />
</button>

<div role="timer" aria-live="polite">
  15 minutes remaining
</div>
```

---

## Privacy & Security

### Data Ownership

**You own your data. Always.**

- ✅ Export all data anytime (JSON, CSV)
- ✅ Delete account = data deleted (GDPR right to erasure)
- ✅ Self-hosting option (complete control)
- ✅ No selling data ever
- ✅ No tracking for advertising

### Privacy Features

- ✅ End-to-end encryption (future)
- ✅ Local-first architecture (data stored on device)
- ✅ Optional cloud sync (user choice)
- ✅ Anonymous analytics (opt-in only)
- ✅ No third-party trackers
- ✅ Open source (audit the code)

### Security Measures

- ✅ HTTPS everywhere (TLS 1.3)
- ✅ JWT authentication
- ✅ Password hashing (bcrypt)
- ✅ Rate limiting
- ✅ SQL injection protection
- ✅ XSS prevention
- ✅ CSRF protection

**[View Security Architecture](diagrams/06-deployment-operations.md#security-layers)**

---

## Comparison with Traditional Tools

### vs. Generic Task Managers

| Feature | Todoist/TickTick | Altair |
|---------|------------------|--------|
| Quick capture | 3-5 seconds | <1 second |
| Task breakdown | Manual | AI-assisted |
| Time blindness | Basic timers | Visual time awareness |
| Working memory | Rely on memory | External support |
| Offline-first | Limited | Full functionality |
| ADHD-optimized | No | Yes |
| Open source | No | Yes |

### vs. Project Management Tools

| Feature | Asana/Trello | Altair |
|---------|--------------|--------|
| Setup time | 30+ minutes | 2 minutes |
| Learning curve | Steep | Gentle |
| Complexity | High | Appropriate |
| ADHD features | None | Core design |
| Mobile experience | Poor | Excellent |
| Offline mode | No | Yes |
| Price | $10-15/month | Free (self-host) |

### vs. ADHD Apps

| Feature | Forest/Habitica | Altair |
|---------|-----------------|--------|
| Gamification | Exploitative | Gentle |
| Streaks | Anxiety-inducing | Optional |
| Task management | Basic | Comprehensive |
| Privacy | Poor | Excellent |
| Customization | Limited | Extensive |
| Open source | No | Yes |

---

## Feature Roadmap

### Phase 1: Foundation (Q4 2025 - Q1 2026) 🚧

- ✅ Quick task capture
- ✅ Basic task management (CRUD)
- ✅ Simple time tracking
- ✅ Offline-first storage
- 🚧 Basic Flutter app
- 🚧 FastAPI backend
- 📅 PostgreSQL integration
- 📅 JWT authentication

### Phase 2: ADHD Features (Q2 2026)

- 📅 AI task breakdown
- 📅 Visual time awareness
- 📅 Focus mode
- 📅 Gentle gamification
- 📅 Auto documentation
- 📅 Context switching support

### Phase 3: Enhanced UX (Q3 2026)

- 📅 Cross-device sync
- 📅 Customizable UI
- 📅 Keyboard shortcuts
- 📅 Voice input
- 📅 Collaboration (shared projects)
- 📅 Advanced analytics

### Phase 4: Community & Polish (Q4 2026)

- 📅 Plugin system
- 📅 Themes and customization
- 📅 Import from other tools
- 📅 Mobile app stores (iOS/Android)
- 📅 Browser extensions
- 📅 Email integration

**[View Full Roadmap](ROADMAP.md)**  
**[View Development Timeline](diagrams/04-roadmap-planning.md#development-timeline-gantt-chart)**

---

## Questions?

**Feature Requests:**
- Open an issue: [github.com/getaltair/altair/issues](https://github.com/getaltair/altair/issues)
- Discuss on Discord: [discord.gg/altair](https://discord.gg/altair)

**General Questions:**
- Email: hello@getaltair.app
- Twitter: [@getaltair](https://twitter.com/getaltair)

---

**Last Updated:** October 2025  
**Status:** Pre-Alpha Development  
**Next:** [Read the Roadmap →](ROADMAP.md)

---

*Built with 💙 by people who understand the ADHD struggle*
