# Altair Architecture

**Technical Reference for Developers**

> This document provides a comprehensive overview of Altair's system architecture, technology stack, and technical decision-making.

📊 **Visual References:**
- [System Architecture Diagrams](diagrams/01-system-architecture.md)
- [Database Schema & ERD](diagrams/02-database-schema-erd.md)
- [Component Architecture](diagrams/05-component-architecture.md)
- [Deployment & Operations](diagrams/06-deployment-operations.md)

---

## Table of Contents

1. [Overview](#overview)
2. [Technology Stack](#technology-stack)
3. [System Architecture](#system-architecture)
4. [Data Model](#data-model)
5. [API Design](#api-design)
6. [Frontend Architecture](#frontend-architecture)
7. [Offline-First Strategy](#offline-first-strategy)
8. [Security](#security)
9. [Performance](#performance)
10. [Deployment](#deployment)
11. [Future Considerations](#future-considerations)

---

## Overview

Altair is built as a **modular monolith** with clear service boundaries, allowing for future microservice extraction if needed. The architecture prioritizes:

- **ADHD-friendly UX** - Fast, predictable, forgiving
- **Offline-first** - Works without internet, syncs when connected
- **Privacy** - Data encryption, user control, self-hosting option
- **Performance** - Sub-100ms API responses, optimized rendering
- **Scalability** - Designed to handle growth
- **Maintainability** - Clean code, good documentation, testable

**[View High-Level Architecture Diagram](diagrams/01-system-architecture.md#high-level-system-architecture)**

### Architectural Principles

1. **Progressive Enhancement** - Works at all capability levels
2. **Graceful Degradation** - Fails safely when services unavailable
3. **Data Ownership** - Users control their data
4. **API-First** - All features accessible via API
5. **Separation of Concerns** - Clear boundaries between components
6. **YAGNI Compliance** - Build what's needed now, prepare for future

---

## Technology Stack

### Backend

| Component | Technology | Version | Rationale |
|-----------|-----------|---------|-----------|
| **Language** | Python | 3.11+ | Async support, rich ecosystem, rapid development |
| **Framework** | FastAPI | 0.104+ | Modern async framework, auto-docs, type safety |
| **Database** | PostgreSQL | 15+ | Robust, ACID compliant, excellent JSON support |
| **Caching** | Redis | 7+ | Optional performance boost, session storage |
| **ORM** | SQLAlchemy | 2.0+ | Powerful ORM with async support |
| **Migrations** | Alembic | Latest | Database version control |
| **Auth** | JWT | - | Stateless authentication |
| **Validation** | Pydantic | 2.0+ | Type-safe data validation |

**[View Backend Component Architecture](diagrams/05-component-architecture.md#backend-component-hierarchy)**

### Frontend

| Component | Technology | Version | Rationale |
|-----------|-----------|---------|-----------|
| **Framework** | Flutter | 3.16+ | Cross-platform, native performance, single codebase |
| **Language** | Dart | 3.0+ | Type-safe, async/await, null safety |
| **State Management** | Riverpod | 2.0+ | Reactive, testable, clear data flow |
| **Local DB** | Hive | Latest | Fast NoSQL for offline storage |
| **SQLite** | SQLite | 3.40+ | Structured offline data (alternative to Hive) |
| **HTTP Client** | Dio | Latest | Robust networking with interceptors |
| **Routing** | go_router | Latest | Declarative routing |

**[View Frontend Component Architecture](diagrams/05-component-architecture.md#flutter-app-architecture)**

### Infrastructure

| Component | Technology | Rationale |
|-----------|-----------|-----------|
| **Container** | Docker | Consistent environments |
| **Orchestration** | Docker Compose | Simple multi-container deployment |
| **Reverse Proxy** | Nginx | SSL termination, load balancing |
| **CI/CD** | GitHub Actions | Integrated with repository |
| **Monitoring** | TBD | Future consideration |

**[View Deployment Architecture](diagrams/06-deployment-operations.md#self-hosted-deployment)**

---

## System Architecture

### High-Level Overview

```
┌─────────────────────────────────────────────────────────┐
│                    Client Layer                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │
│  │ Flutter Web │  │ Flutter iOS │  │Flutter Droid│    │
│  └─────────────┘  └─────────────┘  └─────────────┘    │
└─────────────┬───────────────┬───────────────┬──────────┘
              │               │               │
              └───────────────┴───────────────┘
                              │
                    ┌─────────▼──────────┐
                    │   Nginx (Optional)  │
                    │   SSL Termination   │
                    └─────────┬──────────┘
                              │
              ┌───────────────▼────────────────┐
              │        FastAPI Application      │
              │                                 │
              │  ┌──────────────────────────┐  │
              │  │    Service Layer         │  │
              │  │  • Auth Service          │  │
              │  │  • Task Service          │  │
              │  │  • Project Service       │  │
              │  │  • Time Service          │  │
              │  │  • Docs Service          │  │
              │  │  • Sync Service          │  │
              │  └──────────────────────────┘  │
              └─────────────┬──────────────────┘
                            │
              ┌─────────────┴──────────────┐
              │                            │
      ┌───────▼────────┐         ┌────────▼────────┐
      │  PostgreSQL    │         │  Redis (Cache)  │
      │  Primary DB    │         │  Optional       │
      └────────────────┘         └─────────────────┘
```

**[View Detailed System Architecture](diagrams/01-system-architecture.md#detailed-system-components)**

### Component Responsibilities

#### API Layer (FastAPI)
- Request validation and sanitization
- Authentication and authorization
- Rate limiting and throttling
- Request routing to services
- Response formatting
- Error handling
- API documentation

#### Service Layer
Each service is responsible for specific domain logic:

**Auth Service:**
- User registration and login
- JWT token generation/validation
- Password hashing and verification
- Session management
- OAuth integration (future)

**Task Service:**
- CRUD operations for tasks
- AI-powered task breakdown
- Task prioritization logic
- Quick capture optimization
- Task search and filtering

**Project Service:**
- Project management
- Hierarchical organization
- Project templates
- Bulk operations

**Time Service:**
- Time tracking start/stop
- Duration calculations
- Visual time representation data
- Time reports and analytics

**Docs Service:**
- Note creation and management
- Markdown rendering
- File attachments (future)
- Full-text search

**Sync Service:**
- Conflict detection
- Conflict resolution strategies
- Change tracking
- Delta sync optimization

**[View Service Architecture Details](diagrams/05-component-architecture.md#service-layer-architecture)**

#### Data Layer
- **PostgreSQL:** Primary data store
  - User accounts
  - Tasks and projects
  - Time entries
  - Notes and documents
  - Sync metadata

- **Redis (Optional):** Performance optimization
  - Session storage
  - Rate limiting counters
  - Temporary cache
  - Real-time features (future)

**[View Complete Database Schema](diagrams/02-database-schema-erd.md#full-database-schema)**

---

## Data Model

### Core Entities

Our data model is optimized for ADHD workflows while maintaining data integrity and performance.

#### Users
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(100),
    settings JSONB DEFAULT '{}',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    last_login TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE
);
```

#### Tasks
```sql
CREATE TABLE tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    project_id UUID REFERENCES projects(id) ON DELETE SET NULL,
    parent_task_id UUID REFERENCES tasks(id) ON DELETE CASCADE,

    title VARCHAR(500) NOT NULL,
    description TEXT,
    status VARCHAR(50) DEFAULT 'todo', -- todo, in_progress, done, archived
    priority VARCHAR(20) DEFAULT 'medium', -- low, medium, high, urgent

    -- ADHD-specific fields
    estimated_duration INTEGER, -- minutes
    actual_duration INTEGER, -- minutes
    energy_level VARCHAR(20), -- low, medium, high
    focus_required VARCHAR(20), -- low, medium, high
    context VARCHAR(100), -- tags for context switching

    -- Time tracking
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    due_date TIMESTAMP,

    -- Metadata
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),

    -- Soft delete
    deleted_at TIMESTAMP,

    -- Sync support
    sync_version INTEGER DEFAULT 1,
    last_synced_at TIMESTAMP
);
```

#### Projects
```sql
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    parent_project_id UUID REFERENCES projects(id) ON DELETE CASCADE,

    name VARCHAR(255) NOT NULL,
    description TEXT,
    color VARCHAR(7), -- hex color
    icon VARCHAR(50), -- icon identifier

    status VARCHAR(50) DEFAULT 'active',

    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    deleted_at TIMESTAMP,

    sync_version INTEGER DEFAULT 1,
    last_synced_at TIMESTAMP
);
```

**[View Full Database ERD](diagrams/02-database-schema-erd.md#entity-relationship-diagram)**

### Relationships

```
User (1) ──────< (N) Projects
User (1) ──────< (N) Tasks
User (1) ──────< (N) TimeEntries
User (1) ──────< (N) Notes

Project (1) ────< (N) Tasks
Project (1) ────< (N) SubProjects

Task (1) ───────< (N) SubTasks
Task (1) ───────< (N) TimeEntries
Task (1) ───────< (N) Notes
```

**[View Detailed Relationships](diagrams/02-database-schema-erd.md#relationships-diagram)**

### Indexing Strategy

```sql
-- Performance-critical indexes
CREATE INDEX idx_tasks_user_id ON tasks(user_id);
CREATE INDEX idx_tasks_project_id ON tasks(project_id);
CREATE INDEX idx_tasks_status ON tasks(status) WHERE deleted_at IS NULL;
CREATE INDEX idx_tasks_created_at ON tasks(created_at DESC);
CREATE INDEX idx_tasks_due_date ON tasks(due_date) WHERE due_date IS NOT NULL;

-- Full-text search
CREATE INDEX idx_tasks_search ON tasks USING GIN(to_tsvector('english', title || ' ' || COALESCE(description, '')));

-- Sync optimization
CREATE INDEX idx_tasks_sync_version ON tasks(sync_version, last_synced_at);
```

**[View Complete Index Strategy](diagrams/02-database-schema-erd.md#database-indexes)**

---

## API Design

### RESTful Principles

We follow REST conventions with pragmatic exceptions for ADHD-specific features:

```
GET    /api/v1/tasks           # List tasks
POST   /api/v1/tasks           # Create task (quick capture)
GET    /api/v1/tasks/{id}      # Get task details
PUT    /api/v1/tasks/{id}      # Update task
DELETE /api/v1/tasks/{id}      # Delete task (soft delete)

POST   /api/v1/tasks/quick     # Ultra-fast capture endpoint
POST   /api/v1/tasks/{id}/breakdown  # AI task breakdown
POST   /api/v1/tasks/{id}/start      # Start time tracking
POST   /api/v1/tasks/{id}/stop       # Stop time tracking
```

**[View Complete API Endpoints](diagrams/01-system-architecture.md#api-endpoints)**

### Request/Response Flow

**[View Request Lifecycle Diagram](diagrams/01-system-architecture.md#api-request-flow)**

Example API request flow:

```
1. Client sends authenticated request
2. Nginx forwards to FastAPI
3. FastAPI middleware validates JWT
4. Request routed to appropriate endpoint
5. Pydantic validates request body
6. Service layer processes business logic
7. Database query executed
8. Response serialized via Pydantic
9. JSON returned to client
10. Flutter app updates UI
```

### Quick Capture Optimization

ADHD brains need **instant** task capture. We optimize for this:

```python
@router.post("/tasks/quick", response_model=TaskResponse)
async def quick_capture_task(
    request: QuickTaskCreate,
    current_user: User = Depends(get_current_user),
    db: AsyncSession = Depends(get_db)
):
    """
    Ultra-fast task creation with minimal required fields.
    Optimized for < 50ms response time.
    """
    task = Task(
        user_id=current_user.id,
        title=request.title,
        status="todo",
        priority="medium"
    )
    db.add(task)
    await db.commit()
    return task
```

**Performance Target:** < 50ms end-to-end for quick capture

**[View Quick Capture Flow Diagram](diagrams/03-user-flows.md#quick-task-capture-flow)**

### Authentication

We use **JWT tokens** for stateless authentication:

```
1. User logs in with email/password
2. Server validates credentials
3. Server generates JWT with user claims
4. Client stores JWT (secure storage)
5. Client includes JWT in Authorization header
6. Server validates JWT on each request
```

**Token Structure:**
```json
{
  "sub": "user-uuid",
  "email": "user@example.com",
  "exp": 1234567890,
  "iat": 1234564290
}
```

**Security Measures:**
- Tokens expire after 7 days
- Refresh tokens for seamless renewal
- Password reset via email verification
- Rate limiting on auth endpoints

**[View Authentication Flow](diagrams/01-system-architecture.md#authentication-flow)**

---

## Frontend Architecture

### Flutter App Structure

```
lib/
├── main.dart                 # App entry point
├── app/
│   ├── app.dart              # App configuration
│   └── routes.dart           # Route definitions
├── core/
│   ├── api/                  # API client
│   ├── models/               # Data models
│   ├── providers/            # Riverpod providers
│   ├── services/             # Business logic services
│   └── utils/                # Utilities
├── features/
│   ├── auth/                 # Authentication
│   ├── tasks/                # Task management
│   ├── projects/             # Project management
│   ├── time/                 # Time tracking
│   ├── focus/                # Focus mode
│   └── settings/             # User settings
└── shared/
    ├── widgets/              # Reusable widgets
    ├── theme/                # App theming
    └── constants/            # Constants
```

**[View Flutter Component Hierarchy](diagrams/05-component-architecture.md#flutter-app-architecture)**

### State Management (Riverpod)

We use **Riverpod** for predictable, testable state management:

```dart
// Provider for task list
final tasksProvider = StateNotifierProvider<TasksNotifier, List<Task>>(
  (ref) => TasksNotifier(ref.read(apiClientProvider))
);

// Provider for current user
final currentUserProvider = StateProvider<User?>((ref) => null);

// Provider for sync status
final syncStatusProvider = StateProvider<SyncStatus>(
  (ref) => SyncStatus.idle
);
```

**Benefits:**
- Compile-time safety
- Easy testing
- Clear data flow
- No BuildContext needed
- Automatic disposal

**[View State Management Diagram](diagrams/05-component-architecture.md#state-management-riverpod)**

### UI Components

ADHD-friendly design principles guide our UI:

```dart
// Quick capture button - always accessible
FloatingActionButton(
  onPressed: () => showQuickCaptureDialog(),
  child: Icon(Icons.add),
  tooltip: 'Quick Capture (Ctrl+N)',
)

// Visual time indicator
LinearProgressIndicator(
  value: task.elapsedTime / task.estimatedDuration,
  color: getTimeColor(task), // Red when overdue
)

// Focus mode - minimal distractions
if (focusMode) {
  return SingleTaskView(task);
} else {
  return TaskListView(tasks);
}
```

**[View UI Component Architecture](diagrams/05-component-architecture.md#ui-component-hierarchy)**

---

## Offline-First Strategy

ADHD users can't afford to lose data due to connectivity issues. Our offline-first approach ensures reliability.

### Local Storage

**Flutter App:**
- **Hive:** Fast NoSQL database for unstructured data
- **SQLite:** Structured data with relationships
- **Secure Storage:** Sensitive data (tokens, passwords)

```dart
// Hive for quick local storage
final box = await Hive.openBox('tasks');
await box.put(task.id, task.toJson());

// SQLite for complex queries
final db = await openDatabase('altair.db');
await db.insert('tasks', task.toMap());
```

### Sync Strategy

**[View Sync Architecture Diagram](diagrams/01-system-architecture.md#offline-sync-architecture)**

**Three-way merge algorithm:**

```
1. Client has local changes
2. Server has remote changes
3. Compare versions:
   - If client ahead: Push to server
   - If server ahead: Pull from server
   - If conflict: Resolve using strategy
```

**Conflict Resolution Strategies:**

1. **Last-Write-Wins** (default for most fields)
   - Most recent change wins
   - Simple, predictable

2. **Field-Level Merge** (for specific fields)
   - Title: Last-write-wins
   - Status: Server wins (to prevent status confusion)
   - Time entries: Append both (sum durations)

3. **Manual Resolution** (for critical conflicts)
   - Show user both versions
   - Let user choose or merge

**[View Sync State Machine](diagrams/01-system-architecture.md#sync-state-diagram)**

### Sync Implementation

```python
# Server-side sync endpoint
@router.post("/sync")
async def sync_data(
    request: SyncRequest,
    current_user: User = Depends(get_current_user),
    db: AsyncSession = Depends(get_db)
):
    """
    Delta sync: Only send changes since last sync.
    """
    # Get changes since last sync
    server_changes = await get_changes_since(
        user_id=current_user.id,
        last_sync=request.last_sync_timestamp
    )

    # Apply client changes
    for change in request.changes:
        await apply_change(change, current_user.id, db)

    # Return server changes
    return SyncResponse(
        changes=server_changes,
        timestamp=datetime.now()
    )
```

**[View Sync Flow Diagram](diagrams/01-system-architecture.md#sync-process-flow)**

---

## Security

### Threat Model

**Primary Threats:**
1. Unauthorized access to user data
2. Data leakage in transit
3. SQL injection attacks
4. Cross-site scripting (XSS)
5. CSRF attacks
6. Denial of service (DoS)

### Security Measures

**Authentication:**
- Bcrypt password hashing (cost factor: 12)
- JWT tokens with short expiry
- Refresh token rotation
- Rate limiting on login attempts

**Authorization:**
- Role-based access control (RBAC)
- User can only access own data
- API-level permission checks
- Database-level row security (future)

**Data Protection:**
- TLS 1.3 for all connections
- Encrypted data at rest (PostgreSQL encryption)
- Secure token storage (Flutter secure storage)
- No sensitive data in logs

**Input Validation:**
- Pydantic models for all inputs
- SQL parameterization (no raw SQL)
- HTML sanitization for markdown
- File upload restrictions (future)

**[View Security Architecture](diagrams/06-deployment-operations.md#security-layers)**

### Self-Hosting Security

Users who self-host should follow these guidelines:

1. **Use HTTPS:** Always use SSL/TLS certificates
2. **Firewall:** Restrict access to necessary ports only
3. **Updates:** Keep software up to date
4. **Backups:** Regular encrypted backups
5. **Monitoring:** Monitor for suspicious activity
6. **Strong Passwords:** Enforce password policies

**[View Security Best Practices](SECURITY.md)**

---

## Performance

### Performance Targets

| Metric | Target | Rationale |
|--------|--------|-----------|
| Quick capture | < 50ms | ADHD: instant feedback needed |
| API response | < 100ms | Perceived as instant |
| Page load | < 1s | Maintain focus |
| Sync duration | < 5s | Don't interrupt workflow |
| Database query | < 50ms | Keep API fast |

### Optimization Strategies

**Database:**
- Proper indexing on frequently queried fields
- Connection pooling (SQLAlchemy)
- Query optimization (EXPLAIN ANALYZE)
- Pagination for large datasets
- Materialized views for complex queries (future)

**API:**
- Async/await for concurrent operations
- Response caching (Redis)
- Request batching where appropriate
- GraphQL for complex queries (future consideration)

**Frontend:**
- Lazy loading of routes
- Image optimization
- Virtual scrolling for long lists
- Debouncing for search
- Optimistic UI updates

**[View Performance Optimization Diagram](diagrams/05-component-architecture.md#performance-optimization)**

### Monitoring (Future)

- Response time tracking
- Error rate monitoring
- Database performance metrics
- User behavior analytics (privacy-respecting)

---

## Deployment

### Development Environment

**[View Development Setup Diagram](diagrams/06-deployment-operations.md#development-environment)**

```bash
# Backend
cd backend
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt
python main.py

# Frontend
cd frontend
flutter pub get
flutter run -d chrome
```

### Docker Deployment

**[View Docker Architecture](diagrams/06-deployment-operations.md#docker-deployment)**

```yaml
# docker-compose.yml
version: '3.8'

services:
  postgres:
    image: postgres:15
    environment:
      POSTGRES_DB: altair
      POSTGRES_USER: altair
      POSTGRES_PASSWORD: changeme
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

  redis:
    image: redis:7
    ports:
      - "6379:6379"

  backend:
    build: ./backend
    ports:
      - "8000:8000"
    depends_on:
      - postgres
      - redis
    environment:
      DATABASE_URL: postgresql://altair:changeme@postgres/altair
      REDIS_URL: redis://redis:6379

  nginx:
    image: nginx:latest
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/nginx/ssl
    depends_on:
      - backend
```

### Production Deployment

**Self-Hosted:**

**[View Self-Hosted Deployment Diagram](diagrams/06-deployment-operations.md#self-hosted-deployment)**

1. Set up server (Ubuntu 22.04 LTS recommended)
2. Install Docker and Docker Compose
3. Clone repository
4. Configure environment variables
5. Generate SSL certificates (Let's Encrypt)
6. Run `docker-compose up -d`
7. Configure backups
8. Set up monitoring (future)

**Managed Hosting (Future):**

**[View Managed Hosting Architecture](diagrams/06-deployment-operations.md#managed-hosting-cloud)**

- Kubernetes cluster
- Auto-scaling backend pods
- Managed PostgreSQL (RDS/CloudSQL)
- CDN for static assets
- Load balancer
- Automated backups
- Monitoring and alerting

---

## Future Considerations

### Scalability

**When we need it:**
- 1,000+ concurrent users
- 1M+ tasks in database
- High-frequency sync operations

**Scaling Strategy:**

**[View Scaling Architecture](diagrams/06-deployment-operations.md#managed-hosting-cloud)**

1. **Vertical Scaling** (first step)
   - Larger database instance
   - More powerful API servers

2. **Horizontal Scaling** (if needed)
   - Multiple API server instances
   - Load balancer distribution
   - Read replicas for database
   - Microservices extraction (if beneficial)

3. **Caching Layer** (performance boost)
   - Redis for frequent reads
   - CDN for static assets
   - Application-level caching

### Microservices Migration (If Needed)

If the monolith becomes unwieldy, we can extract services:

```
Monolith → Extract Auth Service
        → Extract Task Service
        → Extract AI Service
        → Keep core as thin orchestrator
```

**When to consider:**
- Single service causing performance issues
- Need independent scaling
- Team growing (separate service ownership)

**[View Microservices Migration Path](diagrams/05-component-architecture.md#future-microservices)**

### AI Integration

**Current:** AI-powered task breakdown via API calls

**Future Possibilities:**
- Natural language task creation
- Smart scheduling suggestions
- Pattern recognition (productivity insights)
- Voice-to-task conversion
- Predictive task estimation

### Collaboration Features

**Multi-user projects:**
- Real-time collaboration
- Task assignment
- Comments and mentions
- Activity feeds
- Permissions system

**Technical Implications:**
- WebSocket for real-time updates
- More complex authorization
- Conflict resolution challenges
- Performance considerations

---

## Technical Decisions

### Why FastAPI?

✅ **Pros:**
- Modern async/await support
- Auto-generated OpenAPI docs
- Type hints and validation
- Fast development
- Excellent performance
- Great community

❌ **Cons:**
- Python GIL (mitigated by async)
- Smaller ecosystem than Flask
- Newer (less battle-tested)

**Alternatives Considered:**
- Django REST Framework (too heavy)
- Flask (not async-first)
- Go (steeper learning curve)
- Node.js (callback hell, type safety issues)

### Why Flutter?

✅ **Pros:**
- Single codebase for all platforms
- Native performance
- Beautiful, customizable UI
- Strong typing (Dart)
- Hot reload development
- Growing ecosystem

❌ **Cons:**
- Larger app size
- Less mature than React Native
- Learning curve for Dart

**Alternatives Considered:**
- React Native (performance concerns)
- Native iOS/Android (2x development effort)
- PWA only (limited offline capabilities)

### Why PostgreSQL?

✅ **Pros:**
- Robust and reliable
- ACID compliant
- Excellent JSON support (JSONB)
- Full-text search
- Free and open source
- Mature ecosystem

❌ **Cons:**
- More complex than SQLite
- Resource intensive

**Alternatives Considered:**
- SQLite (not suitable for concurrent writes)
- MongoDB (ACID limitations, less reliable)
- MySQL (less modern features)

---

## Questions?

For architectural questions or discussions:
- **GitHub Issues:** [github.com/getaltair/altair/issues](https://github.com/getaltair/altair/issues)
- **Discord:** [discord.gg/altair](https://discord.gg/altair)
- **Email:** dev@getaltair.app

---

**Last Updated:** October 2025
**Author:** Altair Development Team
**Status:** Living Document

---

## Related Documentation

- [Features](FEATURES.md) - What we're building
- [Roadmap](ROADMAP.md) - When we're building it
- [Contributing](CONTRIBUTING.md) - How to help build it
- [Visual Diagrams](diagrams/README.md) - All architecture diagrams
