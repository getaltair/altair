// AUTO-GENERATED from registry/attachment-states.json — do not edit
package com.altair.contracts

enum class AttachmentState(val value: String) {
    PENDING("pending"),
    UPLOADED("uploaded"),
    PROCESSING("processing"),
    READY("ready"),
    FAILED("failed"),
    DELETED("deleted");

    companion object {
        fun fromValue(value: String): AttachmentState =
            entries.first { it.value == value }
    }
}
