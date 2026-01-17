package com.getaltair.altair.domain.types.enums

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Type of source for imported documents.
 */
@Serializable
enum class SourceType {
    /** Local or cloud file */
    @SerialName("file")
    FILE,

    /** Web URL */
    @SerialName("uri")
    URI,

    /** File from a watched folder (auto-imported) */
    @SerialName("watched")
    WATCHED,
}
