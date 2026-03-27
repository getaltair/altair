package com.getaltair.altair.data.local.mapper

import com.getaltair.altair.data.local.entity.EntityRelationEntity
import com.getaltair.altair.domain.entity.EntityRelation
import java.time.Instant

fun EntityRelationEntity.toDomain(): EntityRelation = EntityRelation(
    id = id,
    fromEntityType = fromEntityType,
    fromEntityId = fromEntityId,
    toEntityType = toEntityType,
    toEntityId = toEntityId,
    relationType = relationType,
    sourceType = sourceType,
    status = status,
    confidence = confidence,
    evidenceJson = evidenceJson,
    householdId = householdId,
    initiativeId = initiativeId,
    ownerUserId = ownerUserId,
    createdByUserId = createdByUserId,
    updatedByUserId = updatedByUserId,
    createdByProcess = createdByProcess,
    createdAt = Instant.ofEpochMilli(createdAt),
    updatedAt = Instant.ofEpochMilli(updatedAt),
    lastConfirmedAt = lastConfirmedAt?.let { Instant.ofEpochMilli(it) },
)

fun EntityRelation.toEntity(): EntityRelationEntity = EntityRelationEntity(
    id = id,
    fromEntityType = fromEntityType,
    fromEntityId = fromEntityId,
    toEntityType = toEntityType,
    toEntityId = toEntityId,
    relationType = relationType,
    sourceType = sourceType,
    status = status,
    confidence = confidence,
    evidenceJson = evidenceJson,
    householdId = householdId,
    initiativeId = initiativeId,
    ownerUserId = ownerUserId,
    createdByUserId = createdByUserId,
    updatedByUserId = updatedByUserId,
    createdByProcess = createdByProcess,
    createdAt = createdAt.toEpochMilli(),
    updatedAt = updatedAt.toEpochMilli(),
    lastConfirmedAt = lastConfirmedAt?.toEpochMilli(),
)
