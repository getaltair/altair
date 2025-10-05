# Altair Project Roadmap

**Where focus takes flight**

---

## Overview

This roadmap outlines Altair's development journey.
We're building in public and dogfooding from day one—using Altair to manage
Altair's development.
This ensures features actually work for ADHD brains.

### Guiding Principles

1. **Dogfooding First** - Build what we need to manage this project
2. **Open Source Primary** - Self-hosted options before managed services
3. **Progressive Enhancement** - Start simple, add complexity only when needed
4. **Public Advertising** - Host live demos of features as we build them
5. **ADHD-Friendly Pace** - Sustainable development without burnout

---

## Current Status

**Phase:** Pre-Alpha Development  
**Focus:** Foundation & Core Task Management  
**Started:** October 2025  
**Next Milestone:** Dogfooding-Ready MVP (Q4 2025)

---

## Phase 1: Foundation & Dogfooding MVP (Q4 2025 - Q1 2026)

**Goal:** Build enough functionality to manage Altair's own development

### Milestone 1.1: Project Setup (Q4 2025)

**Status:** 🔄 In Progress

- [x] Domain registration (getaltair.app)
- [x] Repository structure (github.com/getaltair/altair)
- [ ] Development environment setup
  - [ ] Docker Compose configuration
  - [ ] Local PostgreSQL setup
  - [ ] FastAPI project scaffolding
  - [ ] Flutter project initialization
- [ ] Basic CI/CD pipeline
  - [ ] GitHub Actions for backend tests
  - [ ] Flutter web build pipeline
  - [ ] Docker image builds
- [ ] Documentation foundation
  - [x] README.md
  - [x] ARCHITECTURE.md
  - [x] ROADMAP.md
  - [x] CONTRIBUTING.md
  - [x] FEATURES.md

### Milestone 1.2: Authentication & User Management (Q4 2025)

**Status:** ⏳ Planned

- [ ] User registration and login
- [ ] JWT-based authentication
- [ ] Password reset flow
- [ ] User profile management
- [ ] Basic settings page

**Dogfooding:** Single-user mode for project creator

### Milestone 1.3: Core Task Management (Q4 2025)

**Status:** ⏳ Planned

**Backend:**

- [ ] Task CRUD operations
- [ ] Project organization
- [ ] Task hierarchy (parent/child tasks)
- [ ] Task status and priority
- [ ] Basic search and filtering

**Frontend:**

- [ ] Task list view
- [ ] Task creation/editing modal
- [ ] Quick capture input (always visible)
- [ ] Keyboard shortcuts for rapid entry
- [ ] Project sidebar navigation

**Dogfooding:** Start managing Altair development in Altair  
**Public Demo:** Host live instance showing task management

### Milestone 1.4: Documentation System (Q4 2025)

**Status:** ⏳ Planned

- [ ] Note/document creation
- [ ] Link notes to tasks/projects
- [ ] Markdown support
- [ ] Quick note capture
- [ ] Search across notes

**Dogfooding:** Document architecture decisions and feature specs in Altair  
**Public Demo:** Share public documentation workspace

### Milestone 1.5: Web Deployment (Q4 2025)

**Status:** ⏳ Planned

- [ ] Production Docker setup
- [ ] Nginx configuration
- [ ] SSL/HTTPS setup
- [ ] Database backup strategy
- [ ] Basic monitoring (uptime, errors)
- [ ] Self-hosting documentation

**Dogfooding:** Deploy personal instance  
**Public Demo:** Public-facing demo instance at demo.getaltair.app

---

## Phase 2: ADHD-Specific Features (Q1 2026)

**Goal:** Implement features that make Altair uniquely helpful for ADHD users

### Milestone 2.1: Visual Time Awareness (Q1 2026)

**Status:** ⏳ Planned

- [ ] Time estimation for tasks
- [ ] Visual duration indicators
- [ ] Time tracking integration
  - [ ] Start/stop timers
  - [ ] Automatic time logging
  - [ ] Time analytics
- [ ] Visual "time budget" for day/week
- [ ] "Time blindness" mode - extra visual cues

**Dogfooding:** Track time spent on development tasks  
**Impact:** Critical for ADHD users struggling with time perception

### Milestone 2.2: AI-Powered Task Breakdown (Q1 2026)

**Status:** ⏳ Planned

**Open-Source First Approach:**

- [ ] Integration with self-hosted LLM (Ollama/LocalAI)
- [ ] Task decomposition prompts and logic
- [ ] Subtask generation
- [ ] Complexity estimation

**Future Alternatives (for non-technical users):**

- [ ] Optional OpenAI API integration
- [ ] Optional Anthropic Claude API integration

**Dogfooding:** Break down large features into manageable tasks  
**Impact:** Reduces overwhelm from large, vague tasks

### Milestone 2.3: Focus Mode & Context Switching (Q1 2026)

**Status:** ⏳ Planned

- [ ] Focus session timer (Pomodoro-style)
- [ ] Distraction blocking reminders
- [ ] "Current focus" indicator
- [ ] Context preservation (save state when switching tasks)
- [ ] Quick context switch with minimal cognitive load

**Dogfooding:** Use during development sessions  
**Impact:** Helps maintain deep work and manage interruptions

---

## Phase 3: Enhanced UX & Gamification (Q2 - Q3 2026)

**Goal:** Make progress visible and rewarding without exploitation

### Milestone 3.1: Progress Visualization (Q2 2026)

**Status:** ⏳ Planned

- [ ] Visual progress indicators
- [ ] Completion animations
- [ ] Project progress dashboard
- [ ] Velocity tracking (tasks/day, week, month)
- [ ] Calendar view with completion heat map

**Dogfooding:** Track Altair development progress  
**Impact:** Visual progress fights ADHD paralysis

### Milestone 3.2: Gentle Gamification (Q2 - Q3 2026)

**Status:** ⏳ Planned

**Core Principles:**

- No exploitative mechanics
- No artificial urgency or FOMO
- Celebrate progress, not perfection
- Personal achievement, not comparison

**Features:**

- [ ] Achievement badges (milestones, streaks)
- [ ] Personal statistics dashboard
- [ ] Streak tracking (with forgiveness for off days)
- [ ] "Level up" moments for big wins
- [ ] Customizable celebration animations

**Dogfooding:** Celebrate development milestones  
**Impact:** Dopamine boost without exploitation

### Milestone 3.3: Mobile App (Flutter) (Q3 2026)

**Status:** ⏳ Planned

- [ ] iOS app
- [ ] Android app
- [ ] Full offline support
- [ ] Background sync
- [ ] Push notifications (optional)
- [ ] Quick capture widget
- [ ] Home screen widgets

**Dogfooding:** Capture ideas on the go  
**Impact:** Critical for ADHD users who need immediate capture

---

## Phase 4: Collaboration & Community (Q4 2026)

**Goal:** Enable team use and build community

### Milestone 4.1: Team Workspaces (Q4 2026)

**Status:** ⏳ Planned

- [ ] Multi-user workspaces
- [ ] Role-based permissions
- [ ] Task assignment
- [ ] Activity feed
- [ ] Comments and discussions

**Dogfooding:** Collaborate with early contributors  
**Impact:** Extends Altair to team settings

### Milestone 4.2: Templates & Community Sharing (Q4 2026)

**Status:** ⏳ Planned

- [ ] Project templates
- [ ] Task templates
- [ ] Community template gallery
- [ ] Import/export workflows
- [ ] Public project showcase

**Dogfooding:** Share Altair's project template  
**Impact:** Reduce setup friction for new users

### Milestone 4.3: Plugin System Foundation (Q4 2026)

**Status:** ⏳ Planned

- [ ] Plugin architecture design
- [ ] Custom field support
- [ ] Webhook system
- [ ] API for third-party integrations
- [ ] Plugin marketplace (future)

**Impact:** Extensibility for power users and integrations

---

## Phase 5: Managed Services & Scale (2027)

**Goal:** Make Altair accessible to non-technical users

### Milestone 5.1: Managed Hosting - Free Tier

**Status:** ⏳ Future

- [ ] Free tier infrastructure
- [ ] Account management system
- [ ] Automated backups
- [ ] Basic support system
- [ ] Usage analytics (opt-in, privacy-first)

**Target:** Remove technical barriers for ADHD community

### Milestone 5.2: Managed Hosting - Paid Tiers

**Status:** ⏳ Future

- [ ] Paid tier features
  - [ ] Increased storage
  - [ ] Advanced analytics
  - [ ] Priority support
  - [ ] Custom domains
- [ ] Billing system
- [ ] Subscription management

**Target:** Sustainable funding for development

### Milestone 5.3: Advanced Features

**Status:** ⏳ Future

- [ ] Email integration
- [ ] Calendar sync
- [ ] Voice task creation
- [ ] Smart scheduling
- [ ] Advanced analytics and insights
- [ ] Habit tracking integration

---

## Feature Priority Framework

### Must Have (Dogfooding Critical)

Features needed to manage Altair's development:

- Task management
- Documentation
- Basic organization

### Should Have (ADHD-Specific)

Features that make Altair uniquely valuable:

- AI task breakdown
- Visual time tracking
- Focus mode
- Gamification

### Could Have (Quality of Life)

Features that improve experience:

- Templates
- Advanced search
- Keyboard shortcuts
- Customization

### Won't Have (Yet)

Explicitly deferred:

- Real-time collaboration (Phase 1-2)
- Advanced analytics (Phase 1-3)
- Third-party integrations (Phase 1-3)
- Native desktop apps (Phase 1-4)

---

## Risks & Mitigation

### Risk: Scope Creep

**Mitigation:** Strict adherence to dogfooding principle—only build what we actively need

### Risk: Developer Burnout

**Mitigation:** Sustainable pace, celebrate small wins, use Altair to manage Altair development

### Risk: Low Adoption

**Mitigation:** Build in public, engage ADHD community early, focus on real pain points

### Risk: Technical Debt

**Mitigation:** Regular refactoring sprints, comprehensive testing, documentation-first approach

---

## How to Contribute to Roadmap

We welcome input on priorities! Here's how:

1. **Feature Requests:** [GitHub Issues](https://github.com/getaltair/altair/issues)
2. **Discussions:** [GitHub Discussions](https://github.com/getaltair/altair/discussions)
3. **Pull Requests:** See [CONTRIBUTING.md](CONTRIBUTING.md)

---

**Last Updated:** October 2025  
**Next Review:** January 2026
