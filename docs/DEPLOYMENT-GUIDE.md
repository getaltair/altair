# Deployment Guide

> **TL;DR:** Standalone = Download + run (30 seconds). Self-hosted = `docker-compose up -d` (10 minutes). Cloud = Deploy to Render/Railway (15 minutes). Choose based on your needs.

## Quick Start

**What you need to know in 60 seconds:**

- **Standalone mode**: Download installer → Double-click → Done
- **Self-hosted sync**: VPS + Docker Compose → `docker-compose up -d`
- **Cloud hosted**: Render/Railway → Connect GitHub → Deploy
- **Requirements**: Standalone = nothing. Sync = Docker + VPS. Cloud = GitHub account.

**Navigation:**

- [Architecture Overview](./ARCHITECTURE-OVERVIEW.md) - System design
- [Data Flow](./DATA-FLOW.md) - How data moves
- [Component Design](./COMPONENT-DESIGN.md) - Component breakdown
- [Development Roadmap](./DEVELOPMENT-ROADMAP.md) - Implementation timeline

---

## Deployment Options

Three deployment modes to match your needs.

```mermaid
graph TB
    Start{Choose Deployment}

    Start --> Standalone[Standalone Mode<br/>No server needed]
    Start --> SelfHost[Self-Hosted Sync<br/>Your own server]
    Start --> Cloud[Cloud Hosted<br/>Managed service]

    Standalone --> S1[Download installer]
    S1 --> S2[Double-click]
    S2 --> S3[✅ Running in 30s]

    SelfHost --> H1[Get VPS]
    H1 --> H2[Install Docker]
    H2 --> H3[docker-compose up -d]
    H3 --> H4[✅ Running in 10min]

    Cloud --> C1[Fork GitHub repo]
    C1 --> C2[Connect to Render]
    C2 --> C3[Click Deploy]
    C3 --> C4[✅ Running in 15min]

    style Standalone fill:#6BCB77,stroke:#000,stroke-width:3px
    style SelfHost fill:#60A5FA,stroke:#000,stroke-width:3px
    style Cloud fill:#FFD93D,stroke:#000,stroke-width:3px
```

### Comparison Matrix

| Aspect              | Standalone       | Self-Hosted         | Cloud Hosted          |
| ------------------- | ---------------- | ------------------- | --------------------- |
| **Setup time**      | 30 seconds       | 10 minutes          | 15 minutes            |
| **Cost**            | $0               | ~$25/month          | ~$40/month            |
| **Maintenance**     | None             | Manual updates      | Auto-updates          |
| **Privacy**         | 100% local       | Full control        | Shared infrastructure |
| **Multi-device**    | ❌ Single device | ✅ All devices      | ✅ All devices        |
| **Internet needed** | Only for AI      | For sync only       | For all features      |
| **Technical skill** | None             | Basic Docker        | None                  |
| **Backup**          | Manual file copy | Your responsibility | Automated             |

---

## Standalone Desktop Deployment

Zero-friction installation for single-device use.

### Deployment Topology

```mermaid
graph TB
    subgraph "User's Computer"
        App[Altair Guidance.app<br/>Native compiled]
        DB[(SQLite<br/>~/altair/guidance.db)]
        Config[Config File<br/>~/.altair/config.json]
    end

    subgraph "External Services (Optional)"
        AI[AI Provider<br/>OpenAI/Anthropic/Ollama]
    end

    App --> DB
    App --> Config
    App -.->|Direct API calls| AI

    style App fill:#FFD93D,stroke:#000,stroke-width:3px
    style DB fill:#6BCB77,stroke:#000,stroke-width:3px
```

### Build Process

How standalone installers are created:

```mermaid
graph LR
    A[Source Code] --> B[Flutter Build<br/>flutter build macos --release]
    B --> C[Native Binary<br/>.app bundle]
    C --> D[Code Signing<br/>Apple Developer ID]
    D --> E[Installer Creation<br/>create-dmg]
    E --> F[Altair-Guidance-1.0.0.dmg<br/>~200MB]

    F --> G1[User Downloads]
    G1 --> H1[Double-click DMG]
    H1 --> I1[Drag to Applications]
    I1 --> J1[✅ Launch App]

    style B fill:#60A5FA,stroke:#000,stroke-width:3px
    style F fill:#FFD93D,stroke:#000,stroke-width:3px
    style J1 fill:#6BCB77,stroke:#000,stroke-width:3px
```

### Installation Steps (User Side)

**macOS:**

```bash
# 1. Download Altair-Guidance-1.0.0.dmg
# 2. Double-click DMG file
# 3. Drag "Altair Guidance.app" to Applications folder
# 4. Launch from Applications
# 5. (First launch) Allow in System Preferences → Security
```

**Windows:**

```bash
# 1. Download Altair-Guidance-Setup-1.0.0.exe
# 2. Double-click installer
# 3. Follow installer wizard (Next → Next → Install)
# 4. Launch from Desktop or Start Menu
```

**Linux:**

```bash
# Download AppImage
wget https://releases.altair.app/guidance/1.0.0/Altair-Guidance-1.0.0-x86_64.AppImage

# Make executable
chmod +x Altair-Guidance-1.0.0-x86_64.AppImage

# Run
./Altair-Guidance-1.0.0-x86_64.AppImage
```

### Build Scripts (Developer Side)

```bash
# scripts/build-standalone.sh
#!/bin/bash

set -e

APP_NAME="altair-guidance"
VERSION="1.0.0"

echo "Building standalone installers for $APP_NAME v$VERSION"

# Build macOS
echo "Building macOS..."
cd apps/$APP_NAME
flutter build macos --release --dart-define=SYNC_ENABLED=false

# Sign macOS app
codesign --deep --force --verify --verbose \
  --sign "Developer ID Application: Your Name" \
  "build/macos/Build/Products/Release/Altair Guidance.app"

# Create DMG
create-dmg \
  --volname "Altair Guidance" \
  --volicon "../../assets/icon.icns" \
  --window-pos 200 120 \
  --window-size 600 300 \
  --icon-size 100 \
  --icon "Altair Guidance.app" 175 120 \
  --app-drop-link 425 120 \
  "Altair-Guidance-$VERSION.dmg" \
  "build/macos/Build/Products/Release/Altair Guidance.app"

# Build Windows
echo "Building Windows..."
flutter build windows --release --dart-define=SYNC_ENABLED=false

# Create installer with NSIS
makensis \
  -DVERSION=$VERSION \
  -DAPP_NAME="Altair Guidance" \
  windows/installer.nsi

# Build Linux AppImage
echo "Building Linux..."
flutter build linux --release --dart-define=SYNC_ENABLED=false

# Package as AppImage
appimagetool \
  build/linux/release/bundle \
  "Altair-Guidance-$VERSION-x86_64.AppImage"

echo "✅ All installers built successfully!"
ls -lh *.dmg *.exe *.AppImage
```

---

## Self-Hosted Deployment

Full-stack deployment with sync on your own infrastructure.

### Deployment Topology

```mermaid
graph TB
    subgraph "User Devices"
        D1[Desktop App<br/>macOS]
        D2[Desktop App<br/>Windows]
        D3[Mobile App<br/>iOS]
    end

    subgraph "Your VPS (Docker Compose)"
        NGINX[Nginx<br/>Reverse Proxy<br/>:80, :443]

        subgraph "Backend Services"
            AUTH[Auth Service<br/>FastAPI<br/>:8000]
            SYNC[PowerSync Service<br/>:8080]
        end

        subgraph "Databases"
            PG[(PostgreSQL<br/>:5432)]
            REDIS[(Redis<br/>:6379)]
        end

        subgraph "Monitoring (Optional)"
            PROM[Prometheus<br/>:9090]
            GRAF[Grafana<br/>:3000]
        end
    end

    D1 -->|HTTPS| NGINX
    D2 -->|HTTPS| NGINX
    D3 -->|HTTPS| NGINX

    NGINX --> AUTH
    NGINX --> SYNC

    AUTH --> PG
    AUTH --> REDIS
    SYNC --> PG

    AUTH --> PROM
    SYNC --> PROM
    PROM --> GRAF

    style NGINX fill:#FFD93D,stroke:#000,stroke-width:3px
    style AUTH fill:#60A5FA,stroke:#000,stroke-width:3px
    style SYNC fill:#6BCB77,stroke:#000,stroke-width:3px
    style PG fill:#FF6B6B,stroke:#000,stroke-width:3px
```

### Docker Compose Setup

Complete `docker-compose.yml` for self-hosting:

```yaml
# infrastructure/docker/docker-compose.yml
version: "3.8"

services:
  # PostgreSQL database
  postgres:
    image: postgres:16-alpine
    container_name: altair-postgres
    environment:
      POSTGRES_DB: altair
      POSTGRES_USER: altair
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes:
      - postgres-data:/var/lib/postgresql/data
      - ./postgres/init.sql:/docker-entrypoint-initdb.d/init.sql
    ports:
      - "5432:5432"
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U altair"]
      interval: 10s
      timeout: 5s
      retries: 5
    restart: unless-stopped

  # Redis cache
  redis:
    image: redis:7-alpine
    container_name: altair-redis
    ports:
      - "6379:6379"
    volumes:
      - redis-data:/data
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 5s
      retries: 5
    restart: unless-stopped

  # Auth service (FastAPI)
  auth-service:
    build:
      context: ../../services/auth-service
      dockerfile: Dockerfile
    container_name: altair-auth
    environment:
      DATABASE_URL: postgresql://altair:${DB_PASSWORD}@postgres:5432/altair
      REDIS_URL: redis://redis:6379/0
      SECRET_KEY: ${SECRET_KEY}
      ENVIRONMENT: production
    ports:
      - "8000:8000"
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy
    restart: unless-stopped

  # PowerSync service
  powersync:
    image: journeyapps/powersync-service:latest
    container_name: altair-powersync
    environment:
      DATABASE_URL: postgresql://altair:${DB_PASSWORD}@postgres:5432/altair
      POWERSYNC_SECRET: ${POWERSYNC_SECRET}
    volumes:
      - ../../services/sync-service/sync-rules.yaml:/app/sync-rules.yaml
    ports:
      - "8080:8080"
    depends_on:
      postgres:
        condition: service_healthy
    restart: unless-stopped

  # Nginx reverse proxy
  nginx:
    image: nginx:alpine
    container_name: altair-nginx
    volumes:
      - ./nginx/nginx.conf:/etc/nginx/nginx.conf:ro
      - ./nginx/ssl:/etc/nginx/ssl:ro
      - certbot-data:/var/www/certbot
    ports:
      - "80:80"
      - "443:443"
    depends_on:
      - auth-service
      - powersync
    restart: unless-stopped

  # Certbot for SSL (Let's Encrypt)
  certbot:
    image: certbot/certbot
    container_name: altair-certbot
    volumes:
      - certbot-data:/var/www/certbot
      - ./nginx/ssl:/etc/letsencrypt
    entrypoint: "/bin/sh -c 'trap exit TERM; while :; do certbot renew; sleep 12h & wait $${!}; done;'"

  # Prometheus (optional - monitoring)
  prometheus:
    image: prom/prometheus:latest
    container_name: altair-prometheus
    volumes:
      - ./prometheus/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus-data:/prometheus
    ports:
      - "9090:9090"
    restart: unless-stopped
    profiles:
      - monitoring

  # Grafana (optional - dashboards)
  grafana:
    image: grafana/grafana:latest
    container_name: altair-grafana
    environment:
      GF_SECURITY_ADMIN_PASSWORD: ${GRAFANA_PASSWORD}
    volumes:
      - grafana-data:/var/lib/grafana
    ports:
      - "3000:3000"
    depends_on:
      - prometheus
    restart: unless-stopped
    profiles:
      - monitoring

volumes:
  postgres-data:
  redis-data:
  prometheus-data:
  grafana-data:
  certbot-data:
```

### Environment Configuration

```bash
# infrastructure/docker/.env.example
# Database
DB_PASSWORD=your-secure-password-here

# Auth Service
SECRET_KEY=your-secret-key-min-32-chars
JWT_ALGORITHM=HS256
ACCESS_TOKEN_EXPIRE_MINUTES=10080  # 7 days

# PowerSync
POWERSYNC_SECRET=your-powersync-secret

# Domain (for SSL)
DOMAIN=altair.yourdomain.com

# Optional Monitoring
GRAFANA_PASSWORD=admin

# Optional AI Service
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
```

### Self-Hosting Steps

**Step 1: Get a VPS**

```bash
# Recommended providers:
# - DigitalOcean: $24/month (4GB RAM, 2 vCPUs)
# - Hetzner: $13/month (4GB RAM, 2 vCPUs)
# - Linode: $24/month (4GB RAM, 2 vCPUs)

# Minimum requirements:
# - 2GB RAM (4GB recommended)
# - 2 vCPUs
# - 50GB SSD storage
# - Ubuntu 22.04 or 24.04 LTS
```

**Step 2: Initial server setup**

```bash
# SSH into your VPS
ssh root@your-server-ip

# Update system
apt update && apt upgrade -y

# Create non-root user
adduser altair
usermod -aG sudo altair
su - altair

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh
sudo usermod -aG docker $USER

# Install Docker Compose
sudo apt install docker-compose-plugin -y

# Verify installation
docker --version
docker compose version
```

**Step 3: Clone and configure**

```bash
# Clone repository
git clone https://github.com/getaltair/altair.git
cd altair/infrastructure/docker

# Copy environment file
cp .env.example .env

# Generate secrets
echo "SECRET_KEY=$(openssl rand -hex 32)" >> .env
echo "DB_PASSWORD=$(openssl rand -hex 16)" >> .env
echo "POWERSYNC_SECRET=$(openssl rand -hex 16)" >> .env

# Edit configuration
nano .env
# Update DOMAIN to your domain name
```

**Step 4: Configure DNS**

```bash
# Add DNS A records pointing to your VPS IP:
# altair.yourdomain.com -> VPS_IP
# (or use subdomain like sync.altair.com)
```

**Step 5: Setup SSL**

```bash
# Initialize SSL certificate
docker compose run --rm certbot certonly \
  --webroot \
  --webroot-path=/var/www/certbot \
  -d altair.yourdomain.com \
  --agree-tos \
  --email your@email.com

# Verify certificate created
ls -la nginx/ssl/live/altair.yourdomain.com/
```

**Step 6: Launch services**

```bash
# Start all services
docker compose up -d

# Check status
docker compose ps

# View logs
docker compose logs -f

# Should see:
# ✅ postgres - healthy
# ✅ redis - healthy
# ✅ auth-service - running
# ✅ powersync - running
# ✅ nginx - running
```

**Step 7: Verify deployment**

```bash
# Test auth service
curl https://altair.yourdomain.com/api/health

# Expected: {"status": "ok"}

# Test PowerSync
curl https://altair.yourdomain.com/sync/health

# Expected: {"status": "healthy"}

# Create first user
curl -X POST https://altair.yourdomain.com/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email": "you@example.com", "password": "secure-password"}'
```

**Step 8: Configure desktop app**

```bash
# On your computer:
# 1. Launch Altair Guidance
# 2. Go to Settings → Sync
# 3. Enable sync
# 4. Enter server URL: https://altair.yourdomain.com
# 5. Login with credentials from Step 7
# 6. ✅ Sync enabled!
```

### Maintenance

**Update services:**

```bash
# Pull latest images
docker compose pull

# Restart services with new images
docker compose up -d

# View logs for errors
docker compose logs -f
```

**Backup database:**

```bash
# Backup PostgreSQL
docker compose exec postgres pg_dump -U altair altair > backup-$(date +%Y%m%d).sql

# Restore backup
cat backup-20251017.sql | docker compose exec -T postgres psql -U altair altair
```

**Monitor resources:**

```bash
# View resource usage
docker stats

# View disk usage
docker system df

# Clean up old images
docker system prune -a
```

---

## Cloud Deployment

Deploy to managed cloud platforms for zero maintenance.

### Deployment Topology (Render.com)

```mermaid
graph TB
    subgraph "User Devices"
        D1[Desktop Apps]
        D2[Mobile Apps]
    end

    subgraph "Render.com (Managed)"
        WEB[Web Service<br/>Auth + PowerSync<br/>Auto-scaling]
        DB[(PostgreSQL<br/>Managed Database)]
        REDIS[(Redis<br/>Managed Cache)]
        CDN[CDN<br/>Static Assets]
    end

    D1 -->|HTTPS| CDN
    D2 -->|HTTPS| CDN
    CDN --> WEB
    WEB --> DB
    WEB --> REDIS

    style WEB fill:#FFD93D,stroke:#000,stroke-width:3px
    style DB fill:#FF6B6B,stroke:#000,stroke-width:3px
    style CDN fill:#60A5FA,stroke:#000,stroke-width:3px
```

### Deploy to Render.com

**Step 1: Prepare repository**

```yaml
# render.yaml (add to repo root)
services:
  # Auth service
  - type: web
    name: altair-auth
    env: python
    buildCommand: "cd services/auth-service && pip install -r requirements.txt"
    startCommand: "cd services/auth-service && uvicorn app.main:app --host 0.0.0.0 --port $PORT"
    envVars:
      - key: DATABASE_URL
        fromDatabase:
          name: altair-postgres
          property: connectionString
      - key: REDIS_URL
        fromDatabase:
          name: altair-redis
          property: connectionString
      - key: SECRET_KEY
        generateValue: true
      - key: ENVIRONMENT
        value: production

  # PowerSync service
  - type: web
    name: altair-powersync
    env: docker
    dockerfilePath: services/sync-service/Dockerfile
    envVars:
      - key: DATABASE_URL
        fromDatabase:
          name: altair-postgres
          property: connectionString

databases:
  - name: altair-postgres
    databaseName: altair
    user: altair
    plan: starter # $7/month

  - name: altair-redis
    plan: starter # $10/month
```

**Step 2: Deploy**

```bash
# 1. Push to GitHub
git push origin main

# 2. Go to render.com
# 3. Click "New" → "Blueprint"
# 4. Connect GitHub repo
# 5. Select altair repository
# 6. Click "Apply"

# Render automatically:
# - Creates PostgreSQL database
# - Creates Redis instance
# - Builds and deploys services
# - Provisions SSL certificate
# - Sets up health checks

# 7. Wait 5-10 minutes for deployment
# 8. ✅ Live at https://altair-xxx.onrender.com
```

**Estimated costs (Render):**

- Web service: $7/month (starter)
- PostgreSQL: $7/month (starter)
- Redis: $10/month (starter)
- **Total: ~$24/month**

### Deploy to Railway.app

**Step 1: Install Railway CLI**

```bash
# Install CLI
npm install -g @railway/cli

# Login
railway login

# Initialize project
railway init
```

**Step 2: Deploy**

```bash
# Create new project
railway up

# Add PostgreSQL
railway add --plugin postgresql

# Add Redis
railway add --plugin redis

# Deploy auth service
cd services/auth-service
railway up

# Deploy PowerSync service
cd ../sync-service
railway up

# ✅ Deployed in 5 minutes
```

**Estimated costs (Railway):**

- Starter plan: $5/month
- Usage-based after free tier
- **Total: ~$15-25/month**

---

## Deployment Checklist

Use this checklist for successful deployment.

### Pre-Deployment

- [ ] Choose deployment method (standalone/self-hosted/cloud)
- [ ] Prepare environment variables
- [ ] Generate secure secrets
- [ ] Configure DNS (if self-hosted or cloud)
- [ ] Prepare SSL certificates (if self-hosted)
- [ ] Review security settings

### Deployment

**Standalone:**

- [ ] Build platform-specific installers
- [ ] Sign applications (macOS/Windows)
- [ ] Test installations on clean systems
- [ ] Upload to release hosting
- [ ] Create download page

**Self-Hosted:**

- [ ] Provision VPS
- [ ] Install Docker & Docker Compose
- [ ] Clone repository
- [ ] Configure environment variables
- [ ] Setup SSL certificates
- [ ] Start services with `docker compose up -d`
- [ ] Verify all services healthy

**Cloud:**

- [ ] Connect GitHub repository
- [ ] Configure render.yaml or railway.toml
- [ ] Add environment variables
- [ ] Deploy services
- [ ] Verify deployment successful

### Post-Deployment

- [ ] Test health endpoints
- [ ] Create first user account
- [ ] Test sync from desktop app
- [ ] Configure monitoring (optional)
- [ ] Setup automated backups
- [ ] Document deployment URLs/credentials
- [ ] Test failover/recovery

---

## Monitoring & Observability

Track system health and performance.

### Health Checks

```bash
# Auth service health
curl https://your-domain.com/api/health

# PowerSync health
curl https://your-domain.com/sync/health

# Database connection
docker compose exec postgres pg_isready -U altair

# Redis connection
docker compose exec redis redis-cli ping
```

### Prometheus Metrics

```yaml
# prometheus/prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: "auth-service"
    static_configs:
      - targets: ["auth-service:8000"]

  - job_name: "powersync"
    static_configs:
      - targets: ["powersync:8080"]

  - job_name: "postgres"
    static_configs:
      - targets: ["postgres-exporter:9187"]
```

### Grafana Dashboards

```bash
# Access Grafana
open http://localhost:3000

# Login: admin / ${GRAFANA_PASSWORD}

# Import dashboards:
# - PostgreSQL: Dashboard ID 9628
# - Redis: Dashboard ID 11835
# - Nginx: Dashboard ID 12708
```

---

## Troubleshooting

Common deployment issues and solutions.

### Services won't start

```bash
# Check logs
docker compose logs -f

# Check service health
docker compose ps

# Restart specific service
docker compose restart auth-service

# Rebuild and restart
docker compose up -d --build
```

### SSL certificate issues

```bash
# Check certificate expiry
openssl x509 -in nginx/ssl/live/yourdomain.com/fullchain.pem -noout -dates

# Renew certificate manually
docker compose run --rm certbot renew

# Force renewal
docker compose run --rm certbot renew --force-renewal
```

### Database connection errors

```bash
# Check PostgreSQL is running
docker compose exec postgres pg_isready

# Check connection from auth service
docker compose exec auth-service env | grep DATABASE_URL

# Connect to database directly
docker compose exec postgres psql -U altair altair
```

### Sync not working

```bash
# Check PowerSync logs
docker compose logs powersync

# Verify sync rules
cat services/sync-service/sync-rules.yaml

# Test sync endpoint
curl https://your-domain.com/sync/stream
```

---

## Security Best Practices

Keep your deployment secure.

### Environment Variables

```bash
# Generate strong secrets
openssl rand -hex 32  # For SECRET_KEY
openssl rand -hex 16  # For DB_PASSWORD

# Never commit .env file
echo ".env" >> .gitignore

# Use environment-specific files
.env.production
.env.staging
.env.development
```

### Firewall Configuration

```bash
# Ubuntu UFW
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow ssh
sudo ufw allow http
sudo ufw allow https
sudo ufw enable

# Check status
sudo ufw status
```

### SSL/TLS

```bash
# Use strong ciphers in nginx.conf
ssl_protocols TLSv1.2 TLSv1.3;
ssl_ciphers 'ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256...';
ssl_prefer_server_ciphers on;

# Enable HSTS
add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
```

### Database Security

```bash
# Use strong passwords
DB_PASSWORD=$(openssl rand -hex 16)

# Restrict PostgreSQL access
# In docker-compose.yml - don't expose port 5432 publicly

# Regular backups
0 2 * * * /opt/altair/backup.sh  # Daily at 2 AM
```

---

## What's Next?

### Related Documentation

- [Architecture Overview](./ARCHITECTURE-OVERVIEW.md) - High-level system design
- [Data Flow](./DATA-FLOW.md) - How data moves through system
- [Component Design](./COMPONENT-DESIGN.md) - Component breakdown
- [Development Roadmap](./DEVELOPMENT-ROADMAP.md) - Implementation timeline

### Getting Started

**For users:**

1. Download standalone app
2. Try it out offline-first
3. Enable sync when you need multi-device

**For self-hosters:**

1. Follow self-hosting steps above
2. Join Discord for support
3. Share your setup for community feedback

---

## FAQ

**Q: Which deployment method should I choose?**
A: Start standalone. Add self-hosted sync if you need multi-device. Consider cloud if you want zero maintenance.

**Q: What's the minimum VPS specs for self-hosting?**
A: 2GB RAM, 2 vCPUs, 50GB SSD. Supports ~100 concurrent users.

**Q: Can I migrate from standalone to self-hosted?**
A: Yes! Export SQLite data, import to PostgreSQL, enable sync in app settings.

**Q: How do I backup my data?**
A: Standalone: Copy `~/altair/*.db` files. Self-hosted: `pg_dump` PostgreSQL database.

**Q: What if my VPS goes down?**
A: Apps continue working offline. Changes sync when VPS returns. Zero data loss.

**Q: Can I use my own domain?**
A: Yes! Point DNS A record to your VPS IP, configure in .env and nginx.conf.

**Q: How do I update to new versions?**
A: Standalone: Download new installer. Self-hosted: `docker compose pull && docker compose up -d`.

---

**Next:** [Development Roadmap](./DEVELOPMENT-ROADMAP.md) → See implementation timeline
