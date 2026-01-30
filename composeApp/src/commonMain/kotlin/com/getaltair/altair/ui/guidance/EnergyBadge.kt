package com.getaltair.altair.ui.guidance

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Energy badge component showing quest energy cost with color coding.
 *
 * Energy levels are color-coded from green (low effort) to red (high effort):
 * - 1: Green (energy1)
 * - 2: Light green
 * - 3: Yellow (energy3/warning)
 * - 4: Orange/Red blend
 * - 5: Red (energy5)
 *
 * @param energyCost Energy cost value (1-5 scale)
 * @param modifier Modifier to be applied to the badge
 */
@Composable
fun EnergyBadge(
    energyCost: Int,
    modifier: Modifier = Modifier
) {
    val colors = AltairTheme.colors

    // Interpolate color based on energy cost
    val badgeColor = when (energyCost) {
        1 -> colors.energy1
        2 -> Color(0xFF4ADE80) // Between green and yellow
        3 -> colors.warning
        4 -> Color(0xFFF97316) // Between yellow and red
        else -> colors.energy5 // 5 or higher
    }

    val shape = RoundedCornerShape(12.dp)

    Box(
        modifier = modifier
            .clip(shape)
            .background(badgeColor.copy(alpha = 0.15f))
            .border(
                width = 1.dp,
                color = badgeColor,
                shape = shape
            )
            .padding(horizontal = 8.dp, vertical = 4.dp)
    ) {
        BasicText(
            text = "E$energyCost",
            style = TextStyle(
                color = badgeColor,
                fontSize = 11.sp,
                letterSpacing = 0.5.sp
            )
        )
    }
}
