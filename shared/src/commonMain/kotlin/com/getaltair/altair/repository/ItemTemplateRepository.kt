package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.tracking.FieldDefinition
import com.getaltair.altair.domain.model.tracking.ItemTemplate
import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.flow.Flow

/**
 * Repository for ItemTemplate entities.
 *
 * ItemTemplates define common field structures for similar items
 * (e.g., "Book" template with author, ISBN, genre fields).
 */
interface ItemTemplateRepository : Repository<ItemTemplate, DomainError> {
    /**
     * Searches templates by name (case-insensitive partial match).
     *
     * @param query The search query
     * @return Either an error on failure, or matching templates
     */
    suspend fun searchByName(query: String): Either<DomainError, List<ItemTemplate>>

    /**
     * Gets a template with its field definitions.
     *
     * @param id The ULID of the template
     * @return Either an error on failure, or the template with its fields
     */
    suspend fun findWithFields(id: Ulid): Either<DomainError, TemplateWithFields>

    /**
     * Returns all templates with their field definitions.
     *
     * @return A Flow emitting templates with their fields
     */
    fun findAllWithFields(): Flow<List<TemplateWithFields>>

    /**
     * Adds a field definition to a template.
     *
     * @param templateId The ULID of the template
     * @param field The field definition to add
     * @return Either an error on failure, or the created field definition
     */
    suspend fun addField(
        templateId: Ulid,
        field: FieldDefinition,
    ): Either<DomainError, FieldDefinition>

    /**
     * Updates a field definition.
     *
     * @param field The field definition to update
     * @return Either an error on failure, or the updated field definition
     */
    suspend fun updateField(field: FieldDefinition): Either<DomainError, FieldDefinition>

    /**
     * Removes a field definition from a template.
     *
     * This will also remove associated CustomField values from items using this template.
     *
     * @param fieldId The ULID of the field definition
     * @return Either an error on failure, or Unit on success
     */
    suspend fun removeField(fieldId: Ulid): Either<DomainError, Unit>

    /**
     * Reorders field definitions within a template.
     *
     * @param templateId The ULID of the template
     * @param orderedFieldIds The field IDs in the desired order
     * @return Either an error on failure, or Unit on success
     */
    suspend fun reorderFields(
        templateId: Ulid,
        orderedFieldIds: List<Ulid>,
    ): Either<DomainError, Unit>

    /**
     * Counts items using a specific template.
     *
     * @param id The ULID of the template
     * @return Either an error on failure, or the count
     */
    suspend fun countItemsUsingTemplate(id: Ulid): Either<DomainError, Int>
}

/**
 * A template with its associated field definitions.
 */
data class TemplateWithFields(
    val template: ItemTemplate,
    val fields: List<FieldDefinition>,
)
