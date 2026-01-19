package com.getaltair.altair.ui.components

import androidx.compose.animation.core.LinearEasing
import androidx.compose.animation.core.RepeatMode
import androidx.compose.animation.core.animateFloat
import androidx.compose.animation.core.infiniteRepeatable
import androidx.compose.animation.core.rememberInfiniteTransition
import androidx.compose.animation.core.tween
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.size
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.StrokeCap
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import com.getaltair.altair.ui.theme.LocalAltairColors

/** Sweep angle for the progress indicator arc. */
private const val PROGRESS_SWEEP_ANGLE = 270f

/** Background track full circle angle. */
private const val FULL_CIRCLE_ANGLE = 360f

/** Start angle offset to begin rotation from the top of the circle. */
private const val START_ANGLE_OFFSET = 90f

/**
 * Altair-styled circular progress indicator.
 *
 * An indeterminate spinner that indicates loading state.
 *
 * @param modifier Modifier to be applied to the indicator.
 * @param size The diameter of the spinner.
 * @param strokeWidth The width of the spinner stroke.
 * @param color The color of the spinner. Defaults to theme's accent color.
 */
@Composable
fun AltairCircularProgressIndicator(
    modifier: Modifier = Modifier,
    size: Dp = 24.dp,
    strokeWidth: Dp = 2.dp,
    color: Color = LocalAltairColors.current.accent,
) {
    val infiniteTransition = rememberInfiniteTransition(label = "progress")
    val rotation by infiniteTransition.animateFloat(
        initialValue = 0f,
        targetValue = 360f,
        animationSpec =
            infiniteRepeatable(
                animation = tween(durationMillis = 1000, easing = LinearEasing),
                repeatMode = RepeatMode.Restart,
            ),
        label = "rotation",
    )

    Canvas(modifier = modifier.size(size)) {
        val startAngle = rotation - START_ANGLE_OFFSET

        drawArc(
            color = color.copy(alpha = 0.2f),
            startAngle = 0f,
            sweepAngle = FULL_CIRCLE_ANGLE,
            useCenter = false,
            style = Stroke(width = strokeWidth.toPx(), cap = StrokeCap.Round),
        )

        drawArc(
            color = color,
            startAngle = startAngle,
            sweepAngle = PROGRESS_SWEEP_ANGLE,
            useCenter = false,
            style = Stroke(width = strokeWidth.toPx(), cap = StrokeCap.Round),
        )
    }
}
