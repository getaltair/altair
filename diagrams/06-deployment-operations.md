# Altair Deployment & Operations Diagrams

## Self-Hosting Deployment Options

```mermaid
graph TD
    subgraph "Option 1: Single Server (Recommended for Start)"
        SINGLE[Single VPS/Cloud Instance]
        SINGLE --> DOCKER[Docker Compose]
        DOCKER --> ALL_SERVICES[All Services<br/>on One Machine]
        ALL_SERVICES --> SNG_PG[(PostgreSQL)]
        ALL_SERVICES --> SNG_API[FastAPI]
        ALL_SERVICES --> SNG_NGINX[Nginx]
    end

    subgraph "Option 2: Separate Database"
        WEB_SERVER[Web Server]
        DB_SERVER[Database Server]

        WEB_SERVER --> WS_API[FastAPI]
        WEB_SERVER --> WS_NGINX[Nginx]
        DB_SERVER --> DS_PG[(PostgreSQL)]

        WS_API -.Network.-> DS_PG
    end

    subgraph "Option 3: Load Balanced (Future)"
        LB[Load Balancer]
        LB --> API1[API Server 1]
        LB --> API2[API Server 2]
        LB --> API3[API Server N]

        API1 -.-> SHARED_DB[(Shared PostgreSQL)]
        API2 -.-> SHARED_DB
        API3 -.-> SHARED_DB

        SHARED_DB --> REPLICA[(Read Replica)]
    end

    style DOCKER fill:#60A5FA,stroke:#3B82F6,color:#fff
    style LB fill:#FB923C,stroke:#EA580C,color:#fff
    style SHARED_DB fill:#14B8A6,stroke:#0D9488,color:#fff
```

## Deployment Workflow

```mermaid
flowchart TD
    START([Developer Commits]) --> GIT[Git Push to GitHub]

    GIT --> CI{CI Pipeline}

    CI --> BUILD_BACK[Build Backend Image]
    CI --> BUILD_FRONT[Build Frontend]

    BUILD_BACK --> TEST_BACK[Run Backend Tests]
    BUILD_FRONT --> TEST_FRONT[Run Frontend Tests]

    TEST_BACK --> DOCKER_BUILD[Build Docker Images]
    TEST_FRONT --> DOCKER_BUILD

    DOCKER_BUILD --> REGISTRY[Push to Container Registry]

    REGISTRY --> TARGET{Deploy Target}

    TARGET -->|Staging| STAGE_DEPLOY[Deploy to Staging]
    TARGET -->|Production| APPROVAL{Manual Approval?}

    APPROVAL -->|Approved| PROD_DEPLOY[Deploy to Production]
    APPROVAL -->|Denied| CANCEL[Cancel Deployment]

    STAGE_DEPLOY --> HEALTH_CHECK[Health Check]
    PROD_DEPLOY --> HEALTH_CHECK

    HEALTH_CHECK --> PASS{Passed?}

    PASS -->|Yes| SUCCESS[Deployment Successful]
    PASS -->|No| ROLLBACK[Automatic Rollback]

    ROLLBACK --> ALERT[Alert Team]
    SUCCESS --> NOTIFY[Notify Team]

    style START fill:#60A5FA,stroke:#3B82F6,color:#fff
    style SUCCESS fill:#14B8A6,stroke:#0D9488,color:#fff
    style ROLLBACK fill:#EF4444,stroke:#DC2626,color:#fff
```

## Production Infrastructure

```mermaid
graph TB
    subgraph "Edge Layer"
        CDN[Cloudflare CDN<br/>Static Assets]
        WAF[Web Application Firewall]
    end

    subgraph "Load Balancing"
        LB[Load Balancer<br/>Nginx/HAProxy]
    end

    subgraph "Application Tier"
        API1[FastAPI Instance 1<br/>Container]
        API2[FastAPI Instance 2<br/>Container]
        API3[FastAPI Instance N<br/>Container]
    end

    subgraph "Caching Layer"
        REDIS_MASTER[(Redis Master)]
        REDIS_REPLICA[(Redis Replica)]
    end

    subgraph "Database Tier"
        PG_MASTER[(PostgreSQL Master)]
        PG_REPLICA1[(PostgreSQL Replica 1)]
        PG_REPLICA2[(PostgreSQL Replica 2)]
    end

    subgraph "Storage"
        S3[Object Storage<br/>S3/MinIO]
        BACKUP[Backup Storage]
    end

    CDN --> WAF
    WAF --> LB

    LB --> API1
    LB --> API2
    LB --> API3

    API1 --> REDIS_MASTER
    API2 --> REDIS_MASTER
    API3 --> REDIS_MASTER

    REDIS_MASTER -.Replicate.-> REDIS_REPLICA

    API1 -.Write.-> PG_MASTER
    API2 -.Write.-> PG_MASTER
    API3 -.Write.-> PG_MASTER

    API1 -.Read.-> PG_REPLICA1
    API2 -.Read.-> PG_REPLICA2
    API3 -.Read.-> PG_REPLICA1

    PG_MASTER -.Replicate.-> PG_REPLICA1
    PG_MASTER -.Replicate.-> PG_REPLICA2

    API1 -.Files.-> S3
    PG_MASTER -.Backup.-> BACKUP

    style CDN fill:#FB923C,stroke:#EA580C,color:#fff
    style LB fill:#60A5FA,stroke:#3B82F6,color:#fff
    style PG_MASTER fill:#14B8A6,stroke:#0D9488,color:#fff
```

## Monitoring & Observability Stack

```mermaid
graph TD
    subgraph "Application Layer"
        APP[Altair Apps]
    end

    subgraph "Metrics Collection"
        PROM[Prometheus<br/>Metrics Collector]
        NODE[Node Exporter<br/>System Metrics]
        PG_EXP[PostgreSQL Exporter]
    end

    subgraph "Log Aggregation"
        LOKI[Loki<br/>Log Aggregation]
        PROMTAIL[Promtail<br/>Log Shipper]
    end

    subgraph "Tracing"
        JAEGER[Jaeger<br/>Distributed Tracing]
    end

    subgraph "Visualization"
        GRAFANA[Grafana<br/>Dashboards]
    end

    subgraph "Alerting"
        ALERT_MGR[Alert Manager]
        NOTIFY[Notifications<br/>Email/Slack/Discord]
    end

    APP --> PROM
    APP --> LOKI
    APP --> JAEGER

    NODE --> PROM
    PG_EXP --> PROM

    APP --> PROMTAIL
    PROMTAIL --> LOKI

    PROM --> GRAFANA
    LOKI --> GRAFANA
    JAEGER --> GRAFANA

    PROM --> ALERT_MGR
    ALERT_MGR --> NOTIFY

    style APP fill:#60A5FA,stroke:#3B82F6,color:#fff
    style GRAFANA fill:#FB923C,stroke:#EA580C,color:#fff
    style ALERT_MGR fill:#14B8A6,stroke:#0D9488,color:#fff
```

## Key Metrics Dashboard Layout

```mermaid
graph TB
    subgraph "System Health Dashboard"
        subgraph "Top Row - Overview"
            UPTIME[System Uptime]
            USERS[Active Users]
            TASKS[Tasks Created Today]
            ERRORS[Error Rate]
        end

        subgraph "Middle Row - Performance"
            CPU[CPU Usage]
            MEM[Memory Usage]
            DISK[Disk I/O]
            NET[Network Traffic]
        end

        subgraph "Bottom Row - Application"
            API_LATENCY[API Latency]
            DB_QUERIES[DB Query Time]
            CACHE_HIT[Cache Hit Rate]
            ACTIVE_FOCUS[Active Focus Sessions]
        end
    end

    style UPTIME fill:#14B8A6,stroke:#0D9488,color:#fff
    style ERRORS fill:#EF4444,stroke:#DC2626,color:#fff
    style API_LATENCY fill:#60A5FA,stroke:#3B82F6,color:#fff
```

## Backup Strategy

```mermaid
flowchart LR
    subgraph "Database Backups"
        PG_FULL[Full Backup<br/>Daily 2 AM]
        PG_INC[Incremental<br/>Every 6 Hours]
        PG_WAL[WAL Archiving<br/>Continuous]
    end

    subgraph "File Backups"
        FILES[User Files<br/>Daily Sync]
        CONFIG[Configuration<br/>On Change]
    end

    subgraph "Backup Storage"
        LOCAL[Local Storage<br/>7 Days]
        S3[S3 Storage<br/>30 Days]
        GLACIER[Cold Storage<br/>1 Year]
    end

    PG_FULL --> LOCAL
    PG_INC --> LOCAL
    PG_WAL --> LOCAL
    FILES --> LOCAL
    CONFIG --> LOCAL

    LOCAL --> S3
    S3 --> GLACIER

    subgraph "Disaster Recovery"
        RESTORE[Restore Process]
        VERIFY[Verification]
        FAILOVER[Failover Plan]
    end

    LOCAL --> RESTORE
    S3 --> RESTORE
    RESTORE --> VERIFY
    VERIFY --> FAILOVER

    style PG_FULL fill:#14B8A6,stroke:#0D9488,color:#fff
    style S3 fill:#FB923C,stroke:#EA580C,color:#fff
    style RESTORE fill:#60A5FA,stroke:#3B82F6,color:#fff
```

## Security Monitoring

```mermaid
graph TD
    subgraph "Security Events"
        LOGIN_FAIL[Failed Login Attempts]
        UNUSUAL[Unusual Access Patterns]
        RATE_LIMIT[Rate Limit Violations]
        SQL_INJ[SQL Injection Attempts]
    end

    subgraph "Detection"
        IDS[Intrusion Detection]
        LOG_ANALYSIS[Log Analysis]
        ANOMALY[Anomaly Detection]
    end

    subgraph "Response"
        AUTO_BLOCK[Auto-Block IP]
        ALERT_SEC[Security Alert]
        AUDIT[Audit Log]
    end

    LOGIN_FAIL --> LOG_ANALYSIS
    UNUSUAL --> ANOMALY
    RATE_LIMIT --> IDS
    SQL_INJ --> IDS

    LOG_ANALYSIS --> AUTO_BLOCK
    ANOMALY --> ALERT_SEC
    IDS --> AUTO_BLOCK
    IDS --> ALERT_SEC

    AUTO_BLOCK --> AUDIT
    ALERT_SEC --> AUDIT

    style IDS fill:#EF4444,stroke:#DC2626,color:#fff
    style AUTO_BLOCK fill:#FB923C,stroke:#EA580C,color:#fff
    style AUDIT fill:#14B8A6,stroke:#0D9488,color:#fff
```

## Scaling Strategy

```mermaid
graph LR
    START[Current State<br/>Single Server] --> METRIC{Monitor Metrics}

    METRIC -->|CPU > 70%| SCALE_VERTICAL[Vertical Scaling<br/>Bigger Server]
    METRIC -->|Requests > Threshold| SCALE_HORIZONTAL[Horizontal Scaling<br/>Add API Instances]
    METRIC -->|DB Slow| DB_OPT{Optimize DB}

    SCALE_VERTICAL --> LIMIT{Hit Limits?}
    LIMIT -->|Yes| SCALE_HORIZONTAL
    LIMIT -->|No| MONITOR

    SCALE_HORIZONTAL --> LOAD_BALANCER[Add Load Balancer]
    LOAD_BALANCER --> MONITOR[Continue Monitoring]

    DB_OPT -->|Queries| INDEX[Add Indexes]
    DB_OPT -->|Volume| READ_REPLICA[Add Read Replicas]
    DB_OPT -->|Write Heavy| PARTITION[Partition Data]

    INDEX --> MONITOR
    READ_REPLICA --> MONITOR
    PARTITION --> MONITOR

    MONITOR --> METRIC

    style START fill:#60A5FA,stroke:#3B82F6,color:#fff
    style SCALE_HORIZONTAL fill:#FB923C,stroke:#EA580C,color:#fff
    style MONITOR fill:#14B8A6,stroke:#0D9488,color:#fff
```

## Performance Optimization Areas

```mermaid
mindmap
  root((Performance<br/>Optimization))
    Frontend
      Code Splitting
        Lazy Loading
        Route-based Splits
      Asset Optimization
        Image Compression
        SVG Usage
        Font Subsetting
      Caching
        Service Worker
        Local Storage
        IndexedDB
    Backend
      Database
        Query Optimization
        Connection Pooling
        Read Replicas
        Indexing Strategy
      API
        Response Caching
        Compression
        Rate Limiting
        Async Operations
      Infrastructure
        CDN Usage
        Load Balancing
        Auto Scaling
    Network
      HTTP/2
        Multiplexing
        Server Push
      Compression
        Gzip
        Brotli
      Request Reduction
        Batch APIs
        GraphQL
```

## Incident Response Flow

```mermaid
flowchart TD
    ALERT[Alert Triggered] --> ASSESS[Assess Severity]

    ASSESS --> SEV{Severity Level}

    SEV -->|P0 Critical| P0_RESPONSE[P0: All Hands]
    SEV -->|P1 High| P1_RESPONSE[P1: On-Call Team]
    SEV -->|P2 Medium| P2_RESPONSE[P2: During Business Hours]
    SEV -->|P3 Low| P3_RESPONSE[P3: Log and Schedule]

    P0_RESPONSE --> INCIDENT[Create Incident]
    P1_RESPONSE --> INCIDENT
    P2_RESPONSE --> INCIDENT
    P3_RESPONSE --> TICKET[Create Ticket]

    INCIDENT --> INVESTIGATE[Investigate]
    INVESTIGATE --> DIAGNOSE[Diagnose Root Cause]
    DIAGNOSE --> FIX[Implement Fix]

    FIX --> VERIFY{Fixed?}
    VERIFY -->|No| ESCALATE[Escalate]
    VERIFY -->|Yes| MONITOR_FIX[Monitor]

    ESCALATE --> INVESTIGATE

    MONITOR_FIX --> STABLE{Stable?}
    STABLE -->|Yes| RESOLVE[Resolve Incident]
    STABLE -->|No| INVESTIGATE

    RESOLVE --> POST_MORTEM[Post-Mortem]
    POST_MORTEM --> DOCUMENT[Document Lessons]
    DOCUMENT --> IMPROVE[Improve Systems]

    TICKET --> SCHEDULE[Schedule for Sprint]

    style ALERT fill:#EF4444,stroke:#DC2626,color:#fff
    style P0_RESPONSE fill:#EF4444,stroke:#DC2626,color:#fff
    style RESOLVE fill:#14B8A6,stroke:#0D9488,color:#fff
```

## Cost Optimization Strategy

```mermaid
graph TD
    COSTS[Infrastructure Costs] --> ANALYZE[Cost Analysis]

    ANALYZE --> CATEGORIES{Cost Categories}

    CATEGORIES --> COMPUTE[Compute Costs]
    CATEGORIES --> STORAGE[Storage Costs]
    CATEGORIES --> NETWORK[Network Costs]
    CATEGORIES --> DB[Database Costs]

    COMPUTE --> C_OPT[Optimize Compute]
    C_OPT --> RIGHTSIZE[Right-size Instances]
    C_OPT --> RESERVED[Reserved Instances]
    C_OPT --> SPOT[Spot Instances for Non-Critical]

    STORAGE --> S_OPT[Optimize Storage]
    S_OPT --> LIFECYCLE[Lifecycle Policies]
    S_OPT --> COMPRESSION[Compress Old Data]
    S_OPT --> ARCHIVAL[Archive to Cold Storage]

    NETWORK --> N_OPT[Optimize Network]
    N_OPT --> CDN_USE[Maximize CDN Usage]
    N_OPT --> REDUCE_EGRESS[Reduce Egress]

    DB --> D_OPT[Optimize Database]
    D_OPT --> QUERY_OPT[Query Optimization]
    D_OPT --> VACUUM[Regular Maintenance]
    D_OPT --> PARTITION_DATA[Partition Tables]

    RIGHTSIZE --> MONITOR[Monitor Savings]
    RESERVED --> MONITOR
    LIFECYCLE --> MONITOR
    CDN_USE --> MONITOR
    QUERY_OPT --> MONITOR

    MONITOR --> REPORT[Monthly Cost Report]

    style COSTS fill:#FB923C,stroke:#EA580C,color:#fff
    style MONITOR fill:#14B8A6,stroke:#0D9488,color:#fff
```

---

## Diagram Index & Quick Reference

### System Overview
1. **System Architecture** - See `01-system-architecture.md`
   - High-level architecture
   - Component relationships
   - Network flow
   - Deployment architecture

### Data Layer
2. **Database Schema** - See `02-database-schema-erd.md`
   - Entity Relationship Diagram
   - Table structures
   - Indexes and constraints
   - Query patterns

### User Experience
3. **User Flows** - See `03-user-flows.md`
   - Quick task capture
   - AI task breakdown
   - Focus mode sessions
   - Onboarding flow

### Project Management
4. **Roadmap & Planning** - See `04-roadmap-planning.md`
   - Timeline (Gantt chart)
   - Feature priority matrix
   - Dependency graphs
   - Sprint planning

### Technical Implementation
5. **Component Architecture** - See `05-component-architecture.md`
   - Frontend components
   - Backend request lifecycle
   - Offline-first architecture
   - AI integration

### Operations
6. **Deployment & Ops** - This file
   - Deployment options
   - Monitoring setup
   - Backup strategy
   - Incident response

---

**Usage Tips:**

1. **For Development** - Reference component and data diagrams
2. **For Planning** - Use roadmap and priority matrices
3. **For Operations** - Follow deployment and monitoring diagrams
4. **For Documentation** - Link to relevant diagrams in docs
5. **For Presentations** - Export diagrams as PNG/SVG

**Tools to Render:**
- **Mermaid Live Editor**: https://mermaid.live
- **VS Code Extension**: Markdown Preview Mermaid Support
- **GitHub**: Renders Mermaid automatically in .md files
- **Export**: Use mermaid-cli for PNG/SVG export

**Maintenance:**
- Update diagrams when architecture changes
- Version control with git
- Keep diagram code readable (good spacing, comments)
- Export static images for presentations
