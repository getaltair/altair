package com.getaltair.altair.contracts

// Source of truth: packages/contracts/sync-streams.json
// Note: these stream names are provisional — Step 4 (Sync Engine) may revise them.

enum class SyncStream(
    val value: String,
) {
    USER_DATA("user_data"),
    HOUSEHOLD("household"),
    GUIDANCE("guidance"),
    KNOWLEDGE("knowledge"),
    TRACKING("tracking"),
    ;

    companion object {
        fun fromValueOrNull(value: String): SyncStream? = entries.find { it.value == value }

        fun fromValue(value: String): SyncStream = fromValueOrNull(value) ?: throw IllegalArgumentException("Unknown SyncStream: '$value'")
    }
}
