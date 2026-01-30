package com.getaltair.altair.ui.theme

import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.sp

/**
 * Extension properties for AltairTypography to provide additional text styles.
 */

/** Medium heading text (18sp, Medium) */
val AltairTypography.headingMedium: TextStyle
    get() = TextStyle(
        fontSize = 18.sp,
        fontWeight = FontWeight.Medium,
        fontFamily = fontFamily
    )

/** Small heading text (14sp, Medium) */
val AltairTypography.headingSmall: TextStyle
    get() = TextStyle(
        fontSize = 14.sp,
        fontWeight = FontWeight.Medium,
        fontFamily = fontFamily
    )

/** Small body text (12sp, Normal) */
val AltairTypography.bodySmall: TextStyle
    get() = TextStyle(
        fontSize = 12.sp,
        fontWeight = FontWeight.Normal,
        fontFamily = fontFamily
    )

/** Medium label text (14sp, Medium) */
val AltairTypography.labelMedium: TextStyle
    get() = TextStyle(
        fontSize = 14.sp,
        fontWeight = FontWeight.Medium,
        fontFamily = fontFamily
    )
