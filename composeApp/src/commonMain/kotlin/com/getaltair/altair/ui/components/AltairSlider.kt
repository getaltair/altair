package com.getaltair.altair.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.detectHorizontalDragGestures
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.offset
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.lerp
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.layout.onSizeChanged
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.unit.dp
import com.getaltair.altair.ui.theme.AltairTheme
import kotlin.math.roundToInt

/**
 * Altair Design System slider component.
 *
 * A styled slider for selecting values within a range.
 *
 * @param value Current slider value
 * @param onValueChange Callback when the value changes
 * @param modifier Modifier to be applied to the slider
 * @param valueRange Range of valid values
 * @param enabled Whether the slider is enabled
 */
@Composable
fun AltairSlider(
    value: Float,
    onValueChange: (Float) -> Unit,
    modifier: Modifier = Modifier,
    valueRange: ClosedFloatingPointRange<Float> = 0f..1f,
    enabled: Boolean = true,
) {
    val colors = AltairTheme.colors
    val shapes = AltairTheme.shapes

    val trackHeight = 6.dp
    val thumbSize = 20.dp
    val density = LocalDensity.current

    var sliderWidth by remember { mutableFloatStateOf(0f) }
    val thumbSizePx = with(density) { thumbSize.toPx() }

    val progress = ((value - valueRange.start) / (valueRange.endInclusive - valueRange.start))
        .coerceIn(0f, 1f)

    Box(
        modifier = modifier
            .fillMaxWidth()
            .height(thumbSize)
            .onSizeChanged { sliderWidth = it.width.toFloat() }
            .pointerInput(enabled) {
                if (!enabled) return@pointerInput
                detectHorizontalDragGestures { change, _ ->
                    change.consume()
                    val newProgress = ((change.position.x - thumbSizePx / 2) / (sliderWidth - thumbSizePx))
                        .coerceIn(0f, 1f)
                    val newValue = valueRange.start + newProgress * (valueRange.endInclusive - valueRange.start)
                    onValueChange(newValue)
                }
            },
        contentAlignment = Alignment.CenterStart,
    ) {
        // Track background
        Box(
            modifier = Modifier
                .fillMaxWidth()
                .height(trackHeight)
                .clip(shapes.full)
                .background(colors.surface),
        )

        // Track fill
        Box(
            modifier = Modifier
                .fillMaxWidth(progress)
                .height(trackHeight)
                .clip(shapes.full)
                .background(colors.accent),
        )

        // Thumb
        Box(
            modifier = Modifier
                .offset {
                    IntOffset(
                        x = ((sliderWidth - thumbSizePx) * progress).roundToInt(),
                        y = 0,
                    )
                }
                .size(thumbSize)
                .clip(CircleShape)
                .background(colors.textPrimary),
        )
    }
}

/**
 * Altair Design System energy slider component.
 *
 * A specialized slider for energy level selection (1-5 scale)
 * with color gradient from green (low effort) to red (high effort).
 *
 * @param energyLevel Current energy level (1-5)
 * @param onEnergyLevelChange Callback when the energy level changes
 * @param modifier Modifier to be applied to the slider
 */
@Composable
fun AltairEnergySlider(
    energyLevel: Int,
    onEnergyLevelChange: (Int) -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = AltairTheme.colors
    val shapes = AltairTheme.shapes

    val trackHeight = 6.dp
    val thumbSize = 20.dp
    val density = LocalDensity.current

    var sliderWidth by remember { mutableFloatStateOf(0f) }
    val thumbSizePx = with(density) { thumbSize.toPx() }

    // Clamp energy level to valid range
    val clampedLevel = energyLevel.coerceIn(1, 5)
    val progress = (clampedLevel - 1) / 4f

    // Interpolate color between energy1 (green) and energy5 (red)
    val trackActiveColor = lerp(colors.energy1, colors.energy5, progress)

    Box(
        modifier = modifier
            .fillMaxWidth()
            .height(thumbSize)
            .onSizeChanged { sliderWidth = it.width.toFloat() }
            .pointerInput(Unit) {
                detectHorizontalDragGestures { change, _ ->
                    change.consume()
                    val newProgress = ((change.position.x - thumbSizePx / 2) / (sliderWidth - thumbSizePx))
                        .coerceIn(0f, 1f)
                    // Snap to discrete levels 1-5
                    val newLevel = (1 + (newProgress * 4).roundToInt()).coerceIn(1, 5)
                    onEnergyLevelChange(newLevel)
                }
            },
        contentAlignment = Alignment.CenterStart,
    ) {
        // Track background
        Box(
            modifier = Modifier
                .fillMaxWidth()
                .height(trackHeight)
                .clip(shapes.full)
                .background(colors.surface),
        )

        // Track fill with gradient color
        Box(
            modifier = Modifier
                .fillMaxWidth(progress)
                .height(trackHeight)
                .clip(shapes.full)
                .background(trackActiveColor),
        )

        // Thumb
        Box(
            modifier = Modifier
                .offset {
                    IntOffset(
                        x = ((sliderWidth - thumbSizePx) * progress).roundToInt(),
                        y = 0,
                    )
                }
                .size(thumbSize)
                .clip(CircleShape)
                .background(colors.textPrimary),
        )
    }
}
