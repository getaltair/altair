package com.getaltair.altair.api

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.dto.tracking.*
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.http.*

/**
 * Client API for Tracking module operations.
 *
 * Provides access to Items, Locations, Containers, and Templates endpoints.
 * All operations require authentication via JWT token in httpClient.
 */
class TrackingApi(private val httpClient: HttpClient) {
    private val baseUrl = "/api/tracking"

    // === ITEMS ===

    /**
     * Retrieves all items for the authenticated user.
     *
     * @param locationId Optional filter by location
     * @param containerId Optional filter by container
     * @param templateId Optional filter by template
     * @param search Optional full-text search query
     * @return Success with list of items, or Failure
     */
    suspend fun getItems(
        locationId: String? = null,
        containerId: String? = null,
        templateId: String? = null,
        search: String? = null
    ): Either<AltairError, List<ItemSummaryResponse>> = try {
        val response = httpClient.get("$baseUrl/items") {
            locationId?.let { parameter("locationId", it) }
            containerId?.let { parameter("containerId", it) }
            templateId?.let { parameter("templateId", it) }
            search?.let { parameter("search", it) }
        }

        if (response.status.isSuccess()) {
            response.body<List<ItemSummaryResponse>>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Creates a new item.
     *
     * @param request Item creation request
     * @return Success with created item, or Failure
     */
    suspend fun createItem(request: CreateItemRequest): Either<AltairError, ItemResponse> = try {
        val response = httpClient.post("$baseUrl/items") {
            contentType(ContentType.Application.Json)
            setBody(request)
        }

        if (response.status.isSuccess()) {
            response.body<ItemResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Retrieves a specific item by ID.
     *
     * @param id Item identifier
     * @return Success with item details, or Failure
     */
    suspend fun getItem(id: String): Either<AltairError, ItemResponse> = try {
        val response = httpClient.get("$baseUrl/items/$id")

        if (response.status.isSuccess()) {
            response.body<ItemResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Updates an existing item.
     *
     * @param id Item identifier
     * @param request Update request with fields to modify
     * @return Success with updated item, or Failure
     */
    suspend fun updateItem(id: String, request: UpdateItemRequest): Either<AltairError, ItemResponse> = try {
        val response = httpClient.put("$baseUrl/items/$id") {
            contentType(ContentType.Application.Json)
            setBody(request)
        }

        if (response.status.isSuccess()) {
            response.body<ItemResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Soft-deletes an item.
     *
     * @param id Item identifier
     * @return Success on deletion, or Failure
     */
    suspend fun deleteItem(id: String): Either<AltairError, Unit> = try {
        val response = httpClient.delete("$baseUrl/items/$id")

        if (response.status.isSuccess()) {
            Unit.right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Updates item quantity.
     *
     * @param id Item identifier
     * @param quantity New quantity value
     * @return Success with updated item, or Failure
     */
    suspend fun updateQuantity(id: String, quantity: Int): Either<AltairError, ItemResponse> = try {
        val response = httpClient.put("$baseUrl/items/$id/quantity") {
            parameter("quantity", quantity)
        }

        if (response.status.isSuccess()) {
            response.body<ItemResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Moves an item to a new location and/or container.
     *
     * @param id Item identifier
     * @param request Move request with target location/container
     * @return Success with updated item, or Failure
     */
    suspend fun moveItem(id: String, request: MoveItemRequest): Either<AltairError, ItemResponse> = try {
        val response = httpClient.put("$baseUrl/items/$id/move") {
            contentType(ContentType.Application.Json)
            setBody(request)
        }

        if (response.status.isSuccess()) {
            response.body<ItemResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Retrieves items with quantity at or below threshold.
     *
     * @param threshold Quantity threshold (default: 5)
     * @return Success with low stock response, or Failure
     */
    suspend fun getLowStock(threshold: Int = 5): Either<AltairError, LowStockResponse> = try {
        val response = httpClient.get("$baseUrl/items/low-stock") {
            parameter("threshold", threshold)
        }

        if (response.status.isSuccess()) {
            response.body<LowStockResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    // === LOCATIONS ===

    /**
     * Retrieves all locations for the authenticated user.
     *
     * @return Success with list of locations, or Failure
     */
    suspend fun getLocations(): Either<AltairError, List<LocationResponse>> = try {
        val response = httpClient.get("$baseUrl/locations")

        if (response.status.isSuccess()) {
            response.body<List<LocationResponse>>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Creates a new location.
     *
     * @param request Location creation request
     * @return Success with created location, or Failure
     */
    suspend fun createLocation(request: CreateLocationRequest): Either<AltairError, LocationResponse> = try {
        val response = httpClient.post("$baseUrl/locations") {
            contentType(ContentType.Application.Json)
            setBody(request)
        }

        if (response.status.isSuccess()) {
            response.body<LocationResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Updates an existing location.
     *
     * @param id Location identifier
     * @param request Update request with fields to modify
     * @return Success with updated location, or Failure
     */
    suspend fun updateLocation(id: String, request: UpdateLocationRequest): Either<AltairError, LocationResponse> = try {
        val response = httpClient.put("$baseUrl/locations/$id") {
            contentType(ContentType.Application.Json)
            setBody(request)
        }

        if (response.status.isSuccess()) {
            response.body<LocationResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Deletes a location.
     *
     * @param id Location identifier
     * @return Success on deletion, or Failure
     */
    suspend fun deleteLocation(id: String): Either<AltairError, Unit> = try {
        val response = httpClient.delete("$baseUrl/locations/$id")

        if (response.status.isSuccess()) {
            Unit.right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    // === CONTAINERS ===

    /**
     * Retrieves all containers for the authenticated user.
     *
     * @return Success with list of containers, or Failure
     */
    suspend fun getContainers(): Either<AltairError, List<ContainerResponse>> = try {
        val response = httpClient.get("$baseUrl/containers")

        if (response.status.isSuccess()) {
            response.body<List<ContainerResponse>>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Creates a new container.
     *
     * @param request Container creation request
     * @return Success with created container, or Failure
     */
    suspend fun createContainer(request: CreateContainerRequest): Either<AltairError, ContainerResponse> = try {
        val response = httpClient.post("$baseUrl/containers") {
            contentType(ContentType.Application.Json)
            setBody(request)
        }

        if (response.status.isSuccess()) {
            response.body<ContainerResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Updates an existing container.
     *
     * @param id Container identifier
     * @param request Update request with fields to modify
     * @return Success with updated container, or Failure
     */
    suspend fun updateContainer(id: String, request: UpdateContainerRequest): Either<AltairError, ContainerResponse> = try {
        val response = httpClient.put("$baseUrl/containers/$id") {
            contentType(ContentType.Application.Json)
            setBody(request)
        }

        if (response.status.isSuccess()) {
            response.body<ContainerResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Moves a container to a different location.
     *
     * @param id Container identifier
     * @param locationId Target location ID (null to unset)
     * @return Success with updated container, or Failure
     */
    suspend fun moveContainer(id: String, locationId: String?): Either<AltairError, ContainerResponse> = try {
        val response = httpClient.put("$baseUrl/containers/$id/move") {
            locationId?.let { parameter("locationId", it) }
        }

        if (response.status.isSuccess()) {
            response.body<ContainerResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Deletes a container.
     *
     * @param id Container identifier
     * @return Success on deletion, or Failure
     */
    suspend fun deleteContainer(id: String): Either<AltairError, Unit> = try {
        val response = httpClient.delete("$baseUrl/containers/$id")

        if (response.status.isSuccess()) {
            Unit.right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    // === TEMPLATES ===

    /**
     * Retrieves all item templates for the authenticated user.
     *
     * @return Success with list of templates, or Failure
     */
    suspend fun getTemplates(): Either<AltairError, List<ItemTemplateResponse>> = try {
        val response = httpClient.get("$baseUrl/templates")

        if (response.status.isSuccess()) {
            response.body<List<ItemTemplateResponse>>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Creates a new item template with field definitions.
     *
     * @param request Template creation request
     * @return Success with created template, or Failure
     */
    suspend fun createTemplate(request: CreateTemplateRequest): Either<AltairError, ItemTemplateResponse> = try {
        val response = httpClient.post("$baseUrl/templates") {
            contentType(ContentType.Application.Json)
            setBody(request)
        }

        if (response.status.isSuccess()) {
            response.body<ItemTemplateResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Deletes an item template.
     *
     * @param id Template identifier
     * @return Success on deletion, or Failure
     */
    suspend fun deleteTemplate(id: String): Either<AltairError, Unit> = try {
        val response = httpClient.delete("$baseUrl/templates/$id")

        if (response.status.isSuccess()) {
            Unit.right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    // === Error Handling ===

    private suspend fun handleError(status: HttpStatusCode, errorBody: String): AltairError {
        return when (status) {
            HttpStatusCode.NotFound -> AltairError.NotFoundError.ItemNotFound(errorBody)
            HttpStatusCode.Unauthorized -> AltairError.AuthError.Unauthorized
            HttpStatusCode.BadRequest -> AltairError.ValidationError.ConstraintViolation(errorBody)
            HttpStatusCode.Conflict -> AltairError.ConflictError.DuplicateEntity("Item", "unknown", errorBody)
            else -> AltairError.NetworkError.ServerError(status.value, errorBody)
        }
    }
}
