# Altair Python/FastAPI Context

## Current Implementation Status

**Stack**: Python 3.12 + FastAPI + PostgreSQL + Redis + Flutter  
**Phase**: MVP Development - Dogfooding Focus  
**Goal**: Public development with transparent progress sharing  

## Architecture Overview

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│   Flutter   │────▶│   FastAPI    │────▶│ PostgreSQL  │
│  PWA/Mobile │     │   + Uvicorn  │     │    + Redis  │
└─────────────┘     └──────────────┘     └─────────────┘
                            │
                    ┌───────▼────────┐
                    │ Background Jobs│
                    │    (Celery)    │
                    └────────────────┘
```

## Key Design Decisions

### Why Python/FastAPI over Go?
- **Developer velocity**: Faster iteration for solo development
- **Rich ecosystem**: Mature libraries for all needs
- **Community**: Larger pool of potential contributors
- **AI/ML integration**: Better Python support for future AI features
- **Trade-off**: Slightly lower performance vs Go, acceptable for MVP

### Why PostgreSQL + Redis over SurrealDB?
- **Maturity**: Battle-tested in production
- **Ecosystem**: Excellent Python support (SQLAlchemy)
- **Operations**: Well-understood backup/recovery
- **Hosting**: Available on all platforms (Railway, Render, etc.)
- **Trade-off**: More complex than single SurrealDB instance

### Why UV over pip/poetry?
- **Speed**: 10-100x faster than pip
- **Simplicity**: Single tool for all package operations
- **Modern**: Built with Rust, actively developed
- **Compatible**: Works with existing requirements.txt

## Development Priorities

### Week 1-2: Core Dogfooding Features
1. ✅ Basic FastAPI setup with auth
2. ✅ Quick task capture endpoint
3. ✅ Task CRUD with state machine
4. ⏳ Public stats endpoint
5. ⏳ Deploy to Railway
6. ⏳ Basic Flutter UI

### Week 3-4: ADHD-Specific Features  
- Pomodoro timer with WebSocket updates
- Task decomposition UI
- Time tracking with pattern analysis
- Daily planning view
- Focus mode

### Month 2: Gamification & Social
- XP and leveling system
- Badge achievements
- Public profile pages
- Activity feed
- Guild system basics

## ADHD-Specific Patterns

### Quick Capture Philosophy
```python
# Maximum 3 seconds from thought to saved
# No forms, no categories, no decisions
# Just: Type → Enter → Done
```

### Task States for Executive Dysfunction
```python
class TaskState(Enum):
    INBOX = "inbox"        # No decision needed
    TRIAGED = "triaged"    # Reviewed when able
    ACTIVE = "active"      # Currently working
    BLOCKED = "blocked"    # Waiting on something
    DONE = "done"         # Celebrate!
    ARCHIVED = "archived"  # Auto after 30 days
```

### Cognitive Load Management
- Each task has a `cognitive_load` score (1-10)
- UI shows total load for active tasks
- Warns when approaching overload
- Suggests breaks based on accumulated load

### Time Blindness Mitigation
- Visual timers with progress rings
- "Elapsed time" always visible
- Historical accuracy shown ("usually takes 2x estimate")
- Gentle notifications about time passing

## Public Dogfooding Strategy

### What to Share
1. **Daily Stats**: Tasks completed, focus time, streak
2. **Weekly Reports**: Progress on features, learnings
3. **Live Coding**: Stream development sessions
4. **Struggles**: Share ADHD challenges openly
5. **Wins**: Celebrate small victories

### Where to Share
- **Twitter/X**: Daily updates with #BuildInPublic
- **LinkedIn**: Weekly long-form updates
- **Personal Blog**: Monthly deep dives
- **GitHub**: All code open from day 1

### Public Endpoints
```python
GET /api/public/stats/{username}     # Development metrics
GET /api/public/activity            # Recent activity feed
GET /api/public/current-task        # What I'm working on now
GET /api/public/focus-session       # Live focus timer
```

## Technical Implementation Notes

### Database Models Philosophy
- **Flat is better than nested**: Avoid complex relationships
- **JSONB for flexibility**: Store ADHD profile, preferences
- **Explicit over magic**: Clear field names, no abbreviations
- **Soft deletes**: Never lose user data

### API Design Principles
- **Resource-based**: RESTful patterns
- **Predictable**: Consistent response formats
- **Forgiving**: Accept multiple input formats
- **Fast**: <200ms response time target

### Error Handling Approach
```python
# Errors should be:
# 1. Encouraging, not shaming
# 2. Actionable, with clear next steps
# 3. Logged for pattern analysis
# 4. Never lose user data

# Bad: "Invalid input"
# Good: "Let's try that again - tasks need a title"
```

### Testing Philosophy
- **Test the critical path first**: Quick capture, task states
- **Integration over unit tests**: How parts work together
- **Real-world scenarios**: Based on actual ADHD usage
- **Performance tests**: Ensure snappy response

## Performance Targets

### Response Times
- Quick capture: <100ms
- Task list: <200ms  
- Public stats: <500ms (cached)
- WebSocket latency: <50ms

### Capacity Planning
- Personal use: 1-10 users
- Beta: 100 users
- V1 Launch: 1,000 users
- Scale considerations only after 100 active users

## Security Considerations

### MVP Security (Good Enough)
- Bcrypt password hashing
- JWT with refresh tokens
- HTTPS only (Railway provides)
- Basic rate limiting
- Input validation with Pydantic

### Future Security
- 2FA support
- OAuth providers
- E2E encryption option
- GDPR compliance
- SOC2 (if enterprise)

## Deployment Strategy

### Phase 1: Railway (Current)
```yaml
# Simple as:
# 1. Push to GitHub
# 2. Railway auto-deploys
# 3. Supabase for PostgreSQL
# 4. Upstash for Redis
```

### Phase 2: Docker (Month 2)
```dockerfile
# Self-hosting option
# Docker Compose for easy setup
# Documentation for non-technical users
```

### Phase 3: Scale (When needed)
```yaml
# Kubernetes only if >1000 active users
# Horizontal scaling
# Read replicas
# Multi-region
```

## Integration Points

### Current Integrations
- GitHub: Code repository
- Railway: Deployment
- Supabase: Database
- Upstash: Redis cache

### Planned Integrations
- OpenAI/Claude API: Task decomposition
- Google Calendar: Sync events
- Slack/Discord: Notifications
- Stripe: Future monetization

### MCP (Model Context Protocol)
- Prepare for MCP integration
- Structure APIs for LLM consumption
- Document patterns for AI assistance

## Success Metrics

### Technical Metrics
- Deployment frequency
- Response times
- Error rates
- Test coverage

### ADHD-Specific Metrics
- Task completion rate
- Average time in INBOX
- Focus session completion
- User retention (daily active)

### Business Metrics
- User signups
- Active users
- Feature adoption
- User feedback

## Common Commands Reference

```bash
# Development
uv run dev           # Start dev server
uv run test         # Run tests
uv run format       # Format code
uv run db-upgrade   # Run migrations

# Deployment  
git push main       # Auto-deploy to Railway
docker build .      # Build container
docker-compose up   # Local testing

# Database
uv run alembic revision --autogenerate -m "msg"
uv run alembic upgrade head
uv run alembic downgrade -1
```

## Remember: The Mission

**Build tools that work WITH ADHD brains, not against them.**

Every feature should:
1. Reduce friction
2. Minimize decisions
3. Provide immediate feedback
4. Celebrate progress
5. Support mistakes
6. Enable hyperfocus
7. Prevent overwhelm

The goal isn't perfection - it's progress. Ship early, iterate often, and learn from real usage.

---
*Context for: Python/FastAPI implementation of Altair*
*Updated: [Current Date]*
*Status: Actively developing MVP*
