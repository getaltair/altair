@file:Suppress("detekt:MaxLineLength")

package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.tracking.FieldDefinition
import com.getaltair.altair.domain.model.tracking.ItemTemplate
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.FieldType
import com.getaltair.altair.repository.ItemTemplateRepository
import com.getaltair.altair.repository.TemplateWithFields
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.datetime.Instant
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive

class SurrealItemTemplateRepository(
    private val db: SurrealDbClient,
    private val userId: Ulid,
) : ItemTemplateRepository {
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    override suspend fun findById(id: Ulid): Either<DomainError, ItemTemplate> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM item_template:${'$'}id WHERE user_id = user:${'$'}userId AND deleted_at IS NONE",
                        mapOf("id" to id.value, "userId" to userId.value),
                    ).bind()
            parseTemplate(result) ?: raise(DomainError.NotFoundError("ItemTemplate", id.value))
        }

    override suspend fun save(entity: ItemTemplate): Either<DomainError, ItemTemplate> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
                db
                    .executeBind(
                        """
                        UPDATE item_template:${'$'}id SET
                            name = ${'$'}name,
                            description = ${'$'}description,
                            icon = ${'$'}icon,
                            updated_at = time::now()
                        WHERE user_id = user:${'$'}userId
                        """.trimIndent(),
                        mapOf(
                            "id" to entity.id.value,
                            "name" to entity.name,
                            "description" to entity.description,
                            "icon" to entity.icon,
                            "userId" to userId.value,
                        ),
                    ).bind()
            } else {
                db
                    .executeBind(
                        """
                        CREATE item_template:${'$'}id CONTENT {
                            user_id: user:${'$'}userId,
                            name: ${'$'}name,
                            description: ${'$'}description,
                            icon: ${'$'}icon
                        }
                        """.trimIndent(),
                        mapOf(
                            "id" to entity.id.value,
                            "userId" to userId.value,
                            "name" to entity.name,
                            "description" to entity.description,
                            "icon" to entity.icon,
                        ),
                    ).bind()
            }
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<DomainError, Unit> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE item_template:${'$'}id SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:${'$'}userId",
                    mapOf("id" to id.value, "userId" to userId.value),
                ).bind()
            // Also soft-delete associated field definitions
            db
                .executeBind(
                    "UPDATE field_definition SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:${'$'}userId AND template_id = item_template:${'$'}templateId",
                    mapOf("userId" to userId.value, "templateId" to id.value),
                ).bind()
        }

    override fun findAll(): Flow<List<ItemTemplate>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM item_template WHERE user_id = user:${'$'}userId AND deleted_at IS NONE ORDER BY name",
                    mapOf("userId" to userId.value),
                )
            emit(result.fold({ emptyList() }, { parseTemplates(it) }))
        }

    override suspend fun searchByName(query: String): Either<DomainError, List<ItemTemplate>> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM item_template WHERE user_id = user:${'$'}userId AND string::lowercase(name) CONTAINS string::lowercase(${'$'}query) AND deleted_at IS NONE",
                        mapOf("userId" to userId.value, "query" to query),
                    ).bind()
            parseTemplates(result)
        }

    override suspend fun findWithFields(id: Ulid): Either<DomainError, TemplateWithFields> =
        either {
            val template = findById(id).bind()
            val fieldsResult =
                db
                    .queryBind(
                        "SELECT * FROM field_definition WHERE user_id = user:${'$'}userId AND template_id = item_template:${'$'}templateId AND deleted_at IS NONE ORDER BY sort_order",
                        mapOf("userId" to userId.value, "templateId" to id.value),
                    ).bind()
            val fields = parseFieldDefinitions(fieldsResult)
            TemplateWithFields(template, fields)
        }

    override fun findAllWithFields(): Flow<List<TemplateWithFields>> =
        flow {
            val templatesResult =
                db.queryBind(
                    "SELECT * FROM item_template WHERE user_id = user:${'$'}userId AND deleted_at IS NONE ORDER BY name",
                    mapOf("userId" to userId.value),
                )
            val templates = templatesResult.fold({ emptyList() }, { parseTemplates(it) })

            val result =
                templates.map { template ->
                    val fieldsResult =
                        db.queryBind(
                            "SELECT * FROM field_definition WHERE user_id = user:${'$'}userId AND template_id = item_template:${'$'}templateId AND deleted_at IS NONE ORDER BY sort_order",
                            mapOf("userId" to userId.value, "templateId" to template.id.value),
                        )
                    val fields = fieldsResult.fold({ emptyList() }, { parseFieldDefinitions(it) })
                    TemplateWithFields(template, fields)
                }
            emit(result)
        }

    override suspend fun addField(
        templateId: Ulid,
        field: FieldDefinition,
    ): Either<DomainError, FieldDefinition> =
        either {
            findById(templateId).bind()
            db
                .executeBind(
                    """
                    CREATE field_definition:${'$'}id CONTENT {
                        user_id: user:${'$'}userId,
                        template_id: item_template:${'$'}templateId,
                        name: ${'$'}name,
                        field_type: ${'$'}fieldType,
                        is_required: ${'$'}isRequired,
                        default_value: ${'$'}defaultValue,
                        enum_options: ${'$'}enumOptions,
                        sort_order: ${'$'}sortOrder
                    }
                    """.trimIndent(),
                    mapOf(
                        "id" to field.id.value,
                        "userId" to userId.value,
                        "templateId" to templateId.value,
                        "name" to field.name,
                        "fieldType" to field.fieldType.name.lowercase(),
                        "isRequired" to field.isRequired,
                        "defaultValue" to field.defaultValue,
                        "enumOptions" to field.enumOptions,
                        "sortOrder" to field.sortOrder,
                    ),
                ).bind()
            findFieldById(field.id).bind()
        }

    override suspend fun updateField(field: FieldDefinition): Either<DomainError, FieldDefinition> =
        either {
            findFieldById(field.id).bind()
            db
                .executeBind(
                    """
                    UPDATE field_definition:${'$'}id SET
                        name = ${'$'}name,
                        field_type = ${'$'}fieldType,
                        is_required = ${'$'}isRequired,
                        default_value = ${'$'}defaultValue,
                        enum_options = ${'$'}enumOptions,
                        sort_order = ${'$'}sortOrder,
                        updated_at = time::now()
                    WHERE user_id = user:${'$'}userId
                    """.trimIndent(),
                    mapOf(
                        "id" to field.id.value,
                        "name" to field.name,
                        "fieldType" to field.fieldType.name.lowercase(),
                        "isRequired" to field.isRequired,
                        "defaultValue" to field.defaultValue,
                        "enumOptions" to field.enumOptions,
                        "sortOrder" to field.sortOrder,
                        "userId" to userId.value,
                    ),
                ).bind()
            findFieldById(field.id).bind()
        }

    override suspend fun removeField(fieldId: Ulid): Either<DomainError, Unit> =
        either {
            findFieldById(fieldId).bind()
            db
                .executeBind(
                    "UPDATE field_definition:${'$'}id SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:${'$'}userId",
                    mapOf("id" to fieldId.value, "userId" to userId.value),
                ).bind()
            // Also remove associated custom field values
            db
                .executeBind(
                    "UPDATE custom_field SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:${'$'}userId AND field_definition_id = field_definition:${'$'}fieldDefinitionId",
                    mapOf("userId" to userId.value, "fieldDefinitionId" to fieldId.value),
                ).bind()
        }

    override suspend fun reorderFields(
        templateId: Ulid,
        orderedFieldIds: List<Ulid>,
    ): Either<DomainError, Unit> =
        either {
            findById(templateId).bind()
            orderedFieldIds.forEachIndexed { index, fieldId ->
                db
                    .executeBind(
                        "UPDATE field_definition:${'$'}id SET sort_order = ${'$'}sortOrder, updated_at = time::now() WHERE user_id = user:${'$'}userId AND template_id = item_template:${'$'}templateId",
                        mapOf(
                            "id" to fieldId.value,
                            "sortOrder" to index,
                            "userId" to userId.value,
                            "templateId" to templateId.value,
                        ),
                    ).bind()
            }
        }

    override suspend fun countItemsUsingTemplate(id: Ulid): Either<DomainError, Int> =
        either {
            findById(id).bind()
            val result =
                db
                    .queryBind(
                        "SELECT count() FROM item WHERE user_id = user:${'$'}userId AND template_id = item_template:${'$'}templateId AND deleted_at IS NONE GROUP ALL",
                        mapOf("userId" to userId.value, "templateId" to id.value),
                    ).bind()
            parseCount(result)
        }

    private suspend fun findFieldById(id: Ulid): Either<DomainError, FieldDefinition> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM field_definition:${'$'}id WHERE user_id = user:${'$'}userId AND deleted_at IS NONE",
                        mapOf("id" to id.value, "userId" to userId.value),
                    ).bind()
            parseFieldDefinition(result) ?: raise(DomainError.NotFoundError("FieldDefinition", id.value))
        }

    private fun parseTemplate(result: String): ItemTemplate? =
        try {
            json
                .parseToJsonElement(result)
                .jsonArray
                .firstOrNull()
                ?.jsonObject
                ?.let { mapToTemplate(it) }
        } catch (e: Exception) {
            null
        }

    private fun parseTemplates(result: String): List<ItemTemplate> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToTemplate(it.jsonObject)
                } catch (e: Exception) {
                    null
                }
            }
        } catch (e: Exception) {
            emptyList()
        }

    private fun mapToTemplate(obj: kotlinx.serialization.json.JsonObject): ItemTemplate {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val userIdStr = obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        return ItemTemplate(
            id = Ulid(id),
            userId = Ulid(userIdStr),
            name = obj["name"]?.jsonPrimitive?.content ?: "",
            description = obj["description"]?.jsonPrimitive?.content,
            icon = obj["icon"]?.jsonPrimitive?.content,
            createdAt = parseInstant(obj["created_at"]?.jsonPrimitive?.content),
            updatedAt = parseInstant(obj["updated_at"]?.jsonPrimitive?.content),
            deletedAt = obj["deleted_at"]?.jsonPrimitive?.content?.let { parseInstant(it) },
        )
    }

    private fun parseFieldDefinition(result: String): FieldDefinition? =
        try {
            json
                .parseToJsonElement(result)
                .jsonArray
                .firstOrNull()
                ?.jsonObject
                ?.let { mapToFieldDefinition(it) }
        } catch (e: Exception) {
            null
        }

    private fun parseFieldDefinitions(result: String): List<FieldDefinition> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToFieldDefinition(it.jsonObject)
                } catch (e: Exception) {
                    null
                }
            }
        } catch (e: Exception) {
            emptyList()
        }

    private fun mapToFieldDefinition(obj: kotlinx.serialization.json.JsonObject): FieldDefinition {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val userIdStr = obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val templateIdStr = obj["template_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val enumOptions =
            obj["enum_options"]?.let { element ->
                if (element is JsonArray) {
                    element.mapNotNull { it.jsonPrimitive.content }
                } else {
                    null
                }
            }
        return FieldDefinition(
            id = Ulid(id),
            userId = Ulid(userIdStr),
            templateId = Ulid(templateIdStr),
            name = obj["name"]?.jsonPrimitive?.content ?: "",
            fieldType = FieldType.valueOf(obj["field_type"]?.jsonPrimitive?.content?.uppercase() ?: "TEXT"),
            isRequired = obj["is_required"]?.jsonPrimitive?.content?.toBoolean() ?: false,
            defaultValue = obj["default_value"]?.jsonPrimitive?.content,
            enumOptions = enumOptions,
            sortOrder = obj["sort_order"]?.jsonPrimitive?.content?.toIntOrNull() ?: 0,
            createdAt = parseInstant(obj["created_at"]?.jsonPrimitive?.content),
            updatedAt = parseInstant(obj["updated_at"]?.jsonPrimitive?.content),
            deletedAt = obj["deleted_at"]?.jsonPrimitive?.content?.let { parseInstant(it) },
        )
    }

    private fun parseCount(result: String): Int =
        try {
            json
                .parseToJsonElement(result)
                .jsonArray
                .firstOrNull()
                ?.jsonObject
                ?.get("count")
                ?.jsonPrimitive
                ?.content
                ?.toIntOrNull() ?: 0
        } catch (e: Exception) {
            0
        }

    private fun parseInstant(value: String?): Instant =
        value?.let {
            try {
                Instant.parse(it)
            } catch (e: Exception) {
                Instant.DISTANT_PAST
            }
        } ?: Instant.DISTANT_PAST
}
