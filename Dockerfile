# Stage 1: Builder - Install dependencies with UV
FROM python:3.12-slim AS builder

# Install UV
COPY --from=ghcr.io/astral-sh/uv:latest /uv /usr/local/bin/uv

# Set working directory
WORKDIR /app

# Copy dependency files
COPY backend/pyproject.toml ./
COPY backend/uv.lock ./

# Install dependencies (production only, no dev group)
RUN uv sync --frozen --no-dev --no-install-project

# Stage 2: Runtime - Minimal image with only what's needed
FROM python:3.12-slim

# Create non-root user for security
RUN useradd -m -u 1000 altair

# Set working directory
WORKDIR /app

# Copy virtual environment from builder
COPY --from=builder --chown=altair:altair /app/.venv /app/.venv

# Copy application code
COPY --chown=altair:altair backend/altair ./altair
COPY --chown=altair:altair backend/alembic ./alembic
COPY --chown=altair:altair backend/alembic.ini ./alembic.ini

# Set environment variables
ENV PATH="/app/.venv/bin:$PATH" \
    PYTHONUNBUFFERED=1 \
    PYTHONDONTWRITEBYTECODE=1

# Switch to non-root user
USER altair

# Expose port (Railway will override with $PORT)
EXPOSE 8000

# Start command (Railway overrides via railway.json)
CMD ["uvicorn", "altair.main:app", "--host", "0.0.0.0", "--port", "8000"]
