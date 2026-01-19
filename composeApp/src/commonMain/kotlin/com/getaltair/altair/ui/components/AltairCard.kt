package com.getaltair.altair.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxScope
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Shape
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Card elevation levels for visual hierarchy.
 */
enum class CardElevation {
    /** No elevation, flat appearance. */
    None,

    /** Low elevation for subtle depth. */
    Low,

    /** Medium elevation for cards requiring attention. */
    Medium,

    /** High elevation for modals and popovers. */
    High,
}

/**
 * Altair-styled card component.
 *
 * A surface container with consistent styling, border, and optional elevation.
 * Use cards to group related content and create visual hierarchy.
 *
 * @param modifier Modifier to be applied to the card.
 * @param backgroundColor Background color of the card.
 * @param borderColor Border color of the card. Set to [Color.Transparent] to hide.
 * @param elevation Visual elevation level for shadow depth.
 * @param shape Corner shape of the card.
 * @param contentPadding Padding applied inside the card.
 * @param content The card content.
 */
@Composable
fun AltairCard(
    modifier: Modifier = Modifier,
    backgroundColor: Color = AltairTheme.Colors.backgroundElevated,
    borderColor: Color = AltairTheme.Colors.border,
    elevation: CardElevation = CardElevation.None,
    shape: Shape = RoundedCornerShape(AltairTheme.Radii.lg),
    contentPadding: Dp = AltairTheme.Spacing.md,
    content: @Composable BoxScope.() -> Unit,
) {
    val shadowElevation = when (elevation) {
        CardElevation.None -> 0.dp
        CardElevation.Low -> 2.dp
        CardElevation.Medium -> 4.dp
        CardElevation.High -> 8.dp
    }

    Box(
        modifier = modifier
            .then(
                if (shadowElevation > 0.dp) {
                    Modifier.shadow(
                        elevation = shadowElevation,
                        shape = shape,
                        ambientColor = Color.Black.copy(alpha = 0.3f),
                        spotColor = Color.Black.copy(alpha = 0.3f),
                    )
                } else {
                    Modifier
                }
            )
            .background(color = backgroundColor, shape = shape)
            .then(
                if (borderColor != Color.Transparent) {
                    Modifier.border(width = 1.dp, color = borderColor, shape = shape)
                } else {
                    Modifier
                }
            )
            .padding(contentPadding),
        content = content,
    )
}

/**
 * Altair-styled surface component.
 *
 * A simpler variant of [AltairCard] without border, suitable for
 * larger background areas.
 *
 * @param modifier Modifier to be applied to the surface.
 * @param backgroundColor Background color of the surface.
 * @param shape Corner shape of the surface.
 * @param content The surface content.
 */
@Composable
fun AltairSurface(
    modifier: Modifier = Modifier,
    backgroundColor: Color = AltairTheme.Colors.background,
    shape: Shape = RoundedCornerShape(0.dp),
    content: @Composable BoxScope.() -> Unit,
) {
    Box(
        modifier = modifier.background(color = backgroundColor, shape = shape),
        content = content,
    )
}
