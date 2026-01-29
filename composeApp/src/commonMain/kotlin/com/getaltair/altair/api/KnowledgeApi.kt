package com.getaltair.altair.api

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.shared.dto.knowledge.*
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.client.statement.*
import io.ktor.http.*

/**
 * Client API for Knowledge module operations.
 * Handles notes, folders, tags, search, and knowledge graph.
 *
 * Uses Arrow's Either for functional error handling:
 * - Left: ApiError with detailed failure information
 * - Right: Successful response data
 */
class KnowledgeApi(private val httpClient: HttpClient) {

    // ========== NOTES ==========

    /**
     * Retrieve all notes with optional filtering.
     *
     * @param folderId Optional folder ID to filter notes
     * @param tagId Optional tag ID to filter notes
     * @param search Optional full-text search query
     * @return Either ApiError or list of note summaries
     */
    suspend fun getNotes(
        folderId: String? = null,
        tagId: String? = null,
        search: String? = null
    ): Either<ApiError, List<NoteSummaryResponse>> {
        val params = buildList {
            folderId?.let { add("folderId=$it") }
            tagId?.let { add("tagId=$it") }
            search?.let { add("q=$it") }
        }.joinToString("&")

        val path = if (params.isEmpty()) {
            "/api/knowledge/notes"
        } else {
            "/api/knowledge/notes?$params"
        }

        return get(path)
    }

    /**
     * Create a new note.
     *
     * @param request Note creation request with title, content, etc.
     * @return Either ApiError or created note response
     */
    suspend fun createNote(request: CreateNoteRequest): Either<ApiError, NoteResponse> =
        post("/api/knowledge/notes", request)

    /**
     * Get detailed note information.
     *
     * @param id The note's unique identifier
     * @return Either ApiError or full note response with tags, attachments, link counts
     */
    suspend fun getNote(id: String): Either<ApiError, NoteResponse> =
        get("/api/knowledge/notes/$id")

    /**
     * Update an existing note.
     *
     * @param id The note's unique identifier
     * @param request Update request with optional fields
     * @return Either ApiError or updated note response
     */
    suspend fun updateNote(id: String, request: UpdateNoteRequest): Either<ApiError, NoteResponse> =
        put("/api/knowledge/notes/$id", request)

    /**
     * Soft-delete a note.
     *
     * @param id The note's unique identifier
     * @return Either ApiError or Unit on success
     */
    suspend fun deleteNote(id: String): Either<ApiError, Unit> =
        delete("/api/knowledge/notes/$id")

    /**
     * Get all notes linking to this note (backlinks).
     *
     * @param noteId The note's unique identifier
     * @return Either ApiError or list of incoming note links
     */
    suspend fun getBacklinks(noteId: String): Either<ApiError, List<NoteLinkResponse>> =
        get("/api/knowledge/notes/$noteId/backlinks")

    /**
     * Get all outgoing links from this note.
     *
     * @param noteId The note's unique identifier
     * @return Either ApiError or list of outgoing note links
     */
    suspend fun getOutgoingLinks(noteId: String): Either<ApiError, List<NoteLinkResponse>> =
        get("/api/knowledge/notes/$noteId/links")

    /**
     * Set tags for a note (replaces all existing tags).
     *
     * @param noteId The note's unique identifier
     * @param request Tag IDs to apply to the note
     * @return Either ApiError or Unit on success
     */
    suspend fun setNoteTags(noteId: String, request: SetNoteTagsRequest): Either<ApiError, Unit> =
        putUnit("/api/knowledge/notes/$noteId/tags", request)

    // ========== FOLDERS ==========

    /**
     * Get all folders in hierarchical order.
     *
     * @return Either ApiError or list of folders
     */
    suspend fun getFolders(): Either<ApiError, List<FolderResponse>> =
        get("/api/knowledge/folders")

    /**
     * Create a new folder.
     *
     * @param request Folder creation request with name and optional parent
     * @return Either ApiError or created folder response
     */
    suspend fun createFolder(request: CreateFolderRequest): Either<ApiError, FolderResponse> =
        post("/api/knowledge/folders", request)

    /**
     * Update an existing folder.
     *
     * @param id The folder's unique identifier
     * @param request Update request with optional fields
     * @return Either ApiError or updated folder response
     */
    suspend fun updateFolder(id: String, request: UpdateFolderRequest): Either<ApiError, FolderResponse> =
        put("/api/knowledge/folders/$id", request)

    /**
     * Delete a folder (moves notes to parent).
     *
     * @param id The folder's unique identifier
     * @return Either ApiError or Unit on success
     */
    suspend fun deleteFolder(id: String): Either<ApiError, Unit> =
        delete("/api/knowledge/folders/$id")

    // ========== TAGS ==========

    /**
     * Get all tags with usage counts.
     *
     * @return Either ApiError or list of tags
     */
    suspend fun getTags(): Either<ApiError, List<TagResponse>> =
        get("/api/knowledge/tags")

    /**
     * Create a new tag.
     *
     * @param request Tag creation request with name and optional color
     * @return Either ApiError or created tag response
     */
    suspend fun createTag(request: CreateTagRequest): Either<ApiError, TagResponse> =
        post("/api/knowledge/tags", request)

    /**
     * Update an existing tag.
     *
     * @param id The tag's unique identifier
     * @param request Update request with optional fields
     * @return Either ApiError or updated tag response
     */
    suspend fun updateTag(id: String, request: UpdateTagRequest): Either<ApiError, TagResponse> =
        put("/api/knowledge/tags/$id", request)

    /**
     * Delete a tag and remove it from all notes.
     *
     * @param id The tag's unique identifier
     * @return Either ApiError or Unit on success
     */
    suspend fun deleteTag(id: String): Either<ApiError, Unit> =
        delete("/api/knowledge/tags/$id")

    // ========== SEARCH ==========

    /**
     * Perform full-text search across all notes.
     *
     * @param query Search query string
     * @return Either ApiError or search results with matching notes
     */
    suspend fun search(query: String): Either<ApiError, SearchResultResponse> =
        get("/api/knowledge/search?q=${query.encodeURLParameter()}")

    // ========== GRAPH ==========

    /**
     * Get knowledge graph structure (nodes and edges).
     *
     * @return Either ApiError or graph response with all notes and links
     */
    suspend fun getGraph(): Either<ApiError, GraphResponse> =
        get("/api/knowledge/graph")

    // ========== HELPER METHODS ==========

    /**
     * Perform GET request and parse response.
     */
    private suspend inline fun <reified T> get(path: String): Either<ApiError, T> = try {
        val response = httpClient.get(path)
        handleResponse(response)
    } catch (e: Exception) {
        ApiError.NetworkError(e.message ?: "Network error").left()
    }

    /**
     * Perform POST request with body and parse response.
     */
    private suspend inline fun <reified R, reified T> post(
        path: String,
        body: R
    ): Either<ApiError, T> = try {
        val response = httpClient.post(path) {
            contentType(ContentType.Application.Json)
            setBody(body)
        }
        handleResponse(response)
    } catch (e: Exception) {
        ApiError.NetworkError(e.message ?: "Network error").left()
    }

    /**
     * Perform PUT request with body and parse response.
     */
    private suspend inline fun <reified R, reified T> put(
        path: String,
        body: R
    ): Either<ApiError, T> = try {
        val response = httpClient.put(path) {
            contentType(ContentType.Application.Json)
            setBody(body)
        }
        handleResponse(response)
    } catch (e: Exception) {
        ApiError.NetworkError(e.message ?: "Network error").left()
    }

    /**
     * Perform PUT request with body and return Unit on success.
     */
    private suspend inline fun <reified R> putUnit(
        path: String,
        body: R
    ): Either<ApiError, Unit> = try {
        val response = httpClient.put(path) {
            contentType(ContentType.Application.Json)
            setBody(body)
        }
        if (response.status.isSuccess()) {
            Unit.right()
        } else {
            parseError(response).left()
        }
    } catch (e: Exception) {
        ApiError.NetworkError(e.message ?: "Network error").left()
    }

    /**
     * Perform DELETE request and return Unit on success.
     */
    private suspend fun delete(path: String): Either<ApiError, Unit> = try {
        val response = httpClient.delete(path)
        if (response.status.isSuccess()) {
            Unit.right()
        } else {
            parseError(response).left()
        }
    } catch (e: Exception) {
        ApiError.NetworkError(e.message ?: "Network error").left()
    }

    /**
     * Handle HTTP response, parsing success or error.
     */
    private suspend inline fun <reified T> handleResponse(
        response: HttpResponse
    ): Either<ApiError, T> =
        if (response.status.isSuccess()) {
            response.body<T>().right()
        } else {
            parseError(response).left()
        }

    /**
     * Parse HTTP error response into typed ApiError.
     */
    private suspend fun parseError(response: HttpResponse): ApiError = try {
        val error = response.body<com.getaltair.altair.shared.dto.auth.ErrorResponse>()
        when (response.status) {
            HttpStatusCode.Unauthorized -> ApiError.Unauthorized(error.message)
            HttpStatusCode.Forbidden -> ApiError.Forbidden(error.message)
            HttpStatusCode.NotFound -> ApiError.NotFound(error.message)
            HttpStatusCode.Conflict -> ApiError.Conflict(error.message)
            HttpStatusCode.BadRequest -> ApiError.ValidationError(error.message, error.details)
            else -> ApiError.ServerError(response.status.value, error.message)
        }
    } catch (e: Exception) {
        ApiError.ServerError(response.status.value, response.status.description)
    }
}
