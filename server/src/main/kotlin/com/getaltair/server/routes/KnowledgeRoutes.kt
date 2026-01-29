@file:Suppress("DEPRECATION")

package com.getaltair.server.routes

import arrow.core.getOrElse
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.knowledge.*
import com.getaltair.altair.shared.dto.auth.ErrorResponse
import com.getaltair.altair.shared.dto.knowledge.*
import com.getaltair.altair.shared.repository.NoteRepository
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.auth.*
import io.ktor.server.auth.jwt.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import kotlinx.datetime.Instant

/**
 * Registers Knowledge module routes under /api/knowledge.
 * All routes require JWT authentication.
 *
 * Endpoints:
 * - Notes: GET, POST, PUT, DELETE, backlinks, tags
 * - Folders: GET, POST, PUT, DELETE
 * - Tags: GET, POST, PUT, DELETE
 * - Search: full-text search
 * - Graph: knowledge graph visualization
 */
fun Route.knowledgeRoutes(noteRepository: NoteRepository) {
    route("/api/knowledge") {
        authenticate("jwt") {
            // ========== NOTES ==========
            route("/notes") {
                /**
                 * GET /api/knowledge/notes
                 * List all notes with optional filtering by folder, tag, or search query.
                 */
                get {
                    val userId = call.userId
                    val folderId = call.request.queryParameters["folderId"]?.let {
                        Ulid.parse(it)                    }
                    val tagId = call.request.queryParameters["tagId"]?.let {
                        Ulid.parse(it)                    }
                    val search = call.request.queryParameters["q"]

                    val result = when {
                        search != null -> noteRepository.search(userId, search)
                        folderId != null -> noteRepository.getByFolder(folderId)
                        tagId != null -> noteRepository.getByTag(tagId)
                        else -> noteRepository.getAllForUser(userId)
                    }

                    result.fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { notes ->
                            call.respond(notes.map { it.toSummaryResponse() })
                        }
                    )
                }

                /**
                 * POST /api/knowledge/notes
                 * Create a new note.
                 */
                post {
                    val userId = call.userId
                    val request = call.receive<CreateNoteRequest>()

                    val folderId = request.folderId?.let {
                        val parsed = Ulid.parse(it)
                        if (parsed == null) {
                            call.respond(HttpStatusCode.BadRequest, ErrorResponse(
                                code = "INVALID_FOLDER_ID",
                                message = "Invalid folder ID format"
                            ))
                            return@post
                        }
                        parsed
                    }

                    val initiativeId = request.initiativeId?.let {
                        val parsed = Ulid.parse(it)
                        if (parsed == null) {
                            call.respond(HttpStatusCode.BadRequest, ErrorResponse(
                                code = "INVALID_INITIATIVE_ID",
                                message = "Invalid initiative ID format"
                            ))
                            return@post
                        }
                        parsed
                    }

                    val now = Instant.fromEpochMilliseconds(System.currentTimeMillis())
                    val note = Note(
                        id = Ulid.generate(),
                        userId = userId,
                        title = request.title,
                        content = request.content,
                        folderId = folderId,
                        initiativeId = initiativeId,
                        embedding = null,
                        createdAt = now,
                        updatedAt = now,
                        deletedAt = null
                    )

                    noteRepository.create(note).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { created ->
                            // Set tags if provided
                            if (request.tagIds.isNotEmpty()) {
                                val tagUlids = request.tagIds.mapNotNull { Ulid.parse(it) }
                                noteRepository.setTags(created.id, tagUlids)
                            }
                            // Parse and save wikilinks
                            noteRepository.parseAndSaveLinks(created.id, created.content)
                            call.respond(HttpStatusCode.Created, created.toResponse())
                        }
                    )
                }

                /**
                 * GET /api/knowledge/notes/{id}
                 * Get detailed note information including tags, attachments, and link counts.
                 */
                get("/{id}") {
                    call.pathParamAsUlid("id") { id ->
                        noteRepository.getById(id).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { note ->
                                // Load related data
                                val tags = noteRepository.getTags(id).getOrElse { emptyList() }
                                val attachments = noteRepository.getAttachments(id).getOrElse { emptyList() }
                                val backlinks = noteRepository.getBacklinks(id).getOrElse { emptyList() }
                                val outgoingLinks = noteRepository.getOutgoingLinks(id).getOrElse { emptyList() }

                                call.respond(note.toResponse(
                                    tags = tags.map { it.toResponse() },
                                    attachments = attachments.map { it.toResponse() },
                                    linkCount = outgoingLinks.size,
                                    backlinkCount = backlinks.size
                                ))
                            }
                        )
                    }
                }

                /**
                 * PUT /api/knowledge/notes/{id}
                 * Update an existing note.
                 */
                put("/{id}") {
                    call.pathParamAsUlid("id") { id ->
                        val request = call.receive<UpdateNoteRequest>()

                        noteRepository.getById(id).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { existing ->
                                val updated = existing.copy(
                                    title = request.title ?: existing.title,
                                    content = request.content ?: existing.content,
                                    folderId = request.folderId?.let { Ulid.parse(it) } ?: existing.folderId,
                                    initiativeId = request.initiativeId?.let { Ulid.parse(it) } ?: existing.initiativeId,
                                    updatedAt = Instant.fromEpochMilliseconds(System.currentTimeMillis())
                                )

                                noteRepository.update(updated).fold(
                                    ifLeft = { error -> call.respondError(error) },
                                    ifRight = { saved ->
                                        // Re-parse links if content changed
                                        if (request.content != null) {
                                            noteRepository.parseAndSaveLinks(saved.id, saved.content)
                                        }
                                        call.respond(saved.toResponse())
                                    }
                                )
                            }
                        )
                    }
                }

                /**
                 * DELETE /api/knowledge/notes/{id}
                 * Soft-delete a note.
                 */
                delete("/{id}") {
                    call.pathParamAsUlid("id") { id ->
                        noteRepository.softDelete(id).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { call.respond(HttpStatusCode.NoContent) }
                        )
                    }
                }

                /**
                 * GET /api/knowledge/notes/{id}/backlinks
                 * Get all notes linking to this note.
                 */
                get("/{id}/backlinks") {
                    call.pathParamAsUlid("id") { id ->
                        noteRepository.getBacklinks(id).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { links ->
                                call.respond(links.map { it.toResponse() })
                            }
                        )
                    }
                }

                /**
                 * GET /api/knowledge/notes/{id}/links
                 * Get all outgoing links from this note.
                 */
                get("/{id}/links") {
                    call.pathParamAsUlid("id") { id ->
                        noteRepository.getOutgoingLinks(id).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { links ->
                                call.respond(links.map { it.toResponse() })
                            }
                        )
                    }
                }

                /**
                 * PUT /api/knowledge/notes/{id}/tags
                 * Set tags for a note (replaces existing tags).
                 */
                put("/{id}/tags") {
                    call.pathParamAsUlid("id") { id ->
                        val request = call.receive<SetNoteTagsRequest>()
                        val tagUlids = request.tagIds.mapNotNull { Ulid.parse(it) }

                        noteRepository.setTags(id, tagUlids).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { call.respond(HttpStatusCode.NoContent) }
                        )
                    }
                }
            }

            // ========== FOLDERS ==========
            route("/folders") {
                /**
                 * GET /api/knowledge/folders
                 * List all folders in hierarchical order.
                 */
                get {
                    val userId = call.userId
                    noteRepository.getFolders(userId).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { folders ->
                            call.respond(folders.map { it.toResponse() })
                        }
                    )
                }

                /**
                 * POST /api/knowledge/folders
                 * Create a new folder.
                 */
                post {
                    val userId = call.userId
                    val request = call.receive<CreateFolderRequest>()

                    val parentId = request.parentId?.let {
                        val parsed = Ulid.parse(it)
                        if (parsed == null) {
                            call.respond(HttpStatusCode.BadRequest, ErrorResponse(
                                code = "INVALID_PARENT_ID",
                                message = "Invalid parent folder ID format"
                            ))
                            return@post
                        }
                        parsed
                    }

                    val folder = Folder(
                        id = Ulid.generate(),
                        userId = userId,
                        name = request.name,
                        parentId = parentId,
                        order = 0, // TODO: Auto-increment within parent
                        createdAt = Instant.fromEpochMilliseconds(System.currentTimeMillis())
                    )

                    noteRepository.createFolder(folder).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { created ->
                            call.respond(HttpStatusCode.Created, created.toResponse())
                        }
                    )
                }

                /**
                 * PUT /api/knowledge/folders/{id}
                 * Update an existing folder.
                 */
                put("/{id}") {
                    call.pathParamAsUlid("id") { id ->
                        val request = call.receive<UpdateFolderRequest>()

                        noteRepository.getFolders(call.userId).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { folders ->
                                val existing = folders.find { it.id == id }
                                if (existing == null) {
                                    call.respondError(AltairError.NotFoundError.FolderNotFound(id.toString()))
                                    return@pathParamAsUlid
                                }

                                val updated = existing.copy(
                                    name = request.name ?: existing.name,
                                    parentId = request.parentId?.let { Ulid.parse(it) } ?: existing.parentId,
                                    order = request.order ?: existing.order
                                )

                                noteRepository.updateFolder(updated).fold(
                                    ifLeft = { error -> call.respondError(error) },
                                    ifRight = { saved -> call.respond(saved.toResponse()) }
                                )
                            }
                        )
                    }
                }

                /**
                 * DELETE /api/knowledge/folders/{id}
                 * Delete a folder (moves notes to parent).
                 */
                delete("/{id}") {
                    call.pathParamAsUlid("id") { id ->
                        noteRepository.deleteFolder(id).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { call.respond(HttpStatusCode.NoContent) }
                        )
                    }
                }
            }

            // ========== TAGS ==========
            route("/tags") {
                /**
                 * GET /api/knowledge/tags
                 * List all tags with usage counts.
                 */
                get {
                    val userId = call.userId
                    noteRepository.getAllTags(userId).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { tags ->
                            // TODO: Load usage counts
                            call.respond(tags.map { it.toResponse(usageCount = 0) })
                        }
                    )
                }

                /**
                 * POST /api/knowledge/tags
                 * Create a new tag.
                 */
                post {
                    val userId = call.userId
                    val request = call.receive<CreateTagRequest>()

                    val tag = Tag(
                        id = Ulid.generate(),
                        userId = userId,
                        name = request.name.lowercase(),
                        color = request.color
                    )

                    noteRepository.createTag(tag).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { created ->
                            call.respond(HttpStatusCode.Created, created.toResponse(usageCount = 0))
                        }
                    )
                }

                /**
                 * PUT /api/knowledge/tags/{id}
                 * Update an existing tag.
                 */
                put("/{id}") {
                    call.pathParamAsUlid("id") { id ->
                        val userId = call.userId
                        val request = call.receive<UpdateTagRequest>()

                        noteRepository.getAllTags(userId).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { tags ->
                                val existing = tags.find { it.id == id }
                                if (existing == null) {
                                    call.respondError(AltairError.NotFoundError.TagNotFound(id.toString()))
                                    return@pathParamAsUlid
                                }

                                val updated = existing.copy(
                                    name = request.name?.lowercase() ?: existing.name,
                                    color = request.color ?: existing.color
                                )

                                noteRepository.createTag(updated).fold(
                                    ifLeft = { error -> call.respondError(error) },
                                    ifRight = { saved -> call.respond(saved.toResponse(usageCount = 0)) }
                                )
                            }
                        )
                    }
                }

                /**
                 * DELETE /api/knowledge/tags/{id}
                 * Delete a tag and remove from all notes.
                 */
                delete("/{id}") {
                    call.pathParamAsUlid("id") { id ->
                        noteRepository.deleteTag(id).fold(
                            ifLeft = { error -> call.respondError(error) },
                            ifRight = { call.respond(HttpStatusCode.NoContent) }
                        )
                    }
                }
            }

            // ========== SEARCH ==========
            /**
             * GET /api/knowledge/search?q={query}
             * Full-text search across all notes.
             */
            get("/search") {
                val userId = call.userId
                val query = call.request.queryParameters["q"] ?: ""

                if (query.isBlank()) {
                    call.respond(HttpStatusCode.BadRequest, ErrorResponse(
                        code = "EMPTY_QUERY",
                        message = "Search query cannot be empty"
                    ))
                    return@get
                }

                noteRepository.search(userId, query).fold(
                    ifLeft = { error -> call.respondError(error) },
                    ifRight = { notes ->
                        call.respond(SearchResultResponse(
                            notes = notes.map { it.toSummaryResponse() },
                            totalCount = notes.size,
                            query = query
                        ))
                    }
                )
            }

            // ========== GRAPH ==========
            /**
             * GET /api/knowledge/graph
             * Get knowledge graph structure (nodes and edges).
             */
            get("/graph") {
                val userId = call.userId

                // Get all notes and links for this user
                noteRepository.getAllForUser(userId).fold(
                    ifLeft = { error -> call.respondError(error) },
                    ifRight = { notes ->
                        val nodes = notes.map { note ->
                            val backlinkCount = noteRepository.getBacklinks(note.id).getOrElse { emptyList() }.size
                            val outgoingCount = noteRepository.getOutgoingLinks(note.id).getOrElse { emptyList() }.size

                            GraphNode(
                                id = note.id.toString(),
                                title = note.title,
                                type = "note",
                                linkCount = backlinkCount + outgoingCount
                            )
                        }

                        val edges = notes.flatMap { note ->
                            noteRepository.getOutgoingLinks(note.id).getOrElse { emptyList() }.map { link ->
                                GraphEdge(
                                    source = link.sourceId.toString(),
                                    target = link.targetId.toString()
                                )
                            }
                        }

                        call.respond(GraphResponse(nodes = nodes, edges = edges))
                    }
                )
            }
        }
    }
}

// ========== DOMAIN TO DTO MAPPERS ==========

/**
 * Convert Note entity to full NoteResponse DTO.
 */
fun Note.toResponse(
    tags: List<TagResponse> = emptyList(),
    attachments: List<AttachmentResponse> = emptyList(),
    linkCount: Int = 0,
    backlinkCount: Int = 0
): NoteResponse = NoteResponse(
    id = id.toString(),
    title = title,
    content = content,
    folderId = folderId?.toString(),
    folderPath = null, // TODO: Compute hierarchical path
    initiativeId = initiativeId?.toString(),
    tags = tags,
    attachments = attachments,
    linkCount = linkCount,
    backlinkCount = backlinkCount,
    createdAt = createdAt.toString(),
    updatedAt = updatedAt.toString()
)

/**
 * Convert Note entity to abbreviated NoteSummaryResponse DTO.
 */
fun Note.toSummaryResponse(): NoteSummaryResponse = NoteSummaryResponse(
    id = id.toString(),
    title = title,
    preview = content.take(200),
    folderId = folderId?.toString(),
    tagCount = 0, // TODO: Load from repository
    updatedAt = updatedAt.toString()
)

/**
 * Convert Folder entity to FolderResponse DTO.
 */
fun Folder.toResponse(): FolderResponse = FolderResponse(
    id = id.toString(),
    name = name,
    parentId = parentId?.toString(),
    path = "", // TODO: Compute hierarchical path from root
    noteCount = 0, // TODO: Load from repository
    childCount = 0, // TODO: Load from repository
    children = null
)

/**
 * Convert Tag entity to TagResponse DTO.
 */
fun Tag.toResponse(usageCount: Int = 0): TagResponse = TagResponse(
    id = id.toString(),
    name = name,
    color = color,
    usageCount = usageCount
)

/**
 * Convert Attachment entity to AttachmentResponse DTO.
 */
fun Attachment.toResponse(): AttachmentResponse = AttachmentResponse(
    id = id.toString(),
    filename = filename,
    mimeType = mimeType,
    sizeBytes = sizeBytes,
    downloadUrl = "" // TODO: Generate presigned URL
)

/**
 * Convert NoteLink entity to NoteLinkResponse DTO.
 */
fun NoteLink.toResponse(): NoteLinkResponse = NoteLinkResponse(
    id = id.toString(),
    sourceId = sourceId.toString(),
    sourceTitle = "", // TODO: Load from repository
    targetId = targetId.toString(),
    targetTitle = "", // TODO: Load from repository
    context = context
)
