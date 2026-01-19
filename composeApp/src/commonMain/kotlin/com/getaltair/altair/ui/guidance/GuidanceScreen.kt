package com.getaltair.altair.ui.guidance

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Explore
import androidx.compose.material3.Icon
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import com.getaltair.altair.ui.components.AltairSurface
import com.getaltair.altair.ui.components.AltairText
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.LocalAltairColors

/**
 * Guidance screen - quest and goal management.
 *
 * This is a placeholder that will be expanded with:
 * - Quest list and creation
 * - Goal tracking
 * - Progress visualization
 * - Milestone management
 */
@Composable
fun GuidanceScreen(modifier: Modifier = Modifier) {
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
                imageVector = Icons.Filled.Explore,
                contentDescription = null,
                tint = colors.accent,
                modifier = Modifier.padding(bottom = AltairTheme.Spacing.md),
            )
            AltairText(
                text = "Guidance",
                style = AltairTheme.Typography.headlineLarge,
                color = colors.textPrimary,
            )
            AltairText(
                text = "Quests and goals to guide your path",
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
