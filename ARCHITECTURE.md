# Altair Architecture

**System Design & Technical Decisions**

---

## Table of Contents

- [Overview](#overview)
- [Tech Stack](#tech-stack)
- [System Architecture](#system-architecture)
- [Data Model](#data-model)
- [API Design](#api-design)
- [Frontend Architecture](#frontend-architecture)
- [Offline-First Strategy](#offline-first-strategy)
- [Security & Privacy](#security--privacy)
- [Deployment](#deployment)
- [Future Considerations](#future-considerations)

---

## Overview

Altair is built as a modular, offline-first application with a clear separation between backend services and frontend clients. The architecture prioritizes:

1. **Simplicity** - Easy to understand, maintain, and deploy
2. **Privacy** - Data ownership and control
3. **Performance** - Responsive even with poor connectivity
4. **Scalability** - From single-user to team deployments
5. **Extensibility** - Plugin system for future features

## Tech Stack

### Backend

**FastAPI (Python 3.11+)**

- Modern async Python framework
- Automatic OpenAPI/Swagger documentation
- Excellent performance with async/await
- Type hints for better code quality
- Easy to learn and maintain

**PostgreSQL 15+**

- Proven reliability and performance
- Rich feature set (JSONB, full-text search, etc.)
- Strong open-source ecosystem
- Excellent tooling and community support
- Well-understood scaling patterns

**Additional Backend Tools:**

- **SQLAlchemy 2.0** - ORM with async support
- **Alembic** - Database migrations
- **Pydantic v2** - Data validation and serialization
- **pytest** - Testing framework
- **Redis** (optional) - Caching and rate limiting

### Frontend

**Flutter 3.16+**

- Single codebase for web and mobile
- Excellent performance and smooth animations
- Rich widget ecosystem
- Strong offline support
- Hot reload for fast development

**State Management:**

- **Riverpod** - Reactive state management
- **Drift** (formerly Moor) - Local SQLite database for offline storage
- **Freezed** - Immutable data classes

### Infrastructure

**Docker & Docker Compose**

- Consistent development and production environments
- Easy local setup
- Simplified deployment

**Nginx**

- Reverse proxy
- Static file serving
- SSL termination

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Client Layer                             │
├─────────────────────────────────────────────────────────────┤
│  Flutter Web App          │         Flutter Mobile App      │
│  (Browser)                │         (iOS/Android)           │
└─────────────────┬─────────┴──────────────┬─────────────────┘
                  │                         │
                  │   HTTPS/WebSocket       │
                  │                         │
┌─────────────────▼─────────────────────────▼─────────────────┐
│                     API Gateway                              │
│                     (Nginx)                                  │
└─────────────────┬────────────────────────────────────────────┘
                  │
┌─────────────────▼────────────────────────────────────────────┐
│                  FastAPI Application                         │
├──────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Auth       │  │   Tasks      │  │  Projects    │      │
│  │   Module     │  │   Module     │  │  Module      │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   AI         │  │   Docs       │  │  Analytics   │      │
│  │   Module     │  │   Module     │  │  Module      │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────┬────────────────────────────────────────────┘
                  │
┌─────────────────▼────────────────────────────────────────────┐
│                   Data Layer                                 │
├──────────────────────────────────────────────────────────────┤
│  PostgreSQL          │  Redis (optional)  │  Object Storage │
│  - User data         │  - Cache           │  - File uploads │
│  - Tasks/Projects    │  - Sessions        │  - Attachments  │
│  - Activity logs     │  - Rate limiting   │                 │
└──────────────────────────────────────────────────────────────┘
```

### Key Design Decisions

**Modular Monolith**

- Start with a single application, organized into clear modules
- Each module handles a specific domain (auth, tasks, projects, etc.)
- Easy to extract into microservices later if needed
- Simpler deployment and debugging for early stages

**API-First**

- FastAPI provides automatic OpenAPI documentation
- Makes it easy to build alternative clients later
- Clear contract between frontend and backend
- Facilitates testing and integration

**Offline-First**

- Flutter apps maintain local SQLite database
- Sync when connection available
- Conflict resolution strategy for concurrent edits
- Optimistic updates for better UX

## Data Model

### Core Entities

```python
# Simplified schema representation

User
├── id: UUID
├── email: String (unique)
├── username: String (unique)
├── created_at: DateTime
└── settings: JSONB

Workspace
├── id: UUID
├── owner_id: UUID (FK -> User)
├── name: String
├── created_at: DateTime
└── members: List[WorkspaceMember]

Project
├── id: UUID
├── workspace_id: UUID (FK -> Workspace)
├── name: String
├── description: Text
├── color: String
├── status: Enum
├── created_at: DateTime
└── archived_at: DateTime (nullable)

Task
├── id: UUID
├── project_id: UUID (FK -> Project)
├── parent_id: UUID (nullable, FK -> Task)
├── title: String
├── description: Text
├── status: Enum
├── priority: Enum
├── estimated_duration: Integer (minutes)
├── actual_duration: Integer (minutes)
├── due_date: DateTime (nullable)
├── created_at: DateTime
├── completed_at: DateTime (nullable)
├── assigned_to: UUID (FK -> User, nullable)
└── metadata: JSONB

Note
├── id: UUID
├── task_id: UUID (FK -> Task, nullable)
├── project_id: UUID (FK -> Project, nullable)
├── content: Text
├── created_at: DateTime
└── created_by: UUID (FK -> User)

TimeEntry
├── id: UUID
├── task_id: UUID (FK -> Task)
├── user_id: UUID (FK -> User)
├── start_time: DateTime
├── end_time: DateTime (nullable)
└── duration: Integer (seconds)
```

### ADHD-Specific Fields

Tasks include ADHD-friendly metadata:

- **Energy level required** - Mental energy estimation
- **Focus type** - Deep work vs. quick task
- **Breakdown suggestions** - AI-generated subtask recommendations
- **Time blindness helpers** - Visual duration indicators
- **Dopamine hooks** - Progress badges, streaks

## API Design

### RESTful Endpoints

```http
Authentication
POST   /api/v1/auth/register
POST   /api/v1/auth/login
POST   /api/v1/auth/refresh
POST   /api/v1/auth/logout

Users
GET    /api/v1/users/me
PATCH  /api/v1/users/me
DELETE /api/v1/users/me

Workspaces
GET    /api/v1/workspaces
POST   /api/v1/workspaces
GET    /api/v1/workspaces/{id}
PATCH  /api/v1/workspaces/{id}
DELETE /api/v1/workspaces/{id}

Projects
GET    /api/v1/workspaces/{workspace_id}/projects
POST   /api/v1/workspaces/{workspace_id}/projects
GET    /api/v1/projects/{id}
PATCH  /api/v1/projects/{id}
DELETE /api/v1/projects/{id}

Tasks
GET    /api/v1/projects/{project_id}/tasks
POST   /api/v1/projects/{project_id}/tasks
GET    /api/v1/tasks/{id}
PATCH  /api/v1/tasks/{id}
DELETE /api/v1/tasks/{id}
POST   /api/v1/tasks/{id}/breakdown  # AI task decomposition

Time Tracking
POST   /api/v1/tasks/{task_id}/time/start
POST   /api/v1/tasks/{task_id}/time/stop
GET    /api/v1/time-entries
```

### WebSocket Endpoints

```text
/ws/sync  - Real-time data synchronization
/ws/focus - Focus session updates
```

## Frontend Architecture

### Flutter App Structure

```text
lib/
├── main.dart
├── core/
│   ├── config/
│   ├── constants/
│   ├── utils/
│   └── theme/
├── data/
│   ├── models/
│   ├── repositories/
│   ├── local/          # Drift database
│   └── remote/         # API clients
├── domain/
│   ├── entities/
│   └── usecases/
├── presentation/
│   ├── providers/      # Riverpod state
│   ├── screens/
│   ├── widgets/
│   └── routes/
└── services/
    ├── auth/
    ├── sync/
    └── notifications/
```

### State Management Flow

```text
User Action
    ↓
UI Widget
    ↓
Riverpod Provider
    ↓
Use Case (business logic)
    ↓
Repository (abstraction)
    ↓
├─→ Local DB (Drift)     [Immediate UI update]
└─→ Remote API (FastAPI) [Background sync]
```

## Offline-First Strategy

### Synchronization Architecture

1. **Local-First Operations**
   - All CRUD operations write to local Drift database first
   - UI updates immediately (optimistic updates)
   - Changes queued for sync

2. **Background Sync**
   - Periodic sync when online (configurable interval)
   - Manual sync trigger available
   - Sync status indicators

3. **Conflict Resolution**
   - Last-write-wins for simple fields
   - Operational transforms for complex data (future)
   - User notification on conflicts requiring manual resolution

4. **Sync Queue**

   ```text
   SyncQueue
   ├── id: UUID
   ├── operation: Enum (CREATE, UPDATE, DELETE)
   ├── entity_type: String
   ├── entity_id: UUID
   ├── payload: JSONB
   ├── created_at: DateTime
   └── synced_at: DateTime (nullable)
   ```

## Security & Privacy

### Authentication

- **JWT-based authentication**
- Access tokens (short-lived, 15 minutes)
- Refresh tokens (longer-lived, 7 days)
- Secure HTTP-only cookies for web

### Authorization

- **Role-based access control (RBAC)**
- Workspace-level permissions
- Project-level permissions
- Fine-grained task permissions (future)

### Data Privacy

- **End-to-end encryption** (planned for future)
- Data export in standard formats
- Right to deletion (GDPR compliant)
- No tracking or analytics by default
- Optional anonymous usage statistics

### Security Best Practices

- Password hashing with argon2
- Rate limiting on API endpoints
- CORS configuration
- SQL injection prevention (parameterized queries)
- XSS protection
- CSRF tokens for state-changing operations

## Deployment

### Self-Hosted Deployment

**Docker Compose (Recommended for single-server)**

```yaml
version: '3.8'
services:
  postgres:
    image: postgres:15-alpine
    volumes:
      - postgres_data:/var/lib/postgresql/data
    environment:
      POSTGRES_DB: altair
      POSTGRES_USER: altair
      POSTGRES_PASSWORD: ${DB_PASSWORD}
  
  redis:
    image: redis:7-alpine
    
  api:
    build: ./backend
    depends_on:
      - postgres
      - redis
    environment:
      DATABASE_URL: postgresql://altair:${DB_PASSWORD}@postgres/altair
      REDIS_URL: redis://redis:6379
      
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./frontend/build/web:/usr/share/nginx/html
    depends_on:
      - api
```

### Managed Service Deployment (Future)

**For non-technical users, optional managed deployment:**

- Free tier: Basic features, community support
- Paid tiers: Advanced features, priority support, higher limits
- Deployed on major cloud providers (AWS, GCP, Azure)

## Future Considerations

### Scalability Paths

1. **Horizontal Scaling**
   - Stateless API servers behind load balancer
   - PostgreSQL read replicas
   - Redis cluster for caching

2. **Microservices Migration**
   - Extract AI module as separate service
   - Separate real-time/WebSocket service
   - Dedicated analytics service

3. **Event-Driven Architecture**
   - Message queue (RabbitMQ, Kafka)
   - Event sourcing for audit trail
   - CQRS pattern for read/write separation

### Technology Additions

- **AI/ML Features**
  - Open-source LLM integration (local or self-hosted)
  - Task breakdown using GPT-4 or Claude API (optional)
  - Natural language task creation

- **Real-time Collaboration**
  - Operational transforms
  - WebRTC for focus sessions
  - Shared workspaces

- **Mobile Native Features**
  - Notifications
  - Widgets
  - Background sync
  - Biometric authentication

### Plugin System

Future plugin architecture for extensibility:

- Hooks for custom task types
- Custom fields and metadata
- Integration with external services
- Theme customization

---

## Technical Decisions FAQ

**Why FastAPI over Django/Flask?**

- Modern async support out of the box
- Automatic API documentation
- Type hints and data validation
- Better performance for API-heavy workloads
- Smaller learning curve than Django

**Why PostgreSQL over other databases?**

- Proven reliability and performance
- Rich feature set (JSONB, full-text search)
- Strong open-source ecosystem
- Better scaling story than SQLite
- More familiar to developers than newer databases

**Why Flutter over React/Vue?**

- True cross-platform (web + mobile) from single codebase
- Excellent performance and smooth animations (crucial for ADHD UX)
- Hot reload speeds up development
- Strong offline support
- Growing ecosystem

**What about SurrealDB or other newer databases?**

- Considered but prioritizing stability and ecosystem maturity
- Can revisit in future if compelling use cases emerge
- Current stack is proven and well-documented

---

**Last Updated:** October 2025  
**Version:** 0.1.0 (Pre-Alpha)
