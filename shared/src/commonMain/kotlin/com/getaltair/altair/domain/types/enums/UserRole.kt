package com.getaltair.altair.domain.types.enums

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Role determining user permissions within the system.
 */
@Serializable
enum class UserRole {
    /** Full administrative access */
    @SerialName("admin")
    ADMIN,

    /** Standard user access */
    @SerialName("member")
    MEMBER,
}
