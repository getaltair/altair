package com.getaltair.altair.domain.common

/**
 * Shared color validation utilities for domain entities.
 */
object ColorValidation {
    /**
     * Regex for validating hex color codes.
     * Supports:
     * - 3 characters: #RGB (shorthand)
     * - 6 characters: #RRGGBB (standard)
     * - 8 characters: #RRGGBBAA (with alpha)
     */
    val HEX_COLOR_REGEX = Regex("^#([0-9A-Fa-f]{3}|[0-9A-Fa-f]{6}|[0-9A-Fa-f]{8})$")

    /**
     * Validates that the given string is a valid hex color code.
     * @param color The color string to validate (nullable)
     * @param fieldName The field name for error messages
     * @throws IllegalArgumentException if the color is not null and not a valid hex color
     */
    fun requireValidHexColor(
        color: String?,
        fieldName: String,
    ) {
        color?.let {
            require(it.matches(HEX_COLOR_REGEX)) {
                "$fieldName must be a hex color (e.g., #RGB, #RRGGBB, or #RRGGBBAA)"
            }
        }
    }
}
