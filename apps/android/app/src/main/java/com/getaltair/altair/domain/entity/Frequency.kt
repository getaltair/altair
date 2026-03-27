package com.getaltair.altair.domain.entity

enum class Frequency(val value: String) {
    DAILY("daily"),
    WEEKLY("weekly"),
    BIWEEKLY("biweekly"),
    MONTHLY("monthly");

    companion object {
        fun fromString(value: String): Frequency =
            entries.find { it.value == value } ?: DAILY
    }
}
