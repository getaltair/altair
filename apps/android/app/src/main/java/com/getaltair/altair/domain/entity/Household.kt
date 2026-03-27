package com.getaltair.altair.domain.entity

import java.time.Instant
import java.util.UUID

data class Household(
    val id: UUID,
    val name: String,
    val createdBy: UUID,
    val createdAt: Instant,
)
