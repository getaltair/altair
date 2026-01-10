package com.getaltair.altair.ui.theme

import androidx.compose.runtime.Immutable
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp

/**
 * Altair Design System spacing tokens.
 *
 * Defines the spacing scale as specified in ADR-008.
 * Provides consistent spacing values for layout and component padding.
 *
 * @property xs Extra small spacing (4dp)
 * @property sm Small spacing (8dp)
 * @property md Medium spacing (16dp)
 * @property lg Large spacing (24dp)
 * @property xl Extra large spacing (32dp)
 */
@Immutable
data class AltairSpacing(
    val xs: Dp = 4.dp,
    val sm: Dp = 8.dp,
    val md: Dp = 16.dp,
    val lg: Dp = 24.dp,
    val xl: Dp = 32.dp
)
