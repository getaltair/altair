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
                    .query<Any>(
                        "SELECT * FROM item_template:${id.value} WHERE user_id = user:${userId.value} AND deleted_at IS NONE",
                    ).bind()
            parseTemplate(result) ?: raise(DomainError.NotFoundError("ItemTemplate", id.value))
        }

    override suspend fun save(entity: ItemTemplate): Either<DomainError, ItemTemplate> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
                db
                    .execute(
                        """
                        UPDATE item_template:${entity.id.value} SET
                            name = '${entity.name.replace("'", "''")}',
                            description = ${entity.description?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            icon = ${entity.icon?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            updated_at = time::now()
                        WHERE user_id = user:${userId.value};
                        """.trimIndent(),
                    ).bind()
            } else {
                db
                    .execute(
                        """
                        CREATE item_template:${entity.id.value} CONTENT {
                            user_id: user:${userId.value},
                            name: '${entity.name.replace("'", "''")}',
                            description: ${entity.description?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            icon: ${entity.icon?.let { "'${it.replace("'", "''")}'" } ?: "NONE"}
                        };
                        """.trimIndent(),
                    ).bind()
            }
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<DomainError, Unit> =
        either {
            findById(id).bind()
            db
                .execute(
                    "UPDATE item_template:${id.value} SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:${userId.value};",
                ).bind()
            // Also soft-delete associated field definitions
            db
                .execute(
                    "UPDATE field_definition SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:${userId.value} AND template_id = item_template:${id.value};",
                ).bind()
        }

    override fun findAll(): Flow<List<ItemTemplate>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM item_template WHERE user_id = user:${userId.value} AND deleted_at IS NONE ORDER BY name",
                )
            emit(result.fold({ emptyList() }, { parseTemplates(it) }))
        }

    override suspend fun searchByName(query: String): Either<DomainError, List<ItemTemplate>> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT * FROM item_template WHERE user_id = user:${userId.value} AND string::lowercase(name) CONTAINS string::lowercase('${query.replace("'", "''")}') AND deleted_at IS NONE",
                    ).bind()
            parseTemplates(result)
        }

    override suspend fun findWithFields(id: Ulid): Either<DomainError, TemplateWithFields> =
        either {
            val template = findById(id).bind()
            val templateId = id.value
            val fieldsResult =
                db
                    .query<Any>(
                        "SELECT * FROM field_definition WHERE user_id = user:${userId.value} " +
                            "AND template_id = item_template:$templateId AND deleted_at IS NONE ORDER BY sort_order",
                    ).bind()
            val fields = parseFieldDefinitions(fieldsResult)
            TemplateWithFields(template, fields)
        }

    override fun findAllWithFields(): Flow<List<TemplateWithFields>> =
        flow {
            val templatesResult =
                db.query<Any>(
                    "SELECT * FROM item_template WHERE user_id = user:${userId.value} AND deleted_at IS NONE ORDER BY name",
                )
            val templates = templatesResult.fold({ emptyList() }, { parseTemplates(it) })

            val result =
                templates.map { template ->
                    val fieldsResult =
                        db.query<Any>(
                            "SELECT * FROM field_definition WHERE user_id = user:${userId.value} AND template_id = item_template:${template.id.value} AND deleted_at IS NONE ORDER BY sort_order",
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
            val enumOptionsValue =
                field.enumOptions?.let { options ->
                    "[${options.joinToString(", ") { "'${it.replace("'", "''")}'" }}]"
                } ?: "NONE"
            db
                .execute(
                    """
                    CREATE field_definition:${field.id.value} CONTENT {
                        user_id: user:${userId.value},
                        template_id: item_template:${templateId.value},
                        name: '${field.name.replace("'", "''")}',
                        field_type: '${field.fieldType.name.lowercase()}',
                        is_required: ${field.isRequired},
                        default_value: ${field.defaultValue?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                        enum_options: $enumOptionsValue,
                        sort_order: ${field.sortOrder}
                    };
                    """.trimIndent(),
                ).bind()
            findFieldById(field.id).bind()
        }

    override suspend fun updateField(field: FieldDefinition): Either<DomainError, FieldDefinition> =
        either {
            findFieldById(field.id).bind()
            val enumOptionsValue =
                field.enumOptions?.let { options ->
                    "[${options.joinToString(", ") { "'${it.replace("'", "''")}'" }}]"
                } ?: "NONE"
            db
                .execute(
                    """
                    UPDATE field_definition:${field.id.value} SET
                        name = '${field.name.replace("'", "''")}',
                        field_type = '${field.fieldType.name.lowercase()}',
                        is_required = ${field.isRequired},
                        default_value = ${field.defaultValue?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                        enum_options = $enumOptionsValue,
                        sort_order = ${field.sortOrder},
                        updated_at = time::now()
                    WHERE user_id = user:${userId.value};
                    """.trimIndent(),
                ).bind()
            findFieldById(field.id).bind()
        }

    override suspend fun removeField(fieldId: Ulid): Either<DomainError, Unit> =
        either {
            findFieldById(fieldId).bind()
            db
                .execute(
                    "UPDATE field_definition:${fieldId.value} SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:${userId.value};",
                ).bind()
            // Also remove associated custom field values
            db
                .execute(
                    "UPDATE custom_field SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:${userId.value} AND field_definition_id = field_definition:${fieldId.value};",
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
                    .execute(
                        "UPDATE field_definition:${fieldId.value} SET sort_order = $index, updated_at = time::now() WHERE user_id = user:${userId.value} AND template_id = item_template:${templateId.value};",
                    ).bind()
            }
        }

    override suspend fun countItemsUsingTemplate(id: Ulid): Either<DomainError, Int> =
        either {
            findById(id).bind()
            val result =
                db
                    .query<Any>(
                        "SELECT count() FROM item WHERE user_id = user:${userId.value} AND template_id = item_template:${id.value} AND deleted_at IS NONE GROUP ALL",
                    ).bind()
            parseCount(result)
        }

    private suspend fun findFieldById(id: Ulid): Either<DomainError, FieldDefinition> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT * FROM field_definition:${id.value} WHERE user_id = user:${userId.value} AND deleted_at IS NONE",
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
