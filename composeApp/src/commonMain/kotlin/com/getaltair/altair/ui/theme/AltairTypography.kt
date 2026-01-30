package com.getaltair.altair.ui.theme

import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.sp

/**
 * Altair typography scale.
 *
 * Follows ADR-008 specifications with semantic naming.
 * Currently uses system default font; can be upgraded to Inter later.
 */
object AltairTypography {
    /** Font family for all text (can be replaced with Inter) */
    val fontFamily = FontFamily.Default

    /** Large display text (32sp, SemiBold) */
    val displayLarge = TextStyle(
        fontSize = 32.sp,
        fontWeight = FontWeight.SemiBold,
        fontFamily = fontFamily
    )

    /** Medium headline text (20sp, Medium) */
    val headlineMedium = TextStyle(
        fontSize = 20.sp,
        fontWeight = FontWeight.Medium,
        fontFamily = fontFamily
    )

    /** Large body text (16sp, Normal) */
    val bodyLarge = TextStyle(
        fontSize = 16.sp,
        fontWeight = FontWeight.Normal,
        fontFamily = fontFamily
    )

    /** Medium body text (14sp, Normal) */
    val bodyMedium = TextStyle(
        fontSize = 14.sp,
        fontWeight = FontWeight.Normal,
        fontFamily = fontFamily
    )

    /** Small label text (12sp, Medium) */
    val labelSmall = TextStyle(
        fontSize = 12.sp,
        fontWeight = FontWeight.Medium,
        fontFamily = fontFamily
    )
}
