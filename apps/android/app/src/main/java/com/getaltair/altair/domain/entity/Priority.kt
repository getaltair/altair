package com.getaltair.altair.domain.entity

enum class Priority(val value: String) {
    LOW("low"),
    MEDIUM("medium"),
    HIGH("high"),
    CRITICAL("critical");

    companion object {
        fun fromString(value: String): Priority =
            entries.find { it.value == value } ?: MEDIUM
    }
}
