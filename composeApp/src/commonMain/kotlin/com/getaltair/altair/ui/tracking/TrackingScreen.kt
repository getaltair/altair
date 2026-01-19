package com.getaltair.altair.ui.tracking

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.BarChart
import androidx.compose.material3.Icon
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import com.getaltair.altair.ui.components.AltairSurface
import com.getaltair.altair.ui.components.AltairText
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.LocalAltairColors

/**
 * Tracking screen - metrics and habit management.
 *
 * This is a placeholder that will be expanded with:
 * - Habit tracking
 * - Metric visualization
 * - Progress charts
 * - Streak management
 */
@Composable
fun TrackingScreen(modifier: Modifier = Modifier) {
    val colors = LocalAltairColors.current

    AltairSurface(modifier = modifier.fillMaxSize()) {
        Column(
            modifier =
                Modifier
                    .fillMaxSize()
                    .padding(AltairTheme.Spacing.lg),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center,
        ) {
            Icon(
                imageVector = Icons.Filled.BarChart,
                contentDescription = null,
                tint = colors.accent,
                modifier = Modifier.padding(bottom = AltairTheme.Spacing.md),
            )
            AltairText(
                text = "Tracking",
                style = AltairTheme.Typography.headlineLarge,
                color = colors.textPrimary,
            )
            AltairText(
                text = "Metrics and habits to measure progress",
                style = AltairTheme.Typography.bodyLarge,
                color = colors.textSecondary,
                modifier = Modifier.padding(top = AltairTheme.Spacing.sm),
            )
            AltairText(
                text = "Coming soon",
                style = AltairTheme.Typography.labelMedium,
                color = colors.textTertiary,
                modifier = Modifier.padding(top = AltairTheme.Spacing.lg),
            )
        }
    }
}
