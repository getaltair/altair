# Development Roadmap

> **TL;DR:** 9-month plan. Guidance MVP in 3 months (dogfooding target). Knowledge in months 4-6. Tracking in months 7-9. Each phase: Foundation → Core features → Polish → Beta.

## Quick Start

**What you need to know in 60 seconds:**

- **Timeline**: 9 months total, 3 apps built sequentially
- **Phase 1** (Months 1-3): Altair Guidance - Task management MVP
- **Phase 2** (Months 4-6): Altair Knowledge - Personal wiki
- **Phase 3** (Months 7-9): Altair Tracking - Inventory management
- **Key milestone**: Dogfooding Guidance by end of Month 3

**Navigation:**

- [Architecture Overview](./ARCHITECTURE-OVERVIEW.md) - System design
- [Data Flow](./DATA-FLOW.md) - How data moves
- [Component Design](./COMPONENT-DESIGN.md) - Component breakdown
- [Deployment Guide](./DEPLOYMENT-GUIDE.md) - How to deploy

---

## Timeline Overview

9-month development plan with focus on shipping and dogfooding.

```mermaid
gantt
    title Altair Development Roadmap (9 Months + Mobile)
    dateFormat  YYYY-MM-DD
    section Phase 1: Guidance
    Infrastructure & Auth       :a1, 2025-10-17, 30d
    Core Task Management       :a2, after a1, 30d
    AI Features & Polish       :a3, after a2, 30d
    Guidance Beta Launch       :milestone, after a3, 0d

    section Phase 1.5: Mobile
    Step 1: iOS Setup          :a4, after a3, 7d
    Step 2: Optimization       :a5, after a4, 7d
    Mobile Beta Ready          :milestone, after a5, 0d

    section Phase 2: Knowledge
    Wiki Foundation            :b1, after a5, 30d
    Smart Connections          :b2, after b1, 30d
    External Brain Features    :b3, after b2, 30d
    Knowledge Beta Launch      :milestone, after b3, 0d

    section Phase 3: Tracking
    Basic Inventory            :c1, after b3, 30d
    Smart Tracking             :c2, after c1, 30d
    Ecosystem Integration      :c3, after c2, 30d
    Tracking Beta Launch       :milestone, after c3, 0d

    section Milestones
    Dogfooding Starts          :milestone, 2026-01-15, 0d
    First 100 Users            :milestone, 2026-04-15, 0d
    V1.0 Launch               :milestone, 2026-07-15, 0d
```

### Key Milestones

| Milestone               | Target Date | Criteria                                             |
| ----------------------- | ----------- | ---------------------------------------------------- |
| **Foundation Complete** | Month 1     | Auth works, SQLite schema ready, first UI component  |
| **Dogfooding Starts**   | Month 3     | Using Guidance daily for own project management      |
| **Guidance Beta**       | Month 3     | 10 external beta testers, < 5 critical bugs          |
| **Knowledge MVP**       | Month 6     | Personal wiki works, syncs across devices            |
| **Tracking MVP**        | Month 9     | Inventory tracking works, integrates with other apps |
| **V1.0 Launch**         | Month 9     | All three apps stable, < 1% crash rate               |

---

## Phase 1: Altair Guidance (Months 1-3)

Task and project management with ADHD-friendly features.

### Month 1: Infrastructure & Foundation

```mermaid
graph TB
    subgraph "Step 1-2: Setup"
        A1[Monorepo Structure]
        A2[CI/CD Pipeline]
        A3[Documentation Site]
    end

    subgraph "Step 3: Auth"
        B1[altair-auth Package]
        B2[JWT Implementation]
        B3[Secure Storage]
    end

    subgraph "Step 4: Database"
        C1[SQLite Schema]
        C2[Migrations System]
        C3[Basic CRUD]
    end

    A1 --> A2 --> A3
    A3 --> B1
    B1 --> B2 --> B3
    B3 --> C1
    C1 --> C2 --> C3

    style A1 fill:#FFD93D,stroke:#000,stroke-width:3px
    style B1 fill:#60A5FA,stroke:#000,stroke-width:3px
    style C1 fill:#6BCB77,stroke:#000,stroke-width:3px
```

**Deliverables:**

- ✅ Repository structure (`apps/`, `packages/`, `services/`)
- ✅ GitHub Actions CI/CD (lint, test, build)
- ✅ altair-auth package (JWT, secure storage)
- ✅ SQLite schema for tasks, projects, tags
- ✅ Basic CRUD repositories
- ✅ First Flutter desktop app runs

**Success criteria:**

- [ ] Can create/read/update/delete tasks locally
- [ ] Auth flow works (login/logout/refresh)
- [ ] Database migrations tested
- [ ] CI pipeline passes on all commits

### Month 2: Core Task Management

```mermaid
graph TB
    subgraph "Step 1-2: altair-ui Package"
        A1[Neo-Brutalist Theme]
        A2[Quick Capture Widget]
        A3[Time Blindness Timer]
        A4[Task Card Component]
    end

    subgraph "Step 3: Features"
        B1[Task Creation & Editing]
        B2[Project Management]
        B3[Tags & Filtering]
        B4[Search Functionality]
    end

    subgraph "Step 4: UX Polish"
        C1[Keyboard Shortcuts]
        C2[Focus Mode]
        C3[Drag & Drop]
        C4[Undo/Redo]
    end

    A1 --> A2 --> A3 --> A4
    A4 --> B1
    B1 --> B2 --> B3 --> B4
    B4 --> C1
    C1 --> C2 --> C3 --> C4

    style A1 fill:#FFD93D,stroke:#000,stroke-width:3px
    style B1 fill:#60A5FA,stroke:#000,stroke-width:3px
    style C1 fill:#6BCB77,stroke:#000,stroke-width:3px
```

**Deliverables:**

- ✅ altair-ui package with reusable components
- ✅ Quick capture (< 3 seconds from keyboard shortcut)
- ✅ Task breakdown and subtasks
- ✅ Project organization
- ✅ Tags, filters, search
- ✅ Time tracking with visual feedback

**Success criteria:**

- [ ] Thought to capture < 3 seconds
- [ ] Can manage 100+ tasks without slowdown
- [ ] Quick capture works from anywhere (global hotkey)
- [ ] Visual progress clear at a glance

### Month 3: AI Features & Beta

```mermaid
graph TB
    subgraph "Step 1: AI Integration"
        A1[AI Service Package]
        A2[OpenAI Integration]
        A3[Anthropic Integration]
        A4[Ollama Support]
    end

    subgraph "Step 2: AI Features"
        B1[Task Breakdown Assistant]
        B2[Smart Prioritization]
        B3[Time Estimates]
        B4[Context Suggestions]
    end

    subgraph "Step 3-4: Polish & Beta"
        C1[Bug Fixes]
        C2[Performance Optimization]
        C3[Standalone Installers]
        C4[Beta Testing]
    end

    A1 --> A2 --> A3 --> A4
    A4 --> B1
    B1 --> B2 --> B3 --> B4
    B4 --> C1
    C1 --> C2 --> C3 --> C4

    style A1 fill:#FFD93D,stroke:#000,stroke-width:3px
    style B1 fill:#60A5FA,stroke:#000,stroke-width:3px
    style C1 fill:#6BCB77,stroke:#000,stroke-width:3px
```

**Deliverables:**

- ✅ AI task breakdown (GPT-4/Claude)
- ✅ Local AI support (Ollama)
- ✅ Smart prioritization suggestions
- ✅ Time estimate assistance
- ✅ Standalone installers (macOS, Windows, Linux)
- ✅ Beta testing with 10 users

**Success criteria:**

- [ ] Using Guidance daily for own tasks (dogfooding)
- [ ] AI breakdown < 5 seconds
- [ ] Installers work on clean systems
- [ ] < 5 critical bugs reported
- [ ] 8/10 beta testers rate "would use daily"

**Dogfooding milestone:**

- Using Guidance to manage Altair development
- Track all tasks, bugs, features in Guidance
- Test every feature personally before shipping

---

## Phase 1.5: Mobile Platform Support

**Priority shift:** Mobile development begins immediately to target the most ubiquitous device platform.

### Step 1: iOS Platform Setup ✅

**Focus areas:**

- Enable iOS platform for altair_guidance
- Configure iOS build settings and permissions
- Test basic app functionality on iOS
- Ensure UI components work on mobile screens

**Deliverables:**

- ✅ iOS platform enabled
- ✅ iOS build configuration complete
- ✅ Basic app runs on iOS simulator/device
- ✅ UI responsive on iPhone screens

**Success criteria:**

- [x] App builds and runs on iOS devices
- [x] Core features work on mobile
- [x] UI adapts to mobile screen sizes
- [x] No critical iOS-specific bugs

### Step 2: Mobile Optimization & Testing ✅

**Focus areas:**

- Mobile-specific UI/UX improvements
- Touch gesture support
- Mobile performance optimization
- Android testing and refinement
- Cross-platform testing (iOS + Android)

**Deliverables:**

- ✅ Touch-optimized UI components
- ✅ Gesture navigation support (swipe-to-delete, long-press, pull-to-refresh)
- ✅ Performance optimization for mobile
- ✅ Tested on emulators (217 tests passing, 7 integration tests)
- ✅ Mobile CI/CD pipeline created
- ✅ Device testing documentation complete
- ⏳ Physical device testing (deferred to user availability)

**Success criteria:**

- [x] Quick capture works on mobile
- [x] Task management smooth on touch devices
- [x] Platform-specific features implemented (SafeArea, keyboard, back button)
- [x] Mobile tests comprehensive and passing
- [ ] Performance validated on physical devices (pending device access)
- [ ] Beta testers can use mobile version daily (pending physical testing)

### Step 3: Physical Device Validation ⏳

**Focus areas:**

- Test on real Android devices
- Test on real iOS devices (requires macOS)
- Measure performance baselines
- Validate across multiple screen sizes
- Prepare for app store submissions

**Deliverables:**

- ⏳ Android device testing complete
- ⏳ iOS device testing complete (requires macOS)
- ⏳ Performance baselines documented
- ⏳ App store assets prepared
- ⏳ Beta distribution via TestFlight/Play Store

**Success criteria:**

- [ ] Tested on 3+ Android devices
- [ ] Tested on 3+ iOS devices
- [ ] Performance meets targets on mid-range devices
- [ ] Ready for public beta distribution

---

## Phase 2: Altair Knowledge (Months 4-6)

Personal wiki and knowledge management system.

### Month 4: Wiki Foundation

**Focus areas:**

- Markdown editor with live preview
- Page organization (folders, tags)
- Basic linking between pages
- Search functionality
- SQLite schema for wiki pages

**Deliverables:**

- ✅ Wiki page CRUD operations
- ✅ Markdown editor component
- ✅ Page hierarchy/organization
- ✅ Basic [[wiki-link]] syntax
- ✅ Full-text search across pages

**Success criteria:**

- [ ] Can create/edit pages with Markdown
- [ ] Internal links work
- [ ] Search returns results < 500ms
- [ ] Supports 1000+ pages without slowdown

### Month 5: Smart Connections

**Focus areas:**

- Backlinks (pages linking to current page)
- Graph view (visualize connections)
- Smart suggestions (related pages)
- Tag-based organization
- Templates for common page types

**Deliverables:**

- ✅ Automatic backlinks
- ✅ Interactive graph visualization
- ✅ AI-powered page suggestions
- ✅ Tag hierarchy and relationships
- ✅ Page templates (meeting notes, project brief, etc.)

**Success criteria:**

- [ ] Backlinks update in real-time
- [ ] Graph view renders 1000+ nodes
- [ ] AI suggests relevant related pages
- [ ] Templates save time on common tasks

### Month 6: External Brain Features

**Focus areas:**

- Daily notes (automatic daily pages)
- Quick capture from anywhere
- Web clipper (save articles)
- PDF annotation
- PowerSync integration for sync

**Deliverables:**

- ✅ Daily notes with templates
- ✅ Quick capture widget (global hotkey)
- ✅ Web content import
- ✅ Basic PDF annotation
- ✅ Multi-device sync (PowerSync)
- ✅ Knowledge beta launch

**Success criteria:**

- [ ] Daily notes auto-created
- [ ] Can clip web pages in < 5 seconds
- [ ] Sync works across devices
- [ ] Using Knowledge daily for notes (dogfooding)
- [ ] 10 beta testers actively using

---

## Phase 3: Altair Tracking (Months 7-9)

Inventory and resource management system.

### Month 7: Basic Inventory

**Focus areas:**

- Item creation and categorization
- Location management
- Barcode scanning (mobile)
- Basic search and filtering
- Photo attachments

**Deliverables:**

- ✅ Item CRUD operations
- ✅ Location hierarchy
- ✅ Barcode scanner (mobile)
- ✅ Photo upload/storage
- ✅ Quantity tracking

**Success criteria:**

- [ ] Can track 1000+ items
- [ ] Barcode scanning works reliably
- [ ] Photo uploads < 2 seconds
- [ ] Locations organize logically

### Month 8: Smart Tracking

**Focus areas:**

- Low stock alerts
- Usage predictions (AI)
- Quick add via photo (AI recognition)
- Shopping list generation
- Expiration tracking

**Deliverables:**

- ✅ Automated alerts (low stock, expiring)
- ✅ AI item recognition from photos
- ✅ Usage pattern analysis
- ✅ Smart shopping lists
- ✅ Expiration date tracking

**Success criteria:**

- [ ] Alerts prevent running out
- [ ] AI correctly identifies 80% of items from photos
- [ ] Shopping lists save time
- [ ] No expired items forgotten

### Month 9: Ecosystem Integration

**Focus areas:**

- Cross-app linking (items ↔ tasks ↔ wiki pages)
- Unified search across all apps
- Activity feed (all app events)
- Shared tags across apps
- PowerSync for all three apps

**Deliverables:**

- ✅ Cross-app resource linking
- ✅ Global search (all apps)
- ✅ Activity feed dashboard
- ✅ Tag synchronization
- ✅ Full ecosystem sync working
- ✅ V1.0 launch

**Success criteria:**

- [ ] All three apps work together seamlessly
- [ ] Unified search finds results across apps
- [ ] Activity feed provides useful overview
- [ ] Sync works reliably across all apps
- [ ] Using all three apps daily (full dogfooding)

---

## Feature Dependency Tree

How features build on each other.

```mermaid
graph TB
    subgraph "Foundation (Month 1)"
        F1[Monorepo Setup]
        F2[Auth System]
        F3[SQLite Schema]
        F4[Basic UI Components]
    end

    subgraph "Core Features (Months 2-3)"
        C1[Task Management]
        C2[Quick Capture]
        C3[AI Integration]
        C4[Sync Foundation]
    end

    subgraph "Advanced Features (Months 4-6)"
        A1[Wiki Pages]
        A2[Backlinks & Graph]
        A3[Smart Search]
        A4[Multi-device Sync]
    end

    subgraph "Integration (Months 7-9)"
        I1[Inventory Tracking]
        I2[Cross-app Links]
        I3[Unified Search]
        I4[Activity Feed]
    end

    F1 --> F2
    F1 --> F3
    F1 --> F4

    F2 --> C1
    F3 --> C1
    F4 --> C2
    C1 --> C3
    C2 --> C3

    C1 --> A1
    C3 --> A2
    C1 --> A3
    F2 --> A4
    C4 --> A4

    A1 --> I1
    A3 --> I2
    A4 --> I2
    A2 --> I3
    A3 --> I3
    I2 --> I4

    style F1 fill:#FFD93D,stroke:#000,stroke-width:3px
    style C1 fill:#60A5FA,stroke:#000,stroke-width:3px
    style A1 fill:#6BCB77,stroke:#000,stroke-width:3px
    style I1 fill:#FF6B6B,stroke:#000,stroke-width:3px
```

### Critical Path

**Month 1:** Foundation → **Month 2:** Task Management → **Month 3:** AI + Beta → **Month 6:** Knowledge + Sync → **Month 9:** Full Ecosystem

**Blockers to watch:**

- PowerSync integration (complex, affects all apps)
- AI integration (API limits, costs)
- Standalone installers (platform-specific issues)
- Performance at scale (1000+ tasks/pages/items)

---

## MVP Criteria by App

What defines "minimum viable" for each app.

### Guidance MVP Checklist

**Core functionality:**

- [ ] Quick capture (< 3 seconds)
- [ ] Create/edit/delete tasks
- [ ] Organize into projects
- [ ] Tag and filter tasks
- [ ] Search tasks (< 500ms)
- [ ] Mark tasks complete
- [ ] Basic time tracking

**ADHD features:**

- [ ] Time blindness timer (visual)
- [ ] Focus mode (hide distractions)
- [ ] Keyboard shortcuts (power users)
- [ ] Quick task breakdown
- [ ] Visual progress indicators

**AI features:**

- [ ] Task breakdown (GPT-4/Claude)
- [ ] Time estimates
- [ ] Smart prioritization

**Technical:**

- [ ] Works 100% offline
- [ ] Sync across devices (optional)
- [ ] < 1 second page loads
- [ ] Standalone installers
- [ ] < 5 critical bugs

**Dogfooding proof:**

- [ ] Using daily for ≥ 2 weeks
- [ ] Managing ≥ 50 active tasks
- [ ] AI breakdown used ≥ 10 times
- [ ] Completed ≥ 1 project with subtasks

### Knowledge MVP Checklist

**Core functionality:**

- [ ] Create/edit wiki pages (Markdown)
- [ ] Internal linking ([[page-name]])
- [ ] Backlinks (auto-generated)
- [ ] Graph view (page connections)
- [ ] Full-text search
- [ ] Tag organization
- [ ] Daily notes (auto-created)

**Smart features:**

- [ ] AI page suggestions
- [ ] Related pages discovery
- [ ] Web clipper
- [ ] Quick capture from anywhere

**Technical:**

- [ ] Works offline
- [ ] Sync with Guidance app
- [ ] Supports 1000+ pages
- [ ] Search < 500ms

**Dogfooding proof:**

- [ ] Using for project notes ≥ 2 weeks
- [ ] Created ≥ 30 interconnected pages
- [ ] Graph view shows useful connections
- [ ] Daily notes habit established

### Tracking MVP Checklist

**Core functionality:**

- [ ] Add/edit items
- [ ] Organize by location
- [ ] Barcode scanning (mobile)
- [ ] Quantity tracking
- [ ] Photo attachments
- [ ] Search items

**Smart features:**

- [ ] Low stock alerts
- [ ] AI item recognition (photos)
- [ ] Shopping list generation
- [ ] Expiration tracking

**Integration:**

- [ ] Link items to tasks
- [ ] Link items to wiki pages
- [ ] Unified search across apps
- [ ] Activity feed

**Technical:**

- [ ] Works offline
- [ ] Syncs with other apps
- [ ] Supports 1000+ items

**Dogfooding proof:**

- [ ] Tracking personal inventory ≥ 2 weeks
- [ ] Scanned ≥ 20 barcodes
- [ ] Used shopping list feature
- [ ] Cross-app links useful

---

## Dogfooding Plan

Using Altair to build Altair (meta!).

```mermaid
graph TB
    subgraph "Phase 1 Complete"
        D1[Switch to Guidance for all task tracking]
        D2[Log bugs as tasks in Guidance]
        D3[Plan sprints in Guidance]
    end

    subgraph "Month 4-6"
        D4[Document decisions in Knowledge]
        D5[Link tasks ↔ wiki pages]
        D6[Architecture notes in Knowledge]
    end

    subgraph "Month 7-9"
        D7[Track development equipment in Tracking]
        D8[Link components to tasks & docs]
        D9[Full ecosystem dogfooding]
    end

    D1 --> D2 --> D3
    D3 --> D4
    D4 --> D5 --> D6
    D6 --> D7
    D7 --> D8 --> D9

    style D1 fill:#FFD93D,stroke:#000,stroke-width:3px
    style D4 fill:#60A5FA,stroke:#000,stroke-width:3px
    style D7 fill:#6BCB77,stroke:#000,stroke-width:3px
```

### Dogfooding Benefits

**Find bugs early:**

- Experience issues before users do
- Real-world usage patterns
- Edge cases discovered naturally

**Feature validation:**

- Does quick capture actually save time?
- Is AI breakdown useful or gimmicky?
- Which features get used daily?

**UX improvements:**

- Identify friction in workflows
- Discover missing features
- Validate ADHD-friendly design choices

**Credibility:**

- "We use it to build it"
- Show real project management in Guidance
- Authentic testimonials from team

---

## Risk Mitigation

Potential issues and how to handle them.

### Technical Risks

**PowerSync integration complexity**

- Risk: Takes longer than expected
- Mitigation: Start with basic sync, iterate
- Fallback: Ship without sync first, add later

**Performance at scale**

- Risk: Slow with 1000+ tasks/pages
- Mitigation: Profile early, optimize incrementally
- Fallback: Pagination, virtual scrolling

**AI API costs**

- Risk: OpenAI/Anthropic bills too high
- Mitigation: Implement rate limiting, caching
- Fallback: Ollama (free, local) as default

### Schedule Risks

**Scope creep**

- Risk: Adding too many features
- Mitigation: Stick to MVP checklists
- Strategy: "One thing well" philosophy

**Dogfooding delays**

- Risk: Don't use Guidance by Month 3
- Mitigation: Force switch, no alternatives
- Accountability: Public commitment to timeline

**Burnout**

- Risk: 9 months is long
- Mitigation: One app at a time, celebrate milestones
- Strategy: Ship early, ship often

### Adoption Risks

**Too complex for users**

- Risk: Features overwhelm new users
- Mitigation: Onboarding flow, progressive disclosure
- Strategy: Start simple, unlock features gradually

**Not ADHD-friendly enough**

- Risk: Doesn't actually help ADHD users
- Mitigation: User testing with ADHD community
- Strategy: Discord feedback, iterate based on needs

---

## Success Metrics

How we measure progress and success.

### Development Metrics

**Velocity:**

- Target: Ship 1 major feature/week
- Track: GitHub issues closed per week
- Goal: Maintain consistent pace

**Quality:**

- Target: < 5 critical bugs per release
- Track: Bug severity distribution
- Goal: 95% of bugs non-critical

**Dogfooding:**

- Target: Daily usage by Month 3
- Track: Personal usage logs
- Goal: Can't live without it

### User Metrics (Post-Launch)

**Engagement:**

- Target: 1000 daily active users by Month 6
- Track: Analytics (privacy-respecting)
- Goal: 80% week-over-week retention

**Performance:**

- Target: 80% task breakdown success rate
- Track: AI feature usage & success
- Goal: Users complete more tasks

**Satisfaction:**

- Target: 8/10 "would recommend" score
- Track: User surveys
- Goal: Genuine enthusiasm from ADHD community

---

## What's Next?

### Immediate Actions (Phase 1 Step 1)

**Day 1-2:**

- [ ] Create GitHub repository
- [ ] Setup monorepo structure (`apps/`, `packages/`, `services/`)
- [ ] Initialize first Flutter app (altair-guidance)
- [ ] Setup CI/CD (GitHub Actions)

**Day 3-4:**

- [ ] Create altair-ui package
- [ ] Implement neo-brutalist theme
- [ ] Build first component (AltairButton)
- [ ] Setup Storybook for component dev

**Day 5:**

- [ ] Create altair-core package
- [ ] Define Task model
- [ ] Setup SQLite database
- [ ] Write first test

### Monthly Check-ins

**End of Month 1:**

- Review: Foundation complete?
- Adjust: Timeline, scope, priorities
- Celebrate: First working features

**End of Month 3:**

- Review: Ready for dogfooding?
- Adjust: Beta timeline, feature cuts
- Celebrate: First public beta

**End of Month 6:**

- Review: Knowledge ready?
- Adjust: Tracking scope, integration priorities
- Celebrate: Two apps shipping

**End of Month 9:**

- Review: V1.0 ready?
- Launch: Public v1.0 release
- Celebrate: Full ecosystem shipped!

---

## Related Documentation

- [Architecture Overview](./ARCHITECTURE-OVERVIEW.md) - High-level system design
- [Data Flow](./DATA-FLOW.md) - How data moves through system
- [Component Design](./COMPONENT-DESIGN.md) - Component breakdown
- [Deployment Guide](./DEPLOYMENT-GUIDE.md) - How to deploy

---

## FAQ

**Q: Why 9 months? Can't it be faster?**
A: Quality over speed. Each app needs 3 months to do it right. Rushing leads to technical debt.

**Q: What if Month 3 dogfooding reveals Guidance isn't working?**
A: Fix it before moving to Knowledge. No point building more apps if first one fails.

**Q: Can features be added after initial release?**
A: Yes! These are MVPs. Iterate based on user feedback.

**Q: What about mobile apps?**
A: Mobile development starts immediately after Phase 1 completion. Android already enabled, iOS to be configured. Mobile and desktop platforms developed in parallel.

**Q: How do you avoid burnout over 9 months?**
A: One app at a time. Celebrate milestones. Ship early, get feedback, stay motivated.

**Q: What if PowerSync doesn't work out?**
A: Ship standalone first. Sync is optional enhancement, not blocker.

---

**Last updated:** October 22, 2025
**Next review:** November 17, 2025 (Monthly)
**Accountability:** Weekly dev logs on Discord
**Recent change:** Added Phase 1.5 (Mobile Platform Support) - prioritizing mobile development after Phase 1 completion
