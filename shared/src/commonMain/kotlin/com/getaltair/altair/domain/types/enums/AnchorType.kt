package com.getaltair.altair.domain.types.enums

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Type of anchor point for annotations on source documents.
 */
@Serializable
enum class AnchorType {
    /** Anchored to the entire document */
    @SerialName("document")
    DOCUMENT,

    /** Anchored to a specific page */
    @SerialName("page")
    PAGE,

    /** Anchored to a heading/section */
    @SerialName("heading")
    HEADING,

    /** Anchored to a text selection */
    @SerialName("selection")
    SELECTION,
}
