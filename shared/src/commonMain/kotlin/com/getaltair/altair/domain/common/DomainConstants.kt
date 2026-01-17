package com.getaltair.altair.domain.common

/**
 * Centralized validation constants for domain entities.
 *
 * These values are also listed in config/detekt/detekt.yml as allowed magic numbers.
 * When adding new constants here, also add them to the detekt configuration.
 */
object DomainConstants {
    /** Maximum energy cost for a Quest or Routine (1-5 scale) */
    const val MAX_ENERGY_COST = 5

    /** Length of a ULID string */
    const val ULID_LENGTH = 26

    /** Maximum day of month for scheduling */
    const val MAX_DAY_OF_MONTH = 31

    /** Base for ULID encoding (Crockford Base32) */
    const val ULID_ENCODING_BASE = 32

    /** Maximum length for tag names */
    const val MAX_TAG_NAME_LENGTH = 50

    /** Maximum length for names and display names (e.g., User.displayName, Initiative.name) */
    const val MAX_NAME_LENGTH = 100

    /** Maximum length for titles (e.g., Quest.title, Epic.title, Note.title) */
    const val MAX_TITLE_LENGTH = 200

    /** Maximum length for filenames */
    const val MAX_FILENAME_LENGTH = 255

    /** Maximum length for context fields and source titles (e.g., SourceDocument.title) */
    const val MAX_CONTEXT_LENGTH = 500

    /** Maximum length for content fields (e.g., InboxItem.content, CustomField.value) */
    const val MAX_CONTENT_LENGTH = 5000

    /** Maximum length for annotation content */
    const val MAX_ANNOTATION_CONTENT_LENGTH = 10_000

    /** Minimum progress value for jobs */
    const val MIN_PROGRESS = 0

    /** Maximum progress value for jobs (percentage) */
    const val MAX_PROGRESS = 100
}
