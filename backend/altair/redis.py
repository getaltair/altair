"""Redis connection management for caching and token blacklisting.

This module provides Redis client initialization and connection management
for features like token blacklisting, session storage, and caching.

Key components:
    - get_redis_client(): Returns a Redis client instance
    - close_redis(): Cleanup function for Redis connection
"""

import redis

from altair.config import settings

_redis_client: redis.Redis | None = None


def get_redis_client() -> redis.Redis:
    """Get or create a Redis client instance.

    Returns:
        redis.Redis: Redis client for cache operations
    """
    global _redis_client
    if _redis_client is None:
        _redis_client = redis.from_url(settings.REDIS_URL, decode_responses=True)
    return _redis_client


def close_redis() -> None:
    """Close the Redis connection if it exists."""
    global _redis_client
    if _redis_client is not None:
        _redis_client.close()
        _redis_client = None
