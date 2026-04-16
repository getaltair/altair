package com.getaltair.altair.ui.knowledge

internal fun decodeUserIdFromJwt(token: String): String =
    try {
        val payloadBase64 = token.split(".").getOrNull(1) ?: return ""
        val padded = payloadBase64.padEnd((payloadBase64.length + 3) / 4 * 4, '=')
        val json = String(android.util.Base64.decode(padded, android.util.Base64.URL_SAFE))
        val subStart = json.indexOf("\"sub\"") + 6
        if (subStart < 6) return ""
        val valueStart = json.indexOf('"', subStart) + 1
        val valueEnd = json.indexOf('"', valueStart)
        json.substring(valueStart, valueEnd)
    } catch (_: Exception) {
        ""
    }

internal fun nowIso(): String =
    kotlinx.datetime.Clock.System
        .now()
        .toString()
