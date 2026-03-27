package com.getaltair.altair.domain.entity

enum class ShoppingListStatus(val value: String) {
    ACTIVE("active"),
    COMPLETED("completed"),
    ARCHIVED("archived");

    companion object {
        fun fromString(value: String): ShoppingListStatus =
            entries.find { it.value == value } ?: ACTIVE
    }
}
