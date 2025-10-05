# Altair Database Schema & ERD

## Entity Relationship Diagram

```mermaid
erDiagram
    USER ||--o{ WORKSPACE_MEMBER : "belongs to"
    USER ||--o{ TASK : "assigned to"
    USER ||--o{ TIME_ENTRY : "tracks"
    USER ||--o{ NOTE : "creates"
    USER {
        uuid id PK
        string email UK
        string username UK
        string password_hash
        jsonb settings
        timestamp created_at
        timestamp last_login
        boolean email_verified
        string timezone
    }
    
    WORKSPACE ||--|{ WORKSPACE_MEMBER : "has"
    WORKSPACE ||--|{ PROJECT : "contains"
    WORKSPACE {
        uuid id PK
        uuid owner_id FK
        string name
        text description
        jsonb settings
        timestamp created_at
        timestamp updated_at
    }
    
    WORKSPACE_MEMBER {
        uuid id PK
        uuid workspace_id FK
        uuid user_id FK
        enum role
        jsonb permissions
        timestamp joined_at
    }
    
    PROJECT ||--|{ TASK : "contains"
    PROJECT ||--o{ NOTE : "has"
    PROJECT {
        uuid id PK
        uuid workspace_id FK
        string name
        text description
        string color
        enum status
        jsonb metadata
        timestamp created_at
        timestamp archived_at
        integer sort_order
    }
    
    TASK ||--o{ TASK : "parent_child"
    TASK ||--o{ TIME_ENTRY : "tracked"
    TASK ||--o{ NOTE : "annotates"
    TASK ||--o{ TASK_DEPENDENCY : "depends_on"
    TASK {
        uuid id PK
        uuid project_id FK
        uuid parent_id FK
        uuid assigned_to FK
        string title
        text description
        enum status
        enum priority
        integer estimated_duration
        integer actual_duration
        date due_date
        timestamp start_date
        timestamp completed_at
        timestamp created_at
        timestamp updated_at
        jsonb metadata
        integer energy_level
        string focus_type
        jsonb ai_breakdown
        integer sort_order
    }
    
    TASK_DEPENDENCY {
        uuid id PK
        uuid task_id FK
        uuid depends_on_task_id FK
        enum dependency_type
        timestamp created_at
    }
    
    NOTE {
        uuid id PK
        uuid task_id FK
        uuid project_id FK
        uuid user_id FK
        text content
        string format
        jsonb metadata
        timestamp created_at
        timestamp updated_at
        boolean pinned
    }
    
    TIME_ENTRY {
        uuid id PK
        uuid task_id FK
        uuid user_id FK
        timestamp start_time
        timestamp end_time
        integer duration_seconds
        text notes
        jsonb metadata
        timestamp created_at
    }
    
    TAG ||--o{ TASK_TAG : "categorizes"
    TAG {
        uuid id PK
        uuid workspace_id FK
        string name
        string color
        timestamp created_at
    }
    
    TASK ||--o{ TASK_TAG : "tagged_with"
    TASK_TAG {
        uuid id PK
        uuid task_id FK
        uuid tag_id FK
        timestamp created_at
    }
    
    SYNC_QUEUE {
        uuid id PK
        uuid user_id FK
        enum operation
        string entity_type
        uuid entity_id
        jsonb payload
        timestamp created_at
        timestamp synced_at
        integer retry_count
        text error_message
    }
    
    AUDIT_LOG {
        uuid id PK
        uuid user_id FK
        string entity_type
        uuid entity_id
        enum action
        jsonb changes
        string ip_address
        string user_agent
        timestamp created_at
    }
```

## Database Schema with Details

```mermaid
classDiagram
    class User {
        +UUID id
        +String email
        +String username
        +String password_hash
        +JSONB settings
        +DateTime created_at
        +DateTime last_login
        +Boolean email_verified
        +String timezone
        +get_workspaces()
        +get_assigned_tasks()
    }
    
    class Workspace {
        +UUID id
        +UUID owner_id
        +String name
        +Text description
        +JSONB settings
        +DateTime created_at
        +DateTime updated_at
        +add_member()
        +remove_member()
        +get_projects()
    }
    
    class Project {
        +UUID id
        +UUID workspace_id
        +String name
        +Text description
        +String color
        +Enum status
        +JSONB metadata
        +DateTime created_at
        +DateTime archived_at
        +Integer sort_order
        +get_tasks()
        +get_progress()
        +archive()
    }
    
    class Task {
        +UUID id
        +UUID project_id
        +UUID parent_id
        +UUID assigned_to
        +String title
        +Text description
        +Enum status
        +Enum priority
        +Integer estimated_duration
        +Integer actual_duration
        +Date due_date
        +DateTime completed_at
        +JSONB metadata
        +Integer energy_level
        +String focus_type
        +JSONB ai_breakdown
        +mark_complete()
        +breakdown_with_ai()
        +add_subtask()
        +start_timer()
    }
    
    class TimeEntry {
        +UUID id
        +UUID task_id
        +UUID user_id
        +DateTime start_time
        +DateTime end_time
        +Integer duration_seconds
        +Text notes
        +stop_timer()
        +calculate_duration()
    }
    
    class Note {
        +UUID id
        +UUID task_id
        +UUID project_id
        +UUID user_id
        +Text content
        +String format
        +DateTime created_at
        +to_markdown()
        +to_html()
    }
    
    User "1" --> "*" Workspace : owns
    User "1" --> "*" Task : assigned
    Workspace "1" --> "*" Project : contains
    Project "1" --> "*" Task : has
    Task "1" --> "*" Task : subtasks
    Task "1" --> "*" TimeEntry : tracked
    Task "1" --> "*" Note : annotated
```

## Table Indexes

```mermaid
graph TD
    subgraph "User Table Indexes"
        U1[PRIMARY: id]
        U2[UNIQUE: email]
        U3[UNIQUE: username]
        U4[INDEX: created_at]
    end
    
    subgraph "Task Table Indexes"
        T1[PRIMARY: id]
        T2[INDEX: project_id]
        T3[INDEX: assigned_to]
        T4[INDEX: status]
        T5[INDEX: due_date]
        T6[INDEX: parent_id]
        T7[COMPOSITE: project_id, status]
        T8[COMPOSITE: assigned_to, status]
        T9[GIN: metadata jsonb_path_ops]
    end
    
    subgraph "Project Table Indexes"
        P1[PRIMARY: id]
        P2[INDEX: workspace_id]
        P3[INDEX: status]
        P4[COMPOSITE: workspace_id, archived_at]
    end
    
    subgraph "TimeEntry Table Indexes"
        TE1[PRIMARY: id]
        TE2[INDEX: task_id]
        TE3[INDEX: user_id]
        TE4[INDEX: start_time]
        TE5[COMPOSITE: user_id, start_time]
    end
    
    style T1 fill:#14B8A6,stroke:#0D9488,color:#fff
    style T7 fill:#FB923C,stroke:#EA580C,color:#fff
    style T9 fill:#FB923C,stroke:#EA580C,color:#fff
```

## ADHD-Specific Fields Detail

```mermaid
mindmap
  root((Task Metadata))
    Energy Level
      1 - Minimal
      2 - Low
      3 - Medium
      4 - High
      5 - Peak
    Focus Type
      Quick Win
        5-15 minutes
        Low cognitive load
      Deep Work
        1+ hours
        High focus required
      Creative
        Variable duration
        Flow state
      Administrative
        Process-heavy
        Can be batched
    AI Breakdown
      breakdown_suggested: bool
      subtasks_generated: array
      complexity_score: int
      estimated_chunks: int
    Time Blindness Helpers
      visual_duration: string
      time_budget_color: string
      countdown_enabled: bool
    Dopamine Hooks
      completion_animation: string
      badge_earned: array
      streak_count: int
```

## Sample Data Relationships

```mermaid
graph TD
    subgraph "Example: User's Project"
        U[User: Alice<br/>alice@example.com]
        W[Workspace: Personal]
        P1[Project: Altair Dev<br/>Status: Active]
        P2[Project: Blog Posts<br/>Status: Active]
    end
    
    subgraph "Altair Dev Tasks"
        T1[Task: Setup Backend<br/>Status: Complete]
        T1_1[Subtask: Install FastAPI<br/>Complete]
        T1_2[Subtask: Setup Database<br/>Complete]
        T2[Task: Build Frontend<br/>Status: In Progress]
        T2_1[Subtask: Create UI Components<br/>In Progress]
    end
    
    subgraph "Time Tracking"
        TIME1[TimeEntry: 2h 30m<br/>Setup Backend]
        TIME2[TimeEntry: 1h 15m<br/>Install FastAPI]
        TIME3[TimeEntry: 45m<br/>Create UI Components]
    end
    
    U --> W
    W --> P1
    W --> P2
    P1 --> T1
    P1 --> T2
    T1 --> T1_1
    T1 --> T1_2
    T2 --> T2_1
    
    T1 --> TIME1
    T1_1 --> TIME2
    T2_1 --> TIME3
    
    U -.assigned.-> T2
    U -.assigned.-> T2_1
    
    style U fill:#60A5FA,stroke:#3B82F6,color:#fff
    style P1 fill:#14B8A6,stroke:#0D9488,color:#fff
    style T1 fill:#6EE7B7,stroke:#14B8A6,color:#000
    style T2 fill:#FCD34D,stroke:#F59E0B,color:#000
```

## Query Patterns

```mermaid
sequenceDiagram
    participant App
    participant DB
    
    Note over App,DB: Get User's Active Tasks
    App->>DB: SELECT * FROM tasks
    DB->>DB: WHERE assigned_to = user_id
    DB->>DB: AND status != 'completed'
    DB->>DB: ORDER BY priority DESC, due_date ASC
    DB-->>App: Return tasks (with indexes)
    
    Note over App,DB: Get Project Progress
    App->>DB: SELECT COUNT(*), SUM(CASE...)
    DB->>DB: FROM tasks
    DB->>DB: WHERE project_id = ?
    DB->>DB: GROUP BY status
    DB-->>App: Return counts by status
    
    Note over App,DB: Search Tasks (Full Text)
    App->>DB: SELECT * FROM tasks
    DB->>DB: WHERE to_tsvector('english', title || description)
    DB->>DB: @@ plainto_tsquery('search terms')
    DB-->>App: Return matching tasks
    
    Note over App,DB: Get Task with Hierarchy
    App->>DB: WITH RECURSIVE task_tree AS (...)
    DB->>DB: Recursive CTE for parent/children
    DB-->>App: Return full task tree
```

## Migration Strategy

```mermaid
graph LR
    V1[Version 1.0<br/>Initial Schema] --> V2[Version 1.1<br/>Add Tags]
    V2 --> V3[Version 1.2<br/>Add Time Entries]
    V3 --> V4[Version 1.3<br/>ADHD Metadata]
    V4 --> V5[Version 2.0<br/>Team Features]
    
    subgraph "Alembic Migrations"
        ALM[alembic revision]
        ALM --> UP[alembic upgrade head]
        ALM --> DOWN[alembic downgrade -1]
    end
    
    V1 -.-> ALM
    V2 -.-> ALM
    V3 -.-> ALM
    V4 -.-> ALM
    
    style V1 fill:#94A3B8,stroke:#64748B,color:#fff
    style V4 fill:#60A5FA,stroke:#3B82F6,color:#fff
    style V5 fill:#FB923C,stroke:#EA580C,color:#fff
```

## Data Retention & Archival

```mermaid
stateDiagram-v2
    [*] --> Active
    Active --> Completed: User Completes
    Active --> Archived: User Archives
    Completed --> Archived: 90 Days Old
    Archived --> SoftDeleted: User Deletes
    SoftDeleted --> HardDeleted: 30 Days Old
    HardDeleted --> [*]
    
    Active --> Active: Updated
    Completed --> Active: Reopened
    Archived --> Active: Restored
    SoftDeleted --> Archived: Restored
    
    note right of Active
        Visible in main views
        Fully searchable
    end note
    
    note right of Completed
        Hidden from active views
        Searchable
        Can be reopened
    end note
    
    note right of Archived
        Not in main views
        Searchable in archive
        Can be restored
    end note
    
    note right of SoftDeleted
        Not visible
        Recoverable
        deleted_at timestamp
    end note
```

---

**Schema Notes:**

1. **UUIDs everywhere** - Better for distributed systems, offline-first
2. **JSONB for flexibility** - ADHD metadata can evolve without migrations
3. **Soft deletes** - Use `deleted_at` timestamp, never hard delete user data
4. **Timestamps** - All tables have `created_at`, mutable tables have `updated_at`
5. **Enums** - Use PostgreSQL ENUMs for status, priority, role
6. **Full-text search** - `tsvector` columns for task/note searching
7. **Indexes** - Composite indexes on common query patterns
8. **Constraints** - Foreign keys with CASCADE for data integrity
