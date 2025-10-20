"""Health check endpoint."""

from fastapi import APIRouter

router = APIRouter()


@router.get("/health")
async def health_check() -> dict[str, str]:
    """Health check endpoint.

    Returns:
        Dict with status
    """
    return {"status": "ok", "service": "ai-service"}
