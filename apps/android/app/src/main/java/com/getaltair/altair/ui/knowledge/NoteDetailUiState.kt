package com.getaltair.altair.ui.knowledge

import com.getaltair.altair.data.local.entity.EntityRelationEntity
import com.getaltair.altair.domain.entity.KnowledgeNote

data class NoteDetailUiState(
    val note: KnowledgeNote? = null,
    val relations: List<EntityRelationEntity> = emptyList(),
    val isLoading: Boolean = true,
    val showDeleteConfirm: Boolean = false,
    val isDeleted: Boolean = false,
)
