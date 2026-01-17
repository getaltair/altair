package com.getaltair.altair.domain.types.enums

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Current status of a user account.
 */
@Serializable
enum class UserStatus {
    /** Account is active and can be used */
    @SerialName("active")
    ACTIVE,

    /** Account has been disabled by an admin */
    @SerialName("disabled")
    DISABLED,

    /** Account has been soft-deleted */
    @SerialName("deleted")
    DELETED,
}
