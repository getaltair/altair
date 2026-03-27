package com.getaltair.altair.data.local.mapper

import com.getaltair.altair.data.local.entity.UserEntity
import com.getaltair.altair.domain.entity.User
import java.time.Instant

fun UserEntity.toDomain(): User = User(
    id = id,
    email = email,
    displayName = displayName,
    createdAt = Instant.ofEpochMilli(createdAt),
    updatedAt = Instant.ofEpochMilli(updatedAt),
)

fun User.toEntity(): UserEntity = UserEntity(
    id = id,
    email = email,
    displayName = displayName,
    createdAt = createdAt.toEpochMilli(),
    updatedAt = updatedAt.toEpochMilli(),
)
