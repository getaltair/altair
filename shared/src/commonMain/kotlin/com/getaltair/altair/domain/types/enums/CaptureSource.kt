package com.getaltair.altair.domain.types.enums

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * How an InboxItem was captured into the system.
 */
@Serializable
enum class CaptureSource {
    /** Typed via keyboard */
    @SerialName("keyboard")
    KEYBOARD,

    /** Voice input/dictation */
    @SerialName("voice")
    VOICE,

    /** Camera capture (photo/scan) */
    @SerialName("camera")
    CAMERA,

    /** Shared from another app */
    @SerialName("share")
    SHARE,

    /** Home screen widget */
    @SerialName("widget")
    WIDGET,

    /** Smartwatch input */
    @SerialName("watch")
    WATCH,
}
