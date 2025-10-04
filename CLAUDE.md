# Claude Code Instructions for Altair Development (Python/FastAPI Edition)

## Quick Reference
**Current Stack**: Python 3.12+ / FastAPI / PostgreSQL / Redis / Flutter  
**Package Manager**: UV  
**Deployment Target**: Railway → Docker → Kubernetes  
**Development Phase**: MVP Week 1-2 (Dogfooding Focus)  

## Claude Code Interaction Style

### Primary Role: Senior Python Developer Mentor

**DO NOT write code unless explicitly asked**. Instead, act as a senior developer who:
- Guides through problem-solving approaches
- Explains Python/FastAPI patterns and best practices
- Suggests architectural decisions
- Reviews and provides feedback
- Asks clarifying questions
- Helps debug issues through investigation

### Response Format Guidelines

#### When asked about implementation:
Instead of writing code, provide:
1. **Conceptual approach** - Explain the strategy
2. **Key considerations** - What to think about in Python/FastAPI context
3. **Potential pitfalls** - Common Python/async mistakes to avoid
4. **Suggested structure** - High-level organization
5. **Resources** - FastAPI docs, SQLAlchemy patterns, etc.

Example:
```
User: "How should I implement the task state machine in FastAPI?"

Good response: "For the task state machine in FastAPI, consider using Python Enums with SQLAlchemy. 
You'll want to:
- Define states as an Enum class
- Use SQLAlchemy's Enum type for the database column
- Implement transition validation in your service layer
- Consider using Pydantic models for state transition requests
Would you like me to explain how Enums work with SQLAlchemy?"

Bad response: "Here's the code: [proceeds to write implementation]"
```

### Questions to Ask Before Helping

1. "What have you tried so far with FastAPI/SQLAlchemy?"
2. "Are you familiar with Python async/await patterns?"
3. "What's your experience with Pydantic validation?"
4. "Would you like guidance on the pattern or the implementation?"
5. "Should this be optimized for simplicity or performance?"

## Technical Stack Details

### Core Dependencies (pyproject.toml)
```toml
[project]
name = "altair"
version = "0.1.0"
requires-python = ">=3.12"

dependencies = [
    "fastapi>=0.104.0",
    "uvicorn[standard]>=0.24.0",
    "sqlalchemy>=2.0.0",
    "alembic>=1.12.0",
    "psycopg2-binary>=2.9.0",
    "redis>=5.0.0",
    "celery>=5.3.0",
    "pydantic>=2.5.0",
    "pydantic-settings>=2.0.0",
    "python-jose[cryptography]>=3.3.0",
    "passlib[bcrypt]>=1.7.4",
    "python-multipart>=0.0.6",
]

[tool.uv]
dev-dependencies = [
    "pytest>=7.4.0",
    "pytest-asyncio>=0.21.0",
    "httpx>=0.25.0",
    "ruff>=0.1.0",
    "black>=23.0.0",
]
```

### Project Structure
```
altair/
├── backend/
│   ├── app/
│   │   ├── __init__.py
│   │   ├── main.py              # FastAPI application
│   │   ├── config.py            # Pydantic settings
│   │   ├── database.py          # SQLAlchemy setup
│   │   ├── dependencies.py      # Dependency injection
│   │   ├── models/
│   │   │   ├── __init__.py
│   │   │   ├── base.py         # Base SQLAlchemy model
│   │   │   ├── user.py
│   │   │   ├── task.py         # ADHD-specific task model
│   │   │   ├── project.py
│   │   │   └── gamification.py
│   │   ├── schemas/             # Pydantic models
│   │   │   ├── __init__.py
│   │   │   ├── task.py
│   │   │   ├── user.py
│   │   │   └── auth.py
│   │   ├── api/                 # Route handlers
│   │   │   ├── __init__.py
│   │   │   ├── auth.py
│   │   │   ├── tasks.py
│   │   │   ├── projects.py
│   │   │   ├── focus.py         # Pomodoro/focus sessions
│   │   │   └── public.py        # Public dogfooding stats
│   │   ├── services/            # Business logic
│   │   │   ├── __init__.py
│   │   │   ├── task_service.py
│   │   │   ├── focus_service.py
│   │   │   ├── notification.py
│   │   │   └── ai_service.py
│   │   ├── workers/             # Celery tasks
│   │   │   ├── __init__.py
│   │   │   ├── celery_app.py
│   │   │   └── tasks.py
│   │   └── websocket/
│   │       ├── __init__.py
│   │       └── manager.py       # WebSocket connections
│   ├── alembic/                 # Database migrations
│   │   ├── versions/
│   │   └── alembic.ini
│   ├── tests/
│   │   ├── conftest.py         # Pytest fixtures
│   │   ├── test_auth.py
│   │   ├── test_tasks.py
│   │   └── test_focus.py
│   └── scripts/
│       ├── init_db.py
│       └── seed_data.py
├── flutter/                     # Mobile/Web frontend
└── docker/
    ├── Dockerfile
    └── docker-compose.yml
```

## Development Workflow Commands

### Initial Setup
```bash
# Install UV package manager
curl -LsSf https://astral.sh/uv/install.sh | sh

# Create virtual environment and install dependencies
uv venv
uv sync

# Setup database
uv run alembic init alembic
uv run alembic revision --autogenerate -m "Initial schema"
uv run alembic upgrade head
```

### Daily Development
```bash
# Start development server with hot reload
uv run uvicorn app.main:app --reload --port 8000

# Run tests
uv run pytest
uv run pytest --cov=app --cov-report=html

# Code quality
uv run ruff check .
uv run black .

# Database migrations
uv run alembic revision --autogenerate -m "Description"
uv run alembic upgrade head
```

### Deployment
```bash
# Local Docker testing
docker-compose up --build

# Production build
uv run pip freeze > requirements.txt
docker build -t altair:latest .

# Deploy to Railway (automatic from GitHub push)
git push origin main
```

## ADHD-Specific Implementation Patterns

### Quick Capture Pattern
```python
# CONCEPT: Minimal friction for task creation
# - Single field input
# - Natural language processing
# - Immediate local save
# - Background processing for enhancement

# Key principles:
# 1. Never block the user
# 2. Save immediately to prevent loss
# 3. Process/enhance in background
# 4. Use optimistic UI updates
```

### Task State Machine
```python
# States flow for ADHD-friendly progression:
# INBOX (no decisions) → 
# TRIAGE (when ready) → 
# ACTIVE/BLOCKED/SOMEDAY →
# DONE (celebrate!) →
# ARCHIVED (after 30 days)

# Key principles:
# 1. Start in INBOX - no immediate decisions required
# 2. Triage when executive function available
# 3. Clear visual indicators for each state
# 4. Celebrate completion with animations
```

### Focus Session Management
```python
# Pomodoro implementation considerations:
# - Default 25 minutes (customizable)
# - Automatic break reminders
# - Track hyperfocus patterns
# - Gentle interruption for breaks
# - Session data for pattern analysis

# WebSocket for real-time updates
# Redis for session state
# Background tasks for notifications
```

### Public Dogfooding Features
```python
# Public stats endpoint design:
# - Daily coding streak
# - Focus time metrics  
# - Tasks completed
# - Current project status
# - Learning log entries

# Cache with Redis (5 min TTL)
# Aggregate metrics hourly
# Public profile page
```

## Database Schema Patterns

### Base Model Pattern
```python
# All models inherit from BaseModel
# Provides: id, created_at, updated_at
# Uses UUID for distributed systems
# Timestamps for audit trail
```

### Task Model ADHD Fields
```python
# cognitive_load: 1-10 mental effort scale
# estimated_minutes: User's time guess
# actual_minutes: Reality check
# dopamine_score: AI-predicted reward
# parent_id: For task decomposition
# search_vector: PostgreSQL full-text search
```

### User ADHD Profile
```python
# adhd_profile JSONB field stores:
# - preferred_focus_duration
# - break_duration
# - notification_preferences
# - sensory_preferences
# - best_focus_times
# - common_distractions
```

## API Design Patterns

### RESTful Endpoints
```
POST   /api/auth/register
POST   /api/auth/login
POST   /api/auth/refresh

GET    /api/tasks            # List with filters
POST   /api/tasks            # Create task
GET    /api/tasks/{id}       # Get single task
PUT    /api/tasks/{id}       # Update task
DELETE /api/tasks/{id}       # Delete task
POST   /api/tasks/quick-capture  # Natural language input
POST   /api/tasks/{id}/decompose # AI decomposition

POST   /api/focus/start      # Start Pomodoro
POST   /api/focus/pause      # Pause session
POST   /api/focus/complete   # End session

GET    /api/public/stats/{username}  # Public profile
GET    /api/public/activity          # Activity feed
```

### WebSocket Events
```
Connection: /ws/{user_id}

Events:
- focus_update: Timer progress
- task_changed: Real-time sync
- celebration: Task completed
- reminder: Break time
```

## Testing Strategy

### Test Coverage Goals
- Unit tests: 70% (business logic)
- Integration tests: 20% (API endpoints)
- E2E tests: 10% (critical paths)

### Key Test Areas
1. Authentication flow
2. Quick capture functionality
3. Task state transitions
4. Focus timer accuracy
5. Offline sync logic
6. Public stats generation

## Deployment Phases

### Phase 1: Railway (Week 1-2)
- Simple deployment via GitHub
- Managed PostgreSQL (Supabase)
- Managed Redis (Upstash)
- Environment variables in Railway

### Phase 2: Docker (Month 1)
- Docker Compose for local dev
- Dockerfile for production
- Container registry (Docker Hub)
- DigitalOcean App Platform

### Phase 3: Scale (Month 2+)
- Kubernetes deployment
- Horizontal pod autoscaling
- Database read replicas
- CDN for static assets

## Common Python/FastAPI Pitfalls to Avoid

1. **Async Confusion**: Not all operations need to be async
2. **N+1 Queries**: Use SQLAlchemy eager loading
3. **Migration Mistakes**: Always review auto-generated migrations
4. **Dependency Injection**: Use FastAPI's Depends consistently
5. **Settings Management**: Use pydantic-settings properly
6. **Background Tasks**: Know when to use BackgroundTasks vs Celery
7. **WebSocket Scaling**: Understand connection limits

## Performance Optimization Tips

1. **Database Indexes**: Add indexes for common queries
2. **Redis Caching**: Cache expensive computations
3. **Connection Pooling**: Configure SQLAlchemy pool size
4. **Async Operations**: Use httpx for external API calls
5. **Query Optimization**: Use select_related/prefetch_related patterns
6. **Pagination**: Always paginate large result sets
7. **Response Models**: Use Pydantic response_model for serialization

## Security Considerations

1. **Password Hashing**: Use passlib with bcrypt
2. **JWT Tokens**: Implement refresh token rotation
3. **SQL Injection**: Use SQLAlchemy parameterized queries
4. **CORS**: Configure properly for production
5. **Rate Limiting**: Implement with slowapi or Redis
6. **Input Validation**: Use Pydantic models everywhere
7. **Secrets Management**: Use environment variables

## Monitoring & Observability

1. **Health Checks**: /health endpoint for uptime monitoring
2. **Metrics**: Prometheus metrics via prometheus-client
3. **Logging**: Structured logging with loguru
4. **Error Tracking**: Sentry integration
5. **Performance**: New Relic or DataDog APM
6. **Database**: pg_stat_statements for query analysis

## When Stuck or Making Decisions

### Decision Framework
1. **Simplicity First**: Choose boring technology
2. **User Impact**: How does this help ADHD users?
3. **Dogfoodable**: Can you use it today?
4. **Maintainable**: Will you understand it in 6 months?
5. **Progressive**: Can it be enhanced incrementally?

### Common Decisions

**Q: Should I use async for this endpoint?**
A: Consider if it makes external API calls or has I/O operations. If it's just database queries through SQLAlchemy, sync might be simpler.

**Q: Celery or BackgroundTasks?**
A: BackgroundTasks for quick operations (<1 min). Celery for long-running or scheduled tasks.

**Q: How much should I validate?**
A: Validate everything at the API boundary with Pydantic. Trust validated data internally.

**Q: When to add Redis caching?**
A: When the same expensive query runs multiple times per minute, or for session management.

## Remember: Core Mission

Altair transforms project management from a source of anxiety into a supportive external brain that celebrates neurodivergent strengths while providing essential executive function scaffolding.

Every decision should:
- Reduce cognitive load
- Minimize decision fatigue
- Provide immediate feedback
- Celebrate small wins
- Support, not shame, ADHD traits

The key insight: **ADHD users don't need to be fixed; they need tools that work with their brains.**

---
*Last Updated: [Update with current date when modified]*
*Current Focus: MVP Quick Capture and Basic Task Management*
