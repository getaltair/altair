# Altair

> ADHD-friendly productivity ecosystem with local-first architecture

[![License](https://img.shields.io/badge/license-AGPL--3.0-blue.svg)](LICENSE)
[![Flutter](https://img.shields.io/badge/Flutter-3.0+-02569B?logo=flutter)](https://flutter.dev)
[![Python](https://img.shields.io/badge/Python-3.12+-3776AB?logo=python&logoColor=white)](https://python.org)

## What is Altair?

Altair is a three-app ecosystem designed specifically for people with ADHD who need powerful task management, knowledge organization, and inventory tracking. Built with a **local-first** philosophy—your data stays on your device by default, with optional sync when you need it.

### The Three Apps

1. **Altair Guidance** 🎯 — Task and project management
   - Quick capture (< 3 seconds thought-to-save)
   - AI-powered task breakdown
   - Visual time tracking for time blindness
   - Offline-first, works without internet

2. **Altair Knowledge** 📚 — Personal wiki and knowledge management _(Phase 2)_
   - Markdown-based notes
   - Automatic backlinks and graph view
   - Daily notes
   - Cross-app linking with Guidance

3. **Altair Tracking** 📦 — Inventory and resource management _(Phase 3)_
   - Barcode scanning
   - Location-based organization
   - Smart alerts (low stock, expiration)
   - Integrates with tasks and wiki

## Key Features

- 🏠 **Local-first** — Works offline, data stays on your device
- 🔄 **Optional sync** — Multi-device sync when you need it (PowerSync)
- 🎨 **Neo-brutalist UI** — High contrast, clear visual feedback
- 🤖 **AI-powered** — Task breakdown, prioritization (OpenAI, Anthropic, Ollama)
- ⚡ **Fast** — Instant captures, < 1s page loads
- 🔐 **Private** — Your data, your control

## Architecture

```
altair/
├── apps/                    # Flutter applications
│   ├── altair-guidance/     # Task management (Phase 1)
│   ├── altair-knowledge/    # Wiki (Phase 2)
│   └── altair-tracking/     # Inventory (Phase 3)
├── packages/                # Shared Flutter packages
│   ├── altair-ui/          # UI components & theme
│   ├── altair-core/        # Business logic & models
│   ├── altair-auth/        # Authentication
│   └── altair-sync/        # PowerSync integration
├── services/                # Backend services (optional)
│   ├── auth-service/       # FastAPI authentication
│   ├── sync-service/       # PowerSync backend
│   └── ai-service/         # AI proxy (optional)
├── infrastructure/          # Deployment configs
│   ├── docker/             # Docker Compose
│   ├── nginx/              # Reverse proxy
│   └── scripts/            # Automation scripts
└── docs/                    # Documentation
```

## Quick Start

### End Users: Download Standalone Installers

**No development tools required!** Download pre-built installers:

- **Linux**: [AppImage](https://github.com/getaltair/altair/releases/latest) (portable, works on all distros)
- **macOS**: [DMG installer](https://github.com/getaltair/altair/releases/latest)
- **Windows**: [Setup installer](https://github.com/getaltair/altair/releases/latest)

See [INSTALLERS.md](docs/INSTALLERS.md) for detailed installation instructions.

### Developers: Running from Source

**Prerequisites:**

- **Flutter** 3.0+ (for desktop/mobile development)
- **Python** 3.12+ (for backend services, optional)
- **uv** (Python package manager, optional)

**Run Altair Guidance:**

```bash
# Navigate to the Guidance app
cd apps/altair-guidance

# Get dependencies
flutter pub get

# Run on Linux desktop
flutter run -d linux

# Run on macOS desktop
flutter run -d macos

# Run on Windows desktop
flutter run -d windows

# Run on Android (requires connected device/emulator)
flutter run -d android
```

### Development Setup

```bash
# Clone the repository
git clone https://github.com/getaltair/altair.git
cd altair

# Install pre-commit hooks
pre-commit install

# Run all tests
./scripts/test-all.sh

# Build all apps
./scripts/build-all.sh
```

## Development Roadmap

### Phase 1: Altair Guidance (Months 1-3)

- ✅ Month 1: Infrastructure & auth
- ✅ Month 2: Core task management (Week 5-8 complete: Quick Capture, Task Editing, Projects, UX Polish)
- ✅ Month 3: AI features (Week 9-11 complete: OpenAI, Anthropic, Ollama integrations, AI Features UI)
- 🚧 Week 12: Polish & Beta (Standalone Installers complete, Beta testing in progress)

### Phase 2: Altair Knowledge (Months 4-6)

- ⏳ Wiki foundation
- ⏳ Smart connections
- ⏳ External brain features

### Phase 3: Altair Tracking (Months 7-9)

- ⏳ Basic inventory
- ⏳ Smart tracking
- ⏳ Ecosystem integration

See [DEVELOPMENT-ROADMAP.md](docs/DEVELOPMENT-ROADMAP.md) for details.

## Documentation

- [Architecture Overview](docs/ARCHITECTURE-OVERVIEW.md) — System design principles
- [Data Flow](docs/DATA-FLOW.md) — How data moves through the system
- [Component Design](docs/COMPONENT-DESIGN.md) — Component structure
- [Deployment Guide](docs/DEPLOYMENT-GUIDE.md) — Deployment options
- [Development Roadmap](docs/DEVELOPMENT-ROADMAP.md) — Timeline and milestones
- [Testing Guide](docs/TESTING.md) — Testing strategies and best practices
- [Installers Guide](docs/INSTALLERS.md) — Building and distributing standalone installers

## Technology Stack

### Frontend

- **Flutter** — Cross-platform UI framework
- **SQLite** — Local database (via sqflite)
- **PowerSync** — Client-side sync engine
- **Bloc** — State management

### Backend (Optional)

- **FastAPI** — Python web framework
- **PostgreSQL** — Server database
- **PowerSync Service** — Sync coordinator
- **Redis** — Caching layer

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Workflow

1. Create a feature branch (`git checkout -b feature/amazing-feature`)
2. Make changes following our conventions
3. Run linters and tests (`pre-commit run --all-files`)
4. Commit with conventional commits (`git commit -m "feat: add amazing feature"`)
5. Push and create a pull request

### Code Standards

- **Conventional Commits** — For commit messages
- **Keep a Changelog** — For CHANGELOG.md updates
- **Pre-commit hooks** — Automated linting and formatting
- **Comprehensive tests** — Unit, widget, and integration tests
- **Inline documentation** — Following language-specific standards

## License

This project is licensed under the GNU Affero General Public License v3.0 or later (AGPL-3.0-or-later) - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built for the ADHD community, by the ADHD community
- Inspired by local-first software principles
- Powered by Flutter, FastAPI, and PowerSync

## Support

- 📖 Documentation: [docs/](docs/)
- 🐛 Issues: [GitHub Issues](https://github.com/getaltair/altair/issues)
- 💬 Discord: [Join our community](https://discord.gg/altair)
- 📧 Email: <support@getaltair.com>

---

**Status**: 🚧 Phase 1 Development (Week 12: Polish & Beta - Standalone Installers Complete)

Made with ❤️ for the ADHD community
