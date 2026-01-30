package com.getaltair.altair.ui.theme

import androidx.compose.ui.unit.dp

/**
 * Altair spacing scale.
 *
 * Consistent spacing values for layout and component padding/margins.
 * Follows ADR-008 specifications.
 */
object AltairSpacing {
    /** Extra small spacing (4dp) */
    val xs = 4.dp

    /** Small spacing (8dp) */
    val sm = 8.dp

    /** Medium spacing (16dp) */
    val md = 16.dp

    /** Large spacing (24dp) */
    val lg = 24.dp

    /** Extra large spacing (32dp) */
    val xl = 32.dp
}

/**
 * Altair border radii.
 *
 * Consistent corner radius values for rounded components.
 * Follows ADR-008 specifications.
 */
object AltairRadii {
    /** Small radius for subtle rounding (4dp) */
    val sm = 4.dp

    /** Medium radius for buttons and inputs (6dp) */
    val md = 6.dp

    /** Large radius for cards and dialogs (8dp) */
    val lg = 8.dp

    /** Full radius for pill-shaped components (9999dp) */
    val full = 9999.dp
}
