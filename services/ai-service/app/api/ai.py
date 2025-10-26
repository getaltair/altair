"""AI endpoints for task assistance."""

import logging

from fastapi import APIRouter, HTTPException

from app.models.requests import (
    ContextSuggestionRequest,
    TaskBreakdownRequest,
    TaskPrioritizationRequest,
    TimeEstimateRequest,
)
from app.models.responses import (
    ContextSuggestionResponse,
    TaskBreakdownResponse,
    TaskPrioritizationResponse,
    TimeEstimateResponse,
)
from app.services.base import AIProvider
from app.services.factory import get_ai_provider

router = APIRouter()
logger = logging.getLogger(__name__)


def _get_provider_from_request(provider_name: str | None, api_key: str | None) -> AIProvider:
    """Get AI provider based on request parameters.

    Args:
        provider_name: Provider name from request (optional)
        api_key: API key from request (optional)

    Returns:
        Configured AI provider

    Raises:
        HTTPException: If provider cannot be initialized
    """
    try:
        # Cast to Literal type expected by factory (mypy requires this)
        from typing import Literal, cast

        typed_provider = cast(Literal["openai", "anthropic", "ollama"] | None, provider_name)
        return get_ai_provider(provider_name=typed_provider, api_key=api_key)
    except ValueError as e:
        logger.error(f"Failed to initialize AI provider: {e}")
        raise HTTPException(status_code=400, detail=str(e)) from e


@router.post("/breakdown", response_model=TaskBreakdownResponse)
async def breakdown_task(
    request: TaskBreakdownRequest,
) -> TaskBreakdownResponse:
    """Break down a task into subtasks.

    Args:
        request: Task breakdown request (includes optional provider and api_key)

    Returns:
        Task breakdown with suggested subtasks

    Raises:
        HTTPException: If AI call fails
    """
    # Get provider based on client request (or use server default)
    provider = _get_provider_from_request(request.provider, request.api_key)

    try:
        logger.info(f"Breaking down task: {request.task_title}")
        result = await provider.breakdown_task(request)
        logger.info(f"Generated {len(result.subtasks)} subtasks")
        return result
    except ValueError as e:
        logger.error(f"Validation error in breakdown: {e}")
        raise HTTPException(status_code=422, detail=str(e)) from e
    except Exception as e:
        logger.error(f"Error breaking down task: {e}")
        raise HTTPException(
            status_code=500,
            detail=f"Failed to break down task: {str(e)}",
        ) from e


@router.post("/prioritize", response_model=TaskPrioritizationResponse)
async def prioritize_tasks(
    request: TaskPrioritizationRequest,
) -> TaskPrioritizationResponse:
    """Get prioritization suggestions for tasks.

    Args:
        request: Task prioritization request (includes optional provider and api_key)

    Returns:
        Priority suggestions for tasks

    Raises:
        HTTPException: If AI call fails
    """
    # Get provider based on client request (or use server default)
    provider = _get_provider_from_request(request.provider, request.api_key)

    try:
        logger.info(f"Prioritizing {len(request.tasks)} tasks")
        result = await provider.prioritize_tasks(request)
        logger.info(f"Generated {len(result.suggestions)} priority suggestions")
        return result
    except ValueError as e:
        logger.error(f"Validation error in prioritization: {e}")
        raise HTTPException(status_code=422, detail=str(e)) from e
    except Exception as e:
        logger.error(f"Error prioritizing tasks: {e}")
        raise HTTPException(
            status_code=500,
            detail=f"Failed to prioritize tasks: {str(e)}",
        ) from e


@router.post("/estimate", response_model=TimeEstimateResponse)
async def estimate_time(
    request: TimeEstimateRequest,
) -> TimeEstimateResponse:
    """Estimate time required for a task.

    Args:
        request: Time estimate request (includes optional provider and api_key)

    Returns:
        Time estimates (optimistic, realistic, pessimistic)

    Raises:
        HTTPException: If AI call fails
    """
    # Get provider based on client request (or use server default)
    provider = _get_provider_from_request(request.provider, request.api_key)

    try:
        logger.info(f"Estimating time for task: {request.task_title}")
        result = await provider.estimate_time(request)
        logger.info(f"Estimated {result.estimate.realistic_minutes} minutes (realistic)")
        return result
    except ValueError as e:
        logger.error(f"Validation error in time estimation: {e}")
        raise HTTPException(status_code=422, detail=str(e)) from e
    except Exception as e:
        logger.error(f"Error estimating time: {e}")
        raise HTTPException(
            status_code=500,
            detail=f"Failed to estimate time: {str(e)}",
        ) from e


@router.post("/suggest", response_model=ContextSuggestionResponse)
async def suggest_context(
    request: ContextSuggestionRequest,
) -> ContextSuggestionResponse:
    """Get contextual suggestions for a task.

    Args:
        request: Context suggestion request (includes optional provider and api_key)

    Returns:
        Contextual suggestions (resources, tips, blockers)

    Raises:
        HTTPException: If AI call fails
    """
    # Get provider based on client request (or use server default)
    provider = _get_provider_from_request(request.provider, request.api_key)

    try:
        logger.info(f"Getting suggestions for task: {request.task_title}")
        result = await provider.suggest_context(request)
        logger.info(f"Generated {len(result.suggestions)} suggestions")
        return result
    except ValueError as e:
        logger.error(f"Validation error in context suggestions: {e}")
        raise HTTPException(status_code=422, detail=str(e)) from e
    except Exception as e:
        logger.error(f"Error getting suggestions: {e}")
        raise HTTPException(
            status_code=500,
            detail=f"Failed to get suggestions: {str(e)}",
        ) from e
