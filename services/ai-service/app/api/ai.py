"""AI endpoints for task assistance."""

import logging

from fastapi import APIRouter, HTTPException, Depends

from app.models.requests import (
    TaskBreakdownRequest,
    TaskPrioritizationRequest,
    TimeEstimateRequest,
    ContextSuggestionRequest,
)
from app.models.responses import (
    TaskBreakdownResponse,
    TaskPrioritizationResponse,
    TimeEstimateResponse,
    ContextSuggestionResponse,
)
from app.services.base import AIProvider
from app.services.factory import get_ai_provider

router = APIRouter()
logger = logging.getLogger(__name__)


def get_provider() -> AIProvider:
    """Dependency to get AI provider instance.

    Returns:
        Configured AI provider

    Raises:
        HTTPException: If provider cannot be initialized
    """
    try:
        return get_ai_provider()
    except ValueError as e:
        logger.error(f"Failed to initialize AI provider: {e}")
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/breakdown", response_model=TaskBreakdownResponse)
async def breakdown_task(
    request: TaskBreakdownRequest,
    provider: AIProvider = Depends(get_provider),
) -> TaskBreakdownResponse:
    """Break down a task into subtasks.

    Args:
        request: Task breakdown request
        provider: AI provider instance

    Returns:
        Task breakdown with suggested subtasks

    Raises:
        HTTPException: If AI call fails
    """
    try:
        logger.info(f"Breaking down task: {request.task_title}")
        result = await provider.breakdown_task(request)
        logger.info(f"Generated {len(result.subtasks)} subtasks")
        return result
    except ValueError as e:
        logger.error(f"Validation error in breakdown: {e}")
        raise HTTPException(status_code=422, detail=str(e))
    except Exception as e:
        logger.error(f"Error breaking down task: {e}")
        raise HTTPException(
            status_code=500,
            detail=f"Failed to break down task: {str(e)}",
        )


@router.post("/prioritize", response_model=TaskPrioritizationResponse)
async def prioritize_tasks(
    request: TaskPrioritizationRequest,
    provider: AIProvider = Depends(get_provider),
) -> TaskPrioritizationResponse:
    """Get prioritization suggestions for tasks.

    Args:
        request: Task prioritization request
        provider: AI provider instance

    Returns:
        Priority suggestions for tasks

    Raises:
        HTTPException: If AI call fails
    """
    try:
        logger.info(f"Prioritizing {len(request.tasks)} tasks")
        result = await provider.prioritize_tasks(request)
        logger.info(f"Generated {len(result.suggestions)} priority suggestions")
        return result
    except ValueError as e:
        logger.error(f"Validation error in prioritization: {e}")
        raise HTTPException(status_code=422, detail=str(e))
    except Exception as e:
        logger.error(f"Error prioritizing tasks: {e}")
        raise HTTPException(
            status_code=500,
            detail=f"Failed to prioritize tasks: {str(e)}",
        )


@router.post("/estimate", response_model=TimeEstimateResponse)
async def estimate_time(
    request: TimeEstimateRequest,
    provider: AIProvider = Depends(get_provider),
) -> TimeEstimateResponse:
    """Estimate time required for a task.

    Args:
        request: Time estimate request
        provider: AI provider instance

    Returns:
        Time estimates (optimistic, realistic, pessimistic)

    Raises:
        HTTPException: If AI call fails
    """
    try:
        logger.info(f"Estimating time for task: {request.task_title}")
        result = await provider.estimate_time(request)
        logger.info(
            f"Estimated {result.estimate.realistic_minutes} minutes (realistic)"
        )
        return result
    except ValueError as e:
        logger.error(f"Validation error in time estimation: {e}")
        raise HTTPException(status_code=422, detail=str(e))
    except Exception as e:
        logger.error(f"Error estimating time: {e}")
        raise HTTPException(
            status_code=500,
            detail=f"Failed to estimate time: {str(e)}",
        )


@router.post("/suggest", response_model=ContextSuggestionResponse)
async def suggest_context(
    request: ContextSuggestionRequest,
    provider: AIProvider = Depends(get_provider),
) -> ContextSuggestionResponse:
    """Get contextual suggestions for a task.

    Args:
        request: Context suggestion request
        provider: AI provider instance

    Returns:
        Contextual suggestions (resources, tips, blockers)

    Raises:
        HTTPException: If AI call fails
    """
    try:
        logger.info(f"Getting suggestions for task: {request.task_title}")
        result = await provider.suggest_context(request)
        logger.info(f"Generated {len(result.suggestions)} suggestions")
        return result
    except ValueError as e:
        logger.error(f"Validation error in context suggestions: {e}")
        raise HTTPException(status_code=422, detail=str(e))
    except Exception as e:
        logger.error(f"Error getting suggestions: {e}")
        raise HTTPException(
            status_code=500,
            detail=f"Failed to get suggestions: {str(e)}",
        )
