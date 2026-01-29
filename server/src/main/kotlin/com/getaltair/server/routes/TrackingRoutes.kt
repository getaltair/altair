@file:Suppress("DEPRECATION")

package com.getaltair.server.routes

import arrow.core.getOrElse
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.tracking.Container
import com.getaltair.altair.shared.domain.tracking.FieldDefinition
import com.getaltair.altair.shared.domain.tracking.Item
import com.getaltair.altair.shared.domain.tracking.ItemTemplate
import com.getaltair.altair.shared.domain.tracking.Location
import com.getaltair.altair.shared.dto.tracking.*
import com.getaltair.altair.shared.repository.ItemRepository
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.auth.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import kotlinx.datetime.Instant

/**
 * Configures Tracking module routes for items, locations, containers, and templates.
 *
 * All routes require JWT authentication.
 * Handles inventory management, hierarchical organization, and low-stock alerting.
 */
fun Route.trackingRoutes(itemRepository: ItemRepository) {
    route("/api/tracking") {
        authenticate("jwt") {
            // === ITEMS ===
            route("/items") {
                get {
                    val userId = call.userId
                    val locationId = call.request.queryParameters["locationId"]
                    val containerId = call.request.queryParameters["containerId"]
                    val templateId = call.request.queryParameters["templateId"]
                    val search = call.request.queryParameters["search"]

                    val result = when {
                        locationId != null -> {
                            val locId = Ulid.parse(locationId)
                            if (locId == null) {
                                call.respond(HttpStatusCode.BadRequest, "Invalid locationId")
                                return@get
                            }
                            itemRepository.getByLocation(locId)
                        }
                        containerId != null -> {
                            val contId = Ulid.parse(containerId)
                            if (contId == null) {
                                call.respond(HttpStatusCode.BadRequest, "Invalid containerId")
                                return@get
                            }
                            itemRepository.getByContainer(contId)
                        }
                        templateId != null -> {
                            val tempId = Ulid.parse(templateId)
                            if (tempId == null) {
                                call.respond(HttpStatusCode.BadRequest, "Invalid templateId")
                                return@get
                            }
                            itemRepository.getByTemplate(tempId)
                        }
                        search != null -> itemRepository.search(userId, search)
                        else -> itemRepository.getAllForUser(userId)
                    }

                    result.fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { items -> call.respond(items.map { it.toSummaryResponse() }) }
                    )
                }

                post {
                    val userId = call.userId
                    val request = call.receive<CreateItemRequest>()

                    val item = Item(
                        id = Ulid.generate(),
                        userId = userId,
                        name = request.name,
                        description = request.description,
                        quantity = request.quantity,
                        templateId = request.templateId?.let { Ulid.parse(it) },
                        locationId = request.locationId?.let { Ulid.parse(it) },
                        containerId = request.containerId?.let { Ulid.parse(it) },
                        initiativeId = request.initiativeId?.let { Ulid.parse(it) },
                        imageKey = null,
                        createdAt = Instant.fromEpochMilliseconds(System.currentTimeMillis()),
                        updatedAt = Instant.fromEpochMilliseconds(System.currentTimeMillis()),
                        deletedAt = null
                    )

                    itemRepository.create(item).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { created -> call.respond(HttpStatusCode.Created, created.toResponse()) }
                    )
                }

                get("/{id}") {
                    call.pathParamAsUlid("id") { id ->
                        itemRepository.getById(id).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { item -> call.respond(item.toResponse()) }
                        )
                    }
                }

                put("/{id}") {
                    call.pathParamAsUlid("id") { id ->
                        val request = call.receive<UpdateItemRequest>()

                        itemRepository.getById(id).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { existing ->
                                val newLocationId = request.locationId
                                val newContainerId = request.containerId

                                val updated = existing.copy(
                                    name = request.name ?: existing.name,
                                    description = request.description ?: existing.description,
                                    quantity = request.quantity ?: existing.quantity,
                                    locationId = if (newLocationId != null)
                                        Ulid.parse(newLocationId) else existing.locationId,
                                    containerId = if (newContainerId != null)
                                        Ulid.parse(newContainerId) else existing.containerId,
                                    updatedAt = Instant.fromEpochMilliseconds(System.currentTimeMillis())
                                )

                                itemRepository.update(updated).fold(
                                    ifLeft = { error -> call.respondError(error) },
                                    ifRight = { item -> call.respond(item.toResponse()) }
                                )
                            }
                        )
                    }
                }

                delete("/{id}") {
                    call.pathParamAsUlid("id") { id ->
                        itemRepository.softDelete(id).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { call.respond(HttpStatusCode.NoContent) }
                        )
                    }
                }

                put("/{id}/quantity") {
                    call.pathParamAsUlid("id") { id ->
                        val quantity = call.request.queryParameters["quantity"]?.toIntOrNull()
                        if (quantity == null) {
                            call.respond(HttpStatusCode.BadRequest, "quantity parameter required")
                            return@pathParamAsUlid
                        }

                        itemRepository.updateQuantity(id, quantity).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { item -> call.respond(item.toResponse()) }
                        )
                    }
                }

                put("/{id}/move") {
                    call.pathParamAsUlid("id") { id ->
                        val request = call.receive<MoveItemRequest>()
                        val locationId = request.locationId?.let { Ulid.parse(it) }
                        val containerId = request.containerId?.let { Ulid.parse(it) }

                        itemRepository.move(id, locationId, containerId).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { item -> call.respond(item.toResponse()) }
                        )
                    }
                }

                get("/low-stock") {
                    val userId = call.userId
                    val threshold = call.request.queryParameters["threshold"]?.toIntOrNull() ?: 5

                    itemRepository.getLowStock(userId, threshold).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { items ->
                            call.respond(LowStockResponse(
                                items = items.map { it.toSummaryResponse() },
                                threshold = threshold,
                                totalCount = items.size
                            ))
                        }
                    )
                }
            }

            // === LOCATIONS ===
            route("/locations") {
                get {
                    val userId = call.userId
                    itemRepository.getLocations(userId).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { locations -> call.respond(locations.map { it.toResponse() }) }
                    )
                }

                post {
                    val userId = call.userId
                    val request = call.receive<CreateLocationRequest>()

                    val location = Location(
                        id = Ulid.generate(),
                        userId = userId,
                        name = request.name,
                        description = request.description,
                        parentId = request.parentId?.let { Ulid.parse(it) },
                        createdAt = Instant.fromEpochMilliseconds(System.currentTimeMillis())
                    )

                    itemRepository.createLocation(location).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { created -> call.respond(HttpStatusCode.Created, created.toResponse()) }
                    )
                }

                put("/{id}") {
                    call.pathParamAsUlid("id") { id ->
                        val request = call.receive<UpdateLocationRequest>()

                        // Fetch existing location
                        itemRepository.getLocations(call.userId).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { locations ->
                                val existing = locations.find { it.id == id }
                                if (existing == null) {
                                    call.respond(HttpStatusCode.NotFound, "Location not found")
                                    return@pathParamAsUlid
                                }

                                val newParentId = request.parentId

                                val updated = existing.copy(
                                    name = request.name ?: existing.name,
                                    description = request.description ?: existing.description,
                                    parentId = if (newParentId != null)
                                        Ulid.parse(newParentId) else existing.parentId
                                )

                                itemRepository.updateLocation(updated).fold(
                                    ifLeft = { error -> call.respondError(error) },
                                    ifRight = { loc -> call.respond(loc.toResponse()) }
                                )
                            }
                        )
                    }
                }

                delete("/{id}") {
                    call.pathParamAsUlid("id") { id ->
                        itemRepository.deleteLocation(id).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { call.respond(HttpStatusCode.NoContent) }
                        )
                    }
                }
            }

            // === CONTAINERS ===
            route("/containers") {
                get {
                    val userId = call.userId
                    itemRepository.getContainers(userId).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { containers -> call.respond(containers.map { it.toResponse() }) }
                    )
                }

                post {
                    val userId = call.userId
                    val request = call.receive<CreateContainerRequest>()

                    val container = Container(
                        id = Ulid.generate(),
                        userId = userId,
                        name = request.name,
                        description = request.description,
                        locationId = request.locationId?.let { Ulid.parse(it) },
                        parentId = request.parentId?.let { Ulid.parse(it) },
                        createdAt = Instant.fromEpochMilliseconds(System.currentTimeMillis())
                    )

                    itemRepository.createContainer(container).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { created -> call.respond(HttpStatusCode.Created, created.toResponse()) }
                    )
                }

                put("/{id}") {
                    call.pathParamAsUlid("id") { id ->
                        val request = call.receive<UpdateContainerRequest>()

                        itemRepository.getContainers(call.userId).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { containers ->
                                val existing = containers.find { it.id == id }
                                if (existing == null) {
                                    call.respond(HttpStatusCode.NotFound, "Container not found")
                                    return@pathParamAsUlid
                                }

                                val newLocationId = request.locationId

                                val updated = existing.copy(
                                    name = request.name ?: existing.name,
                                    description = request.description ?: existing.description,
                                    locationId = if (newLocationId != null)
                                        Ulid.parse(newLocationId) else existing.locationId
                                )

                                itemRepository.updateContainer(updated).fold(
                                    ifLeft = { error -> call.respondError(error) },
                                    ifRight = { cont -> call.respond(cont.toResponse()) }
                                )
                            }
                        )
                    }
                }

                put("/{id}/move") {
                    call.pathParamAsUlid("id") { id ->
                        val locationId = call.request.queryParameters["locationId"]
                            ?.let { Ulid.parse(it) }

                        itemRepository.moveContainer(id, locationId).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { container -> call.respond(container.toResponse()) }
                        )
                    }
                }

                delete("/{id}") {
                    call.pathParamAsUlid("id") { id ->
                        itemRepository.deleteContainer(id).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { call.respond(HttpStatusCode.NoContent) }
                        )
                    }
                }
            }

            // === TEMPLATES ===
            route("/templates") {
                get {
                    val userId = call.userId
                    itemRepository.getTemplates(userId).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { templates -> call.respond(templates.map { it.toResponse() }) }
                    )
                }

                post {
                    val userId = call.userId
                    val request = call.receive<CreateTemplateRequest>()

                    val template = ItemTemplate(
                        id = Ulid.generate(),
                        userId = userId,
                        name = request.name,
                        description = request.description,
                        icon = request.icon,
                        createdAt = Instant.fromEpochMilliseconds(System.currentTimeMillis())
                    )

                    val fields = request.fields.mapIndexed { index, field ->
                        FieldDefinition(
                            id = Ulid.generate(),
                            templateId = template.id,
                            name = field.name,
                            fieldType = field.fieldType,
                            required = field.required,
                            defaultValue = field.defaultValue,
                            enumOptions = field.enumOptions,
                            order = index
                        )
                    }

                    itemRepository.createTemplate(template, fields).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { created -> call.respond(HttpStatusCode.Created, created.toResponse()) }
                    )
                }

                delete("/{id}") {
                    call.pathParamAsUlid("id") { id ->
                        itemRepository.deleteTemplate(id).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { call.respond(HttpStatusCode.NoContent) }
                        )
                    }
                }
            }
        }
    }
}

// === DTO Mappers ===

/**
 * Converts Item domain model to ItemResponse DTO.
 */
private fun Item.toResponse(): ItemResponse = ItemResponse(
    id = id.toString(),
    name = name,
    description = description,
    quantity = quantity,
    templateId = templateId?.toString(),
    templateName = null, // TODO: Resolve template name from repository
    locationId = locationId?.toString(),
    locationPath = null, // TODO: Build hierarchical path
    containerId = containerId?.toString(),
    containerPath = null, // TODO: Build hierarchical path
    initiativeId = initiativeId?.toString(),
    imageUrl = imageKey?.let { "/api/files/$it" },
    customFields = emptyList(), // TODO: Fetch custom fields
    createdAt = createdAt.toString(),
    updatedAt = updatedAt.toString()
)

/**
 * Converts Item domain model to ItemSummaryResponse DTO.
 */
private fun Item.toSummaryResponse(): ItemSummaryResponse = ItemSummaryResponse(
    id = id.toString(),
    name = name,
    quantity = quantity,
    locationPath = null, // TODO: Build hierarchical path
    containerPath = null, // TODO: Build hierarchical path
    imageUrl = imageKey?.let { "/api/files/$it" }
)

/**
 * Converts Location domain model to LocationResponse DTO.
 */
private fun Location.toResponse(): LocationResponse = LocationResponse(
    id = id.toString(),
    name = name,
    description = description,
    parentId = parentId?.toString(),
    path = name, // TODO: Build hierarchical path
    itemCount = 0, // TODO: Query item count
    childCount = 0, // TODO: Query child count
    children = null
)

/**
 * Converts Container domain model to ContainerResponse DTO.
 */
private fun Container.toResponse(): ContainerResponse = ContainerResponse(
    id = id.toString(),
    name = name,
    description = description,
    locationId = locationId?.toString(),
    locationPath = null, // TODO: Build hierarchical path
    parentId = parentId?.toString(),
    itemCount = 0, // TODO: Query item count
    childCount = 0 // TODO: Query child count
)

/**
 * Converts ItemTemplate domain model to ItemTemplateResponse DTO.
 */
private fun ItemTemplate.toResponse(): ItemTemplateResponse = ItemTemplateResponse(
    id = id.toString(),
    name = name,
    description = description,
    icon = icon,
    fields = emptyList(), // TODO: Fetch field definitions
    itemCount = 0 // TODO: Query item count
)
