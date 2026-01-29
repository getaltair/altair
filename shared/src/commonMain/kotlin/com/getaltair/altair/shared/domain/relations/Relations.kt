package com.getaltair.altair.shared.domain.relations

import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * Cross-module relation entities for many-to-many relationships.
 *
 * This file defines the join table entities that enable relationships between
 * entities across different Altair modules (Guidance, Knowledge, Tracking).
 */

/**
 * Links a Quest to a Note.
 *
 * Many-to-many relationship where Quests can reference Notes for:
 * - Context and background information
 * - Research notes and references
 * - Documentation and implementation details
 * - Meeting notes related to the quest
 *
 * Example: A "Implement Auth System" quest might link to notes about
 * OAuth2 flows, security best practices, or API design decisions.
 */
@Serializable
data class QuestNoteLink(
    val id: Ulid,
    val questId: Ulid,
    val noteId: Ulid,
    val createdAt: Instant
)

/**
 * Links a Quest to an Item.
 *
 * Many-to-many relationship where Quests can reference Items they involve:
 * - Tools or equipment needed to complete the quest
 * - Items to be processed or modified
 * - Physical or digital resources required
 *
 * Example: A "Clean Workshop" quest might link to items like
 * "Vacuum Cleaner", "Storage Bins", or "Label Maker".
 */
@Serializable
data class QuestItemLink(
    val id: Ulid,
    val questId: Ulid,
    val itemId: Ulid,
    val createdAt: Instant
)

/**
 * Links a Note to an Item.
 *
 * Many-to-many relationship where Notes can mention or document Items:
 * - Documentation links (user manuals for items)
 * - Maintenance logs and repair notes
 * - Purchase information and receipts
 * - Item specifications and configurations
 *
 * Detected via [[Item:Name]] wikilink syntax or AI entity detection.
 *
 * Example: A "Workshop Setup" note might reference items like
 * "Workbench", "Toolbox", or "Safety Equipment".
 */
@Serializable
data class NoteItemLink(
    val id: Ulid,
    val noteId: Ulid,
    val itemId: Ulid,
    val createdAt: Instant
)

/**
 * Links a Note to a SourceDocument.
 *
 * Many-to-many relationship where Notes can link to and annotate SourceDocuments:
 * - Reference external documents (PDFs, books, articles)
 * - Annotate specific sections of source material
 * - Create literature notes from sources
 * - Build connections between source materials
 *
 * Syntax: [[Source:filename.pdf]] or [[Source:filename.pdf#heading-name]]
 *
 * The sectionAnchor allows linking to specific parts of a document,
 * enabling precise references and annotations.
 */
@Serializable
data class NoteSourceLink(
    val id: Ulid,
    val noteId: Ulid,
    val sourceDocumentId: Ulid,
    val sectionAnchor: String?,     // Optional anchor like "heading-name" or "page-42"
    val createdAt: Instant
)

/**
 * Links a Note to a Tag.
 *
 * Many-to-many relationship for Note categorization and discovery:
 * - Topical tags (e.g., #programming, #philosophy)
 * - Status tags (e.g., #draft, #permanent)
 * - Type tags (e.g., #literature-note, #fleeting-note)
 * - Custom taxonomies
 *
 * Tags enable flexible organization without rigid hierarchies,
 * supporting multiple classification schemes simultaneously.
 */
@Serializable
data class NoteTag(
    val noteId: Ulid,
    val tagId: Ulid
)

/**
 * Polymorphic entity tagging for Quest, Epic, Item, and Initiative.
 *
 * Provides a unified tagging system across all Altair modules:
 * - Quests and Epics (Guidance module)
 * - Items (Tracking module)
 * - Initiatives (cross-module organizational units)
 *
 * Tags are shared in a flat namespace, enabling:
 * - Cross-module discovery (find all entities tagged #urgent)
 * - Consistent categorization
 * - Flexible filtering and grouping
 * - Tag-based automation rules
 *
 * Example use cases:
 * - Tag quest, items, and notes with #home-renovation
 * - Tag urgent items and quests with #priority
 * - Tag all entities related to a project with #project-name
 *
 * @property entityType Must be one of: "quest", "epic", "item", "initiative"
 * @property entityId The ULID of the entity being tagged
 * @property tagId The ULID of the Tag entity
 */
@Serializable
data class EntityTag(
    val entityType: String,
    val entityId: Ulid,
    val tagId: Ulid
) {
    init {
        require(entityType in VALID_ENTITY_TYPES) {
            "Entity type must be one of: $VALID_ENTITY_TYPES"
        }
    }

    companion object {
        val VALID_ENTITY_TYPES = setOf("quest", "epic", "item", "initiative")
    }
}
