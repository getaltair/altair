package com.getaltair.altair.domain.entity

enum class InitiativeStatus(val value: String) {
    ACTIVE("active"),
    PAUSED("paused"),
    COMPLETED("completed"),
    ARCHIVED("archived");

    companion object {
        fun fromString(value: String): InitiativeStatus =
            entries.find { it.value == value } ?: ACTIVE
    }
}

enum class QuestStatus(val value: String) {
    PENDING("pending"),
    IN_PROGRESS("in_progress"),
    COMPLETED("completed"),
    CANCELLED("cancelled");

    companion object {
        fun fromString(value: String): QuestStatus =
            entries.find { it.value == value } ?: PENDING
    }
}

enum class EpicStatus(val value: String) {
    ACTIVE("active"),
    PAUSED("paused"),
    COMPLETED("completed"),
    ARCHIVED("archived");

    companion object {
        fun fromString(value: String): EpicStatus =
            entries.find { it.value == value } ?: ACTIVE
    }
}

enum class RoutineStatus(val value: String) {
    ACTIVE("active"),
    PAUSED("paused"),
    ARCHIVED("archived");

    companion object {
        fun fromString(value: String): RoutineStatus =
            entries.find { it.value == value } ?: ACTIVE
    }
}
