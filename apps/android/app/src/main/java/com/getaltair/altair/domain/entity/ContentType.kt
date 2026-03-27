package com.getaltair.altair.domain.entity

enum class ContentType(val value: String) {
    MARKDOWN("markdown"),
    PLAIN("plain");

    companion object {
        fun fromString(value: String): ContentType =
            entries.find { it.value == value } ?: MARKDOWN
    }
}
