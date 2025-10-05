# Altair Component Architecture & Technical Flows

## Frontend Component Hierarchy

```mermaid
graph TD
    APP[App Root] --> PROVIDERS[Riverpod Providers]
    APP --> ROUTER[Router]

    PROVIDERS --> AUTH_P[Auth Provider]
    PROVIDERS --> TASK_P[Task Provider]
    PROVIDERS --> PROJ_P[Project Provider]
    PROVIDERS --> SYNC_P[Sync Provider]

    ROUTER --> SHELL[App Shell]

    SHELL --> NAV[Navigation Bar]
    SHELL --> MAIN[Main Content Area]
    SHELL --> SIDEBAR[Sidebar]

    MAIN --> ROUTES{Routes}

    ROUTES --> HOME[Home Screen]
    ROUTES --> PROJECTS[Projects Screen]
    ROUTES --> TASKS[Tasks Screen]
    ROUTES --> FOCUS[Focus Mode Screen]
    ROUTES --> ANALYTICS[Analytics Screen]

    TASKS --> TASK_LIST[Task List Component]
    TASK_LIST --> TASK_ITEM[Task Item Widget]
    TASK_ITEM --> TASK_ACTIONS[Action Buttons]

    TASKS --> QUICK_ADD[Quick Add Widget]
    QUICK_ADD --> MODAL[Quick Capture Modal]

    TASKS --> FILTER_BAR[Filter Bar]
    FILTER_BAR --> FILTER_CHIP[Filter Chips]

    style APP fill:#60A5FA,stroke:#3B82F6,color:#fff
    style PROVIDERS fill:#14B8A6,stroke:#0D9488,color:#fff
    style TASKS fill:#FB923C,stroke:#EA580C,color:#fff
```

## Flutter State Management Flow (Riverpod)

```mermaid
sequenceDiagram
    participant UI
    participant Provider
    participant Repository
    participant LocalDB
    participant API

    UI->>Provider: Watch TaskProvider
    Provider->>Provider: Initial State
    Provider->>Repository: getTasks()

    Repository->>LocalDB: Query Local
    LocalDB-->>Repository: Local Tasks
    Repository-->>Provider: Update State
    Provider-->>UI: Render Tasks

    Note over Repository,API: Background Sync

    Repository->>API: GET /tasks
    API-->>Repository: Server Tasks
    Repository->>LocalDB: Merge & Update
    Repository-->>Provider: New State
    Provider-->>UI: Re-render (if changed)

    Note over UI,API: User Creates Task

    UI->>Provider: createTask()
    Provider->>Repository: create()
    Repository->>LocalDB: Insert Optimistically
    Repository-->>Provider: Update State
    Provider-->>UI: Show New Task

    Repository->>API: POST /tasks
    API-->>Repository: Server Response
    Repository->>LocalDB: Update with Server ID
    Repository-->>Provider: Final State
    Provider-->>UI: Final Render
```

## Backend Request Lifecycle

```mermaid
flowchart TD
    REQUEST[HTTP Request] --> NGINX[Nginx<br/>Reverse Proxy]
    NGINX --> FASTAPI[FastAPI App]

    FASTAPI --> MIDDLEWARE{Middleware Chain}

    MIDDLEWARE --> CORS[CORS Handler]
    CORS --> LOGGER[Request Logger]
    LOGGER --> RATE[Rate Limiter]
    RATE --> CONTINUE

    CONTINUE{Continue?} -->|Yes| ROUTE[Route Handler]
    CONTINUE -->|No| ERROR_RESP[Error Response]

    ROUTE --> AUTH_CHECK{Needs Auth?}

    AUTH_CHECK -->|Yes| JWT[Verify JWT]
    AUTH_CHECK -->|No| VALIDATE

    JWT --> VALID{Valid?}
    VALID -->|No| ERROR_RESP
    VALID -->|Yes| LOAD_USER[Load User Context]

    LOAD_USER --> VALIDATE[Validate Request]
    VALIDATE --> PYDANTIC[Pydantic Model]

    PYDANTIC --> VALID_DATA{Valid?}
    VALID_DATA -->|No| ERROR_RESP
    VALID_DATA -->|Yes| SERVICE[Service Layer]

    SERVICE --> BUSINESS[Business Logic]
    BUSINESS --> DB_QUERY[Database Query]
    DB_QUERY --> POSTGRES[(PostgreSQL)]
    POSTGRES --> RESULTS[Query Results]

    RESULTS --> SERIALIZE[Serialize Response]
    SERIALIZE --> CACHE{Cache?}
    CACHE -->|Yes| REDIS[(Redis)]
    CACHE -->|No| RESPONSE

    REDIS --> RESPONSE[JSON Response]
    ERROR_RESP --> RESPONSE

    RESPONSE --> NGINX
    NGINX --> CLIENT[Client]

    style FASTAPI fill:#60A5FA,stroke:#3B82F6,color:#fff
    style SERVICE fill:#14B8A6,stroke:#0D9488,color:#fff
    style POSTGRES fill:#14B8A6,stroke:#0D9488,color:#fff
```

## Offline-First Data Sync Architecture

```mermaid
graph TB
    subgraph "Frontend"
        UI[UI Layer]
        STATE[State Management]
        LOCAL[(Drift SQLite)]
        QUEUE[Sync Queue]
    end

    subgraph "Network Boundary"
        DETECTOR[Connection Detector]
        SYNC_ENGINE[Sync Engine]
    end

    subgraph "Backend"
        API[REST API]
        DB[(PostgreSQL)]
    end

    UI --> STATE
    STATE --> LOCAL
    STATE --> QUEUE

    LOCAL --> DETECTOR
    QUEUE --> SYNC_ENGINE
    DETECTOR --> SYNC_ENGINE

    SYNC_ENGINE -->|Online| API
    API --> DB

    DB -.Response.-> API
    API -.Response.-> SYNC_ENGINE
    SYNC_ENGINE -.Update.-> LOCAL
    SYNC_ENGINE -.Clear.-> QUEUE
    LOCAL -.Notify.-> STATE
    STATE -.Re-render.-> UI

    style LOCAL fill:#14B8A6,stroke:#0D9488,color:#fff
    style SYNC_ENGINE fill:#FB923C,stroke:#EA580C,color:#fff
    style DB fill:#14B8A6,stroke:#0D9488,color:#fff
```

## Conflict Resolution Strategy

```mermaid
flowchart TD
    SYNC_START[Sync Process Starts] --> GET_LOCAL[Get Local Changes]
    GET_LOCAL --> GET_SERVER[Get Server Changes]
    GET_SERVER --> COMPARE{Compare Timestamps}

    COMPARE -->|No Conflict| APPLY_CHANGES[Apply Changes]
    COMPARE -->|Conflict| DETECT_TYPE{Conflict Type}

    DETECT_TYPE -->|Simple Field| LAST_WRITE[Last Write Wins]
    DETECT_TYPE -->|Complex Change| STRATEGY{Resolution Strategy}

    STRATEGY -->|Auto-merge| MERGE[Merge Changes]
    STRATEGY -->|User Choice| PROMPT[Prompt User]

    LAST_WRITE --> SERVER_TIME{Server Newer?}
    SERVER_TIME -->|Yes| USE_SERVER[Use Server Version]
    SERVER_TIME -->|No| USE_LOCAL[Use Local Version]

    MERGE --> AUTO_RESOLVE[Auto-resolve]
    PROMPT --> USER_DECISION[User Selects Version]

    USE_SERVER --> APPLY_CHANGES
    USE_LOCAL --> APPLY_CHANGES
    AUTO_RESOLVE --> APPLY_CHANGES
    USER_DECISION --> APPLY_CHANGES

    APPLY_CHANGES --> UPDATE_LOCAL[Update Local DB]
    UPDATE_LOCAL --> UPDATE_SERVER[Update Server]
    UPDATE_SERVER --> LOG[Log Resolution]
    LOG --> COMPLETE([Sync Complete])

    style DETECT_TYPE fill:#FB923C,stroke:#EA580C,color:#fff
    style PROMPT fill:#60A5FA,stroke:#3B82F6,color:#fff
    style COMPLETE fill:#14B8A6,stroke:#0D9488,color:#fff
```

## AI Task Breakdown Architecture

```mermaid
graph TD
    TASK[User Task Input] --> ANALYZER[Task Analyzer]

    ANALYZER --> COMPLEXITY{Complexity Check}
    COMPLEXITY -->|Low| SKIP[Skip Breakdown]
    COMPLEXITY -->|High| PREPARE[Prepare Prompt]

    PREPARE --> CONTEXT[Add Context]
    CONTEXT --> USER_PREF[User Preferences]
    CONTEXT --> TASK_META[Task Metadata]

    USER_PREF --> PROMPT[Generate Prompt]
    TASK_META --> PROMPT

    PROMPT --> LLM_CLIENT[LLM Client]

    LLM_CLIENT --> CHECK{LLM Available?}
    CHECK -->|Local| OLLAMA[Ollama API]
    CHECK -->|Cloud| OPENAI[OpenAI API]
    CHECK -->|None| FALLBACK[Manual Breakdown]

    OLLAMA --> INFERENCE[LLM Inference]
    OPENAI --> INFERENCE

    INFERENCE --> PARSE[Parse Response]
    PARSE --> VALIDATE[Validate Subtasks]
    VALIDATE --> ESTIMATE[Estimate Durations]
    ESTIMATE --> PRIORITIZE[Suggest Priority]
    PRIORITIZE --> FORMAT[Format Response]

    FORMAT --> PREVIEW[Show Preview to User]
    FALLBACK --> PREVIEW

    PREVIEW --> USER_REVIEW{User Accepts?}
    USER_REVIEW -->|Yes| CREATE[Create Subtasks]
    USER_REVIEW -->|Edit| MODIFY[User Modifies]
    USER_REVIEW -->|No| MANUAL[Manual Entry]

    MODIFY --> CREATE
    CREATE --> SAVE[Save to Database]
    MANUAL --> SAVE

    SAVE --> DONE([Breakdown Complete])

    style ANALYZER fill:#FB923C,stroke:#EA580C,color:#fff
    style INFERENCE fill:#FB923C,stroke:#EA580C,color:#fff
    style PREVIEW fill:#60A5FA,stroke:#3B82F6,color:#fff
    style DONE fill:#14B8A6,stroke:#0D9488,color:#fff
```

## Authentication & Security Flow

```mermaid
sequenceDiagram
    participant Client
    participant API
    participant Auth
    participant DB
    participant JWT

    Note over Client,JWT: Registration
    Client->>API: POST /auth/register
    API->>Auth: Validate Email/Password
    Auth->>Auth: Hash Password (bcrypt)
    Auth->>DB: Insert User
    DB-->>Auth: User Created
    Auth->>JWT: Generate Tokens
    JWT-->>Auth: Access + Refresh Tokens
    Auth-->>API: Return Tokens
    API-->>Client: 201 + Tokens

    Note over Client,JWT: Login
    Client->>API: POST /auth/login
    API->>Auth: Verify Credentials
    Auth->>DB: Get User by Email
    DB-->>Auth: User Data
    Auth->>Auth: Verify Password Hash
    Auth->>JWT: Generate Tokens
    JWT-->>Auth: Access + Refresh Tokens
    Auth-->>API: Return Tokens
    API-->>Client: 200 + Tokens

    Note over Client,JWT: Authenticated Request
    Client->>API: GET /tasks (+ Access Token)
    API->>Auth: Verify Token
    Auth->>JWT: Decode & Validate
    JWT-->>Auth: Valid User ID
    Auth->>DB: Get User Context
    DB-->>Auth: User Data
    Auth-->>API: Authorized
    API->>DB: Query Tasks
    DB-->>API: Tasks Data
    API-->>Client: 200 + Tasks

    Note over Client,JWT: Token Refresh
    Client->>API: POST /auth/refresh (+ Refresh Token)
    API->>Auth: Verify Refresh Token
    Auth->>JWT: Validate Token
    JWT-->>Auth: Valid
    Auth->>JWT: Generate New Access Token
    JWT-->>Auth: New Access Token
    Auth-->>API: New Token
    API-->>Client: 200 + New Access Token
```

## Docker Compose Service Dependencies

```mermaid
graph TD
    POSTGRES[PostgreSQL Container] --> DEPENDS_PG{Services Depending on PG}
    REDIS[Redis Container] --> DEPENDS_REDIS{Services Depending on Redis}

    DEPENDS_PG --> API[FastAPI Container]
    DEPENDS_REDIS -.Optional.-> API

    API --> NGINX[Nginx Container]
    STATIC[Static Files Volume] --> NGINX

    NGINX --> EXPOSE[Exposed Ports]
    EXPOSE --> PORT_80[Port 80 HTTP]
    EXPOSE --> PORT_443[Port 443 HTTPS]

    subgraph "Data Persistence"
        PG_VOL[postgres_data Volume]
        REDIS_VOL[redis_data Volume]
    end

    POSTGRES --> PG_VOL
    REDIS --> REDIS_VOL

    subgraph "Networks"
        BACKEND_NET[backend Network]
        FRONTEND_NET[frontend Network]
    end

    POSTGRES -.-> BACKEND_NET
    REDIS -.-> BACKEND_NET
    API -.-> BACKEND_NET
    API -.-> FRONTEND_NET
    NGINX -.-> FRONTEND_NET

    style POSTGRES fill:#14B8A6,stroke:#0D9488,color:#fff
    style API fill:#60A5FA,stroke:#3B82F6,color:#fff
    style NGINX fill:#FB923C,stroke:#EA580C,color:#fff
```

## Testing Strategy Pyramid

```mermaid
graph TD
    subgraph "Testing Pyramid"
        E2E[End-to-End Tests<br/>Selenium/Playwright<br/>~10 tests]
        INT[Integration Tests<br/>API + DB Tests<br/>~50 tests]
        UNIT[Unit Tests<br/>Function/Component Tests<br/>~200+ tests]
    end

    UNIT --> INT
    INT --> E2E

    subgraph "Backend Tests"
        B_UNIT[pytest Unit Tests]
        B_INT[FastAPI Test Client]
        B_E2E[API Integration Tests]
    end

    subgraph "Frontend Tests"
        F_UNIT[Widget Tests]
        F_INT[Integration Tests]
        F_E2E[Flutter Driver Tests]
    end

    B_UNIT --> UNIT
    F_UNIT --> UNIT
    B_INT --> INT
    F_INT --> INT
    B_E2E --> E2E
    F_E2E --> E2E

    style E2E fill:#FB923C,stroke:#EA580C,color:#fff
    style INT fill:#60A5FA,stroke:#3B82F6,color:#fff
    style UNIT fill:#14B8A6,stroke:#0D9488,color:#fff
```

## CI/CD Pipeline

```mermaid
flowchart LR
    COMMIT[Git Push] --> TRIGGER[GitHub Actions Trigger]

    TRIGGER --> PARALLEL{Run in Parallel}

    PARALLEL --> BACKEND[Backend Pipeline]
    PARALLEL --> FRONTEND[Frontend Pipeline]

    subgraph "Backend Jobs"
        B1[Install Dependencies]
        B2[Lint Code]
        B3[Run Tests]
        B4[Build Docker Image]

        B1 --> B2 --> B3 --> B4
    end

    subgraph "Frontend Jobs"
        F1[Install Dependencies]
        F2[Lint Code]
        F3[Run Tests]
        F4[Build Web App]

        F1 --> F2 --> F3 --> F4
    end

    BACKEND --> B1
    FRONTEND --> F1

    B4 --> MERGE{All Pass?}
    F4 --> MERGE

    MERGE -->|Yes| DEPLOY{Branch?}
    MERGE -->|No| FAIL[Pipeline Failed]

    DEPLOY -->|main| PROD[Deploy to Production]
    DEPLOY -->|develop| STAGING[Deploy to Staging]
    DEPLOY -->|other| SKIP[Skip Deploy]

    PROD --> NOTIFY[Notify Team]
    STAGING --> NOTIFY

    style MERGE fill:#FB923C,stroke:#EA580C,color:#fff
    style PROD fill:#14B8A6,stroke:#0D9488,color:#fff
    style FAIL fill:#EF4444,stroke:#DC2626,color:#fff
```

## Database Connection Pooling

```mermaid
graph LR
    subgraph "FastAPI Instances"
        APP1[FastAPI Instance 1]
        APP2[FastAPI Instance 2]
        APP3[FastAPI Instance N]
    end

    subgraph "Connection Pool"
        POOL[SQLAlchemy Pool<br/>Min: 5<br/>Max: 20<br/>Overflow: 10]
    end

    subgraph "PostgreSQL"
        PG[(PostgreSQL<br/>max_connections: 100)]
    end

    APP1 -.Request.-> POOL
    APP2 -.Request.-> POOL
    APP3 -.Request.-> POOL

    POOL -.Connection.-> PG

    style POOL fill:#60A5FA,stroke:#3B82F6,color:#fff
    style PG fill:#14B8A6,stroke:#0D9488,color:#fff
```

## Error Handling Flow

```mermaid
flowchart TD
    ERROR[Error Occurs] --> CATCH[Error Handler Catches]

    CATCH --> TYPE{Error Type}

    TYPE -->|Validation| VAL_HANDLER[ValidationError Handler]
    TYPE -->|Auth| AUTH_HANDLER[AuthError Handler]
    TYPE -->|Database| DB_HANDLER[DatabaseError Handler]
    TYPE -->|Unknown| GENERIC[Generic Handler]

    VAL_HANDLER --> LOG1[Log Validation Failure]
    AUTH_HANDLER --> LOG2[Log Auth Attempt]
    DB_HANDLER --> LOG3[Log DB Error]
    GENERIC --> LOG4[Log Unknown Error]

    LOG1 --> FORMAT1[Format Response]
    LOG2 --> FORMAT2[Format Response]
    LOG3 --> FORMAT3[Format Response]
    LOG4 --> FORMAT4[Format Response]

    FORMAT1 --> STATUS1[400 Bad Request]
    FORMAT2 --> STATUS2[401 Unauthorized]
    FORMAT3 --> STATUS3[500 Server Error]
    FORMAT4 --> STATUS4[500 Server Error]

    STATUS1 --> CLIENT_MSG[User-Friendly Message]
    STATUS2 --> CLIENT_MSG
    STATUS3 --> ALERT{Critical?}
    STATUS4 --> ALERT

    ALERT -->|Yes| NOTIFY_TEAM[Notify Dev Team]
    ALERT -->|No| CLIENT_MSG

    NOTIFY_TEAM --> CLIENT_MSG
    CLIENT_MSG --> RESPONSE[Return JSON Response]

    RESPONSE --> CLIENT[Send to Client]

    style ERROR fill:#EF4444,stroke:#DC2626,color:#fff
    style CLIENT_MSG fill:#60A5FA,stroke:#3B82F6,color:#fff
    style NOTIFY_TEAM fill:#FB923C,stroke:#EA580C,color:#fff
```

---

**Component Design Principles:**

1. **Separation of Concerns** - Clear boundaries between layers
2. **Dependency Injection** - Testable, mockable components
3. **Error Boundaries** - Graceful degradation
4. **State Management** - Unidirectional data flow
5. **Type Safety** - TypeScript/Dart + Pydantic validation
6. **Async First** - Non-blocking operations
7. **Observability** - Logging, metrics, tracing
8. **Resilience** - Retry logic, circuit breakers, fallbacks
