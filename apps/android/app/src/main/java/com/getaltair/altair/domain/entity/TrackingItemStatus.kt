package com.getaltair.altair.domain.entity

enum class TrackingItemStatus(val value: String) {
    ACTIVE("active"),
    ARCHIVED("archived");

    companion object {
        fun fromString(value: String): TrackingItemStatus =
            entries.find { it.value == value } ?: ACTIVE
    }
}
