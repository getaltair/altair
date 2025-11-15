# 🦅 Altair

> **"Where focus takes flight"**

[![License: AGPL 3.0](https://img.shields.io/badge/License-AGPL%203.0-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Flutter](https://img.shields.io/badge/Flutter-3.16+-blue.svg)](https://flutter.dev)

**Altair** is an open-source, ADHD-focused productivity ecosystem that solves the fragmentation problem by providing three interconnected applications that share data through a local-first architecture.

🌐 **Website:** [getaltair.app](https://getaltair.app)  
📦 **Repository:** [github.com/getaltair/altair](https://github.com/getaltair/altair)

---

## 🎯 Overview

Altair consists of three integrated Flutter applications designed to work together seamlessly:

1. **📋 Guidance** - Task and project management with Quest-Based Agile (QBA)
2. **📚 Knowledge** - Personal knowledge management with AI-powered semantic search
3. **📦 Tracking** - Inventory and asset management for makers

**Core Value Proposition:** Instead of juggling 3+ different productivity tools, Altair provides an integrated ecosystem where your tasks, notes, and inventory work together through a shared local database.

---

## ✨ Key Features

### 🧠 ADHD-Focused Design

- **WIP=1 enforcement** to prevent task switching
- **Energy-based planning** (not time-based) with spoon theory integration
- **Forgiveness features** - easy restart, no punishment for breaks
- **Visual timers** for time blindness
- **Quick capture** with natural language processing
- **Progressive disclosure** to reduce cognitive load

### 🔒 Privacy & Local-First

- **100% local data** - All data stored on your device
- **No cloud required** - Works completely offline
- **User-controlled API keys** - You decide what goes to the cloud
- **Open source** - AGPL 3.0 licensed, fully auditable

### 🔗 Cross-App Integration

- **Shared SurrealDB instance** - Unified data layer
- **gRPC communication** - Efficient inter-app communication for shared features
- **BoM Intelligence** - Parse bills of materials from notes, check inventory
- **Task extraction** - Convert notes to actionable quests
- **Context-aware suggestions** - Surface relevant information across apps

### 🤖 AI-Powered (Optional)

- **Local embeddings** - Semantic search without cloud dependency
- **RAG implementation** - Source-grounded AI responses
- **Task breakdown** - AI-assisted project planning
- **Knowledge graphs** - Automatic relationship detection

---

## 🏗️ Architecture

### Tech Stack

- **Frontend:** Flutter
- **Backend:** Rust 1.91+ with Axum
- **Database:** SurrealDB (local-first, multi-model: graph + document + vector)
- **Communication:** gRPC for cross-app and shared features
- **AI/ML:** Local embeddings via Sentence Transformers, optional local and cloud LLMs
- **Platforms:** Desktop (Linux, Windows, macOS) and Mobile (Android, iOS)

### Architecture Principles

- ✅ **Local-first** - All data stored locally, optional sync
- ✅ **Privacy-focused** - No cloud requirements, user controls API keys
- ✅ **Offline-capable** - Core functionality works without internet
- ✅ **Open-source** - AGPL 3.0 licensed for community contribution
- ✅ **Dogfood-driven** - Built using itself, public demonstration

---

## 🚀 Getting Started

### Prerequisites

- **Flutter 3.16+** with desktop support
- **Rust 1.91+** (for backend services)
- **SurrealDB 2.0+** (with vector support)
- **Dart SDK 3.0+**

### Installation

1. **Clone the repository**

   ```bash
   git clone https://github.com/getaltair/altair.git
   cd altair
   ```

2. **Install Flutter dependencies**

   ```bash
   flutter pub get
   ```

3. **Set up SurrealDB**

   ```bash
   # Follow instructions in docs/SHARED-001-surrealdb-setup.md
   ```

4. **Run the application**

   ```bash
   flutter run -d linux  # or windows, macos, android, ios
   ```

### Development Setup

See the [comprehensive requirements document](docs/altair-comprehensive-requirements.md) for detailed setup instructions and architecture decisions.

---

## 📱 Applications

### 📋 Guidance - Project Management

Quest-Based Agile board with energy management:

- 6-column workflow: Idea Greenhouse → Quest Log → This Cycle's Quest → Next Up → In-Progress (WIP=1) → Harvested
- Epic → Quest → Subquest hierarchy
- Daily energy check-ins (1-5 scale)
- Weekly Harvest Ritual for reflection and planning
- Gamification engine (XP, badges, levels)

### 📚 Knowledge - Personal Knowledge Management

AI-powered note-taking and knowledge base:

- Semantic search with local embeddings
- Daily notes as default entry point
- Bidirectional linking ([[note title]])
- Voice quick-capture with Whisper transcription
- Automatic knowledge graphs
- Task extraction to Guidance

### 📦 Tracking - Inventory Management

Maker-focused inventory and asset tracking:

- Location management with photo attachments
- Bill of Materials (BoM) parsing
- Shopping list generation
- Maintenance and warranty tracking
- QR code generation
- Low stock alerts

---

## 📖 Documentation

- **[Comprehensive Requirements](docs/altair-comprehensive-requirements.md)** - Full project specification
- **[Quest-Based Agile Board](docs/specs/AG-001-quest-based-agile-board.md)** - QBA methodology
- **[Energy Management](docs/specs/AG-002-energy-management.md)** - Energy tracking system
- **[SurrealDB Setup](docs/specs/SHARED-001-surrealdb-setup.md)** - Database configuration
- **[gRPC Communication](docs/specs/SHARED-002-grpc-communication.md)** - Cross-app communication
- **[UI Implementation Guide](docs/altair-ui-implementation-guide.md)** - Design system

---

## 🛣️ Roadmap

### Phase 1: Core MVP (Months 1-3) 🚧

- [x] SurrealDB schema design
- [x] QBA board in Guidance
- [ ] Semantic search in Knowledge
- [ ] Basic inventory CRUD in Tracking
- [ ] Cross-app communication protocol

### Phase 2: Integration Features (Months 4-6)

- [ ] BoM Intelligence (parse notes, check inventory)
- [ ] Cross-app search
- [ ] Proactive suggestions
- [ ] Performance optimization

### Phase 3: Advanced Features (Months 7-9)

- [ ] Full AI/RAG implementation
- [ ] Fuzzy matching algorithms
- [ ] LLM-based parsing
- [ ] Mobile optimization

### Phase 4: Ecosystem Maturity (Months 10-12)

- [ ] Community documentation
- [ ] Video tutorials
- [ ] Optional cloud sync
- [ ] Collaboration features

---

## 🤝 Contributing

We welcome contributions! Altair is built by and for the ADHD community.

### How to Contribute

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Principles

- **ADHD-friendly first** - Every feature evaluated through this lens
- **Progressive disclosure** - Start simple, add complexity as needed
- **Forgiveness over punishment** - Easy restart, no shame
- **Dogfooding** - We use Altair to build Altair

See our [contribution guidelines](CONTRIBUTING.md) for more details.

---

## 🧪 Testing

```bash
# Run all tests
flutter test

# Run with coverage
flutter test --coverage
```

---

## 📄 License

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0) - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- Built with ❤️ for the ADHD community
- Inspired by Quest-Based Agile methodology
- Powered by [SurrealDB](https://surrealdb.com), [Flutter](https://flutter.dev), and open-source tools

---

## 📞 Contact & Support

- **Website:** [getaltair.app](https://getaltair.app)
- **Issues:** [GitHub Issues](https://github.com/getaltair/altair/issues)
- **Discussions:** [GitHub Discussions](https://github.com/getaltair/altair/discussions)

---

## Status

🚧 Active Development - MVP in progress

*Last Updated: November 2025*
