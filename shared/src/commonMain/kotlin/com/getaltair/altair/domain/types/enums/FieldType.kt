package com.getaltair.altair.domain.types.enums

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Data type for custom fields on tracked items.
 */
@Serializable
enum class FieldType {
    /** Free-form text */
    @SerialName("text")
    TEXT,

    /** Numeric value */
    @SerialName("number")
    NUMBER,

    /** Date value */
    @SerialName("date")
    DATE,

    /** True/false toggle */
    @SerialName("boolean")
    BOOLEAN,

    /** URL/link */
    @SerialName("url")
    URL,

    /** Selection from predefined options */
    @SerialName("enum")
    ENUM,
}
