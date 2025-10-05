# Altair

**Where focus takes flight**

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL%203.0-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Status: Pre-Alpha](https://img.shields.io/badge/Status-Pre--Alpha-orange.svg)]()
[![Python 3.11+](https://img.shields.io/badge/Python-3.11+-blue.svg)](https://www.python.org/downloads/)
[![Flutter](https://img.shields.io/badge/Flutter-Latest-blue.svg)](https://flutter.dev/)

[![CI](https://github.com/getaltair/altair/actions/workflows/ci.yml/badge.svg)](https://github.com/getaltair/altair/actions/workflows/ci.yml)
[![Security](https://github.com/getaltair/altair/actions/workflows/security.yml/badge.svg)](https://github.com/getaltair/altair/actions/workflows/security.yml)
[![codecov](https://codecov.io/gh/getaltair/altair/branch/main/graph/badge.svg)](https://codecov.io/gh/getaltair/altair)
[![Code style: black](https://img.shields.io/badge/code%20style-black-000000.svg)](https://github.com/psf/black)
[![Ruff](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json)](https://github.com/astral-sh/ruff)

> ADHD-friendly project management designed for neurodivergent minds. Open source, privacy-first, built by the community.

📊 **[View System Architecture Diagram](diagrams/01-system-architecture.md#high-level-system-architecture)**

---

## What is Altair?

Altair is an ADHD-friendly project management platform that helps neurodivergent individuals capture tasks, break down projects, track time, and maintain focus—all while respecting privacy and keeping your data under your control.

Traditional project management tools assume neurotypical brain patterns: linear thinking, sustained attention, consistent time perception, and reliable working memory. Altair is different—it's designed from the ground up for ADHD brains.

🎯 **[See ADHD Features Mindmap](diagrams/04-roadmap-planning.md#adhd-features-mindmap)**

---

## Why Altair?

### The ADHD Challenge
- **Tasks vanish** from working memory before you can write them down
- **Projects feel overwhelming** when you can't break them into chunks
- **Time blindness** makes estimating and tracking nearly impossible
- **Focus is fragile** and easily disrupted
- **Context switching** destroys productivity
- **Traditional tools** assume you have executive function you don't

### The Altair Solution
✨ **Instant Capture** - Save tasks before they disappear
🧩 **Smart Breakdown** - AI-powered project decomposition
⏱️ **Visual Time Tracking** - Built for time blindness
🎯 **Focus Mode** - Single-task clarity, minimize distractions
🎮 **Gentle Gamification** - Progress without exploitation
📝 **Auto Documentation** - Capture insights without interrupting flow
🔒 **Privacy-First** - Your data stays yours, always
📱 **Offline-First** - Works everywhere, syncs when connected

🚀 **[See Quick Task Capture Flow](diagrams/03-user-flows.md#quick-task-capture-flow)**

---

## Current Status

**Phase:** Pre-Alpha Development
**Version:** 0.1.0-dev
**Timeline:** MVP targeting Q1 2026

We're building in public and dogfooding from day one. Follow our progress:
- **Twitter/X:** [@getaltair](https://twitter.com/getaltair)
- **Discord:** [Join our community](https://discord.gg/altair)
- **Blog:** [getaltair.app/blog](https://getaltair.app/blog)

📅 **[View Full Development Roadmap](diagrams/04-roadmap-planning.md#development-timeline-gantt-chart)**

---

## Quick Start

### For Users (When Released)

We're not ready for general use yet! Join the [waitlist](https://getaltair.app) to be notified when we launch.

### For Developers

Want to contribute? Here's how to get started:

```bash
# Clone the repository
git clone https://github.com/getaltair/altair.git
cd altair

# Backend setup (FastAPI)
cd backend
python -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate
pip install -r requirements.txt
python main.py

# Frontend setup (Flutter)
cd ../frontend
flutter pub get
flutter run -d chrome  # or: flutter run (for mobile)
```

📖 **[See Complete Quick Start Guide](QUICK_START.md)**
🏗️ **[View System Architecture](diagrams/01-system-architecture.md)**
💾 **[Explore Database Schema](diagrams/02-database-schema-erd.md)**

---

## Features

### Core Functionality
- **Task Management** - Quick capture, organization, prioritization
- **Project Breakdown** - AI-assisted decomposition into manageable chunks
- **Time Tracking** - Visual, ADHD-friendly time awareness
- **Focus Sessions** - Distraction-free work modes
- **Documentation** - Integrated notes and markdown support
- **Cross-Platform** - Web, iOS, Android, Desktop (future)

### ADHD-Specific Design
- **Working Memory Support** - Ultra-fast task capture
- **Time Blindness Accommodations** - Visual time representations
- **Executive Function Aids** - Smart suggestions and automation
- **Sensory Considerations** - Reduced motion, customizable UI
- **Flexible Organization** - Multiple ways to view and organize
- **Gentle Notifications** - Non-intrusive, user-controlled

📋 **[Read Detailed Features Documentation](FEATURES.md)**
👤 **[See User Flow Diagrams](diagrams/03-user-flows.md)**

---

## Tech Stack

### Backend
- **Framework:** FastAPI (Python 3.11+)
- **Database:** PostgreSQL 15+
- **Caching:** Redis (optional)
- **Authentication:** JWT tokens
- **API:** RESTful + WebSocket support

### Frontend
- **Framework:** Flutter (Dart)
- **Platforms:** Web, iOS, Android
- **State Management:** Riverpod
- **Offline Support:** Hive/SQLite
- **Sync:** Custom conflict resolution

### Infrastructure
- **Hosting:** Self-hostable + managed option (future)
- **Container:** Docker + Docker Compose
- **CI/CD:** GitHub Actions
- **Monitoring:** TBD

🔧 **[View Architecture Documentation](ARCHITECTURE.md)**
🏢 **[See Deployment Diagrams](diagrams/06-deployment-operations.md)**

---

## Documentation

### For Users
- [Features Overview](FEATURES.md) - What makes Altair special
- [Getting Started](docs/user-guide/getting-started.md) - Your first steps (coming soon)
- [User Guide](docs/user-guide/) - Complete user documentation (coming soon)

### For Developers
- [Architecture](ARCHITECTURE.md) - System design and technical decisions
- [Contributing Guide](CONTRIBUTING.md) - How to help build Altair
- [Quick Start](QUICK_START.md) - Get running in 10 minutes
- [API Documentation](docs/api/) - API reference (coming soon)
- [Visual Diagrams](diagrams/README.md) - **📊 80+ architectural and flow diagrams**

### For the Project
- [Roadmap](ROADMAP.md) - Development timeline and priorities
- [Code of Conduct](CODE_OF_CONDUCT.md) - Community guidelines
- [Security Policy](SECURITY.md) - Security and vulnerability reporting
- [License](LICENSE) - AGPL-3.0 details

📚 **[Browse All Documentation](DOCUMENTATION_INDEX.md)**

---

## Contributing

We welcome contributions from everyone! Whether you have ADHD, build tools for neurodivergent users, or simply care about inclusive technology—there's a place for you here.

### Ways to Contribute
- 🐛 **Report bugs** - Help us find and fix issues
- 💡 **Suggest features** - Share your ADHD pain points
- 📝 **Improve docs** - Make guides clearer
- 💻 **Write code** - Backend, frontend, or infrastructure
- 🎨 **Design** - UI/UX improvements
- 🧪 **Test** - Try features and provide feedback
- 🌍 **Translate** - Help make Altair accessible worldwide

### Getting Started
1. Read our [Code of Conduct](CODE_OF_CONDUCT.md)
2. Check out the [Contributing Guide](CONTRIBUTING.md)
3. Browse [Good First Issues](https://github.com/getaltair/altair/labels/good%20first%20issue)
4. Join our [Discord community](https://discord.gg/altair)

🤝 **[View Contribution Workflow](diagrams/06-deployment-operations.md#contribution-workflow)**

---

## Roadmap

### Phase 1: Foundation (Q4 2025 - Q1 2026) 🚧 Current
- ✅ Tech stack finalized
- ✅ Brand identity created
- ✅ Documentation written
- 🚧 Core backend API
- 🚧 Flutter mobile app
- 🚧 Basic task management
- 📅 Quick capture
- 📅 Simple time tracking

### Phase 2: ADHD Features (Q2 2026)
- AI-powered task breakdown
- Visual time awareness
- Focus mode
- Gentle gamification
- Automatic documentation

### Phase 3: Enhanced UX (Q3 2026)
- Cross-platform sync
- Collaboration features
- Advanced customization
- Performance optimization

### Phase 4: Community & Growth (Q4 2026)
- Public beta launch
- Community features
- Plugin system
- Mobile apps in app stores

### Phase 5: Sustainability (2027+)
- Managed hosting service
- Enterprise features
- Advanced AI capabilities
- International expansion

📆 **[View Full Roadmap with Priorities](ROADMAP.md)**
📈 **[See Sprint Planning Diagrams](diagrams/04-roadmap-planning.md#sprint-1-planning)**

---

## Community

### Join Us
- **Discord:** [https://discord.gg/altair](https://discord.gg/altair) - Daily discussion and support
- **Twitter/X:** [@getaltair](https://twitter.com/getaltair) - Updates and announcements
- **GitHub:** [github.com/getaltair/altair](https://github.com/getaltair/altair) - Code and issues
- **Email:** hello@getaltair.app - General inquiries

### Philosophy
- **Neurodiversity-affirming** - ADHD is not a deficit
- **Privacy-respecting** - Your data is yours
- **Community-driven** - Built by users, for users
- **Transparency** - Open source, open development
- **Accessibility** - Designed for everyone

---

## License

Altair is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)**.

This means:
- ✅ Free to use, forever
- ✅ Free to modify and distribute
- ✅ Source code must remain open
- ✅ Modifications must be shared
- ✅ Cannot be turned into closed-source

Read the [full license](LICENSE) for details.

**Why AGPL-3.0?** We chose AGPL to ensure Altair remains open source forever and that all improvements benefit the entire community, even when hosted as a service.

---

## Support the Project

Altair is free and open source, built by volunteers who believe in neurodiversity-affirming tools.

### Ways to Support
- ⭐ **Star the repo** - Show your support on GitHub
- 🗣️ **Spread the word** - Tell others about Altair
- 🐛 **Report bugs** - Help us improve
- 💻 **Contribute code** - Make Altair better
- 📝 **Improve docs** - Help others understand
- 💰 **Sponsor** - Support development (coming soon)

---

## Acknowledgments

Built with inspiration from:
- The ADHD community sharing their struggles with existing tools
- Open source projects proving that community-driven development works
- Researchers studying ADHD and executive function
- Developers who believe in accessible, inclusive technology

Special thanks to everyone who has contributed ideas, code, feedback, and encouragement.

---

## Questions?

- **General:** Check our [Documentation](DOCUMENTATION_INDEX.md)
- **Technical:** Read the [Architecture docs](ARCHITECTURE.md) or [browse diagrams](diagrams/)
- **Contributing:** See the [Contributing Guide](CONTRIBUTING.md)
- **Security:** Review our [Security Policy](SECURITY.md)
- **Other:** Email us at hello@getaltair.app

---

**Built with 💙 by the ADHD community, for the ADHD community**

*Where focus takes flight* ✨

---

**Last Updated:** October 2025
**Version:** 0.1.0-dev
**Status:** Pre-Alpha Development
