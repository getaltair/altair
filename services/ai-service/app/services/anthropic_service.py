"""Anthropic (Claude) service implementation."""

import json
import logging
from typing import Any

import anthropic
from tenacity import retry, stop_after_attempt, wait_exponential

from app.core.config import settings
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
    SubtaskSuggestion,
    PrioritySuggestion,
    PriorityLevel,
    TimeEstimate,
    ContextSuggestion,
)
from app.services.base import AIProvider

logger = logging.getLogger(__name__)


class AnthropicService(AIProvider):
    """Anthropic Claude-based AI provider."""

    def __init__(self) -> None:
        """Initialize Anthropic service."""
        self.client = anthropic.AsyncAnthropic(api_key=settings.ANTHROPIC_API_KEY)
        self.model = settings.ANTHROPIC_MODEL
        self.max_tokens = settings.ANTHROPIC_MAX_TOKENS
        self.temperature = settings.ANTHROPIC_TEMPERATURE

    @retry(
        stop=stop_after_attempt(settings.AI_MAX_RETRIES),
        wait=wait_exponential(multiplier=settings.AI_RETRY_DELAY, min=1, max=10),
    )
    async def _call_anthropic(self, system: str, user_message: str) -> str:
        """Call Anthropic API with retry logic.

        Args:
            system: System message
            user_message: User message

        Returns:
            Response content from Anthropic

        Raises:
            Exception: If all retries fail
        """
        try:
            message = await self.client.messages.create(
                model=self.model,
                max_tokens=self.max_tokens,
                temperature=self.temperature,
                system=system,
                messages=[{"role": "user", "content": user_message}],
            )

            # Extract text content from the response
            if not message.content:
                raise ValueError("Anthropic returned empty response")

            content = message.content[0]
            if hasattr(content, 'text'):
                return content.text
            else:
                raise ValueError("Anthropic response missing text content")

        except Exception as e:
            logger.error(f"Anthropic API error: {e}")
            raise

    def _parse_json_response(self, response: str) -> dict[str, Any]:
        """Parse JSON from AI response, handling markdown code blocks.

        Args:
            response: Raw response from AI

        Returns:
            Parsed JSON object

        Raises:
            ValueError: If JSON parsing fails
        """
        # Remove markdown code blocks if present
        response = response.strip()
        if response.startswith("```json"):
            response = response[7:]
        if response.startswith("```"):
            response = response[3:]
        if response.endswith("```"):
            response = response[:-3]

        try:
            return json.loads(response.strip())  # type: ignore
        except json.JSONDecodeError as e:
            logger.error(f"Failed to parse JSON: {e}\nResponse: {response}")
            raise ValueError(f"Invalid JSON response from AI: {e}")

    async def breakdown_task(self, request: TaskBreakdownRequest) -> TaskBreakdownResponse:
        """Break down a task into subtasks using Claude."""
        context_str = f"\n\nProject Context: {request.context}" if request.context else ""
        desc_str = f"\nDescription: {request.task_description}" if request.task_description else ""

        system = (
            "You are an ADHD-friendly task breakdown assistant. Break down tasks into "
            "clear, actionable subtasks. Each subtask should be completable in one sitting. "
            "Respond with valid JSON only, no markdown."
        )

        user_message = (
            f"Task: {request.task_title}{desc_str}{context_str}\n\n"
            f"Break this down into {request.max_subtasks} or fewer clear, actionable subtasks. "
            "Respond in this exact JSON format:\n"
            "{\n"
            '  "subtasks": [\n'
            '    {"title": "...", "description": "...", "estimated_minutes": 30, "order": 1},\n'
            '    {"title": "...", "description": "...", "estimated_minutes": 45, "order": 2}\n'
            "  ],\n"
            '  "reasoning": "Why this breakdown makes sense"\n'
            "}"
        )

        response = await self._call_anthropic(system, user_message)
        data = self._parse_json_response(response)

        subtasks = [
            SubtaskSuggestion(
                title=st["title"],
                description=st.get("description"),
                estimated_minutes=st.get("estimated_minutes"),
                order=st.get("order", idx + 1),
            )
            for idx, st in enumerate(data["subtasks"])
        ]

        total_time = sum(st.estimated_minutes or 0 for st in subtasks) if subtasks else None

        return TaskBreakdownResponse(
            original_task=request.task_title,
            subtasks=subtasks,
            total_estimated_minutes=total_time or None,
            reasoning=data.get("reasoning"),
        )

    async def prioritize_tasks(
        self, request: TaskPrioritizationRequest
    ) -> TaskPrioritizationResponse:
        """Suggest prioritization for tasks using Claude."""
        tasks_str = "\n".join(
            [f"{i+1}. {task['title']}: {task.get('description', 'No description')}"
             for i, task in enumerate(request.tasks)]
        )
        context_str = f"\n\nProject Context: {request.context}" if request.context else ""

        system = (
            "You are a task prioritization expert. Analyze tasks and suggest priorities "
            "based on urgency, impact, and dependencies. Respond with valid JSON only."
        )

        user_message = (
            f"Tasks:\n{tasks_str}{context_str}\n\n"
            "Prioritize these tasks. Respond in this exact JSON format:\n"
            "{\n"
            '  "suggestions": [\n'
            '    {"task_title": "...", "priority": "high", "reasoning": "...", '
            '"urgency_score": 0.8, "impact_score": 0.9}\n'
            "  ],\n"
            '  "recommended_order": ["task title 1", "task title 2"]\n'
            "}\n"
            "Priority levels: critical, high, medium, low"
        )

        response = await self._call_anthropic(system, user_message)
        data = self._parse_json_response(response)

        suggestions = [
            PrioritySuggestion(
                task_title=sug["task_title"],
                priority=PriorityLevel(sug["priority"]),
                reasoning=sug["reasoning"],
                urgency_score=sug["urgency_score"],
                impact_score=sug["impact_score"],
            )
            for sug in data["suggestions"]
        ]

        return TaskPrioritizationResponse(
            suggestions=suggestions,
            recommended_order=data["recommended_order"],
        )

    async def estimate_time(self, request: TimeEstimateRequest) -> TimeEstimateResponse:
        """Estimate time required for a task using Claude."""
        desc_str = f"\nDescription: {request.task_description}" if request.task_description else ""
        subtasks_str = (
            f"\nSubtasks:\n" + "\n".join(f"- {st}" for st in request.subtasks)
            if request.subtasks
            else ""
        )

        system = (
            "You are a time estimation expert. Provide realistic time estimates "
            "considering the user's skill level. Respond with valid JSON only."
        )

        user_message = (
            f"Task: {request.task_title}{desc_str}{subtasks_str}\n"
            f"Skill Level: {request.skill_level}\n\n"
            "Estimate the time required. Respond in this exact JSON format:\n"
            "{\n"
            '  "optimistic_minutes": 60,\n'
            '  "realistic_minutes": 90,\n'
            '  "pessimistic_minutes": 120,\n'
            '  "confidence": 0.75,\n'
            '  "factors": ["factor 1", "factor 2"],\n'
            '  "assumptions": ["assumption 1", "assumption 2"]\n'
            "}"
        )

        response = await self._call_anthropic(system, user_message)
        data = self._parse_json_response(response)

        estimate = TimeEstimate(
            optimistic_minutes=data["optimistic_minutes"],
            realistic_minutes=data["realistic_minutes"],
            pessimistic_minutes=data["pessimistic_minutes"],
            confidence=data["confidence"],
        )

        return TimeEstimateResponse(
            task_title=request.task_title,
            estimate=estimate,
            factors=data["factors"],
            assumptions=data["assumptions"],
        )

    async def suggest_context(
        self, request: ContextSuggestionRequest
    ) -> ContextSuggestionResponse:
        """Provide contextual suggestions for a task using Claude."""
        desc_str = f"\nDescription: {request.task_description}" if request.task_description else ""
        context_str = (
            f"\nProject Context: {request.project_context}" if request.project_context else ""
        )

        system = (
            "You are a helpful assistant providing contextual suggestions for tasks. "
            "Suggest resources, tips, potential blockers, or other helpful information. "
            "Respond with valid JSON only."
        )

        user_message = (
            f"Task: {request.task_title}{desc_str}{context_str}\n"
            f"Suggestion Type: {request.suggestion_type}\n\n"
            "Provide helpful contextual suggestions. Respond in this exact JSON format:\n"
            "{\n"
            '  "suggestions": [\n'
            '    {"title": "...", "description": "...", "category": "resource", '
            '"priority": "medium"}\n'
            "  ],\n"
            '  "summary": "Overall summary of suggestions"\n'
            "}\n"
            "Categories: resource, tip, blocker, tool, learning\n"
            "Priorities: critical, high, medium, low"
        )

        response = await self._call_anthropic(system, user_message)
        data = self._parse_json_response(response)

        suggestions = [
            ContextSuggestion(
                title=sug["title"],
                description=sug["description"],
                category=sug["category"],
                priority=PriorityLevel(sug["priority"]) if "priority" in sug else None,
            )
            for sug in data["suggestions"]
        ]

        return ContextSuggestionResponse(
            task_title=request.task_title,
            suggestions=suggestions,
            summary=data.get("summary"),
        )
