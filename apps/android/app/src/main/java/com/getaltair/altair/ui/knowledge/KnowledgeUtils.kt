package com.getaltair.altair.ui.knowledge

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
private data class JwtPayload(
    @SerialName("sub") val sub: String,
)

private val jwtJson = Json { ignoreUnknownKeys = true }

internal fun decodeUserIdFromJwt(token: String): String =
    try {
        val payloadBase64 = token.split(".").getOrNull(1) ?: return ""
        val padded = payloadBase64.padEnd((payloadBase64.length + 3) / 4 * 4, '=')
        val json = String(android.util.Base64.decode(padded, android.util.Base64.URL_SAFE))
        jwtJson.decodeFromString<JwtPayload>(json).sub
    } catch (_: Exception) {
        ""
    }

internal fun nowIso(): String =
    kotlinx.datetime.Clock.System
        .now()
        .toString()
