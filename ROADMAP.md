# Altair Roadmap

**Where focus takes flight** - Development Timeline & Priorities

> This document outlines Altair's development phases, priorities, and timeline from pre-alpha to stable release.

📊 **Visual References:**
- [Development Timeline (Gantt Chart)](diagrams/04-roadmap-planning.md#development-timeline-gantt-chart)
- [Feature Priority Matrix](diagrams/04-roadmap-planning.md#feature-priority-matrix)
- [Sprint Planning Diagrams](diagrams/04-roadmap-planning.md#sprint-1-planning)
- [Dependency Graph](diagrams/04-roadmap-planning.md#feature-dependency-graph)

---

## Current Status

**Phase:** Pre-Alpha Development  
**Version:** 0.1.0-dev  
**Start Date:** October 2025  
**Target MVP:** Q1 2026 (March 2026)

**[View Current Sprint Plan](diagrams/04-roadmap-planning.md#sprint-1-planning)**

---

## Development Philosophy

### Build in Public
We're developing Altair transparently:
- Weekly progress updates on Twitter/Discord
- Open GitHub project board
- Public roadmap (this document)
- Community input on feature priorities

### Dogfooding First
We use Altair to build Altair:
- Test features with real ADHD users (ourselves)
- Identify pain points early
- Iterate based on actual use
- Ship when genuinely useful

### ADHD-Friendly Development
Our process accommodates neurodivergent developers:
- Flexible deadlines (no crunch)
- Clear, achievable milestones
- Celebrate small wins
- Transparent about blockers

**[View ADHD Features Mindmap](diagrams/04-roadmap-planning.md#adhd-features-mindmap)**

---

## Release Timeline

### Phase 1: Foundation & MVP (Q4 2025 - Q1 2026) 🚧 **Current**

**Goal:** Build usable core that solves the #1 ADHD problem: task capture

**Duration:** 3-4 months  
**Status:** In Progress  
**Target:** March 2026

**[View Phase 1 Timeline](diagrams/04-roadmap-planning.md#development-timeline-gantt-chart)**

#### Milestones:

**M1: Backend Foundation** (November 2025) ✅
- [x] FastAPI project structure
- [x] PostgreSQL database setup
- [x] User authentication (JWT)
- [x] Basic task CRUD API
- [x] Docker compose for development

**M2: Frontend Foundation** (December 2025) 🚧
- [ ] Flutter project structure
- [ ] Authentication UI (login/register)
- [ ] Basic task list view
- [ ] Quick capture dialog
- [ ] Offline storage (Hive)

**M3: Dogfooding MVP** (January 2026) 📅
- [ ] Sync mechanism (local ↔ server)
- [ ] Time tracking start/stop
- [ ] Basic project organization
- [ ] Data export functionality
- [ ] Deploy personal instance

**M4: Private Beta** (February-March 2026) 📅
- [ ] Bug fixes from dogfooding
- [ ] Performance optimization (<100ms API)
- [ ] Mobile app (iOS/Android)
- [ ] Invite 10-20 ADHD beta testers
- [ ] Gather feedback

**Key Features:**
- ✅ Ultra-fast task capture
- ✅ Offline-first architecture
- ✅ Basic time tracking
- ✅ Simple project organization
- ✅ Cross-device sync

**Success Criteria:**
- [ ] Daily active use by 2+ team members
- [ ] <50ms quick capture response time
- [ ] Zero data loss in 30 days
- [ ] Positive feedback from 5+ beta testers

**[View Phase 1 Dependency Graph](diagrams/04-roadmap-planning.md#feature-dependency-graph)**

---

### Phase 2: ADHD-Specific Features (Q2 2026)

**Goal:** Add features that make Altair uniquely ADHD-friendly

**Duration:** 3 months  
**Target:** June 2026

**[View Phase 2 Feature Matrix](diagrams/04-roadmap-planning.md#feature-priority-matrix)**

#### Core ADHD Features:

**AI Task Breakdown** (April 2026)
- Natural language task analysis
- Suggested subtask generation
- Time & energy estimation
- Context recommendations
- Integration with OpenAI/Anthropic APIs

**[View AI Breakdown Architecture](diagrams/01-system-architecture.md#ai-task-breakdown-service)**

**Visual Time Awareness** (April-May 2026)
- Progress bars for active tasks
- Time bucket visualization
- Estimated vs actual tracking
- Color-coded time warnings
- Customizable time displays

**[View Time Tracking Components](diagrams/03-user-flows.md#time-tracking-session)**

**Focus Mode** (May 2026)
- Distraction-free single task view
- Integrated Pomodoro timer
- Break reminders
- Ambient sounds integration
- Hyperfocus protection warnings

**[View Focus Mode Flow](diagrams/03-user-flows.md#focus-mode-session)**

**Gentle Gamification** (June 2026)
- Daily completion celebrations
- Weekly progress reviews
- Optional achievement badges
- NO streaks or FOMO mechanics
- Fully opt-in system

**Auto Documentation** (June 2026)
- Quick note capture (Ctrl+/)
- End-of-day reflection prompts
- Session summaries
- Decision logging
- Insight extraction

**[View Documentation Flow](diagrams/03-user-flows.md#documentation-capture)**

**Success Criteria:**
- [ ] AI breaks down 80%+ of complex tasks usefully
- [ ] Users report better time awareness
- [ ] Focus sessions average 25+ minutes
- [ ] Gamification feels supportive, not stressful
- [ ] 50+ active beta testers

---

### Phase 3: Enhanced UX & Performance (Q3 2026)

**Goal:** Polish the experience and optimize performance

**Duration:** 3 months  
**Target:** September 2026

**[View Phase 3 Architecture](diagrams/05-component-architecture.md#performance-optimization)**

#### Key Improvements:

**Cross-Platform Sync** (July 2026)
- Robust conflict resolution
- Delta sync for efficiency
- Real-time sync option (WebSocket)
- Sync status indicators
- Manual sync controls

**[View Sync Architecture](diagrams/01-system-architecture.md#offline-sync-architecture)**

**Performance Optimization** (July-August 2026)
- API response <100ms (99th percentile)
- Frontend rendering <16ms (60fps)
- Lazy loading of large lists
- Image optimization
- Database query optimization

**Advanced Customization** (August 2026)
- Customizable keyboard shortcuts
- UI theme customization
- Notification preferences
- Widget configuration
- Energy level profiles

**Keyboard-First Experience** (August-September 2026)
- Comprehensive keyboard shortcuts
- Command palette (Ctrl+K)
- Vim-like navigation (optional)
- Quick actions
- Accessibility improvements

**Collaboration Features** (September 2026)
- Shared projects (basic)
- Task assignment
- Comments on tasks
- Activity feed
- Permission controls

**Success Criteria:**
- [ ] <100ms API response time (p99)
- [ ] 60fps UI rendering
- [ ] Sync conflicts <1% of operations
- [ ] 100+ active users
- [ ] Net Promoter Score >50

---

### Phase 4: Community & Growth (Q4 2026)

**Goal:** Grow the community and prepare for public launch

**Duration:** 3 months  
**Target:** December 2026

#### Community Building:

**Public Beta Launch** (October 2026)
- Open beta signups
- Waitlist management
- Onboarding flow
- Video tutorials
- Help documentation

**App Store Releases** (October-November 2026)
- iOS App Store submission
- Google Play Store submission
- App Store Optimization (ASO)
- Screenshots and descriptions
- Review response system

**Plugin System** (November 2026)
- Plugin API design
- Developer documentation
- Example plugins
- Plugin marketplace (future)
- Community plugin showcase

**Import/Export** (November-December 2026)
- Import from Todoist
- Import from Trello/Asana
- Import from Notion
- Import from Apple Reminders
- Universal CSV import

**Browser Extensions** (December 2026)
- Chrome extension (quick capture)
- Firefox extension
- Safari extension
- Web clipper functionality
- Right-click menu integration

**Success Criteria:**
- [ ] 1,000+ registered users
- [ ] App store ratings >4.5/5
- [ ] 50+ community contributors
- [ ] 10+ community plugins
- [ ] Positive press coverage

---

### Phase 5: Sustainability & Scale (2027+)

**Goal:** Make Altair sustainable and scale to serve more users

**Duration:** Ongoing  
**Target:** Throughout 2027

#### Sustainability:

**Managed Hosting Service** (Q1 2027)
- Optional paid cloud hosting
- Automated backups
- Performance monitoring
- Customer support
- Transparent pricing ($5-10/month)

**[View Managed Hosting Architecture](diagrams/06-deployment-operations.md#managed-hosting-cloud)**

**Enterprise Features** (Q2 2027)
- Team accounts
- SSO integration
- Advanced permissions
- Audit logs
- SLA guarantees

**Advanced AI** (Q3 2027)
- Better task breakdown
- Predictive scheduling
- Pattern recognition
- Productivity insights
- Natural language interface

**International Expansion** (Q4 2027)
- Multi-language support
- Localization
- Regional compliance (GDPR, etc.)
- Local community building
- International partnerships

**Success Criteria:**
- [ ] 10,000+ total users
- [ ] Break-even on hosting costs
- [ ] Active community governance
- [ ] Sustainable development pace
- [ ] Maintained ADHD-first principles

---

## Feature Prioritization

### Priority Framework

We prioritize features based on:

**[View Priority Matrix](diagrams/04-roadmap-planning.md#feature-priority-matrix)**

1. **ADHD Impact** - How much does this help ADHD users specifically?
2. **Dogfooding Value** - Can we use this immediately ourselves?
3. **Build in Public** - Can this be showcased publicly as "advertising"?
4. **Core vs Nice-to-Have** - Is this essential or enhancement?
5. **Effort vs Value** - Return on development time?

### High Priority (Must Have)

These solve core ADHD pain points:
- ✅ Quick task capture
- ✅ Offline-first architecture
- 📅 Time tracking
- 📅 AI task breakdown
- 📅 Visual time awareness
- 📅 Focus mode

### Medium Priority (Should Have)

These improve the experience significantly:
- 📅 Gentle gamification
- 📅 Auto documentation
- 📅 Keyboard shortcuts
- 📅 Cross-platform sync
- 📅 Mobile apps

### Low Priority (Nice to Have)

These are valuable but not essential:
- 📅 Collaboration features
- 📅 Plugin system
- 📅 Browser extensions
- 📅 Voice input
- 📅 Email integration

### Future Consideration

These might be valuable but require more research:
- Smart scheduling AI
- Calendar integration
- Habit tracking
- Goal setting frameworks
- Community challenges

---

## Risks & Mitigation

**[View Risk Mitigation Diagram](diagrams/04-roadmap-planning.md#risk-mitigation-diagram)**

### Technical Risks

**Risk: Flutter web performance issues**
- Mitigation: Test early, optimize aggressively, consider React fallback

**Risk: Sync conflicts too complex**
- Mitigation: Simple last-write-wins first, iterate based on needs

**Risk: AI task breakdown unreliable**
- Mitigation: Make it optional, allow manual editing, improve over time

### Product Risks

**Risk: Feature creep**
- Mitigation: Strict prioritization, say no to non-ADHD features

**Risk: Too complex for ADHD users**
- Mitigation: Constant dogfooding, user testing, simplify ruthlessly

**Risk: Not differentiated enough**
- Mitigation: Focus on unique ADHD features, avoid copying others

### Business Risks

**Risk: Can't sustain development**
- Mitigation: Managed hosting revenue, sponsorships, keep core free

**Risk: Community doesn't grow**
- Mitigation: Build in public, engage on Twitter/Discord, provide value

**Risk: Competitors emerge**
- Mitigation: Open source advantage, community moat, stay authentic

---

## Community Involvement

### How to Influence the Roadmap

1. **Share ADHD Pain Points** - What frustrates you most?
2. **Vote on Features** - GitHub discussions & Discord polls
3. **Beta Test** - Try new features, give feedback
4. **Contribute Code** - Help build prioritized features
5. **Spread the Word** - More users = more feedback = better prioritization

### Monthly Community Reviews

Last Friday of each month:
- Progress update
- Demo new features
- Community Q&A
- Roadmap adjustments
- Celebrate wins

**Join on Discord:** [discord.gg/altair](https://discord.gg/altair)

---

## Public Milestones

These are our commitments to the community:

**November 2025:** Backend foundation complete ✅  
**December 2025:** Flutter app functional 🚧  
**January 2026:** Personal dogfooding in production 📅  
**March 2026:** Private beta with 10-20 testers 📅  
**June 2026:** ADHD features complete 📅  
**October 2026:** Public beta launch 📅  
**December 2026:** App stores + plugins 📅  
**March 2027:** Managed hosting available 📅  

---

## Questions About the Roadmap?

- **GitHub Discussions:** [github.com/getaltair/altair/discussions](https://github.com/getaltair/altair/discussions)
- **Discord:** [discord.gg/altair](https://discord.gg/altair)
- **Email:** roadmap@getaltair.app

---

**Last Updated:** October 2025  
**Next Review:** November 2025  
**Status:** Living Document

---

## Related Documentation

- [Features](FEATURES.md) - What we're building and why
- [Architecture](ARCHITECTURE.md) - How we're building it
- [Contributing](CONTRIBUTING.md) - How to help build it
- [Visual Diagrams](diagrams/README.md) - All roadmap and planning diagrams

---

*This roadmap is a living document. Dates are estimates, not promises. We'll adjust based on community feedback, technical discoveries, and ADHD realities.* 💙
