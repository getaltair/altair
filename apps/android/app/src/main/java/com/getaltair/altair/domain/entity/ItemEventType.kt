package com.getaltair.altair.domain.entity

enum class ItemEventType(val value: String) {
    CONSUMED("consumed"),
    RESTOCKED("restocked"),
    MOVED("moved"),
    ADJUSTED("adjusted"),
    EXPIRED("expired"),
    DONATED("donated");

    companion object {
        fun fromString(value: String): ItemEventType =
            entries.find { it.value == value } ?: ADJUSTED
    }
}
