# 📋 Altair Comprehensive Requirements Document

**Version 1.0 | Generated November 2025**

> **Tagline:** "Where focus takes flight"  
> **Domain:** getaltair.app  
> **Repository:** github.com/getaltair/altair  
> **License:** AGPL 3.0 (Open Source)

---

## 🎯 Executive Summary

**Altair** is an open-source, ADHD-focused productivity ecosystem consisting of three interconnected Flutter applications:

1. **Guidance** - Task/project management with Quest-Based Agile (QBA)
2. **Knowledge** - Personal knowledge management with AI-powered features
3. **Tracking** - Inventory and asset management for makers

**Core Value Proposition:** Solves fragmentation problem where users need 3+ different tools by providing integrated data across apps via shared local SurrealDB instance.

**Target Audience:** Primary focus on users with ADHD and attention differences, with specific emphasis on makers, researchers, and technical users.

---

## 🏗️ System Architecture

### Core Technical Stack

- **Frontend:** Flutter (desktop-first, mobile secondary)
- **Backend:** Rust 1.91+ with Axum
- **Database:** SurrealDB (local-first, multi-model: graph + document + vector)
- **Communication:** gRPC for cross-app and shared features
- **AI/ML:** Local embeddings via Sentence Transformers, optional local and cloud LLMs
- **Platforms:** Desktop (Linux, Windows, macOS) and Mobile (Android, iOS)
- **Web:** Future consideration only if cloud hosting launched (not PWA)

### Architecture Principles

✅ **Local-first:** All data stored locally, optional sync  
✅ **Privacy-focused:** No cloud requirements, user controls API keys  
✅ **Offline-capable:** Core functionality works without internet  
✅ **Open-source:** AGPL 3.0 licensed for community contribution  
✅ **Dogfood-driven:** Built using itself, public demonstration  

---

## 📱 Application-Specific Requirements

### Altair Guidance - Project Management

#### Core Features (MVP)
1. **Quest-Based Agile Board**
   - 6-column workflow: Idea Greenhouse → Quest Log → This Cycle's Quest → Next Up → In-Progress (WIP=1) → Harvested
   - Epic → Quest → Subquest hierarchy
   - Energy-based planning (not time-based)
   - Visual board with neo-brutalist UI

2. **Energy Management**
   - Daily energy check-ins (1-5 scale)
   - Task filtering by energy requirements
   - Spoon theory integration
   - Pattern recognition over time

3. **Weekly Harvest Ritual**
   - Scheduled reflection sessions
   - Progress review and planning
   - Archive completed work
   - Adapt and pivot without shame

4. **Gamification Engine**
   - XP points for quest completion
   - Achievement badges
   - Level progression system
   - Reward shop for redemption

#### ADHD-Specific Features

- **WIP=1 enforcement** to prevent task switching
- **Forgiveness features** (easy restart, no punishment for breaks)
- **Visual timers** for time blindness
- **AI task breakdown** (Magic ToDo style)
- **Quick capture** with natural language processing
- **Multiple view options** (list, board, calendar)

---

### Altair Knowledge - Personal Knowledge Management

#### Core Features (MVP)
1. **Semantic Search with RAG**
   - Local embeddings (all-MiniLM-L6-v2)
   - SurrealDB vector search
   - Source-grounded results
   - Fallback to keyword search

2. **Note Organization**
   - Daily notes as default entry point
   - Bidirectional linking ([[note title]])
   - Backlinks panel
   - Tags and visual organization

3. **Voice Quick-Capture**
   - Whisper transcription
   - Auto-save to daily notes
   - Mobile widget support
   - Zero-friction entry

4. **AI-Powered Features**
   - Automatic knowledge graphs
   - Task extraction to Guidance
   - Smart templates
   - Podcast generation from notes

### Documentation Types
- QBA methodology guides
- Epic planning documents
- Quest notes and learnings
- Failure recovery library
- Reference materials

---

### Altair Tracking - Inventory Management

#### Core Features (MVP)
1. **Item Tracking**
   - Location management
   - Photo attachments
   - Custom fields
   - QR code generation

2. **Project Integration**
   - Bill of Materials (BoM) parsing
   - Quest material requirements
   - Shopping list generation
   - Tool availability checking

3. **Maintenance & Warranties**
   - Service reminders
   - Warranty tracking
   - Maintenance logs
   - Cost tracking

4. **Smart Features**
   - Low stock alerts
   - Consumption tracking
   - Parametric search
   - Batch operations

#### Maker-Specific Features

- Project kits/bins
- Component specifications
- Distributor integration (future)
- Check-out/check-in system

---

## 🔄 Cross-App Integration Features

#### Phase 1: BoM Intelligence (MVP Priority)
**Timeline:** 4-6 weeks  
**Features:**
- Parse structured BoMs from Knowledge notes
- Match items to Tracking inventory (exact matching initially)
- Generate shopping lists for missing items
- Visual status indicators (available/low/missing)

#### Phase 2: Recipe → Task + Inventory
**Timeline:** Months 4-5  
**Features:**
- Parse project descriptions
- Generate task breakdowns in Guidance
- Link required materials in Tracking
- Create project templates

#### Phase 3: Context-Aware Suggestions
**Timeline:** Months 6-7  
**Features:**
- Real-time relationship detection
- Surface relevant notes during task work
- Show inventory needs for active quests
- Proactive cross-app linking

#### Phase 4: Advanced Intelligence
**Timeline:** Months 8+  
**Features:**
- Inventory depletion tracking
- Duplicate detection
- LLM-based parsing (optional)
- Fuzzy matching algorithms

---

## 🤖 AI/RAG Implementation

#### Architecture Overview
- **Embeddings:** Local via Sentence Transformers (90-95% of OpenAI quality, $0 cost)
- **Vector Storage:** SurrealDB native support
- **LLM Integration:** Abstracted provider interface (Claude, OpenAI, local)
- **Search:** Hybrid approach (semantic + keyword + graph)

#### Implementation Phases
1. **Phase 1 (Weeks 1-8):** Semantic search MVP
2. **Phase 2 (Weeks 9-16):** Conversational AI with chat
3. **Phase 3 (Weeks 17-28):** Cross-app reasoning

#### Privacy & Security
- All processing local by default
- User controls API keys
- Optional cloud mode with clear indication
- Data never leaves device without permission

---

## 🧠 ADHD-Specific Design Principles

#### Core Requirements for All Features
- [ ] Reduces cognitive load (doesn't add complexity)
- [ ] Provides external structure (not willpower-dependent)
- [ ] Works for variable capacity (high and low energy days)
- [ ] Includes forgiveness mechanism (easy restart)
- [ ] Avoids shame language (positive framing)
- [ ] Adapts to user patterns (learns over time)
- [ ] Accessible quickly (low friction access)
- [ ] Provides immediate feedback (dopamine hit)

#### Feature Categories by ADHD Challenge

| Challenge | Required Features |
|-----------|-------------------|
| **Time Blindness** | Visual timers, time tracking, buffer calculation, estimation calibration |
| **Executive Dysfunction** | AI task breakdown, routines, smart suggestions, guided workflows |
| **Motivation Deficits** | Gamification, immediate feedback, streak tracking, rewards |
| **Overwhelm** | Task hiding, limited visibility, progressive disclosure, fresh starts |
| **Focus Regulation** | Pomodoro modifications, distraction blocking, hyperfocus management |
| **Memory** | Quick capture, brain dumps, context preservation, cross-module linking |
| **Initiation** | Task randomizers, body doubling, 5-minute rule, one-click starting |

---

## 📊 Identified Duplications & Conflicts

#### Duplications Found

1. **AI Task Breakdown**
   - Appears in: Guidance features, Knowledge features, ADHD features
   - **Resolution:** Implement once in Guidance, make accessible via API to other apps

2. **Gamification System**
   - Appears in: QBA features, ADHD features, multiple motivation sections
   - **Resolution:** Single gamification service shared across apps

3. **Quick Capture**
   - Appears in: All three apps' requirements
   - **Resolution:** Universal capture with intelligent routing

4. **Energy/Spoon Management**
   - Appears in: QBA system, ADHD features, general requirements
   - **Resolution:** Centralized in Guidance, referenced by others

#### Conflicts Identified

1. **Complexity vs. Simplicity**
   - QBA system has detailed 6-column board
   - ADHD principles emphasize minimal cognitive load
   - **Resolution:** Progressive disclosure - start simple, add complexity as needed

2. **Gamification Approach**
   - Some docs suggest full RPG system
   - Others warn against over-gamification
   - **Resolution:** Optional gamification layers, user chooses engagement level

3. **Cross-App Data Access**
   - Privacy concerns vs. integration benefits
   - **Resolution:** Granular permissions, user controls cross-app access

#### Consolidation Opportunities

1. **Merge Similar Features:**
   - Brain dump + Quick capture → Universal capture system
   - Task breakdown + Recipe parsing → Single AI parsing service
   - Various search features → Unified search infrastructure

2. **Shared Services:**
   - Authentication/permissions
   - Notification system
   - AI/LLM provider abstraction
   - Export/import functionality

3. **Common UI Components:**
   - Neo-brutalist design system
   - Visual timers
   - Progress indicators
   - Tag/label system

---

## 🚀 Implementation Roadmap

#### Phase 1: Core MVP (Months 1-3)
**Guidance:** QBA board, energy tracking, basic gamification  
**Knowledge:** Semantic search, daily notes, backlinks  
**Tracking:** Basic inventory CRUD, location tracking  
**Integration:** Shared SurrealDB, basic cross-app links

#### Phase 2: Integration Features (Months 4-6)
**Priority:** BoM Intelligence  
**Features:** Cross-app search, proactive suggestions  
**Improvements:** Performance optimization, UI polish

#### Phase 3: Advanced Features (Months 7-9)
**AI/RAG:** Full implementation  
**Advanced:** Fuzzy matching, LLM parsing  
**Polish:** Mobile optimization, accessibility

#### Phase 4: Ecosystem Maturity (Months 10-12)
**Community:** Documentation, tutorials  
**Optional:** Cloud sync, collaboration  
**Future:** Web version consideration

---

## 📈 Success Metrics

#### Technical Metrics
- Search relevance >75%
- Response latency <1 second
- Sync reliability >99%
- Zero data loss

#### User Experience Metrics
- Task completion rate increase
- Time to task initiation decrease
- Feature adoption rates
- User retention (dogfooding success)

#### ADHD-Specific Metrics
- Reduced task abandonment
- Increased capture rate
- Streak maintenance (with pauses)
- Energy pattern accuracy

---

## 🔧 Technical Dependencies

#### Required Infrastructure
- Flutter 3.16+ with desktop support
- Rust 1.91+ (for backend services)
- SurrealDB 2.0+ (vector support)

#### Key Libraries
- **Flutter:** Riverpod, drift, flutter_chat_ui
- **Rust:** Axum, tokio, tonic (gRPC), surrealdb
- **AI:** anthropic_sdk_dart, dart_openai
- **Embeddings:** Sentence Transformers (Python service or Rust equivalent)
- **Desktop:** window_manager, tray_manager, hotkey_manager

---

## ⚠️ Risk Mitigation

#### High Priority Risks

1. **Feature Bloat**
   - Risk: Over-complexity defeats ADHD-friendly goal
   - Mitigation: Aggressive feature gating, user testing

2. **Performance Issues**
   - Risk: Slow performance on mobile/older hardware
   - Mitigation: Progressive loading, optimization focus

3. **Integration Complexity**
   - Risk: Cross-app features create maintenance burden
   - Mitigation: Clear API boundaries, modular design

4. **Adoption Barriers**
   - Risk: Too complex for new users
   - Mitigation: Onboarding flow, templates, video tutorials

---

## 📝 Documentation Requirements

#### User Documentation
- Getting started guides (per app)
- ADHD strategy guides
- Integration workflows
- Video tutorials
- FAQ and troubleshooting

#### Developer Documentation
- API references
- SurrealDB schema
- Contribution guidelines
- Architecture decisions
- Testing strategies

---

## 🎯 Key Decisions & Principles

#### Non-Negotiable Requirements
1. **Local-first architecture** - Privacy and ownership
2. **ADHD-friendly design** - Every feature evaluated through this lens
3. **Open source** - Community-driven development
4. **Dogfooding** - Use it to build it
5. **Integration over isolation** - Apps work better together

#### Design Philosophy
- **Progressive disclosure** over feature-complete interfaces
- **Forgiveness** over punishment for inconsistency
- **Flexibility** over rigid workflows
- **Visual clarity** over aesthetic minimalism
- **Quick wins** over long-term planning

---

## 📌 Next Actions

#### Immediate Priorities
1. ✅ Finalize SurrealDB schema design
2. ✅ Implement QBA board in Guidance
3. ✅ Set up embedding service for Knowledge
4. ✅ Create basic inventory CRUD for Tracking
5. ✅ Establish cross-app communication protocol

#### Week 1-2 Goals
- Complete Guidance MVP with QBA
- Test semantic search with 100 documents
- Validate cross-app data access
- Create initial UI mockups
- Begin dogfooding journal

---

## 📚 References & Resources

#### Documentation Reviewed
- adhd-features-altair-guidance.md
- altair-knowledge-features.md
- altair-qba-features-revised.md
- altair-tracking-personal-inventory.md
- bom-intelligence-phase1-mvp-spec.md
- knowledge-proactive-relationship-architecture.md
- proactive-integration-opportunities.md
- AI-Powered_Semantic_Search_and_Chat_for_Altair__Production_RAG_Implementation_Guide.md
- QBA_System_Revised.md

#### External Resources
- SurrealDB Documentation
- Flutter Desktop Guidelines
- ADHD productivity research papers
- Sentence Transformers documentation
- Quest-Based Agile methodology

---

*This document represents a comprehensive synthesis of all Altair project requirements. It should be treated as the authoritative source for development decisions and feature prioritization.*

**Last Updated:** November 2025  
**Status:** Living Document  
**Version:** 1.0
