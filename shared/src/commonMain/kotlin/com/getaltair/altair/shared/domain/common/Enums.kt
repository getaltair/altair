package com.getaltair.altair.shared.domain.common

import kotlinx.serialization.Serializable

// ==================== User Management ====================

/**
 * Defines the role and permission level of a user within the system.
 */
@Serializable
enum class UserRole {
    /** Administrator with full system access */
    ADMIN,
    /** Regular member with standard permissions */
    MEMBER
}

/**
 * Represents the current operational status of a user account.
 */
@Serializable
enum class UserStatus {
    /** Account is active and fully operational */
    ACTIVE,
    /** Account is temporarily disabled */
    DISABLED,
    /** Account has been soft-deleted */
    DELETED
}

// ==================== Initiative (Projects/Areas) ====================

/**
 * Lifecycle status of an Initiative (Project or Area).
 */
@Serializable
enum class InitiativeStatus {
    /** Initiative is currently active and accepting work */
    ACTIVE,
    /** Initiative is temporarily paused */
    PAUSED,
    /** Initiative has reached its completion criteria */
    COMPLETED,
    /** Initiative is no longer active but preserved for reference */
    ARCHIVED
}

// ==================== Quest (Task Execution) ====================

/**
 * Current execution status of a Quest (actionable task).
 */
@Serializable
enum class QuestStatus {
    /** Quest is planned but not yet started */
    BACKLOG,
    /** Quest is currently being worked on (WIP=1 enforcement) */
    ACTIVE,
    /** Quest has been successfully completed */
    COMPLETED,
    /** Quest was intentionally abandoned or cancelled */
    ABANDONED
}

// ==================== Epic (Quest Grouping) ====================

/**
 * Status of an Epic (grouped collection of related Quests).
 */
@Serializable
enum class EpicStatus {
    /** Epic is active with work in progress */
    ACTIVE,
    /** All quests in the epic have been completed */
    COMPLETED,
    /** Epic is no longer active but preserved for reference */
    ARCHIVED
}

// ==================== Universal Inbox ====================

/**
 * The origination method for a captured inbox item.
 */
@Serializable
enum class CaptureSource {
    /** Entered via keyboard/text input */
    KEYBOARD,
    /** Captured via voice transcription */
    VOICE,
    /** Captured from camera/image */
    CAMERA,
    /** Shared from external app */
    SHARE,
    /** Created via home screen widget */
    WIDGET,
    /** Captured from smartwatch */
    WATCH
}

// ==================== Knowledge Management ====================

/**
 * Status of knowledge extraction from a source document.
 */
@Serializable
enum class ExtractionStatus {
    /** Awaiting extraction processing */
    PENDING,
    /** Successfully extracted and processed */
    PROCESSED,
    /** Extraction failed with errors */
    FAILED,
    /** Previously processed but source has changed */
    STALE
}

/**
 * Type of source document being tracked.
 */
@Serializable
enum class SourceType {
    /** Explicit file reference (local or remote) */
    FILE,
    /** URI/URL reference (web page, API endpoint) */
    URI,
    /** File within a watched folder */
    WATCHED
}

/**
 * Type of anchor point for linking notes to source content.
 */
@Serializable
enum class AnchorType {
    /** Links to entire document */
    DOCUMENT,
    /** Links to specific page (for paginated content) */
    PAGE,
    /** Links to heading/section */
    HEADING,
    /** Links to specific text selection/range */
    SELECTION
}

// ==================== Extraction Jobs ====================

/**
 * Current state of a knowledge extraction background job.
 */
@Serializable
enum class JobStatus {
    /** Job is queued and waiting to execute */
    QUEUED,
    /** Job is currently running */
    PROCESSING,
    /** Job completed successfully */
    COMPLETED,
    /** Job failed with errors */
    FAILED
}

// ==================== Watched Folders ====================

/**
 * Operational status of a watched folder for automatic document ingestion.
 */
@Serializable
enum class WatchedFolderStatus {
    /** Actively monitoring for changes */
    ACTIVE,
    /** Monitoring temporarily paused */
    PAUSED,
    /** Error state requiring attention */
    ERROR
}

// ==================== Custom Fields ====================

/**
 * Data type for custom field values.
 */
@Serializable
enum class FieldType {
    /** Free-form text value */
    TEXT,
    /** Numeric value (integer or decimal) */
    NUMBER,
    /** Date or datetime value */
    DATE,
    /** Boolean true/false value */
    BOOLEAN,
    /** URL/URI value with validation */
    URL,
    /** Enumerated value from predefined list */
    ENUM
}

// ==================== Recurring Schedules ====================

/**
 * Relative week position within a month for schedule calculations.
 */
@Serializable
enum class RelativeWeek {
    /** First occurrence of weekday in month */
    FIRST,
    /** Second occurrence of weekday in month */
    SECOND,
    /** Third occurrence of weekday in month */
    THIRD,
    /** Fourth occurrence of weekday in month */
    FOURTH,
    /** Last occurrence of weekday in month */
    LAST
}
