# Altair ⭐

> Where focus takes flight

Altair transforms project management from a source of anxiety into a supportive external brain that celebrates neurodivergent strengths while providing essential executive function scaffolding.

## 🎯 Philosophy

**ADHD users don't need to be fixed; they need tools that work with their brains.**

Altair is designed around core principles:
- **Reduce cognitive load** - Minimize decisions at every step
- **Minimize decision fatigue** - Smart defaults and natural language input
- **Provide immediate feedback** - Real-time updates and confirmations
- **Celebrate small wins** - Gamification and positive reinforcement
- **Support, not shame** - No guilt, no pressure, just progress

## ✨ Key Features

### Quick Capture
- **Zero-friction task entry** - Single field, natural language processing
- **Immediate local save** - Never lose a thought
- **Background enhancement** - AI adds structure without blocking you

### ADHD-Optimized Task Flow
```
INBOX (no decisions required)
  ↓
TRIAGE (when executive function available)
  ↓
ACTIVE / BLOCKED / SOMEDAY
  ↓
DONE (celebrate! 🎉)
  ↓
ARCHIVED (automatic after 30 days)
```

### Focus Sessions
- **Pomodoro timer** with customizable intervals
- **Hyperfocus tracking** - Learn your productivity patterns
- **Gentle break reminders** - Prevent burnout
- **Session analytics** - Understand when you work best

### Task Intelligence
- **Cognitive load scoring** (1-10 mental effort scale)
- **AI task decomposition** - Break overwhelming tasks into manageable steps
- **Estimated vs. actual time** - Reality-check your time perception
- **Dopamine score prediction** - Prioritize rewarding tasks

### Public Dogfooding
- Share your progress publicly
- Daily coding streaks
- Focus time metrics
- Accountability through transparency

## 🛠 Tech Stack

### Backend
- **Python 3.12+** with FastAPI
- **PostgreSQL** for data persistence
- **Redis** for caching and session management
- **Celery** for background tasks
- **SQLAlchemy 2.0** with async support
- **Pydantic v2** for validation

### Frontend
- **Flutter** for mobile (iOS/Android) and web
- **WebSocket** for real-time updates
- **Offline-first** architecture

### Package Management
- **UV** - Fast, modern Python package manager

### Deployment
- **Phase 1:** Railway (current)
- **Phase 2:** Docker containers
- **Phase 3:** Kubernetes scaling

## 🚀 Quick Start

### Prerequisites
- Python 3.12+
- PostgreSQL 14+
- Redis 7+
- Flutter SDK (for mobile/web development)

### Backend Setup

```bash
# Install UV package manager
curl -LsSf https://astral.sh/uv/install.sh | sh

# Clone the repository
git clone https://github.com/getaltair/altair.git
cd altair

# Create virtual environment and install dependencies
uv venv
uv sync

# Setup database
cp .env.example .env  # Edit with your database credentials

# Run migrations
uv run alembic upgrade head

# Start development server
uv run uvicorn app.main:app --reload --port 8000
```

### Frontend Setup

```bash
# Navigate to Flutter directory
cd flutter

# Install dependencies
flutter pub get

# Run on web
flutter run -d chrome

# Run on mobile (with device connected)
flutter run
```

## 📁 Project Structure

```
altair/
├── backend/
│   ├── app/
│   │   ├── main.py              # FastAPI application
│   │   ├── config.py            # Pydantic settings
│   │   ├── models/              # SQLAlchemy models
│   │   ├── schemas/             # Pydantic schemas
│   │   ├── api/                 # Route handlers
│   │   ├── services/            # Business logic
│   │   ├── workers/             # Celery tasks
│   │   └── websocket/           # WebSocket manager
│   ├── alembic/                 # Database migrations
│   └── tests/                   # Pytest tests
├── flutter/                     # Mobile/Web frontend
├── docker/                      # Docker configurations
└── CLAUDE.md                    # Development instructions
```

## 🧪 Development

### Running Tests

```bash
# Backend tests
uv run pytest
uv run pytest --cov=app --cov-report=html

# Flutter tests
cd flutter
flutter test
```

### Code Quality

```bash
# Python linting and formatting
uv run ruff check .
uv run black .

# Flutter analysis
cd flutter
flutter analyze
```

### Database Migrations

```bash
# Create new migration
uv run alembic revision --autogenerate -m "Description"

# Apply migrations
uv run alembic upgrade head

# Rollback migration
uv run alembic downgrade -1
```

## 🎮 API Endpoints

### Authentication
- `POST /api/auth/register` - Create new account
- `POST /api/auth/login` - Login
- `POST /api/auth/refresh` - Refresh token

### Tasks
- `GET /api/tasks` - List tasks (with filters)
- `POST /api/tasks` - Create task
- `POST /api/tasks/quick-capture` - Natural language task creation
- `PUT /api/tasks/{id}` - Update task
- `DELETE /api/tasks/{id}` - Delete task
- `POST /api/tasks/{id}/decompose` - AI task breakdown

### Focus Sessions
- `POST /api/focus/start` - Start Pomodoro
- `POST /api/focus/pause` - Pause session
- `POST /api/focus/complete` - End session

### Public Stats
- `GET /api/public/stats/{username}` - Public profile
- `GET /api/public/activity` - Activity feed

## 🤝 Contributing

We welcome contributions! This project is built with and for neurodivergent developers.

### Contribution Guidelines

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Make your changes** following the coding standards
4. **Write tests** for new functionality
5. **Ensure tests pass** (`uv run pytest`)
6. **Commit your changes** using conventional commits
7. **Push to your fork** (`git push origin feature/amazing-feature`)
8. **Open a Pull Request**

### Commit Convention

We use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `refactor:` Code refactoring
- `test:` Test additions or changes
- `chore:` Maintenance tasks

## 📊 Performance Considerations

- **Database indexes** on common query patterns
- **Redis caching** for expensive computations
- **Connection pooling** for SQLAlchemy
- **Async operations** with httpx for external APIs
- **Pagination** for all list endpoints
- **Query optimization** with eager loading

## 🔒 Security

- **Password hashing** with bcrypt
- **JWT tokens** with refresh rotation
- **SQL injection prevention** via parameterized queries
- **CORS** properly configured
- **Rate limiting** via Redis
- **Input validation** with Pydantic
- **Environment variables** for secrets

## 📝 License

This project is licensed under the **GNU Affero General Public License v3.0** (AGPL-3.0).

See [LICENSE.md](LICENSE.md) for the full license text.

### Why AGPL-3.0?

We believe in open source software that stays open. The AGPL-3.0 ensures that:
- Anyone can use, study, and modify this software
- Modifications must be shared back to the community
- Network use (SaaS) requires sharing source code
- Neurodivergent developers benefit from shared improvements

## 🌟 Roadmap

### Phase 1: MVP (Weeks 1-2) 🚧
- [x] Project initialization
- [x] Flutter scaffolding
- [x] Quick capture functionality
- [x] Basic task CRUD
- [ ] Focus timer

### Phase 2: Core Features (Month 1)
- [ ] AI task decomposition
- [ ] Cognitive load scoring
- [ ] Task state machine
- [ ] WebSocket real-time updates
- [ ] Offline support

### Phase 3: Enhancement (Month 2)
- [ ] Gamification system
- [ ] Pattern analysis
- [ ] Public dogfooding stats
- [ ] Mobile notifications
- [ ] Calendar integration

### Phase 4: Scale (Month 3+)
- [ ] Kubernetes deployment
- [ ] Multi-tenant support
- [ ] Advanced analytics
- [ ] Plugin system
- [ ] API for third-party integrations

## 💬 Community

- **Issues:** [GitHub Issues](https://github.com/getaltair/altair/issues)
- **Discussions:** [GitHub Discussions](https://github.com/getaltair/altair/discussions)
- **Discord:** [discord.gg/altair](https://discord.gg/altair)

## 🙏 Acknowledgments

Built with understanding that neurodivergence is a feature, not a bug.

Special thanks to:
- The ADHD community for sharing their struggles and insights
- The FastAPI and Flutter teams for amazing frameworks
- Everyone who believes better tools can make a difference

## 📚 Resources

- [FastAPI Documentation](https://fastapi.tiangolo.com/)
- [Flutter Documentation](https://docs.flutter.dev/)
- [SQLAlchemy Documentation](https://docs.sqlalchemy.org/)
- [ADHD & Productivity Research](https://chadd.org/)

---

**Where focus takes flight ⭐**

Made with ☕ and hyperfocus by neurodivergent developers, for neurodivergent users.

Learn more at [getaltair.app](https://getaltair.app)
