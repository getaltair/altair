# Altair Auth Service

FastAPI-based authentication and user management service for the Altair ecosystem.

## Features

- JWT-based authentication
- Secure password hashing (bcrypt)
- User registration and management
- Redis-based session management
- PostgreSQL database
- Type-safe with Pydantic v2

## Development

### Setup

```bash
# From the root of the monorepo
cd services/auth-service

# Install dependencies
uv sync

# Copy environment variables
cp .env.example .env

# Edit .env with your configuration
```

### Running

```bash
# Development server with auto-reload
uv run uvicorn app.main:app --reload --host 0.0.0.0 --port 8000

# Production server
uv run uvicorn app.main:app --host 0.0.0.0 --port 8000 --workers 4
```

### Testing

```bash
# Run tests
uv run pytest

# Run tests with coverage
uv run pytest --cov=app --cov-report=html

# Type checking
uv run mypy app
```

### API Documentation

When running in development mode, API documentation is available at:

- Swagger UI: <http://localhost:8000/api/docs>
- ReDoc: <http://localhost:8000/api/redoc>

## Endpoints

### Health

- `GET /api/health` - Health check

### Authentication

- `POST /api/auth/login` - User login
- `POST /api/auth/register` - User registration
- `POST /api/auth/refresh` - Refresh access token

### Users

- `GET /api/users/me` - Get current user
- `GET /api/users/{user_id}` - Get user by ID

## Database Migrations

```bash
# Create a new migration
uv run alembic revision --autogenerate -m "description"

# Apply migrations
uv run alembic upgrade head

# Rollback one migration
uv run alembic downgrade -1
```

## License

AGPL-3.0-or-later
