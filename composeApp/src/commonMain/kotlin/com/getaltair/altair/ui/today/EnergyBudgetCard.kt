package com.getaltair.altair.ui.today

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import com.getaltair.altair.shared.dto.guidance.EnergyBudgetResponse
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.bodySmall
import com.getaltair.altair.ui.theme.headingMedium
import com.getaltair.altair.ui.theme.components.AltairCard

/**
 * Energy Budget Card - Displays daily energy allocation and usage.
 *
 * Shows a visual progress bar with color gradient based on usage:
 * - Green (0-50%): Low usage, energy available
 * - Yellow (50-80%): Medium usage, caution
 * - Red (80-100%+): High usage or over budget
 *
 * @param energyBudget Energy budget data for today
 * @param modifier Optional modifier
 */
@Composable
fun EnergyBudgetCard(
    energyBudget: EnergyBudgetResponse,
    modifier: Modifier = Modifier
) {
    AltairCard(modifier = modifier) {
        Column(
            verticalArrangement = Arrangement.spacedBy(AltairTheme.spacing.sm)
        ) {
            // Header
            BasicText(
                text = "Today's Energy",
                style = AltairTheme.typography.headingMedium.copy(
                    color = AltairTheme.colors.textPrimary
                )
            )

            // Progress bar
            Box(
                modifier = Modifier
                    .fillMaxWidth()
                    .height(8.dp)
                    .clip(RoundedCornerShape(4.dp))
                    .background(AltairTheme.colors.surface)
            ) {
                val progressFraction = (energyBudget.spent.toFloat() / energyBudget.budget.toFloat())
                    .coerceIn(0f, 1f)

                val progressColor = getEnergyColor(energyBudget.percentUsed)

                Box(
                    modifier = Modifier
                        .fillMaxWidth(progressFraction)
                        .height(8.dp)
                        .clip(RoundedCornerShape(4.dp))
                        .background(
                            brush = Brush.horizontalGradient(
                                colors = listOf(progressColor, progressColor.copy(alpha = 0.7f))
                            )
                        )
                )
            }

            // Usage text
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                BasicText(
                    text = "${energyBudget.spent} / ${energyBudget.budget} energy used",
                    style = AltairTheme.typography.bodyMedium.copy(
                        color = AltairTheme.colors.textSecondary
                    )
                )

                val usagePercent = (energyBudget.percentUsed * 100).toInt()
                BasicText(
                    text = "$usagePercent%",
                    style = AltairTheme.typography.bodyMedium.copy(
                        color = getEnergyColor(energyBudget.percentUsed)
                    )
                )
            }

            // Remaining energy
            if (energyBudget.remaining > 0) {
                BasicText(
                    text = "${energyBudget.remaining} energy remaining",
                    style = AltairTheme.typography.bodySmall.copy(
                        color = AltairTheme.colors.textTertiary
                    )
                )
            } else if (energyBudget.remaining < 0) {
                BasicText(
                    text = "${-energyBudget.remaining} energy over budget",
                    style = AltairTheme.typography.bodySmall.copy(
                        color = AltairTheme.colors.error
                    )
                )
            } else {
                BasicText(
                    text = "Budget fully allocated",
                    style = AltairTheme.typography.bodySmall.copy(
                        color = AltairTheme.colors.warning
                    )
                )
            }
        }
    }
}

/**
 * Determines energy color based on usage percentage.
 *
 * @param percentUsed Usage as decimal (0.0-1.0+)
 * @return Color for the current usage level
 */
@Composable
private fun getEnergyColor(percentUsed: Float): Color {
    return when {
        percentUsed < 0.5f -> AltairTheme.colors.energy1  // Green - low usage
        percentUsed < 0.8f -> AltairTheme.colors.warning  // Yellow - medium usage
        else -> AltairTheme.colors.energy5                // Red - high/over budget
    }
}
