# Altair Quick Start Guide

**Get up and running in 10 minutes**

This guide gets you from zero to a running development environment as quickly
as possible. For detailed information, see the [full documentation](DOCUMENTATION_INDEX.md).

---

## Prerequisites

You need **ONE** of these options:

### Option 1: Docker (Easiest)

- [Docker Desktop](https://www.docker.com/products/docker-desktop/) or Docker Engine
- Docker Compose (included with Docker Desktop)

### Option 2: Manual Setup

- Python 3.11+
- Node.js 18+ (for Flutter web)
- Flutter SDK 3.16+
- PostgreSQL 15+
- Git

---

## Quick Setup (Docker)

**Total time: ~5 minutes**

```bash
# 1. Clone the repository
git clone https://github.com/getaltair/altair.git
cd altair

# 2. Copy environment variables
cp .env.example .env

# 3. Start everything
docker-compose up -d

# 4. View logs (optional)
docker-compose logs -f
```

**Done!** 🎉

- **Frontend:** <http://localhost:3000>
- **Backend API:** <http://localhost:8000>
- **API Documentation:** <http://localhost:8000/docs>

---

## Quick Setup (Manual)

**Total time: ~10 minutes**

### Backend Setup

```bash
# 1. Navigate to backend
cd backend

# 2. Create virtual environment
python -m venv venv

# 3. Activate virtual environment
# On macOS/Linux:
source venv/bin/activate
# On Windows:
venv\Scripts\activate

# 4. Install dependencies
pip install -r requirements.txt

# 5. Set up environment
cp .env.example .env
# Edit .env with your database credentials

# 6. Run migrations
alembic upgrade head

# 7. Start backend
uvicorn app.main:app --reload --port 8000
```

### Frontend Setup

Open a **new terminal window**:

```bash
# 1. Navigate to frontend
cd frontend

# 2. Install Flutter dependencies
flutter pub get

# 3. Run web app
flutter run -d chrome
```

**Done!** 🎉

---

## Verify Installation

### Check Backend

Open <http://localhost:8000/docs> - you should see the API documentation (Swagger UI).

Try this endpoint:

```bash
curl http://localhost:8000/health
```

Should return: `{"status": "healthy"}`

### Check Frontend

Open <http://localhost:3000> - you should see the Altair web interface.

---

## First Steps

### Create a User

```bash
# Using curl
curl -X POST http://localhost:8000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "username": "testuser",
    "password": "securepassword123"
  }'
```

Or use the web interface at <http://localhost:3000/register>

### Create Your First Task

```bash
# First, login to get a token
TOKEN=$(curl -X POST http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "securepassword123"
  }' | jq -r '.access_token')

# Create a task
curl -X POST http://localhost:8000/api/v1/tasks \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "My first task",
    "description": "Testing Altair!"
  }'
```

Or use the web interface - it's much easier! 😊

---

## Common Issues

### Port Already in Use

**Problem:** Port 8000 or 3000 is already in use

**Solution:**

```bash
# Find what's using the port
lsof -i :8000  # or :3000

# Kill the process
kill -9 <PID>

# Or change the port
# Backend: uvicorn app.main:app --port 8001
# Frontend: flutter run -d chrome --web-port 3001
```

### Database Connection Error

**Problem:** Can't connect to PostgreSQL

**Solution:**

```bash
# Check if PostgreSQL is running
# macOS:
brew services list
brew services start postgresql@15

# Linux:
sudo systemctl status postgresql
sudo systemctl start postgresql

# Docker:
docker-compose up -d postgres
```

### Python Module Not Found

**Problem:** ModuleNotFoundError

**Solution:**

```bash
# Make sure you're in the virtual environment
source venv/bin/activate  # macOS/Linux
venv\Scripts\activate     # Windows

# Reinstall dependencies
pip install -r requirements.txt
```

### Flutter Command Not Found

**Problem:** `flutter: command not found`

**Solution:**

```bash
# Install Flutter
# Follow instructions at https://docs.flutter.dev/get-started/install

# Or use snap (Linux)
sudo snap install flutter --classic

# Or use Homebrew (macOS)
brew install --cask flutter
```

---

## Development Workflow

### Running Tests

```bash
# Backend tests
cd backend
pytest

# Frontend tests
cd frontend
flutter test
```

### Code Formatting

```bash
# Backend
cd backend
black .
ruff check .

# Frontend
cd frontend
dart format .
flutter analyze
```

### Database Migrations

```bash
cd backend

# Create a new migration
alembic revision -m "description of changes"

# Apply migrations
alembic upgrade head

# Rollback
alembic downgrade -1
```

---

## Hot Reload

Both backend and frontend support hot reload for fast development:

**Backend:** FastAPI auto-reloads when you save Python files

**Frontend:** Flutter hot reloads when you save Dart files

- Press `r` in the terminal to hot reload
- Press `R` for hot restart
- Press `q` to quit

---

## Docker Commands

```bash
# Start all services
docker-compose up -d

# Stop all services
docker-compose down

# Rebuild after code changes
docker-compose up -d --build

# View logs
docker-compose logs -f [service-name]

# Restart a specific service
docker-compose restart backend

# Access database
docker-compose exec postgres psql -U altair

# Run backend shell
docker-compose exec backend bash
```

---

## Environment Variables

Key variables in `.env`:

```bash
# Database
DATABASE_URL=postgresql://altair:password@localhost/altair_dev

# JWT
SECRET_KEY=your-secret-key-here  # Generate with: openssl rand -hex 32
ALGORITHM=HS256
ACCESS_TOKEN_EXPIRE_MINUTES=15

# Environment
ENVIRONMENT=development
DEBUG=True

# CORS (for local development)
CORS_ORIGINS=http://localhost:3000
```

---

## Next Steps

Now that you're set up:

1. **Explore the code:**
   - Backend: `backend/app/`
   - Frontend: `frontend/lib/`

2. **Read the docs:**
   - [Architecture](ARCHITECTURE.md)
   - [Contributing](CONTRIBUTING.md)
   - [Features](FEATURES.md)

3. **Pick an issue:**
   - [Good first issues](https://github.com/getaltair/altair/labels/good%20first%20issue)
   - [GitHub Issues](https://github.com/getaltair/altair/issues)

4. **Join the community:**
   - [GitHub Discussions](https://github.com/getaltair/altair/discussions)

---

## Getting Help

**Still stuck?**

1. Check the [full documentation](DOCUMENTATION_INDEX.md)
2. Search [GitHub Issues](https://github.com/getaltair/altair/issues)
3. Ask in [GitHub Discussions](https://github.com/getaltair/altair/discussions)
4. Email: <hello@getaltair.app>

**ADHD-friendly tip:** It's totally okay if this takes longer than 10 minutes. Take breaks, and don't hesitate to ask for help! 💙

---

## Quick Reference

| What | URL |
|------|-----|
| Frontend | <http://localhost:3000> |
| Backend API | <http://localhost:8000> |
| API Docs | <http://localhost:8000/docs> |
| Health Check | <http://localhost:8000/health> |

| Command | What it does |
|---------|--------------|
| `docker-compose up -d` | Start all services |
| `docker-compose down` | Stop all services |
| `pytest` | Run backend tests |
| `flutter test` | Run frontend tests |
| `black .` | Format backend code |
| `dart format .` | Format frontend code |

---

**Happy coding!** 🚀

---

**Last Updated:** October 2025
