package com.getaltair.altair.data.local.mapper

import com.getaltair.altair.data.local.entity.HouseholdEntity
import com.getaltair.altair.domain.entity.Household
import java.time.Instant

fun HouseholdEntity.toDomain(): Household = Household(
    id = id,
    name = name,
    createdBy = createdBy,
    createdAt = Instant.ofEpochMilli(createdAt),
)

fun Household.toEntity(): HouseholdEntity = HouseholdEntity(
    id = id,
    name = name,
    createdBy = createdBy,
    createdAt = createdAt.toEpochMilli(),
)
