package com.getaltair.altair.shared.domain.common

/**
 * Helper to get current time in milliseconds.
 *
 * This is a workaround for KMP Clock.System access issues.
 */
internal expect fun currentTimeMillis(): Long
