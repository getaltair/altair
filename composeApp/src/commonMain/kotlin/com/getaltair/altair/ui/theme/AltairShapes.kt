package com.getaltair.altair.ui.theme

import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Immutable
import androidx.compose.ui.unit.dp

/**
 * Altair Design System shape tokens.
 *
 * Defines the border radius scale as specified in ADR-008.
 * Provides consistent corner radius values for components.
 *
 * @property sm Small corner radius (4dp)
 * @property md Medium corner radius (6dp)
 * @property lg Large corner radius (8dp)
 * @property full Pill/circular corner radius (9999dp)
 */
@Immutable
data class AltairShapes(
    val sm: RoundedCornerShape = RoundedCornerShape(4.dp),
    val md: RoundedCornerShape = RoundedCornerShape(6.dp),
    val lg: RoundedCornerShape = RoundedCornerShape(8.dp),
    val full: RoundedCornerShape = RoundedCornerShape(9999.dp),
)
