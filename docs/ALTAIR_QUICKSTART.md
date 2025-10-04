# Altair Python/FastAPI Quick Start Guide

## Prerequisites

- Python 3.12+
- PostgreSQL 14+
- Redis 6+
- UV package manager
- Git

## 1. Initial Setup (10 minutes)

### Install UV Package Manager
```bash
# macOS/Linux
curl -LsSf https://astral.sh/uv/install.sh | sh

# Windows
powershell -c "irm https://astral.sh/uv/install.ps1 | iex"
```

### Clone and Setup Project
```bash
# Create project structure
mkdir altair && cd altair
git init

# Create backend structure
mkdir -p backend/{app,tests,alembic,scripts}
mkdir -p backend/app/{api,models,schemas,services,workers,websocket}
mkdir -p docker
mkdir -p flutter

# Create Python project file
cd backend
```

### Create pyproject.toml
```toml
[project]
name = "altair"
version = "0.1.0"
requires-python = ">=3.12"
description = "ADHD-friendly project management"

dependencies = [
    "fastapi>=0.104.0",
    "uvicorn[standard]>=0.24.0",
    "sqlalchemy>=2.0.0",
    "alembic>=1.12.0",
    "psycopg2-binary>=2.9.0",
    "redis>=5.0.0",
    "python-jose[cryptography]>=3.3.0",
    "passlib[bcrypt]>=1.7.4",
    "pydantic>=2.5.0",
    "pydantic-settings>=2.0.0",
    "python-multipart>=0.0.6",
    "httpx>=0.25.0",
]

[tool.uv]
dev-dependencies = [
    "pytest>=7.4.0",
    "pytest-asyncio>=0.21.0",
    "ruff>=0.1.0",
    "black>=23.0.0",
    "ipython>=8.0.0",
]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.ruff]
line-length = 88
target-version = "py312"

[tool.black]
line-length = 88
target-version = ["py312"]
```

### Install Dependencies
```bash
# Create virtual environment and install
uv venv
source .venv/bin/activate  # On Windows: .venv\Scripts\activate
uv sync
```

## 2. Database Setup (5 minutes)

### Local PostgreSQL with Docker
```bash
# Quick PostgreSQL setup
docker run -d \
  --name altair-db \
  -e POSTGRES_USER=altair \
  -e POSTGRES_PASSWORD=altair \
  -e POSTGRES_DB=altair \
  -p 5432:5432 \
  postgres:15-alpine

# Redis for caching/sessions
docker run -d \
  --name altair-redis \
  -p 6379:6379 \
  redis:7-alpine
```

### Or Use Hosted Services (Recommended for MVP)
1. **Supabase** (PostgreSQL): https://app.supabase.com
   - Create new project
   - Copy connection string
   
2. **Upstash** (Redis): https://console.upstash.com
   - Create Redis database  
   - Copy connection string

## 3. Create Core Files (10 minutes)

### app/config.py
```python
from pydantic_settings import BaseSettings
from typing import Optional

class Settings(BaseSettings):
    # Database
    DATABASE_URL: str = "postgresql://altair:altair@localhost/altair"
    REDIS_URL: str = "redis://localhost:6379"
    
    # Security
    SECRET_KEY: str = "your-secret-key-change-this"
    ALGORITHM: str = "HS256"
    ACCESS_TOKEN_EXPIRE_MINUTES: int = 30
    
    # App
    APP_NAME: str = "Altair"
    DEBUG: bool = True
    
    class Config:
        env_file = ".env"

settings = Settings()
```

### app/database.py
```python
from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker
from .config import settings

engine = create_engine(settings.DATABASE_URL)
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
Base = declarative_base()

def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()
```

### app/models/base.py
```python
from sqlalchemy import Column, DateTime
from sqlalchemy.dialects.postgresql import UUID
from sqlalchemy.sql import func
import uuid
from ..database import Base

class BaseModel(Base):
    __abstract__ = True
    
    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    created_at = Column(DateTime(timezone=True), server_default=func.now())
    updated_at = Column(DateTime(timezone=True), onupdate=func.now())
```

### app/models/task.py
```python
from sqlalchemy import Column, String, Integer, Text, Enum, ForeignKey
from sqlalchemy.dialects.postgresql import UUID
import enum
from .base import BaseModel

class TaskState(enum.Enum):
    INBOX = "inbox"
    TRIAGED = "triaged"
    ACTIVE = "active"
    BLOCKED = "blocked"
    DONE = "done"
    ARCHIVED = "archived"

class Task(BaseModel):
    __tablename__ = "tasks"
    
    title = Column(String(255), nullable=False)
    description = Column(Text)
    state = Column(Enum(TaskState), default=TaskState.INBOX)
    cognitive_load = Column(Integer, default=5)
    estimated_minutes = Column(Integer)
    
    # Will add user_id, project_id later
```

### app/schemas/task.py
```python
from pydantic import BaseModel
from typing import Optional
from datetime import datetime
from uuid import UUID

class TaskBase(BaseModel):
    title: str
    description: Optional[str] = None
    cognitive_load: Optional[int] = 5
    estimated_minutes: Optional[int] = None

class TaskCreate(TaskBase):
    pass

class TaskUpdate(BaseModel):
    title: Optional[str] = None
    description: Optional[str] = None
    state: Optional[str] = None
    cognitive_load: Optional[int] = None
    estimated_minutes: Optional[int] = None

class TaskResponse(TaskBase):
    id: UUID
    state: str
    created_at: datetime
    updated_at: Optional[datetime]
    
    class Config:
        from_attributes = True
```

### app/api/tasks.py
```python
from fastapi import APIRouter, Depends, HTTPException
from sqlalchemy.orm import Session
from typing import List
from uuid import UUID
from ..database import get_db
from ..models.task import Task, TaskState
from ..schemas.task import TaskCreate, TaskUpdate, TaskResponse

router = APIRouter()

@router.post("/quick-capture", response_model=TaskResponse)
async def quick_capture(
    text: str,
    db: Session = Depends(get_db)
):
    """ADHD-friendly quick task capture"""
    task = Task(title=text, state=TaskState.INBOX)
    db.add(task)
    db.commit()
    db.refresh(task)
    return task

@router.get("/", response_model=List[TaskResponse])
async def list_tasks(
    skip: int = 0,
    limit: int = 100,
    db: Session = Depends(get_db)
):
    tasks = db.query(Task).offset(skip).limit(limit).all()
    return tasks

@router.post("/", response_model=TaskResponse)
async def create_task(
    task: TaskCreate,
    db: Session = Depends(get_db)
):
    db_task = Task(**task.dict())
    db.add(db_task)
    db.commit()
    db.refresh(db_task)
    return db_task

@router.get("/{task_id}", response_model=TaskResponse)
async def get_task(
    task_id: UUID,
    db: Session = Depends(get_db)
):
    task = db.query(Task).filter(Task.id == task_id).first()
    if not task:
        raise HTTPException(status_code=404, detail="Task not found")
    return task

@router.put("/{task_id}", response_model=TaskResponse)
async def update_task(
    task_id: UUID,
    task_update: TaskUpdate,
    db: Session = Depends(get_db)
):
    task = db.query(Task).filter(Task.id == task_id).first()
    if not task:
        raise HTTPException(status_code=404, detail="Task not found")
    
    update_data = task_update.dict(exclude_unset=True)
    for field, value in update_data.items():
        setattr(task, field, value)
    
    db.commit()
    db.refresh(task)
    return task
```

### app/main.py
```python
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from .api import tasks
from .config import settings
from .database import engine
from .models import base

# Create tables
base.Base.metadata.create_all(bind=engine)

app = FastAPI(
    title="Altair",
    description="ADHD-friendly project management",
    version="0.1.0"
)

# CORS for Flutter app
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Configure properly for production
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Include routers
app.include_router(tasks.router, prefix="/api/tasks", tags=["tasks"])

@app.get("/")
async def root():
    return {"message": "Altair API", "version": "0.1.0"}

@app.get("/health")
async def health():
    return {"status": "healthy"}
```

## 4. Run the Application (2 minutes)

### Start Development Server
```bash
# From backend directory
uv run uvicorn app.main:app --reload --port 8000
```

### Test the API
```bash
# Quick capture
curl -X POST "http://localhost:8000/api/tasks/quick-capture?text=Build%20authentication"

# List tasks
curl "http://localhost:8000/api/tasks"

# Create task with details
curl -X POST "http://localhost:8000/api/tasks" \
  -H "Content-Type: application/json" \
  -d '{"title": "Setup database", "cognitive_load": 7}'
```

### Access Interactive Docs
- Swagger UI: http://localhost:8000/docs
- ReDoc: http://localhost:8000/redoc

## 5. Set Up Database Migrations (5 minutes)

### Initialize Alembic
```bash
uv run alembic init alembic
```

### Update alembic/env.py
```python
# Add after imports
from app.models.base import Base
from app.config import settings

# Update this line
config.set_main_option("sqlalchemy.url", settings.DATABASE_URL)

# Update target_metadata
target_metadata = Base.metadata
```

### Create First Migration
```bash
uv run alembic revision --autogenerate -m "Initial schema"
uv run alembic upgrade head
```

## 6. Deploy to Railway (5 minutes)

### Create Railway Account
1. Sign up at https://railway.app
2. Connect GitHub account

### Prepare for Deployment

#### Create requirements.txt
```bash
uv pip freeze > requirements.txt
```

#### Create railway.json
```json
{
  "$schema": "https://railway.app/railway.schema.json",
  "build": {
    "builder": "NIXPACKS"
  },
  "deploy": {
    "startCommand": "cd backend && uvicorn app.main:app --host 0.0.0.0 --port $PORT",
    "restartPolicyType": "ON_FAILURE",
    "restartPolicyMaxRetries": 3
  }
}
```

#### Create .env.example
```bash
DATABASE_URL=postgresql://user:pass@host/dbname
REDIS_URL=redis://host:6379
SECRET_KEY=your-secret-key-here
```

### Deploy
```bash
# Push to GitHub
git add .
git commit -m "Initial Altair MVP"
git push origin main

# In Railway:
# 1. New Project → Deploy from GitHub repo
# 2. Add PostgreSQL service
# 3. Add Redis service
# 4. Set environment variables
# 5. Deploy!
```

## 7. Next Steps

### Immediate Tasks (Week 1)
- [ ] Add user authentication (JWT)
- [ ] Implement task state transitions
- [ ] Create focus timer with WebSocket
- [ ] Add public stats endpoint
- [ ] Basic Flutter UI

### Testing Setup
```bash
# Create test file
mkdir tests
touch tests/test_tasks.py

# Run tests
uv run pytest
```

### Add Authentication
```python
# Quick auth setup with python-jose and passlib
# See CLAUDE_PYTHON.md for detailed implementation
```

### Add WebSocket for Real-time
```python
# Focus timer updates
# See main documentation for WebSocket patterns
```

## Common Issues & Solutions

### Issue: Database connection refused
```bash
# Check PostgreSQL is running
docker ps
# Restart if needed
docker start altair-db
```

### Issue: Port already in use
```bash
# Use different port
uv run uvicorn app.main:app --reload --port 8001
```

### Issue: Import errors
```bash
# Ensure virtual environment is activated
source .venv/bin/activate
# Reinstall dependencies
uv sync
```

## Useful Commands

```bash
# Development
uv run uvicorn app.main:app --reload  # Start dev server
uv run pytest                          # Run tests
uv run black .                         # Format code
uv run ruff check .                    # Lint code

# Database
uv run alembic upgrade head            # Run migrations
uv run alembic downgrade -1           # Rollback migration
uv run alembic history                 # View migration history

# Docker
docker-compose up -d                   # Start services
docker-compose logs -f                 # View logs
docker-compose down                    # Stop services

# Deployment
git push origin main                   # Deploy to Railway
```

## Resources

- FastAPI Docs: https://fastapi.tiangolo.com
- SQLAlchemy Docs: https://docs.sqlalchemy.org
- UV Docs: https://github.com/astral-sh/uv
- Railway Docs: https://docs.railway.app

## Remember

**Start simple, iterate quickly.**

The goal is to get something working that you can use TODAY, not to build the perfect system. Every feature should help with ADHD challenges:

1. Quick capture reduces friction
2. State machine prevents decision paralysis
3. Public stats create accountability
4. Focus timer combats time blindness

Ship early, ship often, learn from real usage.

---
*Last updated: [Current Date]*
*Questions? Check CLAUDE_PYTHON.md for detailed guidance*
