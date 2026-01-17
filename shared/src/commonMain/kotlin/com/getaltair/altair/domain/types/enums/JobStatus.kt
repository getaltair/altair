package com.getaltair.altair.domain.types.enums

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Status of a background processing job.
 */
@Serializable
enum class JobStatus {
    /** Waiting to be processed */
    @SerialName("queued")
    QUEUED,

    /** Currently being processed */
    @SerialName("processing")
    PROCESSING,

    /** Successfully completed */
    @SerialName("completed")
    COMPLETED,

    /** Processing failed */
    @SerialName("failed")
    FAILED,
}
