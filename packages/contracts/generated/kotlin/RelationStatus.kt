// AUTO-GENERATED from registry/relation-statuses.json — do not edit
package com.altair.contracts

enum class RelationStatus(val value: String) {
    ACCEPTED("accepted"),
    SUGGESTED("suggested"),
    DISMISSED("dismissed"),
    REJECTED("rejected"),
    EXPIRED("expired");

    companion object {
        fun fromValue(value: String): RelationStatus =
            entries.first { it.value == value }
    }
}
