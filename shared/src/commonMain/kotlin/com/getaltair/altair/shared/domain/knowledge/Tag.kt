package com.getaltair.altair.shared.domain.knowledge

import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.serialization.Serializable

/**
 * A user-defined tag for categorizing notes.
 *
 * Tags provide lightweight, non-hierarchical categorization for notes.
 * Unlike folders (which provide tree structure), tags allow many-to-many relationships.
 *
 * ## Tag Conventions
 * - Names are lowercase only (normalized on creation)
 * - Max 50 characters
 * - No special characters or spaces (kebab-case recommended: "project-alpha")
 * - Optional color for visual distinction
 *
 * ## Usage Examples
 * ```
 * #meeting-notes
 * #book-summary
 * #idea
 * #urgent
 * ```
 *
 * ## Tag-Note Relationships
 * Tags are extracted from note content (via #hashtag syntax) and stored in a separate
 * many-to-many join table (not modeled here). This entity represents the tag definition only.
 *
 * @property id Unique identifier for this tag
 * @property userId Owner of this tag
 * @property name Tag name in lowercase (max 50 characters, required, unique per user)
 * @property color Optional hex color code for visual representation (format: #RRGGBB)
 */
@Serializable
data class Tag(
    val id: Ulid,
    val userId: Ulid,
    val name: String,
    val color: String?
) {
    init {
        require(name.length <= MAX_NAME_LENGTH) {
            "Name max $MAX_NAME_LENGTH chars, got ${name.length}"
        }
        require(name.isNotBlank()) { "Name required" }
        require(name == name.lowercase()) {
            "Tag names must be lowercase, got: $name"
        }
        require(name.matches(TAG_NAME_REGEX)) {
            "Tag name must be kebab-case (lowercase letters, numbers, hyphens), got: $name"
        }
        color?.let {
            require(it.matches(COLOR_REGEX)) {
                "Color must be hex format (#RRGGBB), got: $it"
            }
        }
    }

    companion object {
        /** Maximum allowed tag name length in characters */
        const val MAX_NAME_LENGTH = 50

        /** Regex for valid tag names (kebab-case: lowercase letters, numbers, hyphens) */
        private val TAG_NAME_REGEX = Regex("^[a-z0-9]+(-[a-z0-9]+)*$")

        /** Regex for valid hex color codes */
        private val COLOR_REGEX = Regex("^#[0-9A-Fa-f]{6}$")
    }
}
