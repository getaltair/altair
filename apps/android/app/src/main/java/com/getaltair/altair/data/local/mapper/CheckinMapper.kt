package com.getaltair.altair.data.local.mapper

import com.getaltair.altair.data.local.entity.CheckinEntity
import com.getaltair.altair.domain.entity.DailyCheckin
import java.time.Instant
import java.time.LocalDate

fun CheckinEntity.toDomain(): DailyCheckin = DailyCheckin(
    id = id,
    userId = userId,
    date = LocalDate.parse(date),
    energyLevel = energyLevel,
    mood = mood,
    notes = notes,
    createdAt = Instant.ofEpochMilli(createdAt),
)

fun DailyCheckin.toEntity(): CheckinEntity = CheckinEntity(
    id = id,
    userId = userId,
    date = date.toString(),
    energyLevel = energyLevel,
    mood = mood,
    notes = notes,
    createdAt = createdAt.toEpochMilli(),
)
