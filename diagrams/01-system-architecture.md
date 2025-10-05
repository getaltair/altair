# Altair System Architecture Diagrams

## High-Level System Architecture

```mermaid
graph TB
    subgraph "Client Layer"
        WEB[Flutter Web App<br/>Browser]
        IOS[Flutter iOS App]
        AND[Flutter Android App]
    end

    subgraph "API Gateway"
        NGINX[Nginx<br/>Reverse Proxy<br/>SSL Termination]
    end

    subgraph "Application Layer"
        API[FastAPI Application<br/>Python 3.11+<br/>Async/Await]
    end

    subgraph "Service Modules"
        AUTH[Auth Service<br/>JWT Tokens]
        TASKS[Task Service<br/>CRUD + AI]
        PROJ[Project Service<br/>Organization]
        DOCS[Docs Service<br/>Notes + Markdown]
        TIME[Time Service<br/>Tracking]
        SYNC[Sync Service<br/>Conflict Resolution]
    end

    subgraph "Data Layer"
        PG[(PostgreSQL 15+<br/>Primary Database)]
        REDIS[(Redis<br/>Cache + Sessions<br/>Optional)]
        S3[Object Storage<br/>File Uploads<br/>Future]
    end

    WEB --> NGINX
    IOS --> NGINX
    AND --> NGINX
    NGINX --> API

    API --> AUTH
    API --> TASKS
    API --> PROJ
    API --> DOCS
    API --> TIME
    API --> SYNC

    AUTH --> PG
    TASKS --> PG
    PROJ --> PG
    DOCS --> PG
    TIME --> PG
    SYNC --> PG

    API -.-> REDIS
    TASKS -.-> S3

    style WEB fill:#3B82F6,stroke:#1E40AF,color:#fff
    style IOS fill:#3B82F6,stroke:#1E40AF,color:#fff
    style AND fill:#3B82F6,stroke:#1E40AF,color:#fff
    style API fill:#60A5FA,stroke:#3B82F6,color:#fff
    style PG fill:#14B8A6,stroke:#0D9488,color:#fff
```

## Detailed Component Architecture

```mermaid
graph LR
    subgraph "Frontend - Flutter"
        UI[UI Layer<br/>Widgets + Screens]
        PROV[State Management<br/>Riverpod]
        REPO[Repository<br/>Data Abstraction]
        LOCAL[(Local DB<br/>Drift/SQLite)]
        HTTP[HTTP Client<br/>API Calls]
    end

    subgraph "Backend - FastAPI"
        ROUTES[API Routes<br/>Endpoints]
        MIDDLE[Middleware<br/>Auth + CORS]
        SERVICES[Business Logic<br/>Services]
        MODELS[Data Models<br/>SQLAlchemy]
        VALID[Validation<br/>Pydantic]
    end

    UI --> PROV
    PROV --> REPO
    REPO --> LOCAL
    REPO --> HTTP
    HTTP --> ROUTES
    ROUTES --> MIDDLE
    MIDDLE --> SERVICES
    SERVICES --> MODELS
    SERVICES --> VALID
    MODELS --> PG[(PostgreSQL)]

    style UI fill:#3B82F6,stroke:#1E40AF,color:#fff
    style PROV fill:#60A5FA,stroke:#3B82F6,color:#fff
    style ROUTES fill:#60A5FA,stroke:#3B82F6,color:#fff
    style PG fill:#14B8A6,stroke:#0D9488,color:#fff
```

## Network Flow & Communication

```mermaid
sequenceDiagram
    participant User
    participant Flutter
    participant LocalDB
    participant Nginx
    participant FastAPI
    participant PostgreSQL

    User->>Flutter: Create Task
    Flutter->>LocalDB: Save Locally (Optimistic)
    Flutter-->>User: Show Success (Instant)

    Flutter->>Nginx: POST /api/v1/tasks
    Nginx->>FastAPI: Forward Request
    FastAPI->>FastAPI: Validate JWT
    FastAPI->>PostgreSQL: INSERT Task
    PostgreSQL-->>FastAPI: Confirm
    FastAPI-->>Nginx: 201 Created
    Nginx-->>Flutter: Response
    Flutter->>LocalDB: Update with Server ID

    Note over Flutter,PostgreSQL: Offline-First Pattern
```

## Deployment Architecture

```mermaid
graph TB
    subgraph "Docker Compose Stack"
        subgraph "Web Tier"
            NGINX_C[Nginx Container<br/>Port 80/443]
        end

        subgraph "App Tier"
            API_C1[FastAPI Container 1<br/>Port 8000]
            API_C2[FastAPI Container 2<br/>Port 8000<br/>Load Balanced]
        end

        subgraph "Data Tier"
            PG_C[(PostgreSQL Container<br/>Port 5432)]
            REDIS_C[(Redis Container<br/>Port 6379)]
        end

        subgraph "Static Files"
            STATIC[Flutter Web Build<br/>Served by Nginx]
        end
    end

    NGINX_C --> STATIC
    NGINX_C --> API_C1
    NGINX_C --> API_C2
    API_C1 --> PG_C
    API_C2 --> PG_C
    API_C1 -.-> REDIS_C
    API_C2 -.-> REDIS_C

    style NGINX_C fill:#FB923C,stroke:#EA580C,color:#fff
    style API_C1 fill:#60A5FA,stroke:#3B82F6,color:#fff
    style API_C2 fill:#60A5FA,stroke:#3B82F6,color:#fff
    style PG_C fill:#14B8A6,stroke:#0D9488,color:#fff
```

## Authentication Flow

```mermaid
sequenceDiagram
    participant User
    participant Frontend
    participant API
    participant DB

    User->>Frontend: Enter Credentials
    Frontend->>API: POST /auth/login
    API->>DB: Verify User
    DB-->>API: User Data
    API->>API: Generate JWT
    API-->>Frontend: Access Token + Refresh Token
    Frontend->>Frontend: Store Tokens (Secure)

    Note over User,DB: Subsequent Requests

    Frontend->>API: GET /tasks (with JWT)
    API->>API: Verify Token
    API->>DB: Fetch Tasks
    DB-->>API: Task Data
    API-->>Frontend: Tasks JSON
    Frontend-->>User: Display Tasks

    Note over User,DB: Token Refresh

    Frontend->>API: POST /auth/refresh (Refresh Token)
    API->>API: Verify Refresh Token
    API-->>Frontend: New Access Token
```

## Technology Stack Visual

```mermaid
mindmap
  root((Altair<br/>Tech Stack))
    Frontend
      Flutter 3.16+
        Web
        iOS
        Android
      Riverpod
        State Management
        Dependency Injection
      Drift
        Local SQLite
        Offline Storage
    Backend
      FastAPI
        Python 3.11+
        Async/Await
        Auto Docs
      SQLAlchemy 2.0
        ORM
        Migrations
      Pydantic v2
        Validation
        Serialization
    Database
      PostgreSQL 15+
        JSONB Support
        Full Text Search
      Redis Optional
        Caching
        Rate Limiting
    Infrastructure
      Docker
        Containers
        Compose
      Nginx
        Reverse Proxy
        SSL/TLS
    Future
      AI/ML
        Ollama
        LocalAI
      Object Storage
        S3 Compatible
```

## Data Flow: Task Creation with AI Breakdown

```mermaid
flowchart TD
    START([User Creates Task]) --> INPUT[Enter Task Title]
    INPUT --> SAVE_LOCAL[Save to Local DB]
    SAVE_LOCAL --> SHOW[Display in UI]
    SAVE_LOCAL --> API_CALL{Online?}

    API_CALL -->|Yes| SEND[POST to API]
    API_CALL -->|No| QUEUE[Add to Sync Queue]

    SEND --> VALIDATE[Validate Data]
    VALIDATE --> DB_INSERT[Insert to PostgreSQL]
    DB_INSERT --> CHECK_COMPLEX{Task Complex?}

    CHECK_COMPLEX -->|Yes| AI_CALL[Call AI Service]
    CHECK_COMPLEX -->|No| RETURN[Return Task]

    AI_CALL --> LLM[Local LLM Analysis]
    LLM --> SUBTASKS[Generate Subtasks]
    SUBTASKS --> SAVE_SUBS[Save Subtasks]
    SAVE_SUBS --> RETURN

    RETURN --> RESPONSE[Return to Frontend]
    RESPONSE --> UPDATE_LOCAL[Update Local DB]
    UPDATE_LOCAL --> REFRESH[Refresh UI]

    QUEUE --> LATER{Connection<br/>Restored?}
    LATER -->|Yes| SYNC[Sync Queue]
    SYNC --> SEND

    style START fill:#60A5FA,stroke:#3B82F6,color:#fff
    style AI_CALL fill:#FB923C,stroke:#EA580C,color:#fff
    style SAVE_LOCAL fill:#14B8A6,stroke:#0D9488,color:#fff
    style DB_INSERT fill:#14B8A6,stroke:#0D9488,color:#fff
```

## Offline Sync Strategy

```mermaid
stateDiagram-v2
    [*] --> Online
    Online --> Offline: Connection Lost
    Offline --> SyncQueue: User Actions
    SyncQueue --> SyncQueue: Queue Operations
    Offline --> Online: Connection Restored
    SyncQueue --> Syncing: Trigger Sync
    Syncing --> ConflictCheck: Check for Conflicts

    ConflictCheck --> NoConflict: No Conflicts
    ConflictCheck --> Conflict: Conflicts Found

    NoConflict --> ApplyChanges: Apply to Server
    ApplyChanges --> UpdateLocal: Update Local IDs
    UpdateLocal --> Online

    Conflict --> ResolveStrategy: Resolution Strategy
    ResolveStrategy --> LastWriteWins: Simple Fields
    ResolveStrategy --> UserChoice: Complex Fields
    LastWriteWins --> ApplyChanges
    UserChoice --> UserPrompt: Show Conflict UI
    UserPrompt --> ApplyChanges

    Online --> [*]
```

## Module Dependencies

```mermaid
graph TD
    MAIN[main.py<br/>FastAPI App] --> AUTH_M[auth_module]
    MAIN --> TASKS_M[tasks_module]
    MAIN --> PROJ_M[projects_module]
    MAIN --> DOCS_M[docs_module]
    MAIN --> TIME_M[time_module]

    TASKS_M --> AI_S[ai_service]
    TASKS_M --> NOTIF_S[notification_service]
    PROJ_M --> TASKS_M
    DOCS_M --> TASKS_M
    TIME_M --> TASKS_M

    AUTH_M --> DB[database_core]
    TASKS_M --> DB
    PROJ_M --> DB
    DOCS_M --> DB
    TIME_M --> DB

    AUTH_M --> CACHE[cache_service]
    TASKS_M --> CACHE

    AI_S --> LLM[llm_client<br/>Ollama/LocalAI]

    style MAIN fill:#60A5FA,stroke:#3B82F6,color:#fff
    style DB fill:#14B8A6,stroke:#0D9488,color:#fff
    style AI_S fill:#FB923C,stroke:#EA580C,color:#fff
```

## Security Layers

```mermaid
graph TD
    REQUEST[Incoming Request] --> HTTPS{HTTPS?}
    HTTPS -->|No| REJECT[Reject: Require SSL]
    HTTPS -->|Yes| CORS[CORS Check]
    CORS --> RATE[Rate Limiting]
    RATE --> AUTH{Has JWT?}

    AUTH -->|No| PUBLIC{Public Endpoint?}
    PUBLIC -->|No| REJECT2[401 Unauthorized]
    PUBLIC -->|Yes| PROCESS[Process Request]

    AUTH -->|Yes| VALIDATE[Validate JWT]
    VALIDATE --> EXPIRED{Token Valid?}
    EXPIRED -->|No| REJECT3[401 Invalid Token]
    EXPIRED -->|Yes| PERMISSIONS[Check Permissions]

    PERMISSIONS --> AUTHORIZED{Authorized?}
    AUTHORIZED -->|No| REJECT4[403 Forbidden]
    AUTHORIZED -->|Yes| SANITIZE[Sanitize Input]

    SANITIZE --> VALIDATE_DATA[Validate with Pydantic]
    VALIDATE_DATA --> PROCESS
    PROCESS --> AUDIT[Audit Log]
    AUDIT --> RESPONSE[Return Response]

    style HTTPS fill:#14B8A6,stroke:#0D9488,color:#fff
    style AUTH fill:#60A5FA,stroke:#3B82F6,color:#fff
    style PROCESS fill:#60A5FA,stroke:#3B82F6,color:#fff
```

---

**Usage Notes:**
- These diagrams are in Mermaid format
- Can be rendered in GitHub README files
- Compatible with many documentation tools
- Can be exported to SVG/PNG using Mermaid CLI or online editors
- Update as architecture evolves
