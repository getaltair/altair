"""Main FastAPI application entry point."""

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from app.api import auth, health, users
from app.core.config import settings

app = FastAPI(
    title="Altair Auth Service",
    description="Authentication and user management service for Altair ecosystem",
    version="0.1.0",
    docs_url="/api/docs" if settings.ENVIRONMENT != "production" else None,
    redoc_url="/api/redoc" if settings.ENVIRONMENT != "production" else None,
)

# CORS configuration
app.add_middleware(
    CORSMiddleware,
    allow_origins=settings.CORS_ORIGINS,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Include routers
app.include_router(health.router, prefix="/api", tags=["health"])
app.include_router(auth.router, prefix="/api/auth", tags=["authentication"])
app.include_router(users.router, prefix="/api/users", tags=["users"])


@app.on_event("startup")
async def startup_event() -> None:
    """Initialize services on startup."""
    pass


@app.on_event("shutdown")
async def shutdown_event() -> None:
    """Cleanup on shutdown."""
    pass
