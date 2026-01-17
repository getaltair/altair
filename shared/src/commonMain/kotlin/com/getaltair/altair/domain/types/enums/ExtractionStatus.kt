package com.getaltair.altair.domain.types.enums

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Status of AI-powered content extraction from a source document.
 */
@Serializable
enum class ExtractionStatus {
    /** Awaiting extraction */
    @SerialName("pending")
    PENDING,

    /** Currently being processed */
    @SerialName("processing")
    PROCESSING,

    /** Successfully extracted */
    @SerialName("completed")
    COMPLETED,

    /** Extraction failed */
    @SerialName("failed")
    FAILED,

    /** Extraction is outdated and needs refresh */
    @SerialName("stale")
    STALE,
}
